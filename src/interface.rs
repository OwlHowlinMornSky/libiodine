use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::slice::from_raw_parts;

use crate::parameters::ChromaSubsampling;
use crate::parameters::TiffCompression::{Deflate, Lzw, Packbits, Uncompressed};
use crate::{
    compress, compress_into, compress_fromto,
    compress_in_memory,
    compress_to_size, compress_to_size_into, compress_to_size_fromto,
    convert, convert_into, convert_fromto,
    error, CSParameters, SupportedFileTypes, TiffDeflateLevel,
};

#[repr(C)]
pub struct CSI_Result {
    pub success: bool,
    pub code: u64,
    pub error_message: *const c_char,
}
#[repr(C)]
pub struct CSI_Parameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub jpeg_chroma_subsampling: u32,
    pub jpeg_progressive: bool,
    pub jpeg_optimize: bool,
    pub jpeg_preserve_icc: bool,
    pub png_quality: u32,
    pub png_optimization_level: u32,
    pub png_force_zopfli: bool,
    pub png_optimize: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub webp_lossless: bool,
    pub tiff_compression: u32,
    pub tiff_deflate_level: u32,
    pub width: u32,
    pub height: u32,
    pub allow_magnify: bool,
    pub reduce_by_power_of_2: bool,
    pub short_side_pixels: u32,
    pub long_size_pixels: u32,
}

#[repr(C)]
pub struct CByteArray {
    pub data: *mut u8,
    pub length: usize,
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

pub unsafe extern "C" fn csi_compress_in_memory(
    input_data: *const u8,
    input_length: usize,
    params: CSI_Parameters,
    output: *mut CByteArray,
) -> CSI_Result {
    if input_data.is_null() || output.is_null() {
        return CSI_Result {
            success: false,
            code: 1001,
            error_message: CString::new("Null pointer provided").unwrap().into_raw(),
        };
    }

    let input_vec = std::slice::from_raw_parts(input_data, input_length).to_vec();

    let parameters = c_set_parameters(params);

    match compress_in_memory(input_vec, &parameters) {
        Ok(compressed_data) => {
            let output_length = compressed_data.len();
            let output_data = libc::malloc(output_length) as *mut u8;

            if output_data.is_null() {
                return CSI_Result {
                    success: false,
                    code: 1002,
                    error_message: CString::new("Memory allocation failed").unwrap().into_raw(),
                };
            }

            std::ptr::copy_nonoverlapping(compressed_data.as_ptr(), output_data, output_length);

            (*output).data = output_data;
            (*output).length = output_length;

            CSI_Result {
                success: true,
                code: 0,
                error_message: CString::new("").unwrap().into_raw(),
            }
        }
        Err(e) => {
            (*output).data = std::ptr::null_mut();
            (*output).length = 0;

            CSI_Result {
                success: false,
                code: e.code,
                error_message: CString::new(e.to_string()).unwrap().into_raw(),
            }
        }
    }
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
        format,
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

fn c_return_result(result: error::Result<()>) -> CSI_Result {
    match result {
        Ok(_) => CSI_Result {
            success: true,
            code: 0,
            error_message: CString::new("").unwrap().into_raw(),
        },
        Err(e) => CSI_Result {
            success: false,
            code: e.code,
            error_message: CString::new(e.to_string()).unwrap().into_raw(),
        },
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

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_free_byte_array(byte_array: CByteArray) {
    if !byte_array.data.is_null() {
        libc::free(byte_array.data as *mut libc::c_void);
    }
}

// Helper function to free error message strings
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

fn csi_set_parameters(params: CSI_Parameters) -> CSParameters {
    let mut parameters = CSParameters::new();

    parameters.keep_metadata = params.keep_metadata;
    parameters.jpeg.quality = params.jpeg_quality;

    parameters.jpeg.progressive = params.jpeg_progressive;
    parameters.jpeg.optimize = params.jpeg_optimize;
    parameters.jpeg.preserve_icc = params.jpeg_preserve_icc;
    parameters.png.quality = params.png_quality;
    parameters.png.optimization_level = params.png_optimization_level as u8;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.png.optimize = params.png_optimize;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.webp.lossless = params.webp_lossless;


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
        _ => Uncompressed,
    };

    parameters.tiff.deflate_level = match params.tiff_deflate_level {
        1 => TiffDeflateLevel::Fast,
        6 => TiffDeflateLevel::Balanced,
        _ => TiffDeflateLevel::Best,
    };

    parameters
}
