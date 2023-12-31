use serde_derive::{Deserialize, Serialize};

use std::collections::HashMap;

/// stores generic values that most annotated images have
#[derive(Debug, Clone)]
pub struct GenericAnnotation {
    pub label: String,
    pub image_width: i32,
    pub image_height: i32,
    pub image_path: String,
    pub x1y1: GenericLabelPoints,
    pub x2y2: GenericLabelPoints,
}

impl GenericAnnotation {
    pub fn new(
        label: &str,
        image_width: i32,
        image_height: i32,
        image_path: String,
        x1y1: GenericLabelPoints,
        x2y2: GenericLabelPoints,
    ) -> GenericAnnotation {
        return GenericAnnotation {
            label: String::from(label),
            image_width: image_width,
            image_height: image_height,
            image_path: image_path,
            x1y1: x1y1,
            x2y2: x2y2,
        };
    }

    pub fn label(&self) -> String {
        String::from(&self.label)
    }
    pub fn image_width(&self) -> i32 {
        self.image_width
    }
    pub fn image_height(&self) -> i32 {
        self.image_height
    }
    pub fn x1y1(&self) -> GenericLabelPoints {
        self.x1y1
    }
    pub fn x2y2(&self) -> GenericLabelPoints {
        self.x2y2
    }

    pub fn convert2yolo(&self, class_hash: HashMap<String, i32>) -> YoloLabel {
        let x1 = self.x1y1.x;
        let y1 = self.x1y1.y;
        let x2 = self.x2y2.x;
        let y2 = self.x2y2.y;

        let x = ((x1 + x2) / 2.0) / self.image_width as f32;
        let y = ((y1 + y2) / 2.0) / self.image_height as f32;
        let w = (x2 - x1) / self.image_width as f32;
        let h = (y2 - y1) / self.image_height as f32;

        let class_idx = class_hash.get(&self.label).unwrap();

        return YoloLabel {
            label_index: class_idx.to_owned(),
            x: x,
            y: y,
            w: w,
            h: h,
        };
    }

    pub fn convert2labelme(&self, class_hash: HashMap<String, i32>) -> LabelMeLabel {
        todo!();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GenericLabelPoints {
    pub x: f32,
    pub y: f32,
}

impl GenericLabelPoints {
    pub fn new(x: f32, y: f32) -> GenericLabelPoints {
        GenericLabelPoints { x: x, y: y }
    }
}

#[derive(Debug, Clone)]
// xywh
pub struct YoloLabel {
    pub label_index: i32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl YoloLabel {
    pub fn new(label_index: i32, x: f32, y: f32, w: f32, h: f32) -> YoloLabel {
        return YoloLabel {
            label_index: label_index,
            x: x,
            y: y,
            w: w,
            h: h,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelMeShapes {
    pub label: String,
    pub points: Vec<GenericLabelPoints>,
    pub shape_type: String,
    pub flags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// must match actual labelme JSON file
pub struct LabelMeLabel {
    pub version: String,
    pub flags: HashMap<String, String>,
    pub shapes: Vec<LabelMeShapes>,
    pub imagePath: String,
    pub imageData: String,
    pub imageHeight: i32,
    pub imageWidth: i32,
}

impl LabelMeLabel {
    pub fn version(&self) -> String {
        String::from(&self.version)
    }
    pub fn image_width(&self) -> i32 {
        self.imageWidth
    }
    pub fn image_height(&self) -> i32 {
        self.imageHeight
    }
}
