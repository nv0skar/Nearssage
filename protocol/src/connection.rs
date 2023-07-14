// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub static RECEIVING: OnceCell<Histogram> = OnceCell::const_new();
pub static SENDING: OnceCell<Histogram> = OnceCell::const_new();
pub static REQUEST: OnceCell<Counter> = OnceCell::const_new();
pub static MALFORMED_REQUEST: OnceCell<Counter> = OnceCell::const_new();
pub static UNFULFILLED_REQUEST: OnceCell<Counter> = OnceCell::const_new();

/// Received signals errors
#[derive(Error, Debug)]
pub enum ReceivedStatus {
    #[error("Malformed request")]
    MalformedRequest,
    #[error("Connection timed-out")]
    Timeout,
    #[error("IO Error")]
    IOError,
}

impl<
        T: Clone + PartialEq + Send + Sync + Decode + Serialize + DeserializeOwned,
        U: Clone + PartialEq + Send + Sync + Decode + Serialize + DeserializeOwned,
    > Signal<T, U>
{
    /// Receives data from peer, copies it into the buffer and deserializes it
    #[instrument(level = "trace", skip_all)]
    pub async fn receive(stream: &mut UdpStream, buf: &mut [u8]) -> Result<Self, ReceivedStatus> {
        match timeout(Duration::from_millis(CONNECTION_TIMEOUT), stream.read(buf)).await {
            Ok(received) => match received {
                Ok(len) => {
                    tracing::debug!("Received {} bytes", len);
                    if let Some(s) = REQUEST.get() {
                        s.increment(1);
                    }
                    if let Some(s) = RECEIVING.get() {
                        s.record(len as f64);
                    }
                    match Self::decode(&buf[0..len]).await {
                        Ok(req) => {
                            tracing::trace!("Request's data deserialized");
                            Ok(req)
                        }
                        Err(_) => {
                            tracing::warn!("Malformed request");
                            if let Some(s) = MALFORMED_REQUEST.get() {
                                s.increment(1);
                            }
                            Err(ReceivedStatus::MalformedRequest)
                        }
                    }
                }
                Err(_) => {
                    tracing::error!("IO Error");
                    Err(ReceivedStatus::IOError)
                }
            },
            Err(_) => {
                tracing::info!("Connection timed-out");
                Err(ReceivedStatus::Timeout)
            }
        }
    }
}

impl<
        T: Clone + PartialEq + Send + Sync + Encode + Serialize,
        U: Clone + PartialEq + Send + Sync + Encode + Serialize,
    > Signal<T, U>
{
    /// Sends signal to peer
    #[instrument(level = "trace", skip_all, err)]
    pub async fn send(&self, stream: &mut UdpStream) -> Result<usize> {
        let signal = self.encode().await?;
        let buf = signal.as_slice();
        match stream.write_all(buf).await {
            Ok(_) => {
                if let Some(s) = SENDING.get() {
                    s.record(buf.len() as f64);
                };
                Ok(buf.len())
            }
            Err(_) => Err(anyhow!("Failed to send data to peer!")),
        }
    }
}
