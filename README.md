# libiodine

Libiodine is a simple library performing JPEG, PNG, WebP and GIF (experimental) compression/optimization written in
Rust, with a C interface.

It can compress/convert images and write each one into a specified file or a memory buffer.

> [!WARNING]
> starting from v0.6.0 the library is written in Rust and no longer in C. There's a C interface, but it's not backward
> compatible with the <0.6.0.

## Usage example

Libcaesium exposes two functions, auto-detecting the input file type

```Rust
use caesium::parameters::CSParameters;
use caesium::compress;

let mut parameters = CSParameters::new();
parameters.keep_metadata = true;
parameters.jpeg.quality = 60;

let success = compress(input, output, &parameters).is_ok();
```

## Usage in C

Examples are in [UsageInC.md](UsageInC.md).

## Compilation

Compilation is available for all supported platforms: Windows, macOS and Linux.

> [!NOTE]
> if you don't use the `--release` flag, the PNG optimizations can take a very long time to complete, especially
> using the zopfli algorithm.
>

```bash
cargo build --release
```

The result will be a dynamic library usable by external applications through its C interface.

## Supported file types

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Tiff,
    Unkn,
}
```

## Compression vs Optimization

JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file
with 100 quality for 10 times will result in an always different image, even though you can't really see the difference.
Libiodine also supports optimization, by setting the _quality_ to 0. This performs a lossless process, resulting in the
same image, but with a smaller size (10-12% usually).  
GIF optimization is possible, but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to
losslessly convert from PNG or JPEG.
