/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
///
///      service Foo {
///        rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///      }
///
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EMPTY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Empty")] impl :: prost_wkt :: MessageSerde for Empty { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Empty" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Empty" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Empty" , decoder : | buf : & [u8] | { let msg : Empty = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;
