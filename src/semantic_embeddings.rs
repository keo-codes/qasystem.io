use std::collections::HashMap;

#[derive(Clone)]
pub struct Embedding {
    pub vector: Vec<f32>,
    pub text: String,
}

pub struct TfIdfModel {
    vocab: HashMap<String, usize>,
    idf: HashMap<String, f32>,
}

impl TfIdfModel {
    pub fn new(docs: &[String]) -> Self {
        let mut vocab = HashMap::new();
        let mut df = HashMap::new();

        for doc in docs {
            let mut seen = std::collections::HashSet::new();
            for token in tokenize(doc) {
                vocab.entry(token.clone()).or_insert(vocab.len());
                if seen.insert(token.clone()) {
                    *df.entry(token).or_insert(0) += 1;
                }
            }
        }

        let n_docs = docs.len() as f32;
        let mut idf = HashMap::new();
        for (term, freq) in df {
            idf.insert(term, (n_docs / (1.0 + freq as f32)).ln());
        }

        Self { vocab, idf }
    }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut vec = vec![0.0; self.vocab.len()];
        let tokens = tokenize(text);

        let mut tf = HashMap::new();
        for t in tokens {
            *tf.entry(t).or_insert(0) += 1;
        }

        for (term, freq) in tf {
            if let Some(&idx) = self.vocab.get(&term) {
                let idf = *self.idf.get(&term).unwrap_or(&0.0);
                vec[idx] = freq as f32 * idf;
            }
        }

        vec
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace("’", "")
        .replace("'", "")
        .replace(",", "")
        .replace(".", "")
        .replace("|", "")
        .split_whitespace()
        .map(|s| s.to_string())
        .filter(|s| s.len() > 2)
        .collect()
}