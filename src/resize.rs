use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageReader};

use crate::error::CaesiumError;
use crate::utils::get_jpeg_orientation;

#[derive(Copy, Clone)]
pub struct ResizeInfo {
    pub allow_magnify: bool,
    pub reduce_by_power_of_2: bool,
    pub short_side_pixels: u32,
    pub long_size_pixels: u32,
}

pub fn resize_n(
    image_buffer: &[u8],
    width: u32,
    height: u32,
    format: image::ImageFormat,
    exinfo: ResizeInfo,
) -> Result<Vec<u8>, CaesiumError> {
    let (mut desired_width, mut desired_height) = (width, height);
    if format == image::ImageFormat::Jpeg {
        let orientation = get_jpeg_orientation(image_buffer);
        (desired_width, desired_height) = match orientation {
            5..=8 => (height, width),
            _ => (width, height),
        };
    }

    let mut image = ImageReader::new(Cursor::new(image_buffer))
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

    let dimensions = compute_dimensions(image.width(), image.height(), desired_width, desired_height, exinfo);
    if !exinfo.allow_magnify && (image.width() < dimensions.0 || image.height() < dimensions.1) {
        return Ok(image_buffer.to_vec());
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

pub fn resize_image_n(image: DynamicImage, width: u32, height: u32, exinfo: ResizeInfo) -> DynamicImage {
    let dimensions = compute_dimensions(image.width(), image.height(), width, height, exinfo);
    if !exinfo.allow_magnify && (image.width() < dimensions.0 || image.height() < dimensions.1) {
        image
    } else {
        image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3)
    }
}

pub fn compute_dimensions(
    original_width: u32,
    original_height: u32,
    mut desired_width: u32,
    mut desired_height: u32,
    exinfo: ResizeInfo,
) -> (u32, u32) {
    if desired_width == 0 && desired_height == 0 && (exinfo.short_side_pixels != 0 || exinfo.long_size_pixels != 0) {
        if original_width < original_height {
            desired_width = exinfo.short_side_pixels;
            desired_height = exinfo.long_size_pixels;
        } else {
            desired_height = exinfo.short_side_pixels;
            desired_width = exinfo.long_size_pixels;
        }
    }

    let mut n_width = original_width as f32;
    let mut n_height = original_height as f32;
    let ratio = original_width as f32 / original_height as f32;

    if exinfo.reduce_by_power_of_2
        && ((desired_width > 3 && desired_width < original_width)
            || (desired_height > 3 && desired_height < original_height))
    {
        let dw = desired_width as f32;
        let dh = desired_height as f32;
        while (desired_width > 3 && n_width > dw) || (desired_height > 3 && n_height > dh) {
            n_width /= 2.0;
            n_height /= 2.0;
        }
        n_width = n_width.ceil();
        n_height = n_height.ceil();
    } else {
        n_width = desired_width as f32;
        n_height = desired_height as f32;

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
        compute_dimensions(
            original_width,
            original_height,
            300,
            300,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (300, 300)
    )
}

#[test]
fn same_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            800,
            600,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (800, 600)
    )
}

#[test]
fn downscale_on_width() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            750,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (750, 563)
    )
}

#[test]
fn downscale_on_height() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            478,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (637, 478)
    )
}

#[test]
fn downscale_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            300,
            300,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (200, 150)
    )
}

#[test]
fn same_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            800,
            600,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (800, 600)
    )
}

#[test]
fn downscale_on_width_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            750,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (400, 300)
    )
}

#[test]
fn downscale_on_height_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            350,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 0,
                long_size_pixels: 0,
            }
        ),
        (400, 300)
    )
}

#[test]
fn downscale_on_short() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 450,
                long_size_pixels: 0,
            }
        ),
        (600, 450)
    )
}

#[test]
fn downscale_on_long() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 0,
                long_size_pixels: 600,
            }
        ),
        (600, 450)
    )
}

#[test]
fn downscale_on_both() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: false,
                short_side_pixels: 450,
                long_size_pixels: 600,
            }
        ),
        (600, 450)
    )
}

#[test]
fn downscale_on_short_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 450,
                long_size_pixels: 0,
            }
        ),
        (400, 300)
    )
}

#[test]
fn downscale_on_long_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 0,
                long_size_pixels: 300,
            }
        ),
        (200, 150)
    )
}

#[test]
fn downscale_on_both_by_2() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(
            original_width,
            original_height,
            0,
            0,
            ResizeInfo {
                allow_magnify: false,
                reduce_by_power_of_2: true,
                short_side_pixels: 149,
                long_size_pixels: 399,
            }
        ),
        (100, 75)
    )
}
