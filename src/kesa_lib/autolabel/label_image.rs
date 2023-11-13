use yolo::yolo_rs;
use anyhow::Result;
use serde_derive::{Serialize, Deserialize};
use serde_yaml::{self};
use std::path::PathBuf;
use crate::convert_label::convert::ConvertSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    names: Vec<String>
}


pub struct LabelSettings {
    pub model_name: String,
    pub model_config: ModelConfig,
    pub processor: Option<String>
}

impl LabelSettings {
    pub fn new(
        model_name: String,
        model_config: String,
    ) -> LabelSettings {
        todo!()
    } 
}


// load a torchscript model
fn load_yolo(input: &str) -> yolo_rs::YOLO {
    let cuda_device = tch::Device::cuda_if_available();
    let load_model = yolo_rs::YOLO::new(
        &input, 320, 320, cuda_device
    );
    for n in 0..3 {
        load_model.warmup();
    }
    println!("model {::?} ready", &input);
    load_model
}