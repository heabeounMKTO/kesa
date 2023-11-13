mod yolo;
use yolo::yolo_rs;


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