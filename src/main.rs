mod encoding;

use std::{char, result};

use ascii::{AsciiChar, ToAsciiChar, AsciiStr};
use encoding::encode::encoding::{encode_empty_data, encode_single_byte, encode_short_string, encode_long_string};


fn main() {
    let bytes: Vec<u8>  = vec![];

    let name  = "suleiman";
    let bool = true;

    let num = 56;

    // let opt: Option<i32> = None;

    // let result = check_type(&opt);

    // println!("result of checking data type: {}", result);
    
    // let result = AsciiStr::from_ascii(&bytes);


    // println!("Hello, world!: {:?}", result.unwrap());

    let encode_res = encode_empty_data(bool);
    

    // println!("The result of the encoding is: {:?}", encode_res);
    // let charac = '1';
    // let result = encode_single_byte(charac);
    // println!("The result for encoding a single byte is: {:?}", result);

    let result = encode_long_string("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123");

    println!("The result of the op is: {:?}", result);

}




