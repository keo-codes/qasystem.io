use tokenizers::Tokenizer;

pub fn load_tokenizer() -> Tokenizer {
    // You must have tokenizer.json in project root
    Tokenizer::from_file("tokenizer.json").unwrap()
}

pub fn tokenize(tokenizer: &Tokenizer, text: &str) -> Vec<u32> {
    let enc = tokenizer.encode(text, true).unwrap();
    enc.get_ids().to_vec()
}