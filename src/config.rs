use std::{
    fs::{self, File, Metadata},
    collections::HashMap,
    io::{self, BufWriter},
    os::unix::fs::MetadataExt
};
use serde::{Serialize, Deserialize};
use bincode::serialize_into;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
enum RatEntry {
    File(RatEntryFile),
    Dir(RatEntryDir)
}

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    RegFile,
    SymbFile,
    HrdLnkFile,
    CharDev,
    BlocDev
}

#[derive(Serialize, Deserialize, Debug)]
struct RatEntryFile {
    path: String,
    content: Option<Vec<u8>>,
    size: u64,
    target: Option<String>,   // Only if it's a symlink
    inode: u64,      
    dev: u64,
    mode: u32,
    mtime: i64, // modified time stamp
    type: FileType
}

#[derive(Serialize, Deserialize, Debug)]
struct RatEntryDir {
    path: String,
    files: Vec<RatEntry>
}