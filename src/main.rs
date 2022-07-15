use std::{env, process::Command};
use walkdir::{DirEntry, WalkDir};

fn filter_file_type(entry: &DirEntry) -> bool {
    let is_png = match entry.path().extension() {
        Some(str) => str == "png",
        None => false,
    };

    entry.file_type().is_dir() || is_png
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Searching for images in {} and subdirectories...",
        current_dir.display()
    );

    // let args: Vec<String> = env::args().collect();
    // println!("The args include {:?}", args);

    for entry in WalkDir::new(".")
        .follow_links(false) // don't follow symbolic links
        .into_iter()
        .filter_entry(|e| filter_file_type(e))
        .filter_map(|e| e.ok())
    {
        let f_path = entry.path().to_string_lossy();
        let f_extension = entry.path().extension();

        match f_extension {
            Some(str) => {
                println!("f_path: {}, f_extension {}", f_path, str.to_string_lossy());

                Command::new("cwebp")
                    .arg("-lossless")
                    .arg(f_path.to_ascii_lowercase()) // source file
                    .arg("-o")
                    .arg(f_path.to_ascii_lowercase().replace(".png", ".webp")) // output file
                    .spawn()
                    .expect(
                        "Error running cwebp.  Ensure it's installed (brew install webp on MacOS)",
                    );
            }
            None => println!("I'm a directory."),
        }
    }

    println!("Done!");

    Ok(())
}
