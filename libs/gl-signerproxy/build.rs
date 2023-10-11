fn main() {
    //build the protobufs from greenlight's protobuf file
    tonic_build::compile_protos("../proto/greenlight.proto").unwrap();
}
