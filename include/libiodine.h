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
    const char *error_message;
} CSI_Result;

typedef struct CSI_Parameters {
    bool keep_metadata = false;
    uint32_t jpeg_quality = 80;
    uint32_t jpeg_chroma_subsampling = 0; // support 444, 422, 420, 411
    bool jpeg_progressive = true;
    bool jpeg_optimize = false;
    bool jpeg_preserve_icc = true;
    uint32_t  png_quality = 80;
    uint32_t  png_optimization_level = 3;
    bool  png_force_zopfli = false;
    bool  png_optimize = false;
    uint32_t  gif_quality = 80;
    uint32_t  webp_quality = 80;
    bool webp_lossless = false;
    uint32_t  tiff_compression = 2; // support 1:Lzw 2:Deflate 3:Packbits Other Int:Uncompressed
    uint32_t  tiff_deflate_level = 6; // support 1:Fast 6:Balanced Other Int:Best
    uint32_t  width = 0;
    uint32_t  height = 0;
    bool  allow_magnify = false;
    bool  reduce_by_power_of_2 = false;
    uint32_t short_side_pixels = 0;
    uint32_t long_size_pixels = 0;
} CSI_Parameters;

typedef struct CByteArray {
  uint8_t *data;
  uintptr_t length;
} CSI_ByteArray;

CSI_Result csi_compress(const char* input_path, const char* output_path, CSI_Parameters* params);

CSI_Result csi_compress_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params);

CSI_Result csi_compress_fromto(const void* input_buffer, uint64_t ibuflen, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params);

CSI_Result csi_compress_in_memory(const uint8_t *input_data, uintptr_t input_length, struct CSI_Parameters params, struct CSI_ByteArray *output);

CSI_Result csi_compress_to_size(const char* input_path, const char* output_path, CSI_Parameters* params, uint64_t max_output_size, bool return_smallest);

CSI_Result csi_compress_to_size_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params, uint64_t max_output_size, bool return_smallest);

CSI_Result csi_compress_to_size_fromto(const void* input_buffer, uint64_t ibuflen, void* output_buffer, uint64_t obufmaxlen, CSI_Parameters* params, uint64_t max_output_size, bool return_smallest);

CSI_Result csi_convert(const char* input_path, const char* output_path, CSI_SupportedFileTypes format, CSI_Parameters* params);

CSI_Result csi_convert_into(const char* input_path, void* output_buffer, uint64_t obufmaxlen, CSI_SupportedFileTypes format, CSI_Parameters* params);

CSI_Result csi_convert_fromto(const void* input_buffer, uint64_t ibuflen, void* output_buffer, uint64_t obufmaxlen, CSI_SupportedFileTypes format, CSI_Parameters* params);

void csi_free_byte_array(struct CSI_ByteArray byte_array);

void csi_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif
