pub mod decoding {
    pub fn decode_single_byte(data: String) -> Option<char> {
        let clean_data = data.trim_start_matches("0x");
    
        let decode_data = u8::from_str_radix(clean_data, 16).expect("Invalid hex");
    
        let data = decode_data as char;
        Some(data)
    }
}