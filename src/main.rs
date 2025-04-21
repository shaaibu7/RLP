mod decoding;
mod encoding;

use std::{char, result};

use ascii::{AsciiChar, AsciiStr, ToAsciiChar};
use decoding::decode::decoding::{decode_single_byte, decode_string};
use encoding::encode::encoding::{
    encode_empty_data, encode_long_string, encode_short_string, encode_single_byte,
};

fn main() {
    
}
