pub mod decoding {
    pub fn decode_single_byte(data: String) -> Option<char> {
        let clean_data = data.trim_start_matches("0x");

        let decode_data = u8::from_str_radix(clean_data, 16).expect("Invalid hex");

        let data = decode_data as char;
        Some(data)
    }

    fn decode_string_util(data: &[String]) -> String {
        let bytes: Vec<u8> = data
            .iter()
            .map(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).unwrap())
            .collect();

        let result = String::from_utf8(bytes).unwrap();

        result
    }

    pub fn decode_string(data: Vec<String>) -> Option<String> {
        let first_hex = &data[0];

        let clean_data = first_hex.trim_start_matches("0x");

        let decode_data = u8::from_str_radix(clean_data, 16).expect("Invalid hex");

        if decode_data >= 128 && decode_data <= 183 {
            let decode_data = &data[1..];

            let result = decode_string_util(decode_data);

            return Some(result);
        } else if decode_data >= 184 && decode_data <= 191 {
            let decode_data = &data[2..];

            let result = decode_string_util(decode_data);

            return Some(result);
        } else {
            None
        }
    }
}
