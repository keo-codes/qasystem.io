use crate::semantic_embeddings::Embedding;

pub struct VectorIndex {
    pub embeddings: Vec<Embedding>,
}

impl VectorIndex {
    pub fn new() -> Self {
        Self { embeddings: Vec::new() }
    }

    pub fn add(&mut self, emb: Embedding) {
        self.embeddings.push(emb);
    }

    pub fn search(&self, query_vec: &[f32], k: usize) -> Vec<&Embedding> {
        let mut scored: Vec<(f32, &Embedding)> = self.embeddings
            .iter()
            .map(|e| (cosine_similarity(query_vec, &e.vector), e))
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scored.into_iter().take(k).map(|(_, e)| e).collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
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