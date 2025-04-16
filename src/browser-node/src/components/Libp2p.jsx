import { createLibp2p } from 'libp2p';
import { webTransport } from '@libp2p/webtransport';
import { webRTCDirect } from '@libp2p/webrtc'
import { noise } from '@chainsafe/libp2p-noise';
import { yamux } from '@chainsafe/libp2p-yamux';
import { multiaddr } from '@multiformats/multiaddr';
import { identify } from '@libp2p/identify'
import { ping } from '@libp2p/ping';
import * as cbor from 'cborg';

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


export default () => {

        // const serverAddrStr = '/ip4/192.168.122.10/udp/37385/quic-v1/webtransport/certhash/uEiCIzQI1hLKlW2JXgT715yRQHLph4Cm_OBljZlwmytEaAQ/certhash/uEiBuNgitKgFo0RQZHPUFR7fixfBlh95KUgLqls_buIz8Sg/p2p/12D3KooWNQhHaM5XNcpQLYUUdnDYuhhJWEtmSQeanutTn4zNyA6w';
        const serverAddrStr ='/ip4/192.168.122.1/udp/37385/webrtc-direct/certhash/uEiC-d6DrX5z5xu17eJQ715RsQvvphwA0TSvBEAeWXZBKlQ/p2p/12D3KooWJP2jGS3mi4VhutFBFc1NUjyZBhwiEk8eFHJD9vwocKbh'

        const serverAddr = multiaddr(serverAddrStr);
        console.log(serverAddr);

        node.handle('/p2phttp', async ({ stream, protocol }) => {
            console.log('Handling /p2phttp on:', protocol);

        })

        // node.dial(serverAddr);

    const dial_go_node = async (targetHost) => {
        
        const P2PHTTPProtocol = "/p2phttp";

        try {
            // Connect to the Go server
            // await node.dial(serverAddr);
            // output.textContent += `Connected to Go server: ${serverAddrStr}\n`;
      
            // Open a stream with the p2phttp protocol
            const stream = await node.dialProtocol(serverAddr, P2PHTTPProtocol);
            // output.textContent += `Opened stream to ${serverAddrStr} with protocol ${P2PHTTPProtocol}\n`;
      
            console.log(await node.peerStore.all());

            // Construct an HTTP GET request
            // Construct and log the exact request string
            const requestStr = 'GET /hello HTTP/1.1\r\n' +
                         'Host: ' + targetHost + '\r\n' +
                         'Connection: close\r\n' +
                         '\r\n';

            console.log("Raw request string", requestStr)
            // const request = new TextEncoder().encode(requestStr);
            const request = cbor.encode(requestStr)
            // Write the request to the stream
            await stream.sink([request]);
      
            // Read the response
            let response = '';
            for await (const chunk of stream.source) {
              response += new TextDecoder().decode(chunk.subarray());
            }
      
            console.log(`Response from ${targetHost}:`, response);    


          } catch (err) {
            console.log(err.message);
            
          }
      
        
    }



    return <>
    
        <h1>Title</h1>

        
        <button onClick={() => dial_go_node("localhost:43110")}>Local /hello</button>
        <button onClick={() => dial_go_node("localhost:43111")}>Remote /hello</button>

    </>;

}
