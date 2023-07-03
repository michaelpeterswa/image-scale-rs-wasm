use image::Rgba;
use image::{DynamicImage, ImageResult};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Scale image to new size.
#[wasm_bindgen]
pub fn scale(img: Vec<u8>, nx: u32, ny: u32) -> Result<Vec<u8>, String> {
    console_log!("in rust scale");

    let image_format_detected: Result<image::ImageFormat, image::ImageError> =
        image::guess_format(&img);

    let image_format = match image_format_detected {
        Ok(image_format) => image_format,
        Err(error) => {
            return Err(format!(
                "failed to detect image format: {}",
                error.to_string()
            ));
        }
    };

    console_log!("image_format: {:?}", image_format);

    let loaded_image: ImageResult<DynamicImage> =
        image::load_from_memory_with_format(&img, image_format);

    let result_image = match loaded_image {
        Ok(image) => image,
        Err(error) => {
            return Err(format!(
                "failed to load image from memory: {}",
                error.to_string()
            ))
        }
    };

    let scaled_image: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::imageops::resize(
        &result_image,
        nx,
        ny,
        image::imageops::FilterType::CatmullRom,
    );

    let mut png_output_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    let save_result: Result<(), image::ImageError> =
        scaled_image.write_to(&mut png_output_buffer, image::ImageFormat::Png);

    match save_result {
        Ok(_) => (),
        Err(error) => {
            return Err(format!(
                "failed to save image to memory: {}",
                error.to_string()
            ))
        }
    };

    Ok(png_output_buffer.into_inner())
}

#[cfg(test)]
mod tests {
    use crate::scale;
    use image::{ImageBuffer, Rgb};
    use std::io::Cursor;

    #[test]
    fn test_scale() {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(1024, 1024);

        img.put_pixel(512, 512, Rgb([255, 255, 255]));

        let mut png_output_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        let save_result: Result<(), image::ImageError> =
            img.write_to(&mut png_output_buffer, image::ImageFormat::Png);

        match save_result {
            Ok(_) => (),
            Err(error) => panic!("{}", error.to_string()),
        }

        let result: Result<Vec<u8>, String> = scale(png_output_buffer.into_inner(), 512, 512);

        let result_vec: Vec<u8> = match result {
            Ok(vec) => vec,
            Err(error) => panic!("{}", error.to_string()),
        };

        let image_format_detected = image::guess_format(&result_vec).unwrap();

        let loaded_image: Result<image::DynamicImage, image::ImageError> =
            image::load_from_memory_with_format(&result_vec, image_format_detected);

        let result_image: image::DynamicImage = match loaded_image {
            Ok(image) => image,
            Err(error) => panic!("{}", error.to_string()),
        };

        assert_eq!(result_image.width(), 512);
        assert_eq!(result_image.height(), 512);
    }
}
