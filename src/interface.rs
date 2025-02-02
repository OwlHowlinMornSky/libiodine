use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::slice::from_raw_parts;

use crate::{
    compress, compress_into, compress_fromto,
    compress_to_size, compress_to_size_into, compress_to_size_fromto,
    convert, convert_into, convert_fromto,
    CSParameters, error, SupportedFileTypes, TiffDeflateLevel
};
use crate::parameters::ChromaSubsampling;
use crate::parameters::TiffCompression::{Deflate, Lzw, Packbits, Uncompressed};

#[repr(C)]
pub struct CSI_Parameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub jpeg_chroma_subsampling: u32,
    pub jpeg_progressive: bool,
    pub png_quality: u32,
    pub png_optimization_level: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub tiff_compression: u32,
    pub tiff_deflate_level: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
    pub allow_magnify: bool,
    pub reduce_by_power_of_2: bool,
    pub short_side_pixels: u32,
    pub long_size_pixels: u32,
}

#[repr(C)]
pub struct CSI_Result {
    pub success: bool,
    pub code: u64,
    pub error_message: *const c_char,
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    csi_return_result(compress(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    csi_return_result_u64(compress_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer, obufmaxlen,
        &parameters,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_fromto(
    input_buffer: *const c_void,
    ibuflen: u64,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }
    
    csi_return_result_u64(compress_fromto(
        in_file,
        output_buffer, obufmaxlen,
        &parameters,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_to_size(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CSI_Parameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CSI_Result {
    let mut parameters = csi_set_parameters(params);

    csi_return_result(compress_to_size(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_to_size_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CSI_Parameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CSI_Result {
    let mut parameters = csi_set_parameters(params);

    csi_return_result_u64(compress_to_size_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer, obufmaxlen,
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_to_size_fromto(
    input_buffer: *const c_void,
    ibuflen: u64,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CSI_Parameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CSI_Result {
    let mut parameters = csi_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }

    csi_return_result_u64(compress_to_size_fromto(
        in_file,
        output_buffer, obufmaxlen,
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: SupportedFileTypes,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    csi_return_result(convert(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
        format
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_convert_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    format: SupportedFileTypes,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    csi_return_result_u64(convert_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer, obufmaxlen,
        &parameters,
        format
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_convert_fromto(
    input_buffer: *const c_void,
    ibuflen: u64,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    format: SupportedFileTypes,
    params: CSI_Parameters,
) -> CSI_Result {
    let parameters = csi_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }

    csi_return_result_u64(convert_fromto(
        in_file,
        output_buffer, obufmaxlen,
        &parameters,
        format
    ))
}

fn csi_return_result(result: error::Result<()>) -> CSI_Result {
    let mut error_message = CString::new("").unwrap();

    match result {
        Ok(_) => {
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CSI_Result {
                success: true,
                code: 0,
                error_message: em_pointer,
            }
        }
        Err(e) => {
            error_message = CString::new(e.to_string()).unwrap();
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CSI_Result {
                success: false,
                code: e.code,
                error_message: em_pointer,
            }
        }
    }
}

fn csi_return_result_u64(result: error::Result<u64>) -> CSI_Result {
    let mut error_message = CString::new("").unwrap();

    match result {
        Ok(len) => {
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CSI_Result {
                success: true,
                code: len,
                error_message: em_pointer,
            }
        }
        Err(e) => {
            error_message = CString::new(e.to_string()).unwrap();
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CSI_Result {
                success: false,
                code: e.code,
                error_message: em_pointer,
            }
        }
    }
}

fn csi_set_parameters(params: CSI_Parameters) -> CSParameters {
    let mut parameters = CSParameters::new();

    parameters.jpeg.quality = params.jpeg_quality;
    parameters.jpeg.progressive = params.jpeg_progressive;
    parameters.png.quality = params.png_quality;
    parameters.optimize = params.optimize;
    parameters.keep_metadata = params.keep_metadata;
    parameters.png.optimization_level = params.png_optimization_level as u8;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.width = params.width;
    parameters.height = params.height;
    parameters.allow_magnify = params.allow_magnify;
    parameters.reduce_by_power_of_2 = params.reduce_by_power_of_2;
    parameters.short_side_pixels = params.short_side_pixels;
    parameters.long_size_pixels = params.long_size_pixels;
    
    parameters.jpeg.chroma_subsampling = match params.jpeg_chroma_subsampling {
        444 => ChromaSubsampling::CS444,
        422 => ChromaSubsampling::CS422,
        420 => ChromaSubsampling::CS420,
        411 => ChromaSubsampling::CS411,
        _ => ChromaSubsampling::Auto,
    };

    parameters.tiff.algorithm = match params.tiff_compression {
        1 => Lzw,
        2 => Deflate,
        3 => Packbits,
        _ => Uncompressed
    };

    parameters.tiff.deflate_level = match params.tiff_deflate_level {
        1 => TiffDeflateLevel::Fast,
        6 => TiffDeflateLevel::Balanced,
        _ => TiffDeflateLevel::Best
    };

    parameters
}
