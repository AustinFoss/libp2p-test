use libp2p::{
    core::{multiaddr::Protocol, Multiaddr, muxing::StreamMuxerBox, Transport},
    identify, identity::Keypair, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent, StreamProtocol},
    tcp, yamux,
};
use libp2p_webrtc as webrtc;
use libp2p::futures::StreamExt;
use libp2p_request_response::{
    ProtocolSupport, Config,
    cbor::Behaviour as RequestResponseBehaviour,
    Message,
    Event,
    cbor};

use std::{
    error::Error,
    net::Ipv4Addr,
};
use serde::{Deserialize, Serialize};


// Combined network behaviour
#[derive(NetworkBehaviour)]
struct Behaviour {
    // relay: relay::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    reqres: RequestResponseBehaviour<Ping, Pong>
}

pub async fn init_libp2p(_local_key: Keypair, _webrtc_cert: webrtc::tokio::certificate::Certificate, _port: u16) -> Result<(), Box<dyn Error>> {

    let reqres_proto = [(StreamProtocol::new("/reqres/0.0.1"), ProtocolSupport::Full)];
    println!("reqres registering protocols: {:?}", reqres_proto.iter().map(|(p, _)| p).collect::<Vec<_>>());

    let reqres = RequestResponseBehaviour::<Ping, Pong>::new(
        reqres_proto,
        Config::default(),
    );

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(_local_key)
    .with_tokio()
    .with_tcp(
        tcp::Config::default(),
        noise::Config::new,
        yamux::Config::default,
    )?
    .with_quic()
    .with_other_transport(|id_keys| {
        Ok(webrtc::tokio::Transport::new(
            id_keys.clone(),
            _webrtc_cert,
        )
        .map(|(peer_id, conn), _| (peer_id, StreamMuxerBox::new(conn))))
    })?
    .with_behaviour(|key| Behaviour {
        // relay: relay::Behaviour::new(key.public().to_peer_id(),
        //     Default::default()),
        ping: ping::Behaviour::new(ping::Config::new()),
        identify: identify::Behaviour::new(identify::Config::new(
            "/ipfs/id/1.0.0".to_string(),
            key.public(),
        )),
        reqres: reqres
    })?
    .build();

    // Listen on all interfaces
    let listen_addr_tcp = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Tcp(_port));
    swarm.listen_on(listen_addr_tcp)?;

    let listen_addr_quic = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Udp(_port))
        .with(Protocol::QuicV1);
    swarm.listen_on(listen_addr_quic)?;

    let address_webrtc = Multiaddr::from(Ipv4Addr::UNSPECIFIED)
        .with(Protocol::Udp(37385))
        .with(Protocol::WebRTCDirect);
    swarm.listen_on(address_webrtc.clone())?;

    loop {
        match swarm.next().await.expect("Infinite Stream.") {

            SwarmEvent::IncomingConnection { local_addr, send_back_addr, connection_id } => {
                println!("Incoming connection from {}", send_back_addr);
            }

            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("Connection closed with {}: {:?}", peer_id, cause);
            }

            SwarmEvent::Behaviour(BehaviourEvent::Reqres(
                Event::Message {
                    peer,
                    message: Message::Request { request, channel, .. },
                    connection_id: _
                },
            )) => {
                println!("Received Request");
            }

            // Event::Message {
            //     peer,
            //     message:
            //         request_response::Message::Request {
            //             request, channel, ..
            //         },
            //     ..
            // } => { }

            // SwarmEvent::Behaviour(BehaviourEvent::Reqres(Event::Message {
            //     peer,
            //     message,
            //     ..
            // })) => {
            //     match message {
            //         Message::Request {
            //             request,
            //             channel,
            //             request_id,
            //             ..
            //         } => {
            //             // println!(
            //             //     "Received Request from {} with request_id {:?}: {:?}",
            //             //     peer, request_id, request
            //             // );

            //             println!("PRP HTTP!!!");
            //             // Optionally, send a response back using the channel
            //             // For example:
            //             // swarm
            //             //     .behaviour_mut()
            //             //     .reqres
            //             //     .send_response(channel, Pong(request.0))
            //             //     .expect("Failed to send response");
            //         }
            //         Message::Response {
            //             response,
            //             request_id,
            //         } => {
            //             println!(
            //                 "Received Response from {} with request_id {:?}: {:?}",
            //                 peer, request_id, response
            //             );
            //         }
            //     }
            // }

            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                println!("Connected to {} at {:?}", peer_id, endpoint);
            }

            SwarmEvent::Behaviour(event) => {
                if let BehaviourEvent::Identify(identify::Event::Received {
                    info: identify::Info { observed_addr, .. },
                    ..
                }) = &event
                {
                    // swarm.add_external_address(observed_addr.clone());
                    println!("IDENTIFIED!!!!");
                }

                if let BehaviourEvent::Reqres(Event::Message {
                    ..
                }) = &event {
                    println!("P@P HTTP!!!");
                }

                if let BehaviourEvent::Reqres(Event::InboundFailure {
                    ..
                }) = &event {
                    println!("InboundFailure");
                }

                println!("{event:?}")            
            }

            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {address:?}");
            }
            _ => {}

        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Ping(Vec<u8>);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Pong(Vec<u8>);