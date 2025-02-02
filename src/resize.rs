use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageReader};

use crate::error::CaesiumError;
use crate::utils::get_jpeg_orientation;

pub fn resize_n(
    image_buffer: Vec<u8>,
    allow_magnify: bool,
    reduce_by_power_of_2: bool,
    width: u32,
    height: u32,
    short_side_pixels: u32,
    long_size_pixels: u32,
    format: image::ImageFormat,
) -> Result<Vec<u8>, CaesiumError> {
    let buffer_slice: &[u8] = image_buffer.as_slice();
    let (mut desired_width, mut desired_height) = (width, height);
    if format == image::ImageFormat::Jpeg {
        let orientation = get_jpeg_orientation(buffer_slice);
        (desired_width, desired_height) = match orientation {
            5..=8 => (height, width),
            _ => (width, height)
        };
    }
    
    let mut image = ImageReader::new(Cursor::new(&image_buffer))
        .with_guessed_format()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10300,
        })?
        .decode()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10301,
        })?;

    let dimensions = compute_dimensions(image.width(), image.height(), desired_width, desired_height, short_side_pixels, long_size_pixels, reduce_by_power_of_2);
    if !allow_magnify && (image.width() < dimensions.0 || image.height() < dimensions.1) {
        return Ok(image_buffer);
    }
    
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let mut resized_file: Vec<u8> = vec![];
    image
        .write_to(&mut Cursor::new(&mut resized_file), format)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10302,
        })?;

    Ok(resized_file)
}

pub fn resize_image_n(image: DynamicImage, allow_magnify: bool, reduce_by_power_of_2: bool, width: u32, height: u32, short_side_pixels: u32, long_size_pixels: u32) -> DynamicImage {
    let dimensions = compute_dimensions(image.width(), image.height(), width, height, short_side_pixels, long_size_pixels, reduce_by_power_of_2);
    if !allow_magnify && (image.width() < dimensions.0 || image.height() < dimensions.1) {
        image
    }
    else{
        image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3)
    }
}

fn compute_dimensions(
    original_width: u32,
    original_height: u32,
    mut desired_width: u32,
    mut desired_height: u32,
    short_side_pixels: u32,
    long_size_pixels: u32,
    rbpo2: bool,
) -> (u32, u32) {

    let mut n_width = original_width as f32;
    let mut n_height = original_height as f32;
    let ratio = original_width as f32 / original_height as f32;

    if desired_width == 0 && desired_height == 0 && (short_side_pixels != 0 || long_size_pixels != 0) {
        if original_width < original_height {
            desired_width = short_side_pixels;
            desired_height = long_size_pixels;
        }
        else{
            desired_height = short_side_pixels;
            desired_width = long_size_pixels;
        }
    }

    if rbpo2 && ((desired_width > 3 && desired_width < original_width) || (desired_height > 3 && desired_height < original_height)) {
        let mut dw = desired_width as f32;
        let mut dh = desired_height as f32;
        if desired_width == 0 {
            dw = (original_width + 1) as f32;
        }
        if desired_height == 0 {
            dh = (original_height + 1) as f32;
        }

        while n_width > dw || n_height > dh {
            n_width /= 2.0;
            n_height /= 2.0;
        }

        n_width = n_width.ceil();
        n_height = n_height.ceil();
    }
    else {
        if desired_width > 0 && desired_height > 0 {
            return (desired_width, desired_height);
        }
        
        if desired_height == 0 {
            n_height = (n_width / ratio).round();
        }

        if desired_width == 0 {
            n_width = (n_height * ratio).round();
        }
    }
    (n_width as u32, n_height as u32)
}

#[test]
fn downscale_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 300, 300),
        (300, 300)
    )
}

#[test]
fn same_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 800, 600),
        (800, 600)
    )
}

#[test]
fn downscale_on_width() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 750, 0),
        (750, 563)
    )
}

#[test]
fn downscale_on_height() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 0, 478),
        (637, 478)
    )
}
