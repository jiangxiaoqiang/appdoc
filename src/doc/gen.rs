use std::{fs::{self, File}, path::PathBuf, io::{Read, Write}};

use crate::config::initial::get_config;

pub fn start_process(){
    let source_path: String = get_config("source_path");
    let dart_files:Vec<PathBuf> = find_dart_files(&source_path);
    write_to_markdown(dart_files);
}


fn find_dart_files(folder_path: &str) -> Vec<std::path::PathBuf> {
    let mut dart_files = Vec::new();
    let paths = fs::read_dir(folder_path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap_or_default() == "dart" {
            dart_files.push(file_path);
        } else if file_path.is_dir() {
            dart_files.append(&mut find_dart_files(file_path.to_str().unwrap()));
        }
    }
    dart_files
}

pub fn write_to_markdown(dart_files:Vec<PathBuf>){
    let filenames = dart_files;
    let mut markdown = String::new();
    let doc_title: String = get_config("doc_title");
    markdown.push_str("# 项目代码\n\n");
    for filename in filenames {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        markdown.push_str("```dart\n");
        markdown.push_str(&contents);
        markdown.push_str("```\n\n");
    }
    let output_path: String = get_config("output_path");
    let mut output_file = File::create(output_path + "code.md").unwrap();
    output_file.write_all(markdown.as_bytes()).unwrap();
}

