
## Usage in C

*You can find the C header file in the include folder in the project root directory.*

Libcaesium exposes there C functions, auto-detecting the input file type:

### Data Structures

#### CSI_Parameters

Compression options is defined by:

```Rust
#[repr(C)]
pub struct CCSParameters {
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
}
```

The C options struct is slightly different from the Rust one:

```C
typedef struct CSI_Parameters {
    bool keep_metadata;
    uint32_t jpeg_quality;
    uint32_t jpeg_chroma_subsampling;
    bool jpeg_progressive;
    uint32_t  png_quality;
    uint32_t  png_optimization_level;
    bool  png_force_zopfli;
    uint32_t  gif_quality;
    uint32_t  webp_quality;
    uint32_t  tiff_compression;
    uint32_t  tiff_deflate_level;
    bool  optimize;
    uint32_t  width;
    uint32_t  height;
    bool  allow_magnify;
} CSI_Parameters;
```

The option description is the same as the Rust counterpart.  
Valid values for `jpeg_chroma_subsampling` are `[444, 422, 420, 411]`. Any other value will be ignored and will be used
the default option.  
Valid values for `tiff_compression` are `[0 (Uncompressed), 1 (Lzw), 2 (Deflate), 3 (Packbits)]`. Any other value will be
ignored and `0` will be used.  
Valid values for `tiff_deflate_level` are `[1 (Fast), 6 (Balanced), 9 (Best)]`. Any other value will be ignored and `Best`
will be used.

#### CSI_Result

```Rust
#[repr(C)]
pub struct CSI_Result {
    pub success: bool,
    pub code: u64,
    pub error_message: *const c_char,
}
```

in C:

```C
typedef struct CSI_Result {
    bool success;
    uint64_t code;
    char *error_message;
} CSI_Result;
```

If `success` is `true` the compression process ended successfully and `error_message` will be empty.  
On failure, the `error_message` will be filled with a string containing a brief explanation of the error.

### Compress based on quality values

```Rust
pub unsafe extern "C" fn csi_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CSI_Parameters
) -> CSI_Result
```

in C:

```C
CSI_Result csi_compress(
    const char* input_path,
    const char* output_path,
    CSI_Parameters* params
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)

#### Return

A `CSI_Result` struct

### Based on output size

```Rust
pub unsafe extern "C" fn csi_compress_to_size(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CSI_Parameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CSI_Result
```

in C:

```C
CSI_Result csi_compress_to_size(
    const char* input_path,
    const char* output_path,
    CSI_Parameters* params,
    uint64_t max_output_size,
    bool return_smallest
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
- `max_output_size` - the maximum output size, in bytes
- `return_smallest` - whether to return the smallest

#### Return

A `CSI_Result` struct

### Based on convert output

```Rust
pub unsafe extern "C" fn c_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: SupportedFileTypes,
    params: CCSParameters,
) -> CCSResult
```

in C:

```C
CSI_Result csi_convert(
    const char* input_path,
    const char* output_path,
    CSI_SupportedFileTypes format,
    CSI_Parameters* params
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `format` - target image format (see below)
- `parameters` - options struct, containing compression parameters (see below)

#### Return

A `CCSResult` struct
