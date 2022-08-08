use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use ethers::types::{Address, U256};

impl JsonSchema for Address {
    no_ref_schema!();

    fn schema_name() -> String {
        "Address".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("hex (h160)".to_string()),
            metadata: Some(Box::new(Metadata {
                description: Some("Ethereum address in hex prefixed with 0x.".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl JsonSchema for U256 {
    no_ref_schema!();

    fn schema_name() -> String {
        "U256".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("hex (u256)".to_string()),
            metadata: Some(Box::new(Metadata {
                description: Some("Number in hex prefixed by 0x.".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}
