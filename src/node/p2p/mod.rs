pub mod error;
mod getters;
mod types;
mod utils;
// external
use {
    libp2p::{
        core::transport::upgrade,
        mdns,
        mplex::MplexConfig,
        noise::{NoiseConfig, X25519Spec},
        swarm::SwarmBuilder,
        tcp, PeerId, Swarm, Transport,
    },
    std::net::{IpAddr, SocketAddr},
};
// local
use crate::{
    ledger::general::{PbKey, KP},
    node::p2p::{error::P2PError, utils::ListResponse},
};

/// ## Peer-to-peer connection.
#[derive(Debug)]
pub struct P2P {
    socket_addr: SocketAddr,
    kp: KP,
}

impl P2P {
    /// ### Initialize a new peer-to-peer connection
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16, kp: KP) -> Self {
        Self {
            socket_addr: SocketAddr::new(host, port),
            kp,
        }
    }

    /////////////////////////////////////////////////////
    /////////////////////// ACTIONS /////////////////////
    /// ### Discover peer using public key.
    ///
    /// 1. Get `peer_id` from peer's public key
    /// 1. Connect to peer using `peer_id`
    pub fn discover_peer(&self, peer: &PbKey) {
        let _peer_id: PeerId = PeerId::from_public_key(&peer.to_owned().into());
        let (response_sender, mut response_rcv) =
            tokio::sync::mpsc::unbounded_channel::<ListResponse>();

        // set up the auth key for the transport
        // @todo implement from-conversion
        let kp = libp2p::identity::ed25519::Keypair::decode(&mut self.kp.to_bytes()).unwrap();
        let kp = libp2p::identity::Keypair::Ed25519(kp);
        let kp = libp2p::noise::Keypair::<X25519Spec>::new()
            .into_authentic(&kp)
            .expect("should be able to create authentic-key");

        // create transport:
        // 1. use tcp with tokio's async implementation
        // 2. once connection is established, secure the connection by upgrading with `noise` protocol
        // 3. use `NoiseConfig:xx` handshake pattern - its the only one compatible with libp2p
        // 4. multiplex the transport with `Mplex`
        let transport = libp2p::tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1) // V1Lazy
            .authenticate(NoiseConfig::xx(kp.clone()).into_authenticated())
            .multiplex(MplexConfig::new())
            .boxed();

        // define logic of network behaviour with `NetworkBehaviour`
        // 1. handle network behavior with `floodsub` - publish/subscribe to topics
        let mut floodsub_behaviour = libp2p::floodsub::Floodsub::new(self.peer_id());
        let topic = libp2p::floodsub::Topic::new("nodes");
        // 1.1 subscribe to `nodes` topic, to send and receive events
        floodsub_behaviour.subscribe(topic.clone());

        // 2. discover nodes with mdns protocol - will be changed in the future
        let mdns_behaviour = libp2p::mdns::tokio::Behaviour::new(mdns::Config::default()).unwrap();

        // 3. create `swarm` connection manager
        // this handles using the transport and executes network behavior
        // i.e. triggering and receiving events
        let mut swarm =
            SwarmBuilder::with_tokio_executor(transport, floodsub_behaviour, self.peer_id())
                .build();
        // let mut swarm = SwarmBuilder::with_executor(
        //     transport,
        //     floodsub_behaviour,
        //     self.peer_id(),
        //     Box::new(|fut| {
        //         tokio::spawn(fut);
        //     }),
        // )
        // .build();
        Swarm::listen_on(
            &mut swarm,
            format!(
                "/ip4/{}/tcp/{}",
                self.socket_addr().ip(),
                self.socket_addr().port()
            )
            .parse()
            .expect("should be able to get a local socket"),
        )
        .expect("should be able to start swarm");
    }
    pub async fn handle_list_peers() {}
    /////////////////////// ACTIONS /////////////////////
    /////////////////////////////////////////////////////
}
