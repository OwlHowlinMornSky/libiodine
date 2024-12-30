#ifndef LIB_CAESIUM_IODIDE_H
#define LIB_CAESIUM_IODIDE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

typedef enum CSI_SupportedFileTypes {
    Jpeg = 0,
    Png = 1,
    Gif = 2,
    WebP = 3,
    Tiff = 4,
    Unkn = 5
} CSI_SupportedFileTypes;

typedef struct CSI_Result {
    bool success;
    uint64_t code;
    char *error_message;
} CSI_Result;

typedef struct CSI_Parameters {
    bool keep_metadata;
    uint32_t jpeg_quality;
    uint32_t jpeg_chroma_subsampling; // support 444, 422, 420, 411
    bool jpeg_progressive;
    uint32_t  png_quality;
    uint32_t  png_optimization_level;
    bool  png_force_zopfli;
    uint32_t  gif_quality;
    uint32_t  webp_quality;
    uint32_t  tiff_compression; // support 1:Lzw 2:Deflate 3:Packbits Other Int:Uncompressed
    uint32_t  tiff_deflate_level; // support 1:Fast 6:Balanced Other Int:Best
    bool  optimize;
    uint32_t  width;
    uint32_t  height;
} CSI_Parameters;

CSI_Result csi_compress(const char* input_path, const char* output_path, CSI_Parameters* params);

CSI_Result csi_compress_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params);

CSI_Result csi_compress_to_size(const char* input_path, const char* output_path, CSI_Parameters* params, uint64_t max_output_size, bool return_smallest);

CSI_Result csi_compress_to_size_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params, uint64_t max_output_size, bool return_smallest);

CSI_Result csi_convert(const char* input_path, const char* output_path, CSI_SupportedFileTypes format, CSI_Parameters* params);

CSI_Result csi_convert_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_SupportedFileTypes format, CSI_Parameters* params);

#ifdef __cplusplus
}
#endif

#endif
