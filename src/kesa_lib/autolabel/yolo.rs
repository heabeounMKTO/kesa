use serde_derive::{Serialize, Deserialize};
use tch::{self, vision::image};
use tch::{kind, Tensor};
use serde_json::{Value, json};
use tch::IValue;
use std::io;


pub struct YOLO {
    model: tch::CModule,
    device: tch::Device,
    h: i64,
    w: i64
}