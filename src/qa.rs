use std::fs;
use std::collections::HashMap;
use docx_rs::*;

#[derive(Clone)]
pub struct Chunk {
    pub text: String,
}

pub struct QAIndex {
    chunks: Vec<Chunk>,
}

impl QAIndex {
    pub fn new(doc_dir: &str) -> Self {
        let mut chunks = Vec::new();

        for entry in fs::read_dir(doc_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "docx" {
                let bytes = fs::read(&path).unwrap();
                let doc = read_docx(&bytes).unwrap();

                let mut full_text = String::new();

                // Extract raw text
                for c in doc.document.children {
                    if let DocumentChild::Paragraph(p) = c {
                        for r in p.children {
                            if let ParagraphChild::Run(run) = r {
                                for rc in run.children {
                                    if let RunChild::Text(t) = rc {
                                        full_text.push_str(&t.text);
                                        full_text.push('\n');
                                    }
                                }
                            }
                        }
                    }
                }

                // Line-based chunking (better for calendars / tables)
                for line in full_text.lines() {
                    let l = line.trim();
                    if !l.is_empty() {
                        chunks.push(Chunk { text: l.to_string() });
                    }
                }

                // Sliding window chunks (context blocks)
                let window = 4;
                let raw: Vec<String> = full_text
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect();

                for i in 0..raw.len() {
                    let end = (i + window).min(raw.len());
                    let block = raw[i..end].join(" ");
                    if !block.is_empty() {
                        chunks.push(Chunk { text: block });
                    }
                }
            }
        }

        Self { chunks }
    }

    pub fn answer(&self, question: &str) -> String {
        let q_tokens = normalize_tokens(question);

        let mut best_score = 0i32;
        let mut best_chunk = None;

        for chunk in &self.chunks {
            let c_tokens = normalize_tokens(&chunk.text);
            let score = semantic_score(&q_tokens, &c_tokens, &chunk.text, question);

            if score > best_score {
                best_score = score;
                best_chunk = Some(chunk.text.clone());
            }
        }

        match best_chunk {
            Some(t) => clean_answer(t),
            None => "No relevant answer found in document.".to_string(),
        }
    }
}

/* ---------------- NLP UTILITIES ---------------- */

fn normalize_tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace("2026", "year")
        .replace("2025", "year")
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn semantic_score(
    q: &[String],
    c: &[String],
    raw_chunk: &str,
    question: &str,
) -> i32 {
    let mut score = 0;

    let freq = freq_map(c);

    // keyword overlap
    for token in q {
        if freq.contains_key(token) {
            score += 3;
        }
    }

    // month matching
    let months = [
        "january","february","march","april","may","june",
        "july","august","september","october","november","december"
    ];

    for m in months {
        if question.to_lowercase().contains(m) && raw_chunk.to_lowercase().contains(m) {
            score += 15;
        }
    }

    // year relevance
    if raw_chunk.contains("2026") && question.contains("2026") {
        score += 10;
    }

    // question intent boosting
    if question.to_lowercase().contains("when") {
        if raw_chunk.chars().any(|c| c.is_numeric()) {
            score += 5;
        }
    }

    score
}

fn freq_map(tokens: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for t in tokens {
        *map.entry(t.clone()).or_insert(0) += 1;
    }
    map
}

fn clean_answer(text: String) -> String {
    text.replace("  ", " ")
        .replace("\n", " ")
        .trim()
        .to_string()
}