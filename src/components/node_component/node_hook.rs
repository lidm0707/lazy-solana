use std::collections::HashMap;

use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AccountInfo {
    pub pubkey: String,
    pub lamports: u64,
    pub executable: bool,
    pub account_data: String, // To store the string representation of account's data field
}

#[derive(Clone, Props, PartialEq)]
pub struct PropNode {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub data: HashMap<String, String>, // Add pubkey to display account info
    pub executable: bool,
    pub lamports: u64,
    pub account_data_display: String,
}

#[derive(Clone, Props, PartialEq)]
pub struct PropEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Clone, Props, PartialEq)]
pub struct PropNodes {
    pub list_nodes: Signal<Vec<PropNode>>,
    pub list_edges: Signal<Vec<PropEdge>>,
}

impl PropNodes {
    pub fn set_prop_nodes(&mut self, addr_program: String, data: Vec<AccountInfo>) {
        let mut new_edges: Vec<PropEdge> = Vec::new();
        let mut new_nodes: Vec<PropNode> = Vec::new();
        new_nodes.push(PropNode {
            id: 0,
            x: 300.0,
            y: 100.0,
            data: {
                let mut data_map = HashMap::new(); // Renamed to avoid conflict
                data_map.insert("pubkey".to_string(), addr_program.to_owned());
                data_map
            },
            executable: false, // Program ID node is not an account, defaults to not executable
            lamports: 0, // Program ID node has 0 lamports for display purposes
            account_data_display: String::from("Program (No Data Field)"), // Placeholder for program ID node
        });
        for (i, account) in data.iter().enumerate() {
            let mut node_data: HashMap<String, String> = HashMap::new(); // Renamed for clarity
            let node_id = i + 1; // Renamed for clarity
            new_edges.push(PropEdge {
                from: 0usize,
                to: node_id,
            });
            node_data.insert("pubkey".to_string(), account.pubkey.clone());
            new_nodes.push(PropNode {
                id: node_id,
                x: 100.0 + (node_id % 5) as f32 * 100.0, // Use node_id for layout consistency
                y: 200.0 + (node_id / 5) as f32 * 100.0, // Use node_id for layout consistency
                data: node_data,
                executable: account.executable, // Get executable status from AccountInfo
                lamports: account.lamports, // Get lamports from AccountInfo
                account_data_display: account.account_data.clone(), // Store account data string
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
