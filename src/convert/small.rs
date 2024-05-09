use crate::structures::small;
use protobufVsJson::protobuf::data;

use super::ProtobufImpl;

impl ProtobufImpl<data::SmallStructure> for small::SmallStructure {
    fn convert_to_protobuf(&mut self) -> data::SmallStructure {
        return data::SmallStructure {
            val1: self.val1,
            val2: self.val2.clone(),
            val3: self.val3,
            val4: self.val4,
        };
    }
}
