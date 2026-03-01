use burn::{
    module::Module,
    nn::{
        Embedding, EmbeddingConfig,
        Linear, LinearConfig,
    },
    tensor::{Tensor, backend::Backend, Int},
};

#[derive(Module, Debug)]
pub struct TransformerQA<B: Backend> {
    token_emb: Embedding<B>,
    pos_emb: Embedding<B>,
    output: Linear<B>,
}

impl<B: Backend> TransformerQA<B> {
    pub fn new(vocab: usize, max_len: usize, d_model: usize, device: &B::Device) -> Self {
        let token_emb = EmbeddingConfig::new(vocab, d_model).init(device);
        let pos_emb = EmbeddingConfig::new(max_len, d_model).init(device);
        let output = LinearConfig::new(d_model, vocab).init(device);

        Self {
            token_emb,
            pos_emb,
            output,
        }
    }

    pub fn forward(&self, x: Tensor<B, 2, Int>) -> Tensor<B, 3> {
        let h = self.token_emb.forward(x);
        self.output.forward(h)
    }
}