use std::any::type_name;
use ascii::AsciiChar;

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

    

    fn check_type<T>(_: &T) -> &str {
        type_name::<T>()
    }
}