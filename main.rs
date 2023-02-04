use std::env;
use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};

mod extract;
mod writeback;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: Gamemodd [extract|writeback]");
        return;
    }

    let input_folder = Path::new("input");

    match args[1].as_ref() {
        "extract" => {
            if !input_folder.exists() {
                println!("Error: input folder does not exist.");
                return;
            }

            for entry in read_dir(input_folder).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().unwrap() == "dat" {
                    let output_folder = input_folder.join(
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    create_dir_all(&output_folder).unwrap();
                    extract::extract_audiogroup(path, &output_folder).unwrap();
                }
            }
        },
        "writeback" => {
            let output_folder = input_folder.join("output");
            create_dir_all(&output_folder).unwrap();

        for entry in read_dir(input_folder).unwrap() {
				  let path = entry.unwrap().path();
				  if path.is_dir() {
					  let audiogroup_path = output_folder.join(
						  format!("{}.dat", path.file_name().unwrap().to_str().unwrap())
					  );
					  let files: Vec<&str> = read_dir(path)
						  .unwrap()
						  .map(|entry| entry.unwrap().path().to_str().unwrap())
						  .collect();
					writeback::write_to_audiogroup(files.len() as u32, 0x20, &files).unwrap();
				}
			}
