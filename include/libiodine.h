#ifndef LIB_CAESIUM_IODIDE_H
#define LIB_CAESIUM_IODIDE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum SupportedFileTypes {
  Jpeg,
  Png,
  Gif,
  WebP,
  Tiff,
  Unkn,
} SupportedFileTypes;

typedef struct CCSResult {
  bool success;
  uint32_t code;
  const char *error_message;
} CCSResult;

typedef struct CCSParameters {
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
} CCSParameters;

typedef struct CByteArray {
  uint8_t *data;
  uintptr_t length;
} CByteArray;

struct CCSResult c_compress(const char *input_path,
                            const char *output_path,
                            struct CCSParameters params);

struct CCSResult c_compress_in_memory(const uint8_t *input_data,
                                      uintptr_t input_length,
                                      struct CCSParameters params,
                                      struct CByteArray *output);

struct CCSResult c_compress_to_size(const char *input_path,
                                    const char *output_path,
                                    struct CCSParameters params,
                                    uintptr_t max_output_size,
                                    bool return_smallest);

struct CCSResult c_convert(const char *input_path,
                           const char *output_path,
                           enum SupportedFileTypes format,
                           struct CCSParameters params);

void c_free_byte_array(struct CByteArray byte_array);

void c_free_string(char *ptr);

/////////////////////////////////////////////////////////

void iod_free_buffer(CByteArray byte_array);

struct CCSResult iod_compress_in_memory(
    const uint8_t *input_data,
    uintptr_t input_length,
    struct CCSParameters params,
    struct CByteArray *output
);

struct CCSResult iod_compress_to_size_in_memory(
    const uint8_t *input_data,
    uintptr_t input_length,
    struct CCSParameters params,
    uintptr_t max_output_size,
    bool return_smallest,
    struct CByteArray *output
);

struct CCSResult iod_convert_in_memory(
    const uint8_t *input_data,
    uintptr_t input_length,
    enum SupportedFileTypes format,
    struct CCSParameters params,
    struct CByteArray *output
);

#ifdef __cplusplus
}
#endif

#endif
