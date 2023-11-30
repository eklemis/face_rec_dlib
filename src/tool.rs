use dlib_face_recognition::*;
use image::*;
use std::path::Path;

pub fn tick<R>(name: &str, f: impl Fn() -> R) -> R {
    let now = std::time::Instant::now();
    let result = f();
    println!("[{}] elapsed time: {}ms", name, now.elapsed().as_millis());
    result
}
pub fn draw_rectangle(image: &mut RgbImage, rect: &Rectangle, colour: Rgb<u8>) {
    for x in rect.left..rect.right {
        image.put_pixel(x as u32, rect.top as u32, colour);
        image.put_pixel(x as u32, rect.bottom as u32, colour);
    }

    for y in rect.top..rect.bottom {
        image.put_pixel(rect.left as u32, y as u32, colour);
        image.put_pixel(rect.right as u32, y as u32, colour);
    }
}

pub fn draw_point(image: &mut RgbImage, point: &Point, colour: Rgb<u8>) {
    image.put_pixel(point.x() as u32, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32 + 1, colour);
    image.put_pixel(point.x() as u32, point.y() as u32 + 1, colour);
}

pub fn get_full_file_name(file_path: &str) -> String {
    Path::new(file_path)
        .file_name() // This gets the full file name, including the extension
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .unwrap_or_else(|| "".to_string())
}
