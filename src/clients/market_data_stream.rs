use crate::config::AlpacaConfig;
use crate::models::{AlpacaError, AlpacaResult};
use crate::wss::common::{SerializationFormat, StreamType, WebSocketConnection};
use crate::wss::market_data::MarketDataMessage;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;

/// Market data feed types
#[derive(Debug, Clone)]
pub enum Feed {
    /// IEX feed (free tier)
    Iex,
    /// SIP feed (paid tier)  
    Sip,
    /// Options feed
    Options,
    /// Crypto feed
    Crypto,
    /// News feed
    News,
}

impl Feed {
    /// Get the WebSocket URL for this feed
    pub fn url(&self) -> &'static str {
        match self {
            Feed::Iex => "wss://stream.data.alpaca.markets/v2/iex",
            Feed::Sip => "wss://stream.data.alpaca.markets/v2/sip",
            Feed::Options => "wss://stream.data.alpaca.markets/v1beta1/options",
            Feed::Crypto => "wss://stream.data.alpaca.markets/v1beta3/crypto/us",
            Feed::News => "wss://stream.data.alpaca.markets/v1beta1/news",
        }
    }
}

/// High-level market data stream client (MessagePack only)
pub struct MarketDataStreamClient {
    config: AlpacaConfig,
    connection: Option<WebSocketConnection>,
    feed: Feed,
}

impl MarketDataStreamClient {
    /// Create a new market data stream client (MessagePack format only)
    pub fn new(config: AlpacaConfig, feed: Feed) -> Self {
        Self {
            config,
            connection: None,
            feed,
        }
    }

    /// Connect to the market data WebSocket stream and authenticate (MessagePack only)
    pub async fn connect(&mut self) -> AlpacaResult<()> {
        let url = self.feed.url();
        let mut connection = WebSocketConnection::connect(
            url,
            StreamType::MarketData,
            SerializationFormat::MessagePack,
        )
        .await?;

        // Authenticate using connection method
        connection.authenticate(&self.config).await?;

        self.connection = Some(connection);
        Ok(())
    }

    /// Subscribe to multiple data types at once
    pub async fn subscribe(
        &mut self,
        trades: Option<&[&str]>,
        quotes: Option<&[&str]>,
        bars: Option<&[&str]>,
    ) -> AlpacaResult<()> {
        let mut subscribe_message = serde_json::json!({
            "action": "subscribe"
        });

        if let Some(trades) = trades {
            subscribe_message["trades"] = serde_json::Value::Array(
                trades
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(quotes) = quotes {
            subscribe_message["quotes"] = serde_json::Value::Array(
                quotes
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(bars) = bars {
            subscribe_message["bars"] = serde_json::Value::Array(
                bars.iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(ref mut connection) = self.connection {
            connection.send_message(&subscribe_message).await?;
        } else {
            return Err(AlpacaError::WebSocketError("Not connected".to_string()));
        }
        Ok(())
    }

    /// Unsubscribe from multiple data types at once
    pub async fn unsubscribe(
        &mut self,
        trades: Option<&[&str]>,
        quotes: Option<&[&str]>,
        bars: Option<&[&str]>,
    ) -> AlpacaResult<()> {
        let mut unsubscribe_message = serde_json::json!({
            "action": "unsubscribe"
        });

        if let Some(trades) = trades {
            unsubscribe_message["trades"] = serde_json::Value::Array(
                trades
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(quotes) = quotes {
            unsubscribe_message["quotes"] = serde_json::Value::Array(
                quotes
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(bars) = bars {
            unsubscribe_message["bars"] = serde_json::Value::Array(
                bars.iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            );
        }

        if let Some(ref mut connection) = self.connection {
            connection.send_message(&unsubscribe_message).await?;
        } else {
            return Err(AlpacaError::WebSocketError("Not connected".to_string()));
        }
        Ok(())
    }

    /// Get the next message from the market data stream
    pub async fn next_message(&mut self) -> AlpacaResult<Option<Vec<MarketDataMessage>>> {
        if let Some(ref mut connection) = self.connection {
            if let Some(message) = connection.stream.next().await {
                let message = message.map_err(|e| AlpacaError::WebSocketError(e.to_string()))?;
                return self.handle_message(message).await;
            }
        }
        Ok(None)
    }

    /// Handle incoming market data messages (array format)
    async fn handle_message(
        &self,
        message: Message,
    ) -> AlpacaResult<Option<Vec<MarketDataMessage>>> {
        match &message {
            Message::Text(_) | Message::Binary(_) => {
                if let Some(ref connection) = self.connection {
                    // Market data comes in array format: [{"T": "t", ...}, {"T": "q", ...}]
                    match connection.deserialize_message::<Vec<MarketDataMessage>>(&message) {
                        Ok(messages) => Ok(Some(messages)),
                        Err(e) => {
                            // Log the error but don't fail - might be control messages
                            eprintln!(
                                "Failed to parse market data message: {} - Message type: {:?}",
                                e, message
                            );
                            Ok(None)
                        }
                    }
                } else {
                    Err(AlpacaError::WebSocketError("Not connected".to_string()))
                }
            }
            Message::Close(_) => Err(AlpacaError::WebSocketError("Connection closed".to_string())),
            _ => Ok(None), // Ignore other message types
        }
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    /// Get the current feed being used
    pub fn feed(&self) -> &Feed {
        &self.feed
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
}

impl Drop for MarketDataStreamClient {
    fn drop(&mut self) {
        // Close the connection when the client is dropped
        if let Some(mut connection) = self.connection.take() {
            tokio::spawn(async move {
                let _ = connection.stream.close(None).await;
            });
        }
    }
}
