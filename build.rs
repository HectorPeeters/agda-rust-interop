use std::process::Command;

const AGDA_SOURCE: &str = "Test.agda";

fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-changed=src/{}", AGDA_SOURCE);

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::fs::copy(
        format!("src/{}", AGDA_SOURCE),
        format!("{}/{}", out_dir.to_str().unwrap(), AGDA_SOURCE),
    )?;

    Command::new("agda2rust")
        .current_dir(out_dir)
        .arg("--rust-no-main")
        .arg(AGDA_SOURCE)
        .output()?;

    Ok(())
}
