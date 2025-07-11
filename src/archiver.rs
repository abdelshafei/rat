mod compressor;

use std::fs::Metadata;

struct ratEntry_file<'a> {
    dir_path: &'a  str,
    path: &'a  str,
    metadata: Metadata, 
}

struct ratEntry_dir<'a> {
    path: &'a str&
}

fn archiveFile(rat_name: str&, paths: [str&]&) -> () {
    
}