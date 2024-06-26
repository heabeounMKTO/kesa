use image::DynamicImage;

use crate::label::{LabelmeAnnotation, Shape, YoloAnnotation};
use anyhow::{Error, Result};
use std::collections::HashMap;

/// output formats functions traits
pub trait OutputFormat: Sized {
    fn to_yolo_vec(&self) -> Result<Vec<YoloAnnotation>, anyhow::Error>;
    fn to_yolo(&self) -> Result<YoloAnnotation, anyhow::Error>;
    fn to_labelme(
        &self,
        all_classes: &Vec<String>,
        original_dimension: &(u32, u32),
        filename: &str,
        image_file: &DynamicImage,
        inference_dimension: &(u32, u32),
    ) -> Result<LabelmeAnnotation, anyhow::Error>;
    fn to_shape(
        &self,
        all_classes: &Vec<String>,
        original_dimension: &(u32, u32),
        inference_dimension: &(u32, u32),
    ) -> Result<Vec<Shape>, anyhow::Error>;
}
