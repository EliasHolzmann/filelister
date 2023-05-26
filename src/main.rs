use std::{path::PathBuf, fs};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args.get(0).unwrap_or(&String::new()));
        return;
    }
    let path = PathBuf::from(args[1].clone());

    list_contents_for(path);
}

fn list_contents_for(path: PathBuf) {
    for dir in fs::read_dir(path).unwrap() {
        let dir_entry = dir.unwrap();

        let file_type = dir_entry.metadata().unwrap().file_type();

        if file_type.is_dir() {
            list_contents_for(dir_entry.path());
        } else if file_type.is_file() {
            println!("{:010} {}", dir_entry.metadata().unwrap().len(), dir_entry.path().display());
        } else {
            panic!("Unexpected file type");
        }
    }
}
