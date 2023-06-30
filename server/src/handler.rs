// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Connection handler which stores clients
#[derive(Clone, Default)]
pub struct Handler(AtomicRefCell<Arc<DashMap<SocketAddr, AtomicRefCell<Client>>>>);

impl Handler {
    /// Starts serving connections
    #[instrument(
        name = "server_run"
        skip(self),
        fields(
            listen_addr = %CONFIG.get().unwrap().serve_addr
    ))]
    pub async fn run(&self) -> Result<()> {
        let listener = UdpListener::bind(CONFIG.get().unwrap().serve_addr).await?;
        loop {
            self.connection(listener.accept().await?);
        }
    }

    /// Handles new connection
    #[instrument(
        name = "server_new_connection"
        skip(self),
        fields(
            peer_addr = %peer.1
    ))]
    pub fn connection(&self, peer: (UdpStream, SocketAddr)) {
        let (mut stream, peer_addr) = peer;
        let mut this: Handler = self.clone();
        let mut buf = NetworkBuf::new();
        tokio::spawn(async move {
            let (client, receiver) = Client::new();
            this.new_client(peer_addr, client.clone()).await;
            tracing::trace!("Connection added to pool");
            analytics::metrics::CLIENT.get().and_then(|s| {
                s.record(this.0.borrow().len() as f64);
                Some(())
            });
            loop {
                let mut client = client.borrow_mut();
                tokio::select! {
                    peer_receiver = timeout(
                        Duration::from_millis(CONNECTION_TIMEOUT),
                        client.receive(&mut stream, &mut buf),
                    ) => {
                        match peer_receiver.ok() {
                            Some(_) => (),
                            None => {
                                let _signal = client.build_signal(Subsignal::Disconnect).await.unwrap();
                                client.send(&mut stream, _signal).await;
                                this.close_connection(stream).await;
                                break;
                            }
                        }
                    }
                    _receiver_channel = receiver.recv_async() => {
                        tracing::debug!("Data received from client's receiver channel");
                    }
                }
            }
        });
    }

    /// Receives a stream of bytes, deserializes it into the buffer and handles signals
    pub async fn handle(&mut self, stream: &mut UdpStream, buf: &mut NetworkBuf) {}

    /// Closes the peer's connection
    pub async fn close_connection(&mut self, stream: UdpStream) {
        tracing::info!("Connection closed");
        stream.shutdown();
        self.remove_client(stream.peer_addr().unwrap()).await;
    }

    /// Appends a new client to the pool
    #[instrument(
        level = "trace",
        skip(self, client),
        fields(
            addr = %addr
    ))]
    pub async fn new_client(&self, addr: SocketAddr, client: AtomicRefCell<Client>) {
        self.0.borrow_mut().insert(addr, client);
    }

    /// Get client either by the user's id or the socket address
    #[instrument(
        level = "trace",
        skip(self),
        fields(
            find = %find
    ))]
    pub async fn get_client(
        &self,
        find: Either<SocketAddr, UserID>,
    ) -> Option<AtomicRefCell<Client>> {
        find.either(
            |addr| self.0.borrow_mut().get(&addr).map(|s| s.value().clone()),
            |user_id| {
                self.0
                    .borrow_mut()
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
    #[instrument(
        level = "trace",
        skip(self),
        fields(
            addr = %addr
    ))]
    pub async fn remove_client(&self, addr: SocketAddr) {
        self.0.borrow_mut().remove(&addr);
    }
}
