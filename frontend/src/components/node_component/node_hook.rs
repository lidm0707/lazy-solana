use std::collections::HashMap;

use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AccountInfo {
    pub pubkey: String,
    pub lamports: u64,
}

#[derive(Clone, Props, PartialEq)]
pub struct PropNode {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub data: HashMap<String, String>, // Add pubkey to display account info
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
                let mut data = HashMap::new();
                data.insert("pubkey".to_string(), addr_program.to_owned());
                data
            },
        });
        for (i, account) in data.iter().enumerate() {
            let mut data: HashMap<String, String> = HashMap::new();
            let i = i + 1;
            new_edges.push(PropEdge {
                from: 0usize,
                to: i,
            });
            data.insert("pubkey".to_string(), account.pubkey.clone().clone());
            new_nodes.push(PropNode {
                id: i,
                x: 100.0 + (i % 5) as f32 * 100.0,
                y: 200.0 + (i / 5) as f32 * 100.0,
                data: data,
            })
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
