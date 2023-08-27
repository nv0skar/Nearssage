// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub static CONNECTION: OnceCell<Histogram> = OnceCell::const_new();
pub static CLIENT: OnceCell<Histogram> = OnceCell::const_new();

/// Connection handler which stores clients
#[derive(Clone, Default)]
pub struct Handler(AtomicRefCell<Arc<DashMap<SocketAddr, AtomicRefCell<Client>>>>);

impl Handler {
    /// Starts serving connections
    #[instrument(name = "server_run", skip(self))]
    pub async fn run(&self, addr: SocketAddr) -> Result<()> {
        let listener = UdpListener::bind(addr).await?;
        loop {
            self.to_owned().handle_connection(listener.accept().await?);
        }
    }

    /// Handles new connection
    #[instrument(
        name = "server_new_connection"
        skip_all,
        fields(
            peer_addr = %peer.1
    ))]
    fn handle_connection(mut self, peer: (UdpStream, SocketAddr)) {
        let (mut stream, _) = peer;
        let mut client = Option::default();
        let (sending_channel, receiving_channel) = bounded::<ServerSignal>(CLIENT_BUF_LEN);
        // let mut buf = NetworkBuf::new();
        tokio::spawn(async move {
            tracing::trace!("New connection");
            if let Some(s) = CONNECTION.get() {
                s.record(self.borrow().len() as f64);
            }
            loop {
                tokio::select! {
                    peer_receiver = self.handle_signal(&mut stream, &mut client, &sending_channel) => {
                        if let Err(err) = peer_receiver {
                            tracing::info!("Will close connection! {}", err.to_string());
                            self.close_connection(&mut stream, client.is_some()).await;
                            break;
                        }
                    }
                    _received = receiving_channel.recv_async() => {
                        tracing::debug!("Data received from client's receiver channel");
                    }
                }
            }
        });
    }

    /// Closes the peer's connection
    async fn close_connection(&mut self, stream: &mut UdpStream, client: bool) {
        tracing::info!("Connection closed");
        let _ = stream.shutdown().await;
        if client {
            self.remove_client(stream.peer_addr().unwrap()).await;
        }
    }

    /// Handle client's signals
    #[instrument(skip_all, err)]
    async fn handle_signal(
        &mut self,
        stream: &mut UdpStream,
        client: &mut Option<AtomicRefCell<Client>>,
        sending_channel: &Sender<ServerSignal>,
        // buf: &mut NetworkBuf,
    ) -> Result<()> {
        match ClientSignal::receive(
            stream,
            // buf.buffer(client.as_ref().map_or(false, |s| s.borrow().id().is_err())),
        )
        .await
        {
            Ok(req) => match req {
                Signal::Handshake(pk_exchange) => {
                    tracing::debug!("Handshake initialized");
                    let (new_client, gen_pk_exchange) =
                        Client::new(sending_channel.to_owned(), &pk_exchange.take().await?).await?;
                    tracing::trace!("New client");
                    self.new_client(stream.peer_addr()?, client.insert(new_client).clone())
                        .await;
                    if let Some(s) = CLIENT.get() {
                        s.record(self.borrow().len() as f64);
                    }
                    ServerSignal::Handshake(
                        Checksumed::new(
                            Signed::new(
                                IDENTITY.get().context("Identity is not set!")?,
                                gen_pk_exchange,
                            )
                            .await?,
                        )
                        .await?,
                    )
                    .send(stream)
                    .await?;
                    Ok(())
                }
                Signal::Subsignal(height, _) => {
                    let _client = client
                        .clone()
                        .context("Non-handshaked connection attempted to send subsignal")?;
                    let mut client = _client.borrow_mut();
                    match client.handle_subsignal(height, req).await? {
                        Some(resolution) => match resolution {
                            SubsignalHandlerResolution::Send(signal) => {
                                signal.send(stream).await?;
                                Ok(())
                            }
                            SubsignalHandlerResolution::Disconnect => {
                                self.close_connection(stream, true).await;
                                Ok(())
                            }
                        },
                        None => Ok(()),
                    }
                }
                Signal::Error(_) => Err(anyhow!("Stream invalidated by client!")),
            },
            Err(status) => Err(anyhow!(status)),
        }
    }

    /// Appends a new client to the pool
    #[instrument(level = "trace", skip(self, client))]
    async fn new_client(&self, addr: SocketAddr, client: AtomicRefCell<Client>) {
        self.borrow_mut().insert(addr, client);
    }

    /// Get client either by the user's id or the socket address
    #[instrument(level = "trace", skip(self))]
    async fn get_client(&self, find: Either<SocketAddr, UserID>) -> Option<AtomicRefCell<Client>> {
        let _client_pool = self.borrow_mut();
        find.either(
            |addr| _client_pool.get(&addr).map(|s| s.value().clone()),
            |user_id| {
                _client_pool
                    .iter()
                    .find(|s| {
                        s.value()
                            .borrow_mut()
                            .id()
                            .ok()
                            .map_or(false, |s| s == user_id)
                    })
                    .map(|s| s.value().clone())
            },
        )
    }

    /// Removes a client from the pool
    #[instrument(level = "trace", skip(self))]
    async fn remove_client(&self, addr: SocketAddr) {
        self.borrow_mut().remove(&addr);
    }
}

impl Deref for Handler {
    type Target = AtomicRefCell<Arc<DashMap<SocketAddr, AtomicRefCell<Client>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
