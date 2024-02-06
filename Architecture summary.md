# Architecture summary

On the server side:
There are 2 binaries used to simulate the greenlight backend server included in the public repo. The binary in gl-signerproxy which starts a server by calling the 'run' function defined in hsmproxy.rs, and the binary in gl-plugin that's located in gl-plugin/src/bin/plugin.rs. Let's refer to the binary in gl-signerproxy as the signer-proxy and the one in gl-plugin as the node-server.

## signer-proxy
The signer-proxy's 'run' function calls 'process_requests' on startup. 'process_requests' takes control of the file descriptor used by one of the processes in the core lightning node, called 'hsmd', to listen and respond to signing requests from core lightning.

## plugin.rs - hsm server

The hsm_server is represented by the 'StagingHsmServer' struct in hsm.rs and its primary purpose is to send the messages it receives from the hsmproxy.rs to the 'stage' (defined in stager.rs). The hsm_server listens to requests using its 'request' grpc method and gets the processed request from a stream returned by the stage when a message is sent. The processed request then gets returned to the caller (hsmproxy) where it'll get sent back to the core lightning node. This stage is the glue component between the signer-proxy and the rest of the node-server.

## plugin.rs - node server
plugin.rs starts 3 processes with the functions start_node_server, start_hsm_server, and gl_plugin::plugin.start(). 

The node_server is composed of 2 parts, the part of the server that uses the core-lightning grpc interface and the part of the server that uses the greenlight grpc interface. 

NodeServer -> WrappedNodeServer -> PluginNodeServer

Both components are created using a struct called 'NodeServer' which takes in a 'WrappedNodeServer' requiring an instance of 'PluginNodeServer' for construction. The difference is the core-lightning component is wrapped with a 'NodeServer' struct defined in a *core-lightning* grpc package (generated from a protofile defined in core-lightning), and the greenlight component is wrapped with a 'NodeServer' struct defined in a *greenlight* grpc package. There's also authentication logic defined in 'SignatureContextLayer' for requests handled by the server.

The greenlight part of the node-server allows the signer to connect and wait for requests using the server's 'stream_hsm_requests' grpc method. The method listens to signing requests from the stage in a loop so it knows when to send messages to the remote signer for processing. This continues until the signer disconnects, or if the signer didn't send an ack to one of the messages it was sent.

After the requests are processed by the signer, the signer calls 'respond_hsm_request' which sends the processed request back to the node-server. The node-server sends the processed request to the stage.

The core lightning part of the node_server listens for calls to the grpc methods intended for the interface exposed on the core lightning node. When a request for the core-lightning part comes in, it gets sent to the core-lightning node and a request to the hsmproxy is sent if the core-lightning request needs something to be signed.

### The request to the signer
The request to the signer includes 'ctxrequests' and the 'signer_state'. The node-server syncs with the signerstate but doesn't alter it much (?). The ctxrequests stores a vec called 'requests' which is used to store signed requests made to the node by a client. The requests in the context are populated in the SignatureContextLayer. The SignatureContextLayer calls the inner grpc methods to access the node and awaits that call, but the request being processed gets removed in a separate task after the grpc request has been processed.


## Full Flow 
client -> core-lightning -> hsmproxy -> hsm_server -> stage -> node_server -> signer -> node_server -> stage -> hsm_server -> hsmproxy -> core-lightning



## Signer

### Signer State
The 'Signer' struct has a state struct that is shared with the MemoryPersister, which is an implementation of the Persist trait defined within the VLS library. A serialized form of this state's internal 'values' map is what's given back to the node-server on completion of 'process_request'.

The signer state keeps track of the node_state, the channels, and information about the blockchain being used (how many blocks, whether it's signet, testnet, etc). The state almost never needs to be manually accessed as there are VLS methods that can be called on the handler to indirectly make these changes. For example the function 'update_state_from_request' takes in a handler, accesses the node on the handler, and calls 'add_invoice' to update the node_state (of the type NodeState, defined in VLS). The node-server maintains it's own signer state (defined in 'PluginNodeServer').