use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn get_idl_accounts(&self) -> Vec<String> {
        let account_fields = self.types.clone();
        let mut accounts_string = Vec::<String>::new();
        for types in account_fields.iter() {
            for field in types.type_detail.fields.clone().iter() {
                accounts_string.push(format!(
                    "{}::::{}",
                    field.name.clone(),
                    field.field_type.clone()
                ))
            }
        }
        accounts_string
    }
}
