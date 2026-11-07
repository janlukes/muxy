use std::{env, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let resources = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../../packaging/windows");
    let resource = resources.join("app.rc");
    let icon = resources.join("Muxy.ico");
    println!("cargo:rerun-if-changed={}", resource.display());
    println!("cargo:rerun-if-changed={}", icon.display());
    let output = PathBuf::from(env::var("OUT_DIR")?).join("app.res");
    let status = Command::new("rc.exe")
        .arg("/nologo")
        .arg("/I")
        .arg(&resources)
        .arg("/fo")
        .arg(&output)
        .arg(&resource)
        .status()?;
    if !status.success() {
        return Err(std::io::Error::other("Windows icon resource compilation failed").into());
    }
    println!("cargo:rustc-link-arg-bin=muxy-app={}", output.display());
    Ok(())
}
