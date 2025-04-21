mod decoding;
mod encoding;

use std::{char, result};

use ascii::{AsciiChar, AsciiStr, ToAsciiChar};
use decoding::decode::decoding::{decode_single_byte, decode_string};
use encoding::encode::encoding::{
    encode_empty_data, encode_long_string, encode_short_string, encode_single_byte,
};

fn main() {
    // encoding for empty data

    let string_data = encode_empty_data(""); // empty string encoding returns "0x80"
    
    assert_eq!(string_data.unwrap(), "0x80");

    let bool_data = encode_empty_data(false); // false value encoding returns "0x80"

    assert_eq!(bool_data.unwrap(), "0x80");

    let encoding_data: Vec<u8> = vec![];

    let vec_data = encode_empty_data(encoding_data); // empty list encoding returns "0xc0"

    assert_eq!(vec_data.unwrap(), "0xc0");



    // encoding for single byte

    let single_byte = encode_single_byte('A'); // encoding a char which is a byte should return 
                                                                    // the hex representation of the char encode directly
    assert_eq!(single_byte.unwrap(), "0x41");

    let single_byte = encode_single_byte('a');

    assert_eq!(single_byte.unwrap(), "0x61");


    // decoding for single byte data

    let single_byte_decode = encode_single_byte('A');

    let decoding_result = decode_single_byte(single_byte_decode.unwrap());

    assert_eq!(decoding_result.unwrap(), 'A');

    let single_byte_decode = encode_single_byte('a');

    let decoding_result = decode_single_byte(single_byte_decode.unwrap());

    assert_eq!(decoding_result.unwrap(), 'a');


    // encoding for short strings

    let short_string = encode_short_string("dog");

    assert_eq!(short_string.unwrap(), ["0x83", "0x64", "0x6f", "0x67"]);

    // decoding for short strings

    let short_string_decode = encode_short_string("dog");

    let decode_result = decode_string(short_string_decode.unwrap());

    assert_eq!(decode_result.unwrap(), "dog");


    //encoding for long strings

    let data = "The quick brown fox jumps over the lazy dog, really fasts";

    let long_string_encoding = encode_long_string(data);

    let long_string_decoding = decode_string(long_string_encoding.unwrap());

    assert_eq!(long_string_decoding.unwrap(), data);

    
}
