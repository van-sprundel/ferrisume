use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let dest_path = out_dir.join("theme");

    // Copy default theme files
    let theme_dir = PathBuf::from("themes/default");
    copy_dir_all(theme_dir, &dest_path).unwrap();

    println!("cargo:rerun-if-changed=themes/default");
}
