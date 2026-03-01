use burn::data::dataset::Dataset;
use docx_rs::*;
use std::fs;

#[derive(Clone)]
pub struct DocumentSample {
    pub text: String,
}

pub struct DocxDataset {
    samples: Vec<DocumentSample>,
}

impl DocxDataset {
    pub fn new(dir: &str) -> Self {
        let mut samples = Vec::new();

        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "docx" {
                let bytes = fs::read(&path).unwrap();
                let doc = read_docx(&bytes).unwrap();

                let mut text = String::new();

                for c in doc.document.children {
                    if let docx_rs::DocumentChild::Paragraph(p) = c {
                        for r in p.children {
                            if let docx_rs::ParagraphChild::Run(run) = r {
                                for rc in run.children {
                                    if let docx_rs::RunChild::Text(t) = rc {
                                        text.push_str(&t.text);
                                        text.push(' ');
                                    }
                                }
                            }
                        }
                    }
                }

                samples.push(DocumentSample { text });
            }
        }

        Self { samples }
    }
}

impl Dataset<DocumentSample> for DocxDataset {
    fn get(&self, index: usize) -> Option<DocumentSample> {
        self.samples.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.samples.len()
    }
}