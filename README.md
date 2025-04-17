# LibP2P Custom Proto. Test

This repo contains a Rust Libp2p node that was originally based off the [rust relay-server example](https://github.com/libp2p/rust-libp2p/tree/master/examples/relay-server) as well as a browser node from the [js-libp2p custom protocols example](https://github.com/libp2p/js-libp2p-examples/tree/main/examples/js-libp2p-example-custom-protocols). I disabled the relay server protocol in the rust node but left `ping` & `identify` protocols available in both.

I added WebRTC support so that it could talk to a browser node. My browser node is found in `/src/browser_node/` and is an Astro project with the libp2p node running from a SolidJS JSX component.

After installing the rust dependencies and the running the node you'll see the PeerId and multiaddrs printed to the console:

```bash
cargo run

...

Peer ID: 12D3KooWJP2jGS3mi4VhutFBFc1NUjyZBhwiEk8eFHJD9vwocKbh
reqres registering protocols: ["/reqres/0.0.1"]
Listening on /ip4/127.0.0.1/tcp/46603
Listening on /ip4/10.0.0.167/tcp/46603
Listening on /ip4/10.0.2.2/tcp/46603
Listening on /ip4/192.168.122.1/tcp/46603
Listening on /ip4/10.10.144.2/tcp/46603
Listening on /ip4/127.0.0.1/udp/35423/quic-v1
Listening on /ip4/10.0.0.167/udp/35423/quic-v1
Listening on /ip4/10.0.2.2/udp/35423/quic-v1
Listening on /ip4/192.168.122.1/udp/35423/quic-v1
Listening on /ip4/10.10.144.2/udp/35423/quic-v1
Listening on /ip4/127.0.0.1/udp/37385/webrtc-direct/certhash/uEiC-d6DrX5z5xu17eJQ715RsQvvphwA0TSvBEAeWXZBKlQ
Listening on /ip4/10.0.0.167/udp/37385/webrtc-direct/certhash/uEiC-d6DrX5z5xu17eJQ715RsQvvphwA0TSvBEAeWXZBKlQ
```

It saves a peerId keypair as a pem file and webrtc cert in a protobuf format to the `/crypto` directory so it doesn't keep changing every time you restart the rust node. You can take the last multiaddr and the peerid and use them for the default values found in `/src/browser_node/src/components/Libp2p.jsx`.

```js
const [getMultiaddr, setMultiAddr] = createSignal('/ip4/10.0.0.167/udp/37385/webrtc-direct/certhash/uEiC-d6DrX5z5xu17eJQ715RsQvvphwA0TSvBEAeWXZBKlQ');
const [getRemotePeerId, setRemotePeerId] = createSignal('12D3KooWJP2jGS3mi4VhutFBFc1NUjyZBhwiEk8eFHJD9vwocKbh');
```

Then cd into the browser_node directory, install deps, and run the dev server:

```bash
cd src/browser_node && pnpm i && pnpm dev
```

You can then click the `Dial Remote Custom Proto` button from the UI and should see feedback from both the browser's dev console and the rust node cli logs.

## WIP

Unfortunately what happens right now is the `ping` and `identify` protocols are working as there will be 3 `BehaviourEvent`'s outputed from the rust node. `Ping` shows the milliseconds for the ping upon receiving the connection from the browser, and `Identify` shows a bunch of information including the `/reqres/0.0.1` custom protocol being registered, as well as a response from that protocol.

When I only use the `reqres` protocol there's no beheviour events that are registered. Which is really strange because the comiler throws an error when I adjust "Reqres" in the lines that contain `SwarmEvent::Behaviour(BehaviourEvent::Reqres(...` showing that the protocol is being processed as part of the larger behaviour enum, but maybe the events actually being emitted by the `reqres` behaviour aren't actually of the type `SwarmEvent::Behaviour`? Maybe?

In the browser node it reports the rust node's peer info from the peer store and also shows the `/reqres/0.0.1` protocol as registered, the message trying to be delivered, and then an error `Stream Err: The operation was aborted`. This error is fired before the rust node reports that the connection is closed after a few seconds.

Something isn't letting the custom protocol to be recognized on the Rust node's side of the connection.