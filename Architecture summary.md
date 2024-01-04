# Architecture summary

On the server side:
To start a simulated gl-backend server we can run the binary in gl-signerproxy which starts a server by calling the 'run' function defined in hsmproxy.rs. We also have to start the binary in gl-plugin that's located in gl-plugin/src/bin/plugin.rs.

## hsmproxy.rs
The hsmproxy.rs's 'run' function calls the 'process_requests' function which is used to listen to a signing requests from core lightning. It does this by taking control of the file descriptor used by one of the processes in the core lightning node called 'hsmd'.


## plugin.rs - node server
plugin.rs starts 3 processes with the functions start_node_server, start_hsm_server, and gl_plugin::plugin.start(). 

The node_server is composed of 2 parts, the part of the server that uses the core-lightning grpc interface and the part of the server that uses the greenlight grpc interface. 

Both components are created using a struct called 'WrappedNodeServer' which requires an instance of 'PluginNodeServer' for construction. The difference is the core-lightning component is wrapped with a 'NodeServer' struct defined in a *core-lightning* grpc proto file, and the greenlight component is wrapped with a 'NodeServer' struct defined in a *greenlight* grpc proto file. There's also authentication logic defined in 'SignatureContextLayer' for requests handled by the server.

The greenlight part of the node_server allows the signer to connect and wait for requests using the server's 'stream_hsm_requests' grpc method. The server then loops in 'stream_hsm_requests' and listens to signing requests from the stage to send to the remote signer for processing. When there's something detected on the stage, it's sent to the signer and the loop continues, only failing if the signer didn't send an ack.

After the requests are processed by the signer, the signer calls 'respond_hsm_request' which sends the processed request back to the node. The node sends the processed request to the stage.

The core lightning part of the node_server listens for calls to the grpc methods intended for the interface exposed on the core lightning node. When a request for the core-lightning part comes in it gets sent to the core-lightning node and a request to the hsmproxy is sent if the core-lightning request needs something to be signed.


## plugin.rs - hsm server

The hsm_server is represented by the 'StagingHsmServer' struct in hsm.rs and its primary purpose is to send the messages it receives from the hsmproxy.rs to the 'stage' (defined in stager.rs). The hsm_server listens to requests using its 'request' grpc method and gets the processed request from a stream returned by the stage when a message is sent. The processed request then gets returned to the caller (hsmproxy) where it'll get sent back to the core lightning node.

## Full Flow 
client -> core-lightning -> hsmproxy -> hsm_server -> stage -> node_server -> signer -> node_server -> stage -> hsm_server -> hsmproxy -> core-lightning

