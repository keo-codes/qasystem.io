pub struct Metrics {
    pub loss: f64,
    pub accuracy: f64,
    pub perplexity: f64,
}

impl Metrics {
    pub fn compute(loss: f64, correct: usize, total: usize) -> Self {
        let acc = correct as f64 / total as f64;
        let ppl = loss.exp();
        Self { loss, accuracy: acc, perplexity: ppl }
    }
}