use dioxus::prelude::*;

use crate::idl_helper::idl_node::{IDLNode, IDLNodeField};

#[derive(Clone, Props, PartialEq, Debug)]
pub struct PropNode {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub discriminator: Vec<u8>,
    pub name_account: String,
    pub fields: Vec<IDLNodeField>,
}

#[derive(Clone, Props, PartialEq, Debug)]
pub struct PropEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Clone, Props, PartialEq, Debug)]
pub struct PropNodes {
    pub list_nodes: Signal<Vec<PropNode>>,
    pub list_edges: Signal<Vec<PropEdge>>,
}

impl PropNodes {
    pub fn set_prop_nodes(&mut self, addr_program: String, data: Vec<IDLNode>) {
        let mut new_edges: Vec<PropEdge> = Vec::new();
        let mut new_nodes: Vec<PropNode> = Vec::new();
        new_nodes.push(PropNode {
            id: 0,
            x: 300.0,
            y: 100.0,
            discriminator: vec![],
            name_account: addr_program, // Initialize account_data as an empty vector
            fields: Vec::new(),         // Initialize executable as false
        });
        for (i, account) in data.iter().enumerate() {
            let node_id = i + 1; // Renamed for clarity
            new_edges.push(PropEdge {
                from: 0usize,
                to: node_id,
            });
            new_nodes.push(PropNode {
                id: node_id,
                x: 100.0 + (node_id % 5) as f32 * 100.0, // Use node_id for layout consistency
                y: 200.0 + (node_id / 5) as f32 * 100.0, // Use node_id for layout consistency
                discriminator: account.discriminator.clone(),
                name_account: account.name.clone(), // Initialize account_data as an empty vector
                fields: account.fields.clone(),     // Initialize executable as false
            });
        }

        self.list_nodes.set(new_nodes);
        self.list_edges.set(new_edges);
    }
}

pub fn use_nodes() -> PropNodes {
    PropNodes {
        list_nodes: use_signal(|| Vec::new()),
        list_edges: use_signal(|| Vec::new()),
    }
}
