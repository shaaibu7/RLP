use std::any::type_name;
use ascii::{AsciiChar, AsciiStr};

pub mod encoding {

    use super::*;

    pub fn encode_empty_data<T>(data: T) -> Option<String> {
        let data_type = check_type(&data);
    
        if data_type == "&str" || data_type == "bool"  {
            Some(format!("0x{:02x}", 0x80))
        } else if data_type == "alloc::vec::Vec<u8>" {
            Some(format!("0x{:02x}", 0xc0))
        } else {
            None
        }
    }

    pub fn encode_single_byte(data: char) -> Option<String> {
        let encode_data = AsciiChar::from_ascii(data);
        let char_to_hex = encode_data.unwrap().as_byte();
        let encoded_data = format!("0x{:02x}", char_to_hex);
        Some(encoded_data)
    }

    pub fn encode_short_string(data: &str) -> Option<Vec<String>> {
        let ascii_str = AsciiStr::from_ascii(data).expect("Invalid ascii data");
     
        let result: Vec<char> = data.chars().collect();
     
        if result.len() > 55 {
         return None
        }
     
        let append_data = result.len() as u8 + 0x80;
        let append_data_hex = format!("0x{:02x}", append_data);
     
        let mut char_to_hex: Vec<String> = ascii_str.as_bytes()
                                     .iter()
                                     .map(|b| format!("0x{:02x}", b))
                                     .collect();
     
         char_to_hex.insert(0, append_data_hex.to_string());
         Some(char_to_hex)
     }

    

    fn check_type<T>(_: &T) -> &str {
        type_name::<T>()
    }
}