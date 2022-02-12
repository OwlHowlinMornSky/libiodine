use std::collections::HashMap;
use std::sync::Once;
use std::fs;
use std::path::Path;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        if fs::metadata(file).is_ok() {
            fs::remove_file(file).unwrap();
        }
    });
}

pub fn cleanup(file: &str) {
    if fs::metadata(file).is_ok() {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn compress_80_with_metadata() {
    let output = "tests/samples/output/compressed_80_metadata.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 80;
    pars.keep_metadata = true;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    assert!(metadata_is_equal(Path::new("tests/samples/uncompressed_드림캐쳐.jpg"), Path::new(output)));
    cleanup(output)
}

#[test]
fn optimize_with_metadata() {
    let output = "tests/samples/output/optimized_metadata.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.optimize = true;
    pars.keep_metadata = true;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    assert!(metadata_is_equal(Path::new("tests/samples/uncompressed_드림캐쳐.jpg"), Path::new(output)));
    cleanup(output)
}

#[test]
fn resize_optimize_with_metadata() {
    let output = "tests/samples/output/resized_optimized_metadata.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.optimize = true;
    pars.keep_metadata = true;
    pars.width = 200;
    pars.height = 200;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    assert!(metadata_is_equal(Path::new("tests/samples/uncompressed_드림캐쳐.jpg"), Path::new(output)));
    cleanup(output)
}

fn extract_exif(path: &Path) -> HashMap<String, String> {
    let file = std::fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader).unwrap();
    let mut exif_map = HashMap::new();
    for f in exif.fields() {
        exif_map.insert(format!("{}", f.tag), f.display_value().to_string() as String);
    }

    exif_map
}

fn metadata_is_equal(input: &Path, output: &Path) -> bool {
    let original_exif_map = extract_exif(input);
    let compressed_exif_map = extract_exif(output);

    original_exif_map.eq(&compressed_exif_map)
}