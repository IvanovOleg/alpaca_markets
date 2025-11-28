use crate::config::AlpacaConfig;
use crate::models::{AlpacaError, AlpacaResult};
use crate::wss::common::{RunOptions, SerializationFormat, StreamType, WebSocketConnection};
use crate::wss::trading::{TradingSubscribeData, TradingSubscribeRequest, TradingWebSocketMessage};
use futures_util::StreamExt;
use serde_json;
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

/// High-level trading stream client for easy subscription management
#[derive(Debug)]
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
            // Handle control frames - these are normal and should be ignored
            Message::Ping(_) => {
                println!("🏓 Received Ping (keepalive)");
                Ok(None)
            }
            Message::Pong(_) => {
                println!("🏓 Received Pong (keepalive)");
                Ok(None)
            }
            Message::Frame(_) => {
                println!("📦 Received raw Frame (skipping)");
                Ok(None)
            }
            _ => {
                if let Some(ref connection) = self.connection {
                    match connection.deserialize_message(&message) {
                        Ok(parsed_message) => Ok(Some(parsed_message)),
                        Err(e) => {
                            // Log the raw message for debugging
                            if let Message::Text(text) = &message {
                                eprintln!("Failed to parse trading message: {}", e);
                                eprintln!("📝 Raw message: {}", text);
                            } else if let Message::Binary(data) = &message {
                                eprintln!("Failed to parse trading message: {}", e);
                                eprintln!("📦 Binary message: {} bytes", data.len());
                            }
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

    /// Run the client with a message handler (convenience method)
    ///
    /// This method handles:
    /// - Automatic reconnection on connection loss
    /// - Graceful shutdown on Ctrl+C
    /// - Error recovery and logging
    /// - Message loop management
    ///
    /// # Example
    /// ```rust
    /// client.run(|message| async move {
    ///     match message {
    ///         TradingWebSocketMessage::TradeUpdate(update) => {
    ///             println!("Order {} is now {}", update.order.id, update.order.status);
    ///         }
    ///         _ => {}
    ///     }
    ///     Ok(())
    /// }).await?;
    /// ```
    pub async fn run<F, Fut>(&mut self, handler: F) -> AlpacaResult<()>
    where
        F: FnMut(TradingWebSocketMessage) -> Fut,
        Fut: std::future::Future<Output = AlpacaResult<()>>,
    {
        self.run_with_options(handler, RunOptions::default()).await
    }

    /// Run with configuration options
    ///
    /// Provides full control over reconnection behavior, timeouts, and error handling.
    ///
    /// # Example
    /// ```rust
    /// let options = RunOptions {
    ///     auto_reconnect: true,
    ///     max_reconnect_attempts: 10,
    ///     reconnect_delay_ms: 2000,
    ///     stop_on_handler_error: true,
    ///     timeout_secs: 300, // 5 minutes
    ///     verbose: true,
    /// };
    ///
    /// client.run_with_options(|message| async move {
    ///     handle_message(message).await
    /// }, options).await?;
    /// ```
    pub async fn run_with_options<F, Fut>(
        &mut self,
        mut handler: F,
        options: RunOptions,
    ) -> AlpacaResult<()>
    where
        F: FnMut(TradingWebSocketMessage) -> Fut,
        Fut: std::future::Future<Output = AlpacaResult<()>>,
    {
        let mut reconnect_attempts = 0;
        let start_time = std::time::Instant::now();

        if options.verbose {
            println!("🚀 Starting trading stream with options: {:#?}", options);
        }

        loop {
            tokio::select! {
                // Handle incoming messages
                message_result = self.next_message() => {
                    match message_result {
                        Ok(Some(message)) => {
                            reconnect_attempts = 0; // Reset on successful message
                            if let Err(e) = handler(message).await {
                                if options.stop_on_handler_error {
                                    if options.verbose {
                                        eprintln!("❌ Handler error, stopping: {}", e);
                                    }
                                    return Err(e);
                                }
                                if options.verbose {
                                    eprintln!("⚠️ Handler error (continuing): {}", e);
                                }
                            }
                        }
                        Ok(None) => {
                            // No message, continue
                            continue;
                        }
                        Err(e) => {
                            if options.auto_reconnect && reconnect_attempts < options.max_reconnect_attempts {
                                reconnect_attempts += 1;
                                let delay = options.reconnect_delay_ms * (1_u64 << reconnect_attempts.min(5));

                                if options.verbose {
                                    eprintln!("🔄 Connection lost, attempting reconnection #{} in {}ms: {}",
                                             reconnect_attempts, delay, e);
                                }

                                tokio::time::sleep(Duration::from_millis(delay)).await;

                                if let Err(reconnect_err) = self.connect().await {
                                    if options.verbose {
                                        eprintln!("❌ Reconnection failed: {}", reconnect_err);
                                    }
                                    continue;
                                }

                                if options.verbose {
                                    println!("✅ Reconnected successfully");
                                }
                            } else {
                                if options.verbose {
                                    eprintln!("❌ Connection error (max retries reached): {}", e);
                                }
                                return Err(e);
                            }
                        }
                    }
                }

                // Handle graceful shutdown
                _ = tokio::signal::ctrl_c() => {
                    if options.verbose {
                        println!("👋 Shutdown signal received, disconnecting...");
                    }
                    let _ = self.disconnect().await;
                    return Ok(());
                }

                // Optional timeout
                _ = tokio::time::sleep(Duration::from_secs(options.timeout_secs)), if options.timeout_secs > 0 => {
                    if options.verbose {
                        println!("⏰ Timeout reached after {}s", start_time.elapsed().as_secs());
                    }
                    let _ = self.disconnect().await;
                    return Ok(());
                }
            }
        }
    }

    /// Run with a channel-based approach for decoupled processing
    ///
    /// This method sends all messages to the provided channel and handles
    /// the WebSocket connection management automatically.
    ///
    /// # Example
    /// ```rust
    /// let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    ///
    /// tokio::spawn(async move {
    ///     client.run_with_channel(tx).await
    /// });
    ///
    /// while let Some(message) = rx.recv().await {
    ///     handle_message(message).await;
    /// }
    /// ```
    pub async fn run_with_channel(
        &mut self,
        sender: tokio::sync::mpsc::UnboundedSender<TradingWebSocketMessage>,
    ) -> AlpacaResult<()> {
        self.run(|message| {
            let sender = sender.clone();
            async move {
                if sender.send(message).is_err() {
                    return Err(AlpacaError::WebSocketError("Channel closed".to_string()));
                }
                Ok(())
            }
        })
        .await
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
