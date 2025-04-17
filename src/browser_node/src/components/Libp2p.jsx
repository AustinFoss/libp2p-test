import { createLibp2p } from 'libp2p';
import { peerIdFromString } from '@libp2p/peer-id'
import { webTransport } from '@libp2p/webtransport';
import { webRTCDirect } from '@libp2p/webrtc'
import { noise } from '@chainsafe/libp2p-noise';
import { yamux } from '@chainsafe/libp2p-yamux';
import { multiaddr } from '@multiformats/multiaddr';
import { identify } from '@libp2p/identify'
import { ping } from '@libp2p/ping';
import * as cbor from 'cborg';

import { lpStream } from 'it-length-prefixed-stream'

import { createSignal } from 'solid-js';

let node = await createLibp2p({
    transports: [webTransport(), webRTCDirect()],
    connectionEncryption: [noise()],
    streamMuxers: [yamux()],
    connectionGater: {
        denyDialMultiaddr: () => {
            // by default we refuse to dial local addresses from browsers since they
            // are usually sent by remote peers broadcasting undialable multiaddrs and
            // cause errors to appear in the console but in this example we are
            // explicitly connecting to a local node so allow all addresses
            return false
        }
    },  
    connectionManager: {
        debug: true,
    },
    services: {
        identifyPush: identify(),
        ping: ping()
    }
});
    
const P2PHTTPProtocol = "/reqres/0.0.1";

node.handle(P2PHTTPProtocol, ({ stream }) => {
    console.log('Handling custom proto: ', P2PHTTPProtocol);
})

export default () => {

    const [getMultiaddr, setMultiAddr] = createSignal('/ip4/10.0.0.167/udp/37385/webrtc-direct/certhash/uEiC-d6DrX5z5xu17eJQ715RsQvvphwA0TSvBEAeWXZBKlQ');
    const [getRemotePeerId, setRemotePeerId] = createSignal('12D3KooWJP2jGS3mi4VhutFBFc1NUjyZBhwiEk8eFHJD9vwocKbh');

    const dial_go_node = async (targetHost) => {

        console.log(getMultiaddr());
        console.log(getRemotePeerId());

        try {

            const serverAddr = multiaddr(getMultiaddr()  + '/p2p/' + getRemotePeerId())
            
            // Open a stream with the p2phttp protocol
            const stream = await node.dialProtocol(serverAddr , P2PHTTPProtocol);

            console.log(await node.peerStore.get(peerIdFromString(getRemotePeerId())))
        
            // Construct an HTTP GET request
            // Construct and log the exact request string
            // const requestStr = 'GET /hello HTTP/1.1\r\n' +
            //                 'Host: ' + targetHost + '\r\n' +
            //                 'Connection: close\r\n' +
            //                 '\r\n';
            const requestStr = { message: 'Message'}

            console.log("Raw request string\n", requestStr)
            
            const request = new TextEncoder().encode(JSON.stringify(requestStr));
            // const request = cbor.encode(requestStr)

            // Write the request to the stream
            // await stream.sink([request]);

            const lp = lpStream(stream)

            // send the query
            await lp.write(request)
        
            // Read the response
            const res = await lp.read()
            const output = JSON.parse(new TextDecoder().decode(res.subarray()))
            
        
            console.log(`Response from ${targetHost}:`, response);    


        } catch (err) {
            console.log("Stream Err: " + err.message);        
        }
        
            
    }



    return <>

    <p>Remote Node Peer ID: </p>
        <input 
            type="text"
            value={getRemotePeerId()}
            onInput={(e) => setRemotePeerId(e.currentTarget.value)}
        />
        <br />
        <br />

        <p>Remote Node Multi Addr: </p>
        <input 
            type="text"
            value={getMultiaddr()}
            onInput={(e) => setMultiAddr(e.currentTarget.value)}
        />
        <br />
        <br />
        <button onClick={() => dial_go_node("localhost:43110")}>Dial Remote Custom Proto</button>

    </>;

}
