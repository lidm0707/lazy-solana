use base64::{Engine as _, engine::general_purpose};
use std::pin::Pin;

use dioxus::{logger::tracing, prelude::*};
use reslt::prelude::*;

use crate::{
    components::table_component::table::DataTable,
    fecth::rpc_service::search_account,
    idl_helper::{idl_decode::use_spec_idl, idl_node::IDLNodeField},
};

#[derive(Debug, Clone, PartialEq, Eq)] // Props derive is not needed here
pub struct AccountDetail {
    pub programe_addr: String,
    pub discriminator: Vec<u8>,
    pub name_account: String,
    pub fields: Vec<IDLNodeField>,
}

pub fn get_table_data(
    _start: usize,
    _end: usize,
    _sort: (String, bool),
) -> Pin<Box<dyn 'static + Future<Output = (PropData<DataTable>, usize)>>> {
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let program_address = use_context::<Signal<AccountDetail>>()().programe_addr;
    let target_discriminator = use_context::<Signal<AccountDetail>>()().discriminator;

    Box::pin(async move {
        match search_account(rpc_url, program_address, target_discriminator).await {
            Ok(result_data) => {
                // Map the fetched result into DataTable format
                let table_data: Vec<DataTable> = result_data
                    .result
                    .into_iter()
                    .map(|entry| {
                        let tys = use_context::<Signal<AccountDetail>>()().fields;
                        let mut ty_list = Vec::<String>::new();
                        let mut name_list = Vec::<String>::new();
                        for ty in tys.iter() {
                            ty_list.push(ty.ty.clone());
                            name_list.push(ty.name.clone());
                        }
                        let mut vec8: Vec<u8> = vec![];
                        let data_base64 = entry.account.data.get(0).cloned().unwrap_or_default();
                        if let Ok(decoded_data) = general_purpose::STANDARD.decode(data_base64) {
                            vec8.extend_from_slice(&decoded_data);
                        }
                        let data = use_spec_idl(ty_list.clone(), &vec8).unwrap_or_default();
                        tracing::debug!("data decode : {:?}", data);

                        let mut word_data = String::new();
                        for (i, word) in data.iter().enumerate() {
                            word_data.push_str(&format!("{}: {}\n", name_list[i], word.clone()));
                        }

                        DataTable {
                            addr: entry.pubkey,
                            data: word_data,
                            space: entry.account.space.to_string(),
                        }
                    })
                    .collect();
                // Wrap data into PropData with count
                let prop_data = PropData {
                    data_vec: table_data,
                };
                let total_count = prop_data.data_vec.len();

                (prop_data, total_count)
            }
            Err(err) => {
                eprintln!("Error fetching account data: {:?}", err);
                // Return empty data and count 0 on error
                tracing::debug!("error data {:?}", err);
                (PropData { data_vec: vec![] }, 0)
            }
        }
    })
}
