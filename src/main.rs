use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::{Duration, Instant};
use tch::{nn, vision, Device, Tensor, no_grad};
use image::{ImageBuffer, RgbImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // デバイスを選択（CPU または CUDA）
    let device = if tch::Cuda::is_available() {
        Device::Cuda(0)
    } else {
        Device::Cpu
    };

    // YOLOv7の学習済みモデルの読み込み
    let weights_path = "weights/yolov7.pt";
    let yolo7 = vision::yolo::Yolo::load(weights_path)?;

    // 画面キャプチャの設定
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    loop {
        let start = Instant::now();

        // キャプチャーを試行
        let frame = match capturer.frame() {
            Ok(frame) => frame,
            Err(error) => {
                if error.kind() == WouldBlock {
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        // 画面キャプチャのデータを画像に変換
        let width = capturer.width();
        let height = capturer.height();
        let buffer: Vec<u8> = frame.to_vec();
        let image: RgbImage = ImageBuffer::from_raw(width as u32, height as u32, buffer).unwrap();
        
        // 画像をテンソルに変換
        let image_tensor = vision::image::load_from_image(image)?.to_device(device);

        // 前処理: 画像をリサイズ
        let resized_image = vision::image::resize(&image_tensor, 640, 640);

        let input_tensor = resized_image.unsqueeze(0);

        // 推論を実行
        let output = no_grad(|| yolo7.forward(&input_tensor))?;
        let detections = vision::yolo::non_max_suppression(&output, 0.5, 0.5)?;

        for det in detections {
            println!("Detection: {:?}", det);
        }

        // フレームごとの処理時間を計算
        let duration = start.elapsed();
        println!("Frame Time: {:?}", duration);
    }
}
