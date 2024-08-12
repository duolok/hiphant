use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Define the source directory
    let src_dir = "animals";
    
    // Get the output directory for the build artifacts
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("animals");

    // Create the destination directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&dest_path) {
        panic!("Failed to create destination directory {}: {}", dest_path.display(), e);
    }

    // Walk the source directory and copy each file
    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Ensure that the entry is a file before attempting to copy
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
