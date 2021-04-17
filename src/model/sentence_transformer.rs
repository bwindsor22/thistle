use std::path::Path;

use rust_bert::Config;
use tch::{no_grad, Device, Tensor};

use crate::model::bert::{Bert, Features};
use crate::model::pooling::{Pooling, PoolingConfig};

pub struct SentenceTransformer {
    pub bert: Bert,
    pub pooling: Pooling,
}

impl SentenceTransformer {
    pub fn new(model_path: &Path, device: Device) -> failure::Fallible<SentenceTransformer> {
        let bert_model_path = model_path.join("0_BERT");
        let pooling_config_path = model_path.join("1_Pooling/config.json");

        let bert = Bert::new(&bert_model_path.as_path(), None, None, device);
        let pooling = Pooling::new(
            &(&bert.vs.root() / "pooling"),
            &PoolingConfig::from_file(Path::new(&pooling_config_path)),
        );

        Ok(SentenceTransformer { bert, pooling })
    }

    pub fn encode(&self, text: &str) -> Vec<f64> {
        let mut longest_seq = 0;

        let tokens = self.bert.tokenize(text);
        longest_seq = longest_seq.max(tokens.len());

        let mut features = Features::default();
        let mut input_ids_feature = Vec::new();
        let mut token_type_ids_feature = Vec::new();
        let mut input_mask_feature = Vec::new();

        let (input_ids, token_type_ids, input_mask, _sentence_length) =
            self.bert.get_sentence_features(&tokens, longest_seq);
        input_ids_feature.push(Tensor::of_slice(&input_ids));
        token_type_ids_feature.push(Tensor::of_slice(&token_type_ids));
        input_mask_feature.push(Tensor::of_slice(&input_mask));

        features.input_ids =
            Some(Tensor::stack(input_ids_feature.as_slice(), 0).to(self.bert.vs.device()));
        features.token_type_ids =
            Some(Tensor::stack(token_type_ids_feature.as_slice(), 0).to(self.bert.vs.device()));
        features.input_mask =
            Some(Tensor::stack(input_mask_feature.as_slice(), 0).to(self.bert.vs.device()));

        let features = no_grad(|| self.bert.forward_t(features));

        let features = no_grad(|| self.pooling.forward_t(features));

        Vec::<f64>::from(&features.sentence_embedding.unwrap())
    }
}
