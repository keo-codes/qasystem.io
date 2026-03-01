use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct QAPair {
    pub question: String,
    pub answer: String,
}

pub fn generate(texts: Vec<String>) {
    let mut pairs = Vec::new();

    for t in texts {
        pairs.push(QAPair {
            question: format!("What is mentioned in the document?"),
            answer: t.chars().take(50).collect(),
        });
    }

    let json = serde_json::to_string_pretty(&pairs).unwrap();
    let mut file = File::create("data/synthetic/qa.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}