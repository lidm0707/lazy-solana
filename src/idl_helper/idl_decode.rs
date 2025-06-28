use anyhow::Result;

pub fn use_spec_idl(idl_types: Vec<String>, data: &[u8]) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut position_data = 0usize;

    for i in idl_types.into_iter() {
        match i.as_str() {
            "string" => {
                let len_bytes_start = position_data;
                let len_bytes_end = len_bytes_start + 4;
                println!("{:?}", len_bytes_end);
                // Check if there are enough bytes for the string length
                if len_bytes_end > data.len() {
                } else {
                    let length =
                        u32::from_le_bytes(data[len_bytes_start..len_bytes_end].try_into()?);
                    println!("{:?}", length);

                    let string_data_start = len_bytes_end;
                    let string_data_end = string_data_start + length as usize;

                    // Check if there are enough bytes for the actual string data
                    if string_data_end > data.len() {}
                    let string_data = data[string_data_start..string_data_end].to_vec();
                    result.push(String::from_utf8(string_data)?); // Use '?' for error propagation
                    position_data = string_data_end;
                }
            }
            "u8" => {
                let end_pos = position_data + 1;
                if end_pos > data.len() {}
                let value = data[position_data];
                result.push(value.to_string());
                position_data = end_pos;
            }
            "u16" => {
                let end_pos = position_data + 2;
                if end_pos > data.len() {}
                let bytes: [u8; 2] = data[position_data..end_pos].try_into()?;
                let value = u16::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "u32" => {
                let end_pos = position_data + 4;
                if end_pos > data.len() {}
                let bytes: [u8; 4] = data[position_data..end_pos].try_into()?;
                let value = u32::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "u64" => {
                let end_pos = position_data + 8;
                if end_pos > data.len() {}
                let bytes: [u8; 8] = data[position_data..end_pos].try_into()?;
                let value = u64::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "u128" => {
                let end_pos = position_data + 16;
                if end_pos > data.len() {}
                let bytes: [u8; 16] = data[position_data..end_pos].try_into()?;
                let value = u128::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "i8" => {
                let end_pos = position_data + 1;
                if end_pos > data.len() {}
                // For single-byte types, `try_into()` to `[u8; 1]` is fine
                // or you could simply cast: `data[position_data] as i8`
                let value = i8::from_le_bytes(data[position_data..end_pos].try_into()?);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "i16" => {
                let end_pos = position_data + 2;
                if end_pos > data.len() {}
                let bytes: [u8; 2] = data[position_data..end_pos].try_into()?;
                let value = i16::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "i32" => {
                let end_pos = position_data + 4;
                if end_pos > data.len() {}
                let bytes: [u8; 4] = data[position_data..end_pos].try_into()?;
                let value = i32::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "i64" => {
                let end_pos = position_data + 8;
                if end_pos > data.len() {}
                let bytes: [u8; 8] = data[position_data..end_pos].try_into()?;
                let value = i64::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            "i128" => {
                let end_pos = position_data + 16;
                if end_pos > data.len() {}
                let bytes: [u8; 16] = data[position_data..end_pos].try_into()?;
                let value = i128::from_le_bytes(bytes);
                result.push(value.to_string());
                position_data = end_pos;
            }
            _ => {
                // If an unknown type is encountered, return an error.
                // Alternatively, you could skip it or handle it based on your requirements.
            }
        }
    }

    // Return the result wrapped in Ok()
    Ok(result)
}
