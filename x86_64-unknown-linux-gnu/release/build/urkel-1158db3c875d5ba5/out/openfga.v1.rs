#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub schema_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub type_definitions: ::prost::alloc::vec::Vec<TypeDefinition>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeDefinition {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub relations: ::std::collections::HashMap<::prost::alloc::string::String, Userset>,
    /// A map whose keys are the name of the relation and whose value is the Metadata for that relation.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<Metadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub rewrite: ::core::option::Option<Userset>,
    #[prost(message, optional, tag = "3")]
    pub type_info: ::core::option::Option<RelationTypeInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationTypeInfo {
    #[prost(message, repeated, tag = "1")]
    pub directly_related_user_types: ::prost::alloc::vec::Vec<RelationReference>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(map = "string, message", tag = "1")]
    pub relations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        RelationMetadata,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationMetadata {
    #[prost(message, repeated, tag = "1")]
    pub directly_related_user_types: ::prost::alloc::vec::Vec<RelationReference>,
}
/// RelationReference represents a relation of a particular object type (e.g. 'document#viewer').
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationReference {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(oneof = "relation_reference::RelationOrWildcard", tags = "2, 3")]
    pub relation_or_wildcard: ::core::option::Option<
        relation_reference::RelationOrWildcard,
    >,
}
/// Nested message and enum types in `RelationReference`.
pub mod relation_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RelationOrWildcard {
        #[prost(string, tag = "2")]
        Relation(::prost::alloc::string::String),
        #[prost(message, tag = "3")]
        Wildcard(super::Wildcard),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wildcard {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Usersets {
    #[prost(message, repeated, tag = "1")]
    pub child: ::prost::alloc::vec::Vec<Userset>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Difference {
    #[prost(message, optional, boxed, tag = "1")]
    pub base: ::core::option::Option<::prost::alloc::boxed::Box<Userset>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub subtract: ::core::option::Option<::prost::alloc::boxed::Box<Userset>>,
}
/// Urkel Note: Serde had a hard time deserializing oneOf type
/// camelCase -> snake_case conversion needed here
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Userset {
    #[prost(message, optional, tag = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub this: ::core::option::Option<DirectUserset>,
    #[prost(message, optional, tag = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_userset: ::core::option::Option<ObjectRelation>,
    #[prost(message, optional, tag = "3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: ::core::option::Option<TupleToUserset>,
    #[prost(message, optional, tag = "4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union: ::core::option::Option<Usersets>,
    #[prost(message, optional, tag = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection: ::core::option::Option<Usersets>,
    #[prost(message, optional, boxed, tag = "6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference: ::core::option::Option<::prost::alloc::boxed::Box<Difference>>,
}
/// A DirectUserset is a sentinel message for referencing
/// the direct members specified by an object/relation mapping.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUserset {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectRelation {
    #[prost(string, tag = "1")]
    pub object: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub relation: ::prost::alloc::string::String,
}
/// camelCase -> snake_case conversion needed here
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleToUserset {
    /// The target object/relation
    #[prost(message, optional, tag = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tupleset: ::core::option::Option<ObjectRelation>,
    #[prost(message, optional, tag = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_userset: ::core::option::Option<ObjectRelation>,
}
/// Object represents an OpenFGA Object.
///
/// An Object is composed of a type and identifier (e.g. 'document:1')
///
/// See <https://openfga.dev/docs/concepts#what-is-an-object>
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleKey {
    #[prost(string, optional, tag = "1")]
    pub object: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub relation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tuple {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<TupleKey>,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleKeys {
    #[prost(message, repeated, tag = "1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKey>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextualTupleKeys {
    #[prost(message, repeated, tag = "1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKey>,
}
/// A UsersetTree contains the result of an Expansion.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersetTree {
    #[prost(message, optional, tag = "1")]
    pub root: ::core::option::Option<userset_tree::Node>,
}
/// Nested message and enum types in `UsersetTree`.
pub mod userset_tree {
    /// A leaf node contains either
    /// - a set of users (which may be individual users, or usersets
    ///    referencing other relations)
    /// - a computed node, which is the result of a computed userset
    ///    value in the authorization model
    /// - a tupleToUserset nodes, containing the result of expanding
    ///    a tupleToUserset value in a authorization model.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Leaf {
        #[prost(oneof = "leaf::Value", tags = "1, 2, 3")]
        pub value: ::core::option::Option<leaf::Value>,
    }
    /// Nested message and enum types in `Leaf`.
    pub mod leaf {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag = "1")]
            Users(super::Users),
            #[prost(message, tag = "2")]
            Computed(super::Computed),
            #[prost(message, tag = "3")]
            TupleToUserset(super::TupleToUserset),
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nodes {
        #[prost(message, repeated, tag = "1")]
        pub nodes: ::prost::alloc::vec::Vec<Node>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Users {
        #[prost(string, repeated, tag = "1")]
        pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Computed {
        #[prost(string, tag = "1")]
        pub userset: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TupleToUserset {
        #[prost(string, tag = "1")]
        pub tupleset: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub computed: ::prost::alloc::vec::Vec<Computed>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Difference {
        #[prost(message, optional, boxed, tag = "1")]
        pub base: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
        #[prost(message, optional, boxed, tag = "2")]
        pub subtract: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "node::Value", tags = "2, 5, 6, 7")]
        pub value: ::core::option::Option<node::Value>,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag = "2")]
            Leaf(super::Leaf),
            #[prost(message, tag = "5")]
            Difference(::prost::alloc::boxed::Box<super::Difference>),
            #[prost(message, tag = "6")]
            Union(super::Nodes),
            #[prost(message, tag = "7")]
            Intersection(super::Nodes),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assertion {
    #[prost(message, optional, tag = "1")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(bool, tag = "2")]
    pub expectation: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assertions {
    #[prost(message, repeated, tag = "1")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleChange {
    #[prost(message, optional, tag = "1")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(enumeration = "TupleOperation", tag = "2")]
    pub operation: i32,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub deleted_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// buf:lint:ignore ENUM_ZERO_VALUE_SUFFIX
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TupleOperation {
    Write = 0,
    Delete = 1,
}
impl TupleOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TupleOperation::Write => "TUPLE_OPERATION_WRITE",
            TupleOperation::Delete => "TUPLE_OPERATION_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TUPLE_OPERATION_WRITE" => Some(Self::Write),
            "TUPLE_OPERATION_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub authorization_model_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub user: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsResponse {
    #[prost(string, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedListObjectsRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub user: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
}
/// The response for a StreamedListObjects RPC.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedListObjectsResponse {
    #[prost(string, tag = "1")]
    pub object: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(message, optional, tag = "3")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag = "4")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub tuples: ::prost::alloc::vec::Vec<Tuple>,
    #[prost(string, tag = "2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub writes: ::core::option::Option<TupleKeys>,
    #[prost(message, optional, tag = "3")]
    pub deletes: ::core::option::Option<TupleKeys>,
    #[prost(string, optional, tag = "4")]
    pub authorization_model_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    #[prost(string, optional, tag = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(message, optional, tag = "3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
    #[prost(string, optional, tag = "4")]
    pub authorization_model_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false. Making it true has performance implications.
    #[prost(bool, optional, tag = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    #[prost(bool, tag = "1")]
    pub allowed: bool,
    /// For internal use only.
    #[prost(string, optional, tag = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(string, tag = "3")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandResponse {
    #[prost(message, optional, tag = "1")]
    pub tree: ::core::option::Option<UsersetTree>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelResponse {
    #[prost(message, optional, tag = "1")]
    pub authorization_model: ::core::option::Option<AuthorizationModel>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAuthorizationModelRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub type_definitions: ::prost::alloc::vec::Vec<TypeDefinition>,
    #[prost(string, tag = "3")]
    pub schema_version: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAuthorizationModelResponse {
    #[prost(string, tag = "1")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelsRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag = "3")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub authorization_models: ::prost::alloc::vec::Vec<AuthorizationModel>,
    #[prost(string, tag = "2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAssertionsRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAssertionsResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAssertionsRequest {
    #[prost(string, tag = "1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAssertionsResponse {
    #[prost(string, tag = "1")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChangesRequest {
    #[prost(string, optional, tag = "1")]
    pub store_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag = "4")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChangesResponse {
    #[prost(message, repeated, tag = "1")]
    pub changes: ::prost::alloc::vec::Vec<TupleChange>,
    #[prost(string, tag = "2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoreRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoreResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoreRequest {
    #[prost(string, tag = "1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoreResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoreRequest {
    #[prost(string, tag = "1")]
    pub store_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoreResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreRequest {
    #[prost(string, tag = "1")]
    pub store_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresRequest {
    #[prost(message, optional, tag = "1")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag = "2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresResponse {
    #[prost(message, repeated, tag = "1")]
    pub stores: ::prost::alloc::vec::Vec<Store>,
    #[prost(string, tag = "2")]
    pub continuation_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod open_fga_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OpenFgaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OpenFgaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OpenFgaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OpenFgaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OpenFgaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Read",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Read"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Write",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Write"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> std::result::Result<tonic::Response<super::CheckResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Check",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Check"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn expand(
            &mut self,
            request: impl tonic::IntoRequest<super::ExpandRequest>,
        ) -> std::result::Result<tonic::Response<super::ExpandResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Expand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Expand"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_authorization_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAuthorizationModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAuthorizationModelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAuthorizationModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "ReadAuthorizationModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_authorization_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAuthorizationModelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAuthorizationModelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAuthorizationModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "ReadAuthorizationModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_authorization_model(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteAuthorizationModelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteAuthorizationModelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/WriteAuthorizationModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "WriteAuthorizationModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_assertions(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteAssertionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteAssertionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/WriteAssertions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "WriteAssertions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_assertions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAssertionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAssertionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAssertions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ReadAssertions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadChangesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ReadChanges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_store(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/CreateStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "CreateStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_store(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/UpdateStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "UpdateStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_store(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/DeleteStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "DeleteStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_store(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/GetStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "GetStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_stores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStoresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStoresResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ListStores",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ListStores"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn streamed_list_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamedListObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamedListObjectsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/StreamedListObjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("openfga.v1.OpenFGAService", "StreamedListObjects"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListObjectsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ListObjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ListObjects"));
            self.inner.unary(req, path, codec).await
        }
    }
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_AUTHORIZATION_MODEL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.AuthorizationModel")] impl :: prost_wkt :: MessageSerde for AuthorizationModel { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "AuthorizationModel" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.AuthorizationModel" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.AuthorizationModel" , decoder : | buf : & [u8] | { let msg : AuthorizationModel = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TYPE_DEFINITION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.TypeDefinition")] impl :: prost_wkt :: MessageSerde for TypeDefinition { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "TypeDefinition" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.TypeDefinition" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.TypeDefinition" , decoder : | buf : & [u8] | { let msg : TypeDefinition = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_RELATION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Relation")] impl :: prost_wkt :: MessageSerde for Relation { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Relation" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Relation" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Relation" , decoder : | buf : & [u8] | { let msg : Relation = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_RELATION_TYPE_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.RelationTypeInfo")] impl :: prost_wkt :: MessageSerde for RelationTypeInfo { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "RelationTypeInfo" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.RelationTypeInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.RelationTypeInfo" , decoder : | buf : & [u8] | { let msg : RelationTypeInfo = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_METADATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Metadata")] impl :: prost_wkt :: MessageSerde for Metadata { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Metadata" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Metadata" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Metadata" , decoder : | buf : & [u8] | { let msg : Metadata = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_RELATION_METADATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.RelationMetadata")] impl :: prost_wkt :: MessageSerde for RelationMetadata { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "RelationMetadata" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.RelationMetadata" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.RelationMetadata" , decoder : | buf : & [u8] | { let msg : RelationMetadata = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_RELATION_REFERENCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.RelationReference")] impl :: prost_wkt :: MessageSerde for RelationReference { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "RelationReference" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.RelationReference" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.RelationReference" , decoder : | buf : & [u8] | { let msg : RelationReference = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WILDCARD : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Wildcard")] impl :: prost_wkt :: MessageSerde for Wildcard { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Wildcard" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Wildcard" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Wildcard" , decoder : | buf : & [u8] | { let msg : Wildcard = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_USERSETS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Usersets")] impl :: prost_wkt :: MessageSerde for Usersets { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Usersets" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Usersets" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Usersets" , decoder : | buf : & [u8] | { let msg : Usersets = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DIFFERENCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Difference")] impl :: prost_wkt :: MessageSerde for Difference { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Difference" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Difference" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Difference" , decoder : | buf : & [u8] | { let msg : Difference = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_USERSET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Userset")] impl :: prost_wkt :: MessageSerde for Userset { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Userset" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Userset" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Userset" , decoder : | buf : & [u8] | { let msg : Userset = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DIRECT_USERSET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.DirectUserset")] impl :: prost_wkt :: MessageSerde for DirectUserset { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "DirectUserset" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.DirectUserset" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.DirectUserset" , decoder : | buf : & [u8] | { let msg : DirectUserset = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_OBJECT_RELATION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ObjectRelation")] impl :: prost_wkt :: MessageSerde for ObjectRelation { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ObjectRelation" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ObjectRelation" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ObjectRelation" , decoder : | buf : & [u8] | { let msg : ObjectRelation = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TUPLE_TO_USERSET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.TupleToUserset")] impl :: prost_wkt :: MessageSerde for TupleToUserset { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "TupleToUserset" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.TupleToUserset" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.TupleToUserset" , decoder : | buf : & [u8] | { let msg : TupleToUserset = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_OBJECT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Object")] impl :: prost_wkt :: MessageSerde for Object { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Object" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Object" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Object" , decoder : | buf : & [u8] | { let msg : Object = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TUPLE_KEY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.TupleKey")] impl :: prost_wkt :: MessageSerde for TupleKey { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "TupleKey" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.TupleKey" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.TupleKey" , decoder : | buf : & [u8] | { let msg : TupleKey = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TUPLE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Tuple")] impl :: prost_wkt :: MessageSerde for Tuple { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Tuple" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Tuple" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Tuple" , decoder : | buf : & [u8] | { let msg : Tuple = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TUPLE_KEYS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.TupleKeys")] impl :: prost_wkt :: MessageSerde for TupleKeys { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "TupleKeys" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.TupleKeys" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.TupleKeys" , decoder : | buf : & [u8] | { let msg : TupleKeys = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CONTEXTUAL_TUPLE_KEYS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ContextualTupleKeys")] impl :: prost_wkt :: MessageSerde for ContextualTupleKeys { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ContextualTupleKeys" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ContextualTupleKeys" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ContextualTupleKeys" , decoder : | buf : & [u8] | { let msg : ContextualTupleKeys = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_USERSET_TREE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.UsersetTree")] impl :: prost_wkt :: MessageSerde for UsersetTree { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "UsersetTree" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.UsersetTree" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.UsersetTree" , decoder : | buf : & [u8] | { let msg : UsersetTree = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSERTION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Assertion")] impl :: prost_wkt :: MessageSerde for Assertion { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Assertion" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Assertion" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Assertion" , decoder : | buf : & [u8] | { let msg : Assertion = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ASSERTIONS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Assertions")] impl :: prost_wkt :: MessageSerde for Assertions { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Assertions" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Assertions" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Assertions" , decoder : | buf : & [u8] | { let msg : Assertions = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TUPLE_CHANGE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.TupleChange")] impl :: prost_wkt :: MessageSerde for TupleChange { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "TupleChange" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.TupleChange" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.TupleChange" , decoder : | buf : & [u8] | { let msg : TupleChange = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_STORE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.Store")] impl :: prost_wkt :: MessageSerde for Store { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "Store" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.Store" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.Store" , decoder : | buf : & [u8] | { let msg : Store = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_OBJECTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ListObjectsRequest")] impl :: prost_wkt :: MessageSerde for ListObjectsRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ListObjectsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ListObjectsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ListObjectsRequest" , decoder : | buf : & [u8] | { let msg : ListObjectsRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_OBJECTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ListObjectsResponse")] impl :: prost_wkt :: MessageSerde for ListObjectsResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ListObjectsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ListObjectsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ListObjectsResponse" , decoder : | buf : & [u8] | { let msg : ListObjectsResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_STREAMED_LIST_OBJECTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.StreamedListObjectsRequest")] impl :: prost_wkt :: MessageSerde for StreamedListObjectsRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "StreamedListObjectsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.StreamedListObjectsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.StreamedListObjectsRequest" , decoder : | buf : & [u8] | { let msg : StreamedListObjectsRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_STREAMED_LIST_OBJECTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.StreamedListObjectsResponse")] impl :: prost_wkt :: MessageSerde for StreamedListObjectsResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "StreamedListObjectsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.StreamedListObjectsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.StreamedListObjectsResponse" , decoder : | buf : & [u8] | { let msg : StreamedListObjectsResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadRequest")] impl :: prost_wkt :: MessageSerde for ReadRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadRequest" , decoder : | buf : & [u8] | { let msg : ReadRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadResponse")] impl :: prost_wkt :: MessageSerde for ReadResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadResponse" , decoder : | buf : & [u8] | { let msg : ReadResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteRequest")] impl :: prost_wkt :: MessageSerde for WriteRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteRequest" , decoder : | buf : & [u8] | { let msg : WriteRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteResponse")] impl :: prost_wkt :: MessageSerde for WriteResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteResponse" , decoder : | buf : & [u8] | { let msg : WriteResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CHECK_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.CheckRequest")] impl :: prost_wkt :: MessageSerde for CheckRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "CheckRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.CheckRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.CheckRequest" , decoder : | buf : & [u8] | { let msg : CheckRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CHECK_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.CheckResponse")] impl :: prost_wkt :: MessageSerde for CheckResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "CheckResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.CheckResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.CheckResponse" , decoder : | buf : & [u8] | { let msg : CheckResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EXPAND_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ExpandRequest")] impl :: prost_wkt :: MessageSerde for ExpandRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ExpandRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ExpandRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ExpandRequest" , decoder : | buf : & [u8] | { let msg : ExpandRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EXPAND_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ExpandResponse")] impl :: prost_wkt :: MessageSerde for ExpandResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ExpandResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ExpandResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ExpandResponse" , decoder : | buf : & [u8] | { let msg : ExpandResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_AUTHORIZATION_MODEL_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAuthorizationModelRequest")] impl :: prost_wkt :: MessageSerde for ReadAuthorizationModelRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAuthorizationModelRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAuthorizationModelRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAuthorizationModelRequest" , decoder : | buf : & [u8] | { let msg : ReadAuthorizationModelRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_AUTHORIZATION_MODEL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAuthorizationModelResponse")] impl :: prost_wkt :: MessageSerde for ReadAuthorizationModelResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAuthorizationModelResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAuthorizationModelResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAuthorizationModelResponse" , decoder : | buf : & [u8] | { let msg : ReadAuthorizationModelResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_AUTHORIZATION_MODEL_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteAuthorizationModelRequest")] impl :: prost_wkt :: MessageSerde for WriteAuthorizationModelRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteAuthorizationModelRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteAuthorizationModelRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteAuthorizationModelRequest" , decoder : | buf : & [u8] | { let msg : WriteAuthorizationModelRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_AUTHORIZATION_MODEL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteAuthorizationModelResponse")] impl :: prost_wkt :: MessageSerde for WriteAuthorizationModelResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteAuthorizationModelResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteAuthorizationModelResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteAuthorizationModelResponse" , decoder : | buf : & [u8] | { let msg : WriteAuthorizationModelResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_AUTHORIZATION_MODELS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAuthorizationModelsRequest")] impl :: prost_wkt :: MessageSerde for ReadAuthorizationModelsRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAuthorizationModelsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAuthorizationModelsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAuthorizationModelsRequest" , decoder : | buf : & [u8] | { let msg : ReadAuthorizationModelsRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_AUTHORIZATION_MODELS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAuthorizationModelsResponse")] impl :: prost_wkt :: MessageSerde for ReadAuthorizationModelsResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAuthorizationModelsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAuthorizationModelsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAuthorizationModelsResponse" , decoder : | buf : & [u8] | { let msg : ReadAuthorizationModelsResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_ASSERTIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteAssertionsRequest")] impl :: prost_wkt :: MessageSerde for WriteAssertionsRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteAssertionsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteAssertionsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteAssertionsRequest" , decoder : | buf : & [u8] | { let msg : WriteAssertionsRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WRITE_ASSERTIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.WriteAssertionsResponse")] impl :: prost_wkt :: MessageSerde for WriteAssertionsResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "WriteAssertionsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.WriteAssertionsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.WriteAssertionsResponse" , decoder : | buf : & [u8] | { let msg : WriteAssertionsResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_ASSERTIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAssertionsRequest")] impl :: prost_wkt :: MessageSerde for ReadAssertionsRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAssertionsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAssertionsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAssertionsRequest" , decoder : | buf : & [u8] | { let msg : ReadAssertionsRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_ASSERTIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadAssertionsResponse")] impl :: prost_wkt :: MessageSerde for ReadAssertionsResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadAssertionsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadAssertionsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadAssertionsResponse" , decoder : | buf : & [u8] | { let msg : ReadAssertionsResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_CHANGES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadChangesRequest")] impl :: prost_wkt :: MessageSerde for ReadChangesRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadChangesRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadChangesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadChangesRequest" , decoder : | buf : & [u8] | { let msg : ReadChangesRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_READ_CHANGES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ReadChangesResponse")] impl :: prost_wkt :: MessageSerde for ReadChangesResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ReadChangesResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ReadChangesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ReadChangesResponse" , decoder : | buf : & [u8] | { let msg : ReadChangesResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CREATE_STORE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.CreateStoreRequest")] impl :: prost_wkt :: MessageSerde for CreateStoreRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "CreateStoreRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.CreateStoreRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.CreateStoreRequest" , decoder : | buf : & [u8] | { let msg : CreateStoreRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CREATE_STORE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.CreateStoreResponse")] impl :: prost_wkt :: MessageSerde for CreateStoreResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "CreateStoreResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.CreateStoreResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.CreateStoreResponse" , decoder : | buf : & [u8] | { let msg : CreateStoreResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPDATE_STORE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.UpdateStoreRequest")] impl :: prost_wkt :: MessageSerde for UpdateStoreRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "UpdateStoreRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.UpdateStoreRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.UpdateStoreRequest" , decoder : | buf : & [u8] | { let msg : UpdateStoreRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPDATE_STORE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.UpdateStoreResponse")] impl :: prost_wkt :: MessageSerde for UpdateStoreResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "UpdateStoreResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.UpdateStoreResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.UpdateStoreResponse" , decoder : | buf : & [u8] | { let msg : UpdateStoreResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DELETE_STORE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.DeleteStoreRequest")] impl :: prost_wkt :: MessageSerde for DeleteStoreRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "DeleteStoreRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.DeleteStoreRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.DeleteStoreRequest" , decoder : | buf : & [u8] | { let msg : DeleteStoreRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DELETE_STORE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.DeleteStoreResponse")] impl :: prost_wkt :: MessageSerde for DeleteStoreResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "DeleteStoreResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.DeleteStoreResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.DeleteStoreResponse" , decoder : | buf : & [u8] | { let msg : DeleteStoreResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_STORE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.GetStoreRequest")] impl :: prost_wkt :: MessageSerde for GetStoreRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "GetStoreRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.GetStoreRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.GetStoreRequest" , decoder : | buf : & [u8] | { let msg : GetStoreRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_STORE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.GetStoreResponse")] impl :: prost_wkt :: MessageSerde for GetStoreResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "GetStoreResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.GetStoreResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.GetStoreResponse" , decoder : | buf : & [u8] | { let msg : GetStoreResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_STORES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ListStoresRequest")] impl :: prost_wkt :: MessageSerde for ListStoresRequest { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ListStoresRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ListStoresRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ListStoresRequest" , decoder : | buf : & [u8] | { let msg : ListStoresRequest = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_STORES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/openfga.v1.ListStoresResponse")] impl :: prost_wkt :: MessageSerde for ListStoresResponse { fn package_name (& self) -> & 'static str { "openfga.v1" } fn message_name (& self) -> & 'static str { "ListStoresResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/openfga.v1.ListStoresResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/openfga.v1.ListStoresResponse" , decoder : | buf : & [u8] | { let msg : ListStoresResponse = :: prost :: Message :: decode (buf) ? ; Ok (Box :: new (msg)) } } } } ;
