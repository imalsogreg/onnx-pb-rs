fn main() {
    prost_build::compile_protos(&["onnx.proto3"], &["vendored/onnx"]).unwrap();
}
