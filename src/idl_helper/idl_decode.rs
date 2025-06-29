use anyhow::{Result, anyhow};
use dioxus_logger::tracing;

pub fn use_spec_idl(idl_types: Vec<String>, data: &[u8]) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut position_data = 0usize;

    tracing::debug!("types: {:?}", &data[0..7]);
    let data = &data[8..];
    tracing::debug!("u8: {:?}", data);
    for idl_type in idl_types.into_iter() {
        match idl_type.as_str() {
            "string" => {
                let len_bytes_start = position_data;
                // Check if enough bytes exist for string length (4 bytes for u32)
                if position_data + 4 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for string length at position {}",
                        position_data
                    ));
                }
                let length =
                    u32::from_le_bytes(data[position_data..len_bytes_start + 4].try_into()?);
                position_data += 4;

                let end_string = position_data + length as usize;
                if end_string > data.len() {
                    tracing::debug!("position_data: {:?}", position_data);
                    tracing::debug!(" data.len(): {:?}", data.len());
                    tracing::debug!("length: {:?}", length);
                    tracing::debug!("end_string: {:?}", end_string);
                    return Err(anyhow!(
                        "Not enough bytes for string data of length {} at position {}",
                        length,
                        position_data
                    ));
                }

                let string_data = data[position_data..end_string].to_vec();
                result.push(String::from_utf8(string_data)?);
                position_data = end_string;
            }
            "u8" => {
                if position_data + 1 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for u8 at position {}",
                        position_data
                    ));
                }
                let value = data[position_data];
                result.push(value.to_string());
                position_data += 1;
            }
            "u16" => {
                if position_data + 2 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for u16 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 2] = data[position_data..position_data + 2].try_into()?;
                let value = u16::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 2;
            }
            "u32" => {
                if position_data + 4 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for u32 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 4] = data[position_data..position_data + 4].try_into()?;
                let value = u32::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 4;
            }
            "u64" => {
                if position_data + 8 > data.len() {
                    tracing::debug!("u64: {:?}", position_data + 8);

                    return Err(anyhow!(
                        "Not enough bytes for u64 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 8] = data[position_data..position_data + 8].try_into()?;
                let value = u64::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 8;
            }
            "u128" => {
                if position_data + 16 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for u128 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 16] = data[position_data..position_data + 16].try_into()?;
                let value = u128::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 16;
            }
            "i8" => {
                if position_data + 1 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for i8 at position {}",
                        position_data
                    ));
                }
                let value = data[position_data] as i8;
                result.push(value.to_string());
                position_data += 1;
            }
            "i16" => {
                if position_data + 2 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for i16 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 2] = data[position_data..position_data + 2].try_into()?;
                let value = i16::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 2;
            }
            "i32" => {
                if position_data + 4 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for i32 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 4] = data[position_data..position_data + 4].try_into()?;
                let value = i32::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 4;
            }
            "i64" => {
                if position_data + 8 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for i64 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 8] = data[position_data..position_data + 8].try_into()?;
                let value = i64::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 8;
            }
            "i128" => {
                if position_data + 16 > data.len() {
                    return Err(anyhow!(
                        "Not enough bytes for i128 at position {}",
                        position_data
                    ));
                }
                let bytes: [u8; 16] = data[position_data..position_data + 16].try_into()?;
                let value = i128::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data += 16;
            }
            _ => {
                return Err(anyhow!("Unknown IDL type: {}", idl_type));
            }
        }
    }
    tracing::debug!("result: {:?}", result);

    Ok(result)
}
