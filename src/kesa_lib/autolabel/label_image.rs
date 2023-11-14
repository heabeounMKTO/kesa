use crate::{convert_label::convert::ConvertSettings, yolo::yolo_rs};
use anyhow::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use serde_yaml::{self};
use std::{os::unix::process, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    names: Vec<String>,
    input_size_h: i64,
    input_size_w: i64,
}

pub struct LabelSettings {
    pub model_name: String,
    pub model_config: ModelConfig,
    pub processor: Option<String>,
}

pub fn read_model_config(input: &str) -> Result<ModelConfig> {
    let f = std::fs::File::open(input)?;
    let model_config: ModelConfig = serde_yaml::from_reader(f)?;
    Ok(model_config)
}

impl LabelSettings {
    pub fn new(
        model_name: String,
        model_config: String,
        processor: Option<String>,
    ) -> Result<LabelSettings, Error> {
        Ok(LabelSettings {
            model_name: model_name,
            model_config: read_model_config(&model_config)?,
            processor: match processor {
                Some(ref String) => processor,
                None => Some(String::from("local")),
            },
        })
    }

    // loads specific torchscript model with config
    pub fn load_model(&self) -> yolo_rs::YOLO {
        let cuda_device = tch::Device::cuda_if_available();
        let load_model = yolo_rs::YOLO::new(
            &self.model_name,
            self.model_config.input_size_h.to_owned(),
            self.model_config.input_size_w.to_owned(),
            cuda_device,
        );

        // warmup
        for n in 0..10 {
            load_model.warmup();
        }
        println!("model {:?} ready !", &self.model_name);
        load_model
    }
}
