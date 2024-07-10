use std::{fs::File, io::Read, path::PathBuf};

pub fn read(path: &PathBuf) -> String {
    let mut contents = String::new();
 
    if let Ok(mut file) = File::open(path) {
        if file.read_to_string(&mut contents).is_ok() {
            contents
        } else {
            println!("Error reading instructions file");
            std::process::exit(1);
        }
    } else {
        println!("Error reading instructions file");
        std::process::exit(1);
    }
}
