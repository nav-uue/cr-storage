use std::path::{Path, PathBuf};
use std::process::Command;


pub struct FileArgs {
    pub path: PathBuf,
    pub name: String,
    pub size: u64
}


/// Create a small file to act the disk image
pub fn create_image_file(args: FileArgs) -> Result<(), std::io::Error> {

    println!("Create image file: {}", args.name);

    let full_path = args.path.join(&args.name);

    // create file in target directory
    let file = std::fs::File::create(&full_path)?;

    println!("Path: {}", &full_path.display());
 
    println!("Size: {}", &args.size);

    // Write zero bytes to simulate an image. 32MB minimum size for the ext4 FS 
    file.set_len(&args.size * 1024 * 1024)?;

    Ok(())
}