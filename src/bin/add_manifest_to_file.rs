use std::path::PathBuf;

use c2pa::{assertions::User, create_signer, Manifest, Result, SigningAlg};
// use tempfile::tempdir;

fn main() {
    let result = execute();
    println!("{:?}", result);
}

fn execute() -> Result<()> {
    let mut manifest = Manifest::new("my_app".to_owned());
    manifest.add_assertion(&User::new(
        "org.contentauth.mylabel",
        r#"{"my_tag":"Anything I want"}"#,
    ))?;

    let source = PathBuf::from("tests/fixtures/C.jpg");
    // let dir = tempdir()?;
    // let dest = dir.path().join("test_file.jpg");
    let dest = PathBuf::from("target/test_file.jpg");

    // Create a ps256 signer using certs and key files
    let signcert_path = "tests/fixtures/certs/ps256.pub";
    let pkey_path = "tests/fixtures/certs/ps256.pem";
    let signer = create_signer::from_files(signcert_path, pkey_path, SigningAlg::Ps256, None)?;

    // embed a manifest using the signer
    manifest.embed(&source, &dest, &*signer)?;

    Ok(())
}
