use crate::kesa_utils::file_utils::{get_model_config_from_yaml, ModelConfig};
use crate::{convert_label::convert::ConvertSettings, kesa_utils::file_utils, yolo::yolo_rs};
use anyhow::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use std::{os::unix::process, path::PathBuf};

pub struct LabelSettings {
    pub model_name: String,
    pub model_config: ModelConfig,
    pub processor: Option<String>,
}

impl LabelSettings {
    pub fn new(
        model_name: String,
        model_config: String,
        processor: Option<String>,
    ) -> Result<LabelSettings, Error> {
        Ok(LabelSettings {
            model_name: model_name,
            model_config: get_model_config_from_yaml(&model_config)?,
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
