extern crate serde;
extern crate serde_json;

use serde_json::Error;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;

fn main () {
    let text = read_text();

    let mut i = 0;
    let mut j = 0;
    let mut word_counts: HashMap<String, u64> = HashMap::new();
    let mut character = false;
    for c in text.chars() {
        if c == '\n' || c == ' ' || c == '.' || c == ',' || c == '!'
        || c == '?' || c == ';' || c == '"' || c == '(' || c == ')' {
            if character {
                let result = text.get(j..i);
                let word;
                match result {
                    Some(x) => word = String::from(x),
                    None    => word = "farts".to_string(),
                }
                let count = word_counts.entry(word).or_insert(0);
                *count += 1;
            }
            j = i + 1;
            character = false;
        } else {
            character = true;
        }
        i += 1;
    }

    write_results(word_counts).expect("crap");
}

fn read_text () -> String {
    let mut file = File::open("text/atlasshrugged.txt").expect("Couldn't open");
    let metadata = fs::metadata("text/atlasshrugged.txt").expect("Couldn't get metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Couldn't read");
    return String::from_utf8(buffer).expect("Couldn't convert buffer to string");
}

fn write_results (result: HashMap<String, u64>) -> Result<(), Error> {
    // Serialize it to a JSON string.
    let j = serde_json::to_string(&result)?;

    let mut file = File::create("out/result.json").expect("Could not create file");
    file.write_all(j.as_bytes()).expect("Could not write to file");

    Ok(())
}
