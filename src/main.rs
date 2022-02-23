use std::collections::HashMap;
use std::fs::File;
use rusqlite::{params, Connection, Result};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct InvertedIndex {
    token: String,
    document_ids: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
    "CREATE TABLE IF NOT EXISTS inverted_index (
            token           TEXT PRIMARY KEY,
            document_ids    BLOB
        )",
[],
    )?;

    conn.execute("begin transaction a", [])?;

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
    // println!("{:?}", _inverted_index);

    for (token, _document_ids) in _inverted_index {
        let tmp: Vec<String> = _document_ids.iter().map(|i| i.to_string()).collect();
        let document_ids: String = tmp.join(",");
        conn.execute(
            "INSERT INTO inverted_index (token, document_ids) VALUES (?1, ?2)",
            params![token, document_ids],
        )?;
    }

    conn.execute("commit transaction a", [])?;

    let mut stmt = conn.prepare("SELECT * FROM inverted_index")?;
    let index_iter = stmt.query_map([], |row| {
        Ok(InvertedIndex {
            token: row.get(0)?,
            document_ids: row.get(1)?,
        })
    })?;

    for index in index_iter {
        println!("Found index {:?}", index.unwrap());
    }

    Ok(())
}

pub fn divide_bigram(str: String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let chars: Vec<char> = str.chars().collect();
    for i in 0..chars.len()-1 {
        let token_vec: Vec<String> = chars[i..i+2].iter().map(|c| c.to_string()).collect();
        let token = token_vec.join("");
        ret.push(token);
    }
    return ret;
}
