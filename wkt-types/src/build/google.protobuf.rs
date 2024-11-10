// This file is @generated by prost-build.
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of these
/// variants. Absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag = "2")]
        FloatValue(f64),
        /// Represents a int value.
        #[prost(int64, tag = "3")]
        IntValue(i64),
        /// Represents a uint value.
        #[prost(uint64, tag = "4")]
        UIntValue(u64),
        /// Represents a string value.
        #[prost(string, tag = "5")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag = "6")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag = "7")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag = "8")]
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
/// The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::NullValue => "NULL_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NULL_VALUE" => Some(Self::NullValue),
            _ => None,
        }
    }
}

# [allow (dead_code)] const _ : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Struct")] impl :: prost_wkt :: MessageSerde for Struct { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Struct" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Struct" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: with_capacity (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Struct" , decoder : | buf : & [u8] | { let msg : Struct = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } impl :: prost :: Name for Struct { const PACKAGE : & 'static str = "google.protobuf" ; const NAME : & 'static str = "Struct" ; fn type_url () -> String { "type.googleapis.com/google.protobuf.Struct" . to_string () } } } ;

# [allow (dead_code)] const _ : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Value")] impl :: prost_wkt :: MessageSerde for Value { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Value" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Value" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: with_capacity (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Value" , decoder : | buf : & [u8] | { let msg : Value = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } impl :: prost :: Name for Value { const PACKAGE : & 'static str = "google.protobuf" ; const NAME : & 'static str = "Value" ; fn type_url () -> String { "type.googleapis.com/google.protobuf.Value" . to_string () } } } ;

# [allow (dead_code)] const _ : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.ListValue")] impl :: prost_wkt :: MessageSerde for ListValue { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "ListValue" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.ListValue" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: with_capacity (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.ListValue" , decoder : | buf : & [u8] | { let msg : ListValue = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } impl :: prost :: Name for ListValue { const PACKAGE : & 'static str = "google.protobuf" ; const NAME : & 'static str = "ListValue" ; fn type_url () -> String { "type.googleapis.com/google.protobuf.ListValue" . to_string () } } } ;