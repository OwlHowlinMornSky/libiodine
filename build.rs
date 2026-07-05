fn main() {
    // Only runs when for Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();

        // VFT_DLL = 2
        res.set("FileType", "2");

        res.set("FileDescription", "iodine - Image Compression Library");
        res.set("LegalCopyright", "Copyright (C) 2025-2026 Tyler Parret True <https://github.com/OwlHowlinMornSky>; Copyright (C) 2016-2026 Matteo Paonessa <matteo.paonessa@gmail.com>");
        res.set("CompanyName", "Tyler Parret True, Matteo Paonessa");
        //res.set("OriginalFilename", "iodine.dll");

        // `winresource` will get these from Cargo.toml automatically, but override is available.
        // res.set("ProductName", "My Product");
        // res.set("FileVersion", "1.0.0.0");
        // res.set("ProductVersion", "1.0.0.0");

        // 编译并链接资源文件
        res.compile().unwrap();
    }
}
