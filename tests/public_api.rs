#[test]
fn public_api() {
    // Install a compatible nightly toolchain if it is missing
    rustup_toolchain::install(public_api::MINIMUM_NIGHTLY_RUST_VERSION).unwrap();

    // Build rustdoc JSON
    let rustdoc_json = rustdoc_json::Builder::default()
        .toolchain(public_api::MINIMUM_NIGHTLY_RUST_VERSION)
        .features(["runtime-tokio-hyper"])
        .build()
        .unwrap();

    // Derive the public API from the rustdoc JSON
    let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
        .omit_auto_derived_impls(true)
        .omit_blanket_impls(true)
        .build()
        .unwrap();

    // Assert that the public API looks correct
    let content = expect_test::expect_file!["public-api.txt"].data();
    // We do not use assert_eq here since it prints waaaay too much
    if content != public_api.to_string() {
        panic!(
            r#"Error: The API changed.

        Use the following command if you want to commit these changes:
        UPDATE_EXPECT=1 cargo test --features runtime-tokio-hyper"#
        );
    }
}
