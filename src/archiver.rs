use std::{
    fs,
    collections::HashMap,
    io::BufWriter,
    os::unix::fs::MetadataExt
};
use serde::{Serialize, Deserialize};
pub use de::{BorrowDecode, Decode};
pub use enc::Encode;
use bincode::{encode_into_writer, Config}; 

enum FileType {
    RegFile,
    SymbFile,
    HrdLnkFile,
    CharDev,
    BlocDev
}

#[derive(Serialize, Deserialize, Debug)]
struct RatEntryFile {
    _path: String,
    _content: Option<Vec<u8>>,
    _size: u64,
    _target: Option<String>,   // Only if it's a symlink
    _inode: u64,      
    _dev: u64,
    _mode: u32,
    _m_time: u64, // modified time stamp
    _type: FileType
}

#[derive(Serialize, Deserialize, Debug)]
struct RatEntryDir {
    _path: String,
    _files: Vec<RatEntryFile>
}

fn serialize_file_entry(path: &String, data: &Metadata, writer: &mut BufWriter<File>, seen: &mut HashMap<(u64, u64), String>) -> std::io::Result<()>  {

    let mut entry: RatEntryFile;

    if data.is_file() {

        if seen.contains_key((data.ino(), data.dev())) { // If hard linked to an already encountered file
            entry = RatEntryFile {
                _path: path.to_string(),
                _content: None,
                _size: data.size(),
                _target: None,   
                _inode: data.ino(),      
                _dev: data.dev(),
                _mode: data.mode(),
                _m_time: data.mtime(),
                _type: FileType::HrdLnkFile
            }
        } else {
            entry = RatEntryFile {
                _path: path.to_string(),
                _content: Some(fs::read(&path)?),
                _size: data.size(),
                _target: None,   
                _inode: data.ino(),      
                _dev: data.dev(),
                _mode: data.mode(),
                _m_time: data.mtime(),
                _type: FileType::RegFile
            }
        }

        seen.insert((entry._inode, entry._dev), fs::canonicalize(&path)?.to_string_lossy().to_string())
    } else if data.is_symlink() {
        entry = RatEntryFile {
            _path: path.to_string(),
            _content: None,
            _size: data.size(),
            _target: Some(fs::read_link(&path)?.to_string_lossy().to_string()),   
            _inode: data.ino(),      
            _dev: data.dev(),
            _mode: data.mode(),
            _m_time: data.mtime(),
            _type: FileType::SymbFile
        }
    }

    encode_into_writer(entry, &mut writer, Config::default())?;
    Ok(())
}

pub fn archive_file(_rat_name: String, _paths: &Vec<String>) -> std::io::Result<()> {

    let mut archive = File::create(format!("{}.rat", _rat_name))?;
    let mut writer = BufWriter::new(archive);
    let mut seen: HashMap<(u64, u64), String> = HashMap::new(); // Checks if two files are the same file or hard-linked by mapping each dev and inode number to an absolute path

    for path in _paths.iter() {

        let data = match fs::symlink_metadata(path) {
            Ok(data) => data,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("PATH ERR: {} does not exist!", path))),
        };

        if if data.is_dir() {
            
        } else { // if its a file or a symbolic link

        }

    }
}

pub fn extract_file(_rat_name: String, _path_out: String) -> () {

}