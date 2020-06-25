# How to use `Google Protocol Buffer` in `Rust`

In this demo, we got a cargo workspace which has a `protocol` sub-project. 

The `protocol` sub-project has a `proto_defs` folder which contains all `messages` (the term in Protocol Buffer to describe a transfer unit). 
And then use `protobuf-codegen-pure` to generate the `Rust` source code from the `message` definition, output will place in `protocol/src` folder.

This [link](https://developers.google.com/protocol-buffers/docs/proto3#scalar) lists all the valid data types which you can use in the `message` definition.

Btw, `protobuf-codegen-pure` still **NOT** supported the `optional` keyword at this moment, do not use it in the `message` definition!!!

## How protocol buffer works

```rust
syntax = "proto3";

message GpsLocation {
    string alarm = 1;
    string status = 2;
    bool is_precise = 3;
    string lat = 4;
    string lon = 5;
    uint32 altitude = 6;
    float speed = 7;
    uint32 direction = 8;
    string gps_time = 9;
    string recv_time = 10;
    uint32 lbs_network_type = 11;
    Lbs lbs = 12;
}
```

As you can see above, each attribute got a data type, name and an index number, that index number use to track which attribute at which position in order.
That means the serialize data **Does Not** include the attribute name at all, that's why the serialize data size is very small!!!


## How to generate the `Rust` struct source code we can use


```
cargo build
cp /Volumes/NiftyDrive/GitHub/proto-verb-data/samples/*.dat target/debug/
cd ./target/debug
./proto-verb-data
```

### Output 

> "ababillarse (first-person singular present me ababillo, first-person singular preterite me ababillé, past participle ababillado)"

[https://github.com/ian-hamlin/verb-data]: https://github.com/ian-hamlin/verb-data
