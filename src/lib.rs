pub mod protobuf {
    pub mod data {
        include!(concat!(env!("OUT_DIR"), "/protobuf.data.rs"));
    }
}
