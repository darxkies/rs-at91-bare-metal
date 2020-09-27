use std::{env, error::Error, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out_dir.display());

    Build::new().file("src/startup.S")
		.compiler("arm-none-eabi-gcc")
		.compile("startup"); 

    println!("cargo:rerun-if-changed=startup.S"); 

    Ok(())
}
