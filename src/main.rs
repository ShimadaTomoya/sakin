pub mod inverted_index {
    pub struct Token {
        id: u64,
        body: String
    }

    pub struct Document {
        id: u64,
        body: String
    }

    pub fn build_token(id: u64, body: String) -> Token {
        Token {
            id,
            body
        }
    }

    pub fn build_document(id: u64, body: String) -> Document {
        Document {
            id,
            body
        }
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

fn main() {
    let mut _inverted_index: HashMap::<String,Vec<u64>> = HashMap::new();
    let file = File::open("./resource/sample.txt").expect("file not found");

    let mut document_id: u64 = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let tokens = divide_bigram(line.unwrap_or("".to_string()));
        for token in tokens {
            _inverted_index.entry(token)
            .and_modify(|vec| vec.push(document_id))
            .or_insert([document_id].to_vec());
        }
        document_id += 1;
    }
    println!("{:?}", _inverted_index);
}

pub fn divide_bigram(str: String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut prev:char = '-';
    for char in str.chars() {
        if char != '-' {
            let mut prev_str = prev.to_string();
            prev_str.push_str(&char.to_string());
            ret.push(prev_str);
        }
        prev = char;
    }
    return ret;
}