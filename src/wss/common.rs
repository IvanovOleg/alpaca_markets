// WebSocket common utilities and helpers for Alpaca streams

use crate::config::AlpacaConfig;
use crate::models::{AlpacaError, AlpacaResult};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, connect_async_with_config,
    tungstenite::{
        Message, handshake::client::generate_key, http::Request, protocol::WebSocketConfig,
    },
};

/// WebSocket stream type for different endpoints
#[derive(Debug, Clone, PartialEq)]
pub enum StreamType {
    Trading,
    MarketData,
}

/// Message serialization format
#[derive(Debug, Clone, PartialEq)]
pub enum SerializationFormat {
    Json,
    MessagePack,
}

/// WebSocket connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Initial connection established, waiting for server messages
    Connected,
    /// Market data: received "connected" message, ready to authenticate
    ReadyToAuth,
    /// Authentication sent, waiting for response
    Authenticating,
    /// Fully authenticated and ready for data
    Authenticated,
    /// Connection failed or closed
    Disconnected,
}

/// Common WebSocket connection wrapper
pub struct WebSocketConnection {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub format: SerializationFormat,
    pub stream_type: StreamType,
    pub state: ConnectionState,
}

impl WebSocketConnection {
    /// Get the appropriate WebSocket URL for a stream type and config
    pub fn get_url(config: &AlpacaConfig, stream_type: &StreamType) -> &'static str {
        match stream_type {
            StreamType::Trading => {
                if config.is_paper {
                    "wss://paper-api.alpaca.markets/stream"
                } else {
                    "wss://api.alpaca.markets/stream"
                }
            }
            StreamType::MarketData => "wss://stream.data.alpaca.markets/v2/iex",
        }
    }

    /// Connect to a WebSocket endpoint with specified stream type and format
    pub async fn connect(
        url: &str,
        stream_type: StreamType,
        format: SerializationFormat,
    ) -> AlpacaResult<WebSocketConnection> {
        let stream = if stream_type == StreamType::MarketData
            && format == SerializationFormat::MessagePack
        {
            // Build request with proper WebSocket headers and Content-Type
            let key = generate_key();
            let request = Request::builder()
                .uri(url)
                .header(
                    "Host",
                    url.split("://")
                        .nth(1)
                        .unwrap_or("")
                        .split("/")
                        .next()
                        .unwrap_or(""),
                )
                .header("Upgrade", "websocket")
                .header("Connection", "upgrade")
                .header("Sec-WebSocket-Key", key)
                .header("Sec-WebSocket-Version", "13")
                .header("Content-Type", "application/msgpack")
                .header(
                    "Sec-WebSocket-Extensions",
                    "permessage-deflate; client_max_window_bits",
                )
                .body(())
                .map_err(|e| {
                    AlpacaError::WebSocketError(format!("Failed to build request: {}", e))
                })?;

            let config = WebSocketConfig::default();
            let (stream, _) = connect_async_with_config(request, Some(config), false)
                .await
                .map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
            stream
        } else {
            // Standard connection for trading streams and market data with JSON
            let (stream, _) = connect_async(url)
                .await
                .map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
            stream
        };

        Ok(WebSocketConnection {
            stream,
            format,
            stream_type,
            state: ConnectionState::Connected,
        })
    }

    /// Authenticate WebSocket connection using state-based message handling
    pub async fn authenticate(&mut self, config: &AlpacaConfig) -> AlpacaResult<()> {
        // For trading streams, send auth immediately without waiting for initial message
        if self.stream_type == StreamType::Trading && self.state == ConnectionState::Connected {
            let auth_request = serde_json::json!({
                "action": "auth",
                "key": config.api_key,
                "secret": config.secret_key
            });
            self.send_message(&auth_request).await?;
            self.state = ConnectionState::Authenticating;
        }
        // For market data streams, wait for "connected" message first

        while self.state != ConnectionState::Authenticated {
            // If market data stream becomes ready to auth, send auth immediately
            if self.stream_type == StreamType::MarketData
                && self.state == ConnectionState::ReadyToAuth
            {
                let auth_message = serde_json::json!({
                    "action": "auth",
                    "key": config.api_key,
                    "secret": config.secret_key
                });
                self.send_message(&auth_message).await?;
                self.state = ConnectionState::Authenticating;
                continue; // Continue to next iteration without waiting for message
            }

            if let Some(message) = self.stream.next().await {
                let message = message.map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
                self.handle_message(message).await?;
            } else {
                return Err(AlpacaError::WebSocketError(
                    "Connection closed during authentication".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Handle incoming WebSocket message based on connection state
    pub async fn handle_message(&mut self, message: Message) -> AlpacaResult<Option<Value>> {
        let parsed_message: Value = self.parse_message(&message)?;

        match (&self.stream_type, &self.state) {
            // Trading stream: this shouldn't happen as auth is sent immediately in authenticate()
            (StreamType::Trading, ConnectionState::Connected) => Err(AlpacaError::WebSocketError(
                "Trading stream should not receive messages in Connected state".to_string(),
            )),

            // Trading stream: check authentication response
            (StreamType::Trading, ConnectionState::Authenticating) => {
                if let (Some(stream), Some(data)) =
                    (parsed_message.get("stream"), parsed_message.get("data"))
                {
                    if stream == "authorization" {
                        if let Some(status) = data.get("status") {
                            if status == "authorized" {
                                self.state = ConnectionState::Authenticated;
                                return Ok(None); // Auth success, don't return this message
                            }
                        }
                    }
                }
                Err(AlpacaError::AuthenticationError)
            }

            // Market data stream: wait for "connected" message
            (StreamType::MarketData, ConnectionState::Connected) => {
                if let Some(array) = parsed_message.as_array() {
                    if let Some(first) = array.first() {
                        if let (Some(msg_type), Some(msg)) = (first.get("T"), first.get("msg")) {
                            if msg_type == "success" && msg.as_str() == Some("connected") {
                                self.state = ConnectionState::ReadyToAuth;
                                return Ok(None); // Don't return the connected message
                            }
                        }
                    }
                }
                Err(AlpacaError::WebSocketError(
                    "Expected 'connected' message".to_string(),
                ))
            }

            // Market data stream: this shouldn't happen, auth should be sent immediately when ReadyToAuth
            (StreamType::MarketData, ConnectionState::ReadyToAuth) => {
                Err(AlpacaError::WebSocketError(
                    "Market data stream should not receive messages in ReadyToAuth state"
                        .to_string(),
                ))
            }

            // Market data stream: check authentication response
            (StreamType::MarketData, ConnectionState::Authenticating) => {
                if let Some(array) = parsed_message.as_array() {
                    if let Some(first) = array.first() {
                        if let (Some(msg_type), Some(msg)) = (first.get("T"), first.get("msg")) {
                            if msg_type == "success" && msg.as_str() == Some("authenticated") {
                                self.state = ConnectionState::Authenticated;
                                return Ok(None); // Auth success, don't return this message
                            }
                        }
                    }
                }
                Err(AlpacaError::AuthenticationError)
            }

            // Authenticated state: return actual data messages
            (_, ConnectionState::Authenticated) => Ok(Some(parsed_message)),

            // Disconnected state
            (_, ConnectionState::Disconnected) => Err(AlpacaError::WebSocketError(
                "Connection is disconnected".to_string(),
            )),

            // Invalid state combinations
            (StreamType::Trading, ConnectionState::ReadyToAuth) => {
                Err(AlpacaError::WebSocketError(
                    "Trading streams don't use ReadyToAuth state".to_string(),
                ))
            }
        }
    }

    /// Parse a WebSocket message based on connection format
    pub fn parse_message(&self, message: &Message) -> AlpacaResult<Value> {
        match (&self.stream_type, message) {
            // Trading streams always use binary-encoded JSON
            (StreamType::Trading, Message::Binary(bytes)) => {
                let text = String::from_utf8(bytes.to_vec()).map_err(|e| {
                    AlpacaError::SerializationError(format!(
                        "Invalid UTF-8 in binary message: {}",
                        e
                    ))
                })?;
                serde_json::from_str(&text)
                    .map_err(|e| AlpacaError::SerializationError(e.to_string()))
            }
            // Market data streams use MessagePack
            (StreamType::MarketData, Message::Binary(bytes)) => {
                #[cfg(feature = "websocket")]
                {
                    rmp_serde::from_slice(bytes)
                        .map_err(|e| AlpacaError::SerializationError(e.to_string()))
                }
                #[cfg(not(feature = "websocket"))]
                {
                    Err(AlpacaError::WebSocketError(
                        "MessagePack feature not enabled".to_string(),
                    ))
                }
            }
            // Unsupported combinations
            _ => Err(AlpacaError::SerializationError(format!(
                "Unsupported message type for stream: stream_type={:?}, message_type={:?}",
                self.stream_type,
                match message {
                    Message::Text(_) => "Text",
                    Message::Binary(_) => "Binary",
                    _ => "Other",
                }
            ))),
        }
    }

    /// Send a message to the WebSocket using the connection's format and stream type
    pub async fn send_message(&mut self, message: &serde_json::Value) -> AlpacaResult<()> {
        match &self.stream_type {
            // Trading streams always use binary-encoded JSON
            StreamType::Trading => {
                let json_text = serde_json::to_string(message)?;
                let bytes = json_text.into_bytes();
                self.stream
                    .send(Message::Binary(bytes.into()))
                    .await
                    .map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
            }
            // Market data streams use MessagePack
            StreamType::MarketData => {
                #[cfg(feature = "websocket")]
                {
                    let bytes = rmp_serde::to_vec(message)
                        .map_err(|e| AlpacaError::SerializationError(e.to_string()))?;
                    self.stream
                        .send(Message::Binary(bytes.into()))
                        .await
                        .map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
                }
                #[cfg(not(feature = "websocket"))]
                {
                    return Err(AlpacaError::WebSocketError(
                        "MessagePack feature not enabled".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Deserialize a message from WebSocket using the connection's format and stream type
    pub fn deserialize_message<T>(&self, message: &Message) -> AlpacaResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        match (&self.stream_type, message) {
            // Trading streams always use binary-encoded JSON
            (StreamType::Trading, Message::Binary(bytes)) => {
                let text = String::from_utf8(bytes.to_vec()).map_err(|e| {
                    AlpacaError::SerializationError(format!(
                        "Invalid UTF-8 in binary message: {}",
                        e
                    ))
                })?;
                serde_json::from_str(&text)
                    .map_err(|e| AlpacaError::SerializationError(e.to_string()))
            }
            // Market data streams use MessagePack
            (StreamType::MarketData, Message::Binary(bytes)) => {
                #[cfg(feature = "websocket")]
                {
                    rmp_serde::from_slice(bytes)
                        .map_err(|e| AlpacaError::SerializationError(e.to_string()))
                }
                #[cfg(not(feature = "websocket"))]
                {
                    Err(AlpacaError::WebSocketError(
                        "MessagePack feature not enabled".to_string(),
                    ))
                }
            }
            // Unsupported combinations
            _ => Err(AlpacaError::SerializationError(format!(
                "Unsupported message type for stream: stream_type={:?}, message_type={:?}",
                self.stream_type,
                match message {
                    Message::Text(_) => "Text",
                    Message::Binary(_) => "Binary",
                    _ => "Other",
                }
            ))),
        }
    }
}
