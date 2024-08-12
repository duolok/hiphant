use std::env;
use std::fs;
use std::path::{Path};

fn main() {
    let home_dir = env::var("HOME").expect("Could not find the home directory");

    let src_dir = Path::new(&home_dir).join(".config/hiphant/animals");

    if !src_dir.exists() {
        panic!("Source directory {} does not exist", src_dir.display());
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("animals");

    if let Err(e) = fs::create_dir_all(&dest_path) {
        panic!("Failed to create destination directory {}: {}", dest_path.display(), e);
    }

    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let dest_file = dest_path.join(file_name);

            if let Err(e) = fs::copy(&path, &dest_file) {
                panic!(
                    "Failed to copy {} to {}: {}",
                    path.display(),
                    dest_file.display(),
                    e
                );
            }
        }
    }
}

