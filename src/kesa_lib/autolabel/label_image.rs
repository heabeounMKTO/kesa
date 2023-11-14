use yolo::yolo_rs;
use anyhow::Result;
use serde_derive::{Serialize, Deserialize};
use serde_yaml::{self};
use std::{path::PathBuf, os::unix::process};
use crate::{convert_label::convert::ConvertSettings, yolo::yolo_rs};


#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    names: Vec<String>
}


pub struct LabelSettings {
    pub model_name: String,
    pub model_config: ModelConfig,
    pub processor: Option<String>
}

fn read_model_config(input: &str) -> Result<ModelConfig> {
    let f = std::fs::File::open(input)?;
    let model_config: ModelConfig = serde_yaml::from_reader(f)?;
    Ok(model_config)
}

impl LabelSettings {
    pub fn new(
        model_name: String,
        model_config: String,
        processor: Option<String>
    ) -> LabelSettings {
        LabelSettings { model_name: model_name 
                        , model_config: read_model_config(&model_config) 
                        , processor: match processor {
                                Some(ref String) => processor,
                                None => Some(String::from("local"))
                        }  }    
    }

    pub fn load_model(&self) -> yolo_rs::YOLO {
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