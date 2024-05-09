pub mod large;
pub mod mid;
pub mod small;

pub trait ProtobufImpl<A, B> {
    fn convert_to_protobuf(&mut self) -> B;
    fn convert_all(all: Vec<A>) -> Vec<B>;
}
