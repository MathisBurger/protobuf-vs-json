pub mod large;
pub mod mid;
pub mod small;

pub trait ProtobufImpl<T> {
    fn convert_to_protobuf(&mut self) -> T;
}
