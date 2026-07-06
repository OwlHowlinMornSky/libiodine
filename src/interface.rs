use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::slice::from_raw_parts;

use crate::parameters::ChromaSubsampling;
use crate::parameters::TiffCompression::{Deflate, Lzw, Packbits, Uncompressed};
use crate::{
    compress, compress_fromto, compress_in_memory, compress_into, compress_to_size, compress_to_size_fromto,
    compress_to_size_into, convert, convert_fromto, convert_into, error, CSParameters, SupportedFileTypes,
    TiffDeflateLevel,
};

#[repr(C)]
pub struct CByteArray {
    pub data: *mut u8,
    pub length: usize,
}

#[repr(C)]
pub struct CCSParameters {
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
pub struct CCSResult {
    pub success: bool,
    pub code: u32,
    pub error_message: *const c_char,
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    c_return_result(compress(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_compress_in_memory(
    input_data: *const u8,
    input_length: usize,
    params: CCSParameters,
    output: *mut CByteArray,
) -> CCSResult {
    if input_data.is_null() || output.is_null() {
        return CCSResult {
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
                return CCSResult {
                    success: false,
                    code: 1002,
                    error_message: CString::new("Memory allocation failed").unwrap().into_raw(),
                };
            }

            std::ptr::copy_nonoverlapping(compressed_data.as_ptr(), output_data, output_length);

            (*output).data = output_data;
            (*output).length = output_length;

            CCSResult {
                success: true,
                code: 0,
                error_message: CString::new("").unwrap().into_raw(),
            }
        }
        Err(e) => {
            (*output).data = std::ptr::null_mut();
            (*output).length = 0;

            CCSResult {
                success: false,
                code: e.code,
                error_message: CString::new(e.to_string()).unwrap().into_raw(),
            }
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_compress_to_size(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CCSResult {
    let mut parameters = c_set_parameters(params);

    c_return_result(compress_to_size(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: SupportedFileTypes,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    c_return_result(convert(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
        format,
    ))
}

fn c_return_result(result: error::Result<()>) -> CCSResult {
    match result {
        Ok(_) => CCSResult {
            success: true,
            code: 0,
            error_message: CString::new("").unwrap().into_raw(),
        },
        Err(e) => CCSResult {
            success: false,
            code: e.code,
            error_message: CString::new(e.to_string()).unwrap().into_raw(),
        },
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_free_byte_array(byte_array: CByteArray) {
    if !byte_array.data.is_null() {
        libc::free(byte_array.data as *mut libc::c_void);
    }
}

// Helper function to free error message strings
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

fn c_set_parameters(params: CCSParameters) -> CSParameters {
    let mut parameters = CSParameters::new();

    parameters.jpeg.quality = params.jpeg_quality;
    parameters.jpeg.progressive = params.jpeg_progressive;
    parameters.jpeg.optimize = params.jpeg_optimize;
    parameters.jpeg.preserve_icc = params.jpeg_preserve_icc;
    parameters.png.quality = params.png_quality;
    parameters.png.optimize = params.png_optimize;
    parameters.keep_metadata = params.keep_metadata;
    parameters.png.optimization_level = params.png_optimization_level as u8;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.webp.lossless = params.webp_lossless;
    parameters.width = params.width;
    parameters.height = params.height;

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

    parameters.exinfo.allow_magnify = params.allow_magnify;
    parameters.exinfo.reduce_by_power_of_2 = params.reduce_by_power_of_2;
    parameters.exinfo.short_side_pixels = params.short_side_pixels;
    parameters.exinfo.long_size_pixels = params.long_size_pixels;

    parameters
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn iod_free_buffer(byte_array: CByteArray) {
    if !byte_array.data.is_null() {
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            byte_array.data,
            byte_array.length,
        )));
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn iod_compress_in_memory(
    input_data: *const u8,
    input_length: usize,
    params: CCSParameters,
    output: *mut CByteArray,
) -> CCSResult {
    if input_data.is_null() || output.is_null() {
        return CCSResult {
            success: false,
            code: 1001,
            error_message: CString::new("Null pointer provided").unwrap().into_raw(),
        };
    }

    let input_vec = std::slice::from_raw_parts(input_data, input_length).to_vec();

    let parameters = c_set_parameters(params);

    match compress_in_memory(input_vec, &parameters) {
        Ok(compressed_data) => {
            let boxed_slice: Box<[u8]> = compressed_data.into_boxed_slice();
            let output_length = boxed_slice.len();
            let output_data = Box::into_raw(boxed_slice) as *mut u8;

            (*output).data = output_data;
            (*output).length = output_length;

            CCSResult {
                success: true,
                code: 0,
                error_message: CString::new("").unwrap().into_raw(),
            }
        }
        Err(e) => {
            (*output).data = std::ptr::null_mut();
            (*output).length = 0;

            CCSResult {
                success: false,
                code: e.code,
                error_message: CString::new(e.to_string()).unwrap().into_raw(),
            }
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    csi_return_result_u64(compress_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer,
        obufmaxlen,
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
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }

    csi_return_result_u64(compress_fromto(in_file, output_buffer, obufmaxlen, &parameters))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_compress_to_size_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    params: CCSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CCSResult {
    let mut parameters = c_set_parameters(params);

    csi_return_result_u64(compress_to_size_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer,
        obufmaxlen,
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
    params: CCSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CCSResult {
    let mut parameters = c_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }

    csi_return_result_u64(compress_to_size_fromto(
        in_file,
        output_buffer,
        obufmaxlen,
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn csi_convert_into(
    input_path: *const c_char,
    output_buffer: *mut c_void,
    obufmaxlen: u64,
    format: SupportedFileTypes,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    csi_return_result_u64(convert_into(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        output_buffer,
        obufmaxlen,
        &parameters,
        format,
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
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    let in_file: Vec<u8>;
    unsafe {
        in_file = from_raw_parts(input_buffer as *const u8, ibuflen as usize).to_vec();
    }

    csi_return_result_u64(convert_fromto(in_file, output_buffer, obufmaxlen, &parameters, format))
}

fn csi_return_result_u64(result: error::Result<u64>) -> CCSResult {
    match result {
        Ok(len) => CCSResult {
            success: true,
            code: len as u32,
            error_message: CString::new("").unwrap().into_raw(),
        },
        Err(e) => CCSResult {
            success: false,
            code: e.code,
            error_message: CString::new(e.to_string()).unwrap().into_raw(),
        },
    }
}
