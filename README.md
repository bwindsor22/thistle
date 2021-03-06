# Big Data and ML - Spring 2021
Brad Windsor (bw1879), Kevin Choi (kc2296)

## Thistle
0. [Final project proposal](https://github.com/nyu-bigdata-class/final-project-proposals/blob/main/bw1879-kc2296/project.md)

1. One needs to download the pretrained BERT model from https://public.ukp.informatik.tu-darmstadt.de/reimers/sentence-transformers/v0.2/ and perform a conversion into a Rust-compatible form first. Run the following commands:
```
mkdir -p models/bert-base-nli-stsb-mean-tokens

wget -P models https://public.ukp.informatik.tu-darmstadt.de/reimers/sentence-transformers/v0.2/bert-base-nli-stsb-mean-tokens.zip

unzip models/bert-base-nli-stsb-mean-tokens.zip -d models/bert-base-nli-stsb-mean-tokens

python3 -m venv thistle-env

source thistle-env/bin/activate

pip install torch

export PWD=`pwd`

python3 utils/convert_model.py $PWD/models/bert-base-nli-stsb-mean-tokens/0_BERT/pytorch_model.bin
```

2. Modifying Rust. This project uses some features of Rust that are not yet on the stable build. To use the nightly build, set:
```
rustup toolchain install nightly

rustup default nightly
```

3. Integration testing
```
cargo test
```

### Running MS MARCO dataset
Data preparation
```
mkdir data
cd data
wget https://msmarco.blob.core.windows.net/msmarcoranking/triples.train.small.tar.gz
tar -xf triples.train.small.tar.gz
export SIZE=10000 # or any other size
head -n $SIZE triples.train.small.tsv > data.tsv
LC_ALL=C tr -dc '\0-\177' <data.tsv >data_cleaned.tsv
cd ..
```

To run:
```
# see run_eval.rs
cargo run > output100.txt
```

### References
* [BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding](https://arxiv.org/pdf/1810.04805.pdf)
* [RoBERTa: A Robustly Optimized BERT Pretraining Approach](https://arxiv.org/pdf/1907.11692.pdf)
* [Sentence-BERT: Sentence Embeddings using Siamese BERT-Networks](https://arxiv.org/pdf/1908.10084.pdf)
* https://github.com/UKPLab/sentence-transformers
* https://github.com/guillaume-be/rust-bert
* https://github.com/cpcdoy/rust-sbert
* https://github.com/mladvladimir/rust-sentence-transformers
* https://github.com/granne/granne
* https://github.com/rust-cv/hnsw
* https://github.com/jean-pierreBoth/hnswlib-rs
* https://github.com/ritchie46/lsh-rs
* https://github.com/microsoft/MSMARCO-Passage-Ranking
