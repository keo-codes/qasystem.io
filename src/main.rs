use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Event {
    text: String,
    month: String,
    date: String,
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace(",", " ")
        .replace(".", " ")
        .replace("?", " ")
        .replace("!", " ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn build_vocab(docs: &[String]) -> HashMap<String, usize> {
    let mut vocab = HashMap::new();
    for doc in docs {
        for tok in tokenize(doc) {
            if !vocab.contains_key(&tok) {
                let idx = vocab.len();
                vocab.insert(tok, idx);
            }
        }
    }
    vocab
}

fn tf(doc: &[String], vocab: &HashMap<String, usize>) -> Vec<f32> {
    let mut vec = vec![0.0; vocab.len()];
    for tok in doc {
        if let Some(&i) = vocab.get(tok) {
            vec[i] += 1.0;
        }
    }
    let norm: f32 = vec.iter().sum();
    if norm > 0.0 {
        for v in vec.iter_mut() {
            *v /= norm;
        }
    }
    vec
}

fn idf(docs: &[Vec<String>], vocab: &HashMap<String, usize>) -> Vec<f32> {
    let mut vec = vec![0.0; vocab.len()];
    let n_docs = docs.len() as f32;

    for (term, &i) in vocab {
        let mut df = 0.0;
        for doc in docs {
            if doc.contains(term) {
                df += 1.0;
            }
        }
        vec[i] = (n_docs / (1.0 + df)).ln();
    }
    vec
}

fn tfidf(tf: &[f32], idf: &[f32]) -> Vec<f32> {
    tf.iter().zip(idf.iter()).map(|(a, b)| a * b).collect()
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }

    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

fn load_events(path: &str) -> Vec<Event> {
    let content = fs::read_to_string(path).expect("Failed to read calendar file");

    // Simple structured format:
    // MONTH|DATE|EVENT
    let mut events = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            events.push(Event {
                month: parts[0].to_string(),
                date: parts[1].to_string(),
                text: parts[2].to_string(),
            });
        }
    }

    events
}

fn semantic_answer(question: &str, events: &[Event]) -> String {
    let docs: Vec<String> = events.iter().map(|e| e.text.clone()).collect();
    let tokenized_docs: Vec<Vec<String>> = docs.iter().map(|d| tokenize(d)).collect();

    let vocab = build_vocab(&docs);
    let idf_vec = idf(&tokenized_docs, &vocab);

    let doc_vectors: Vec<Vec<f32>> = tokenized_docs
        .iter()
        .map(|doc| tfidf(&tf(doc, &vocab), &idf_vec))
        .collect();

    let q_tokens = tokenize(question);
    let q_tf = tf(&q_tokens, &vocab);
    let q_vec = tfidf(&q_tf, &idf_vec);

    let mut best_score = 0.0;
    let mut best_idx = None;

    for (i, vec) in doc_vectors.iter().enumerate() {
        let score = cosine(&q_vec, vec);
        if score > best_score {
            best_score = score;
            best_idx = Some(i);
        }
    }

    if let Some(i) = best_idx {
        let e = &events[i];
        format!(
            "MONTH: {} DATE: {} EVENT: {}",
            e.month, e.date, e.text
        )
    } else {
        "No relevant answer found in document.".to_string()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- ask \"Your question here\"");
        return;
    }

    let command = &args[1];

    if command == "ask" {
        let question = args[2..].join(" ");
        println!("[SYSTEM] Question: {}", question);

        // Structured calendar file
        // Format: MONTH|DATE|EVENT
        let events = load_events("data/documents/calendar_2026.txt");

        let answer = semantic_answer(&question, &events);
        println!("Answer: {}", answer);
        println!("Inference executed.");
    } else {
        println!("Unknown command.");
    }
}