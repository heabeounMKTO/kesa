mod fileutils;
mod image_augmentations;
mod image_utils;
mod label;
mod output;
mod splash;

use anyhow::{bail, Error, Result};
use clap::Parser;
use fileutils::{get_all_classes, open_image, ExportFolderOptions};
use image::DynamicImage;
use image_augmentations::augmentations::{AugmentationType, ImageAugmentation};
use indicatif::ProgressBar;
use label::{read_labels_from_file, LabelmeAnnotation};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use spinoff::{spinners, Color, Spinner};
use splash::print_splash;
use std::collections::HashMap;
use std::{fs, path::PathBuf};

use crate::fileutils::{get_all_classes_hash, get_all_jsons, write_data_yaml, write_yolo_to_txt};

#[derive(Parser, Debug)]
struct CliArguments {
    #[arg(long)]
    folder: String,

    #[arg(long)]
    workers: Option<i64>,

    #[arg(long)]
    /// export format , labelme or yolo?!
    /// by default is labelme
    format: Option<String>,

    #[arg(long)]
    /// image variations to create
    /// by default is 5 times
    variations: Option<i32>,
}

fn main() -> Result<(), Error> {
    print_splash();
    let args = CliArguments::parse();

    let workers = match &args.workers {
        Some(ref _i64) => args.workers,
        None => Some(4),
    };
    
    let aug_variations = match &args.variations {
        Some(ref _i32) => args.variations,
        None => Some(5)
    };

    let export_format = match &args.format {
        Some(ref String) => args.format,
        None => Some(String::from("labelme")),
    }
    .unwrap();

    println!("export format {:?}", &export_format);

    rayon::ThreadPoolBuilder::new()
        .num_threads(workers.unwrap().try_into().unwrap())
        .build_global()
        .unwrap();
    let mut spinner0 = Spinner::new(
        spinners::Hearts,
        "[info]::kesa_aug: collecting jsons..",
        Color::White,
    );
    let all_json = get_all_jsons(&args.folder)?;
    let all_classes = get_all_classes(&all_json)?;
    let classes_hash = get_all_classes_hash(&all_classes)?;
    spinner0.success(format!("[info]::kesa_aug: found {:?} json files", &all_json.len()).as_str());
    let prog = ProgressBar::new(all_json.len().to_owned() as u64);

    all_json.par_iter().for_each(|file| {
        prog.inc(1);
        for _ in 0..(aug_variations.unwrap()) {
            // idk how can this cause a panic ok\
            //
            // (comments before disaster)
            let do_aug = get_random_aug().unwrap();

            // FUCK THEM <<RESULT>> HANDLING KIDS
            create_augmentation(do_aug, &file, &classes_hash, &export_format, &args.folder)
                .unwrap();
        }
    });
    prog.finish_with_message("[info]::kesa_aug: created augmentations!\n");
    Ok(())
}

fn create_augmentation(
    aug_type: AugmentationType,
    json_path: &PathBuf,
    class_hash: &HashMap<String, i64>,
    export_format: &str,
    folder: &str,
) -> Result<(), Error> {
    let label = read_labels_from_file(json_path.to_str().unwrap())?;

    let mut img_path = PathBuf::from(folder);
    img_path.push(&label.imagePath);

    let img = open_image(&img_path)?;

    let mut aug = ImageAugmentation {
        image: img,
        coords: label,
    };
    match &aug_type {
        AugmentationType::FlipVeritcal => {
            aug.flip_v();
        }
        AugmentationType::FlipHorizontal => {
            aug.flip_h();
        }
        AugmentationType::RandomBrightness => {
            aug.random_brightness((-100, 100));
        }
        AugmentationType::UnSharpen => {
            aug.unsharpen(10.0, 2);
        }
        AugmentationType::HueRotate30 => {
            aug.huerotate(30);
        }
        AugmentationType::HueRotate60 => {
            aug.huerotate(60);
        }
        AugmentationType::HueRotate90 => {
            aug.huerotate(90);
        }
        AugmentationType::HueRotate120 => {
            aug.huerotate(120);
        }
        AugmentationType::HueRotate180 => {
            aug.huerotate(180);
        }
        AugmentationType::HueRotate210 => {
            aug.huerotate(210);
        }
        AugmentationType::HueRotate270 => {
            aug.huerotate(270);
        }
        AugmentationType::Grayscale => {
            aug.grayscale();
        }
        AugmentationType::Rotate90 => {
            aug.rotate_90_counterclockwise();
        }
    }
    aug.write_annotations(&PathBuf::from(folder), class_hash)?;
    Ok(())
}

fn get_random_aug() -> Result<AugmentationType, Error> {
    let mut rng = rand::thread_rng();
    // get random number that
    // corresponds toa  augmentation type
    let aug_t = Uniform::from(0..13).sample(&mut rng);
    let do_aug = match aug_t {
        0 => AugmentationType::FlipHorizontal,
        1 => AugmentationType::FlipVeritcal,
        2 => AugmentationType::RandomBrightness,
        3 => AugmentationType::UnSharpen,
        4 => AugmentationType::HueRotate30,
        5 => AugmentationType::HueRotate60,
        6 => AugmentationType::HueRotate90,
        7 => AugmentationType::HueRotate120,
        8 => AugmentationType::HueRotate180,
        9 => AugmentationType::HueRotate210,
        10 => AugmentationType::HueRotate270,
        11 => AugmentationType::Grayscale,
        12 => AugmentationType::Rotate90,
        _ => panic!("[error]::kesa_aug: unknown augmentation type!"),
    };
    Ok(do_aug)
}