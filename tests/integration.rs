use iodine::parameters::CSParameters;

#[test]
fn unknown_file_type() {
    let output = "tests/samples/output/should_not_be_there";
    let params = CSParameters::new();
    let result = iodine::compress(
        String::from("tests/samples/output/.gitkeep"),
        String::from(output),
        &params,
    );
    assert!(result.is_err())
}
