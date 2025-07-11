use std::fs::Metadata;

struct RatEntryFile {
    absolutePath: String,
    dir_path: String,
    path: String,
    metadata: Metadata, 
}

struct RatEntryDir {
    absolutePath: String,
    path: String,
}

pub fn archive_file(_rat_name: String, _paths: &Vec<String>) -> () {
    
}

pub fn extract_file(_rat_name: String, _path_out: String) -> () {

}