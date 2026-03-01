# Question and Answering System with Rust and Burn Framework 

**Word Document Question Answering** built in Rust with Burn framework  
(developed in < 9 days)

## Quick Summary

- **Goal**: Semantic QA over `.docx` documents  
- **Current state**: Structured + semantic retrieval (TF-IDF + cosine)  
- **Language**: Rust  
- **ML framework**: Burn  
- **Status**:  
  - Phase 1 (Structured QA) → Complete  
  - Phase 2 (Semantic QA) → Complete  
  - Phase 3 (Full transformer training) → Planned

## System Flow (Current)
Document Store (.docx / structured text)
          ↓
     Document Loader
          ↓
Structured Data Representation
          ↓
     Tokenization Layer
          ↓
   Embedding Generation
          ↓
 Vector Similarity Engine
          ↓
 Semantic Retrieval Engine
          ↓
     QA Inference Engine
          ↓
  Command-Line Interface

  
## Planned Transformer (Phase 3)
      Input Tokens
               ↓
   Token Embeddings (256-dim)
               ↓
Positional Embeddings (256-dim)
               ↓
  Transformer Encoder Layer 1
               ↓
  Transformer Encoder Layer 2
               ↓
  Transformer Encoder Layer 3
               ↓
  Transformer Encoder Layer 4
               ↓
  Transformer Encoder Layer 5
               ↓
  Transformer Encoder Layer 6
               ↓
  Contextual Representations
               ↓
   Output Projection Layer
               ↓
  Answer Span Prediction Head

  
## Example Questions & Answers

- **Q:** What month and date will the 2026 End of year Graduation Ceremony be held?  
  **A:** December 2026-12-31 – End of Year Graduation Ceremony

- **Q:** Which events occur in December?  
  **A:** Day of Reconciliation, Christmas Day, End of Year Graduation Ceremony

- **Q:** What public holidays occur in 2026?  
  **A:** Human Rights Day, Freedom Day, Workers Day, Youth Day, National Women’s Day, Heritage Day, Day of Reconciliation, Christmas Day

## Key Design Choices

- Rust + Burn for performance & future ML pipeline  
- Phased approach due to < 9-day constraint  
- Modular structure (data / retrieval / inference separation)  
- TF-IDF embeddings (fast, low-resource) → transformer planned

## Challenges

- Extreme time limit (< 9 days) → solved with phased architecture  
- Burn dev-dependencies issue → `features = ["test"]` commented out  
- Large scope → modular + staged implementation

## What Works Well

- Grounded answers from documents  
- Stable semantic retrieval  
- Clean, extensible architecture

## Limitations (now)

- No neural model yet  
- No contextual embeddings  
- Basic multi-answer handling  
- No temporal/entity reasoning

## Next Steps (Phase 3)

- Burn Dataset + training pipeline  
- Transformer-based QA model  
- End-to-end training & checkpointing  
- Neural inference

**A functional ML system skeleton built under tight time pressure — ready to grow into full transformer-powered document QA.**
