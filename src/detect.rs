use clap::Parser;
use dlib_face_recognition::*;
use image::*;

use crate::tool::*;

pub fn detect(input_photo_path: &str, output_photo_path: &str) {
    let mut image = image::open(input_photo_path).unwrap().to_rgb8();
    let matrix = ImageMatrix::from_image(&image);

    let detector = FaceDetector::default();

    let Ok(cnn_detector) = FaceDetectorCnn::default() else {
        panic!("Unable to load cnn face detector!");
    };

    let Ok(landmarks) = LandmarkPredictor::default() else {
        panic!("Unable to load landmark predictor!");
    };

    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    let face_locations = tick("FaceDetector", || detector.face_locations(&matrix));

    for r in face_locations.iter() {
        draw_rectangle(&mut image, &r, red);

        let landmarks = landmarks.face_landmarks(&matrix, &r);

        for point in landmarks.iter() {
            draw_point(&mut image, &point, red);
        }
    }

    let face_locations = tick("FaceDetectorCnn", || cnn_detector.face_locations(&matrix));

    for r in face_locations.iter() {
        draw_rectangle(&mut image, &r, green);

        let landmarks = tick("LandmarkPredictor", || {
            landmarks.face_landmarks(&matrix, &r)
        });

        for point in landmarks.iter() {
            draw_point(&mut image, &point, green);
        }
    }

    if let Err(e) = image.save(output_photo_path) {
        println!("Error saving the image: {e}");
    } else {
        println!("Output image saved to {}", output_photo_path);
    }
}
