# Big Data and ML - Spring 2021
Brad Windsor (bw1879), Kevin Choi (kc2296)

## Thistle
0. [Final project proposal](https://github.com/nyu-bigdata-class/final-project-proposals/blob/main/bw1879-kc2296/project.md)

1. One needs to download the pretrained BERT model from https://public.ukp.informatik.tu-darmstadt.de/reimers/sentence-transformers/v0.2/ and perform a conversion into a Rust-compatible form first. Run the following commands:
```
mkdir -p models/bert-base-nli-stsb-mean-tokens

wget -P models https://public.ukp.informatik.tu-darmstadt.de/reimers/sentence-transformers/v0.2/bert-base-nli-stsb-mean-tokens.zip

unzip models/bert-base-nli-stsb-mean-tokens.zip -d models/bert-base-nli-stsb-mean-tokens

pip install torch

python utils/convert_model.py models/bert-base-nli-stsb-mean-tokens
```

2. Modifying Rust
This project uses some features of rust that are not yet on the stable build. To use the nightly build, set 
``` 
rustup toolchain install nightly

rustup default nightly
```

### References
* [BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding](https://arxiv.org/pdf/1810.04805.pdf)
* [RoBERTa: A Robustly Optimized BERT Pretraining Approach](https://arxiv.org/pdf/1907.11692.pdf)
* [Sentence-BERT: Sentence Embeddings using Siamese BERT-Networks](https://arxiv.org/pdf/1908.10084.pdf)
* https://github.com/UKPLab/sentence-transformers
* https://github.com/guillaume-be/rust-bert
* https://github.com/cpcdoy/rust-sbert
* https://github.com/mladvladimir/rust-sentence-transformers