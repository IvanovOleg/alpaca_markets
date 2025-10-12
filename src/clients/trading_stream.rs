use crate::config::AlpacaConfig;
use crate::models::{AlpacaError, AlpacaResult};
use crate::wss::common::{SerializationFormat, StreamType, WebSocketConnection};
use crate::wss::trading::{TradingSubscribeData, TradingSubscribeRequest, TradingWebSocketMessage};
use futures_util::StreamExt;
use serde_json;
use tokio_tungstenite::tungstenite::Message;

/// High-level trading stream client for easy subscription management
pub struct TradingStreamClient {
    config: AlpacaConfig,
    connection: Option<WebSocketConnection>,
    format: SerializationFormat,
}

impl TradingStreamClient {
    /// Create a new trading stream client with JSON format (default)
    pub fn new(config: AlpacaConfig) -> Self {
        Self::with_format(config, SerializationFormat::Json)
    }

    /// Create a new trading stream client with specified format
    pub fn with_format(config: AlpacaConfig, format: SerializationFormat) -> Self {
        Self {
            config,
            connection: None,
            format,
        }
    }

    /// Create a new trading stream client with MessagePack format
    pub fn new_msgpack(config: AlpacaConfig) -> Self {
        Self::with_format(config, SerializationFormat::MessagePack)
    }

    /// Connect to the trading WebSocket stream and authenticate
    pub async fn connect(&mut self) -> AlpacaResult<()> {
        // Get URL and connect
        let url = WebSocketConnection::get_url(&self.config, &StreamType::Trading);
        let mut connection =
            WebSocketConnection::connect(url, StreamType::Trading, self.format.clone()).await?;

        // Authenticate using connection method
        connection.authenticate(&self.config).await?;

        // Subscribe to all trading updates by default
        let subscribe_request = TradingSubscribeRequest {
            action: "listen".to_string(),
            data: TradingSubscribeData {
                streams: vec!["trade_updates".to_string(), "account_updates".to_string()],
            },
        };

        let subscribe_message = serde_json::to_value(&subscribe_request)?;
        connection.send_message(&subscribe_message).await?;

        self.connection = Some(connection);
        Ok(())
    }

    /// Connect with custom stream subscriptions
    pub async fn connect_with_streams(&mut self, streams: Vec<String>) -> AlpacaResult<()> {
        // Get URL and connect
        let url = WebSocketConnection::get_url(&self.config, &StreamType::Trading);
        let mut connection =
            WebSocketConnection::connect(url, StreamType::Trading, self.format.clone()).await?;

        // Authenticate using connection method
        connection.authenticate(&self.config).await?;

        // Subscribe to custom streams
        let subscribe_request = TradingSubscribeRequest {
            action: "listen".to_string(),
            data: TradingSubscribeData { streams },
        };

        let subscribe_message = serde_json::to_value(&subscribe_request)?;
        connection.send_message(&subscribe_message).await?;

        self.connection = Some(connection);
        Ok(())
    }

    /// Get the next message from the trading stream
    pub async fn next_message(&mut self) -> AlpacaResult<Option<TradingWebSocketMessage>> {
        if let Some(ref mut connection) = self.connection {
            if let Some(message) = connection.stream.next().await {
                let message = message.map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
                return self.handle_message(message).await;
            }
        }
        Ok(None)
    }

    /// Handle incoming trading messages
    async fn handle_message(
        &self,
        message: Message,
    ) -> AlpacaResult<Option<TradingWebSocketMessage>> {
        match message {
            Message::Close(_) => Err(AlpacaError::WebSocketError("Connection closed".to_string())),
            _ => {
                if let Some(ref connection) = self.connection {
                    match connection.deserialize_message(&message) {
                        Ok(parsed_message) => Ok(Some(parsed_message)),
                        Err(e) => {
                            eprintln!("Failed to parse trading message: {}", e);
                            Ok(None)
                        }
                    }
                } else {
                    Err(AlpacaError::WebSocketError("Not connected".to_string()))
                }
            }
        }
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    /// Disconnect from the stream
    pub async fn disconnect(&mut self) -> AlpacaResult<()> {
        if let Some(mut connection) = self.connection.take() {
            connection
                .stream
                .close(None)
                .await
                .map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
        }
        Ok(())
    }

    /// Subscribe to additional streams on an existing connection
    pub async fn subscribe_to_streams(&mut self, streams: Vec<String>) -> AlpacaResult<()> {
        if let Some(ref mut connection) = self.connection {
            let subscribe_request = TradingSubscribeRequest {
                action: "listen".to_string(),
                data: TradingSubscribeData { streams },
            };

            let subscribe_message = serde_json::to_value(&subscribe_request)?;
            connection.send_message(&subscribe_message).await?;
        } else {
            return Err(AlpacaError::WebSocketError("Not connected".to_string()));
        }
        Ok(())
    }
}

impl Drop for TradingStreamClient {
    fn drop(&mut self) {
        // Close the connection when the client is dropped
        if let Some(mut connection) = self.connection.take() {
            tokio::spawn(async move {
                let _ = connection.stream.close(None).await;
            });
        }
    }
}
