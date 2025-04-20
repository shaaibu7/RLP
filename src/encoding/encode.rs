use std::{any::type_name, result};

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

    

    fn check_type<T>(_: &T) -> &str {
        type_name::<T>()
    }
}