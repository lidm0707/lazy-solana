use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct IDLNode {
    pub name: String,
    pub discriminator: Vec<u8>,
    pub fields: Vec<IDLNodeField>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IDLNodeField {
    pub name: String,
    pub ty: String,
}

impl IDLNode {
    pub fn new(name: String, discriminator: Vec<u8>, fields: Vec<IDLNodeField>) -> Self {
        IDLNode {
            name,
            discriminator,
            fields,
        }
    }
}
