use burn::tensor::{Tensor, Int};
use burn::tensor::backend::Backend;
use crate::model::TransformerQA;

pub fn answer<B: Backend>(
    model: &TransformerQA<B>,
    input: Tensor<B, 2, Int>,
) {
    let _out = model.forward(input);
    println!("Inference executed.");
}