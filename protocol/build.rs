extern crate protobuf_codegen_pure;
fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/",
        input: &["proto_defs/device_response.proto"],
        includes: &["proto_defs"],
        customize: protobuf_codegen_pure::Customize {
            // ToDo: Wait for v3 to be released and add the option for hiding _set _get
            // expose_fields: Some(true),
            // generate_accessors: Some(false),
            ..Default::default()
        },
    }).expect("protoc");
}
