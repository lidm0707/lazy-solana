use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::idl_helper::idl_node::{IDLNode, IDLNodeField};

#[derive(Debug, Serialize, Deserialize)]
pub struct Idl {
    pub address: String,
    pub metadata: Metadata,
    pub instructions: Vec<Instruction>,
    pub accounts: Vec<Account>,
    pub types: Vec<IdlType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub spec: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub accounts: Vec<InstructionAccount>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionAccount {
    pub name: String,
    pub writable: Option<bool>,
    pub signer: Option<bool>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdlType {
    pub name: String,
    #[serde(rename = "type")]
    pub type_detail: TypeDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeDetail {
    pub kind: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

impl Idl {
    pub fn new(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn get_idl_idl_node(&self) -> Vec<IDLNode> {
        let accounts = self.accounts.clone();
        let account_fields = self.types.clone();
        let mut idl_nodes = Vec::<IDLNode>::new();
        let mut temp_name_account: HashMap<String, Vec<StructField>> = HashMap::new();
        for field in account_fields.iter() {
            let field_name = field.name.clone();
            let field_type = field.type_detail.fields.clone();
            temp_name_account.insert(field_name, field_type);
        }

        for account in accounts.iter() {
            let account_name = account.name.clone();
            let account_discriminator = account.discriminator.clone();

            let temp_fields = temp_name_account.get(&account_name);
            let account_fields = match temp_fields {
                Some(fields) => fields
                    .iter()
                    .map(|field| IDLNodeField {
                        name: field.name.clone(),
                        ty: field.field_type.clone(),
                    })
                    .collect(),
                None => Vec::new(),
            };

            idl_nodes.push(IDLNode::new(
                account_name,
                account_discriminator,
                account_fields,
            ));
        }
        idl_nodes
    }
}
