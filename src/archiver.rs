use std::{
    fs::{self, File, Metadata},
    collections::HashMap,
    io::{self, BufWriter},
    os::unix::fs::MetadataExt
};
use serde::{Serialize, Deserialize};
use bincode::serialize_into;
use std::io::Write;

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
    _path: String,
    _content: Option<Vec<u8>>,
    _size: u64,
    _target: Option<String>,   // Only if it's a symlink
    _inode: u64,      
    _dev: u64,
    _mode: u32,
    _m_time: i64, // modified time stamp
    _type: FileType
}

#[derive(Serialize, Deserialize, Debug)]
struct RatEntryDir {
    _path: String,
    _files: Vec<RatEntry>
}

fn serialize_file_entry(path: &String, data: &Metadata, writer: &mut BufWriter<File>, seen: &mut HashMap<(u64, u64), Option<String>>) -> std::io::Result<()>  {

    let entry: RatEntryFile; // Assigning only once to the varibale so we dont need the mut keyword

    if data.is_file() {

        let canonical = Some(fs::canonicalize(path)?.to_string_lossy().to_string());

        if let Some(existing) = seen.get(&(data.ino(), data.dev())) {

            if existing == &canonical { // Same path referencing the same file (non hard link)
                entry = RatEntryFile {
                    _path: path.to_string(),
                    _content: Some(fs::read(&path)?),
                    _size: data.size(),
                    _target: None,   
                    _inode: data.ino(),      
                    _dev: data.dev(),
                    _mode: data.mode(),
                    _m_time: data.mtime(),
                    _type: FileType::HrdLnkFile
                };

                serialize_into(writer, &entry).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            } else { // Hard linked
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
                };

                serialize_into(writer, &entry).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
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
            };

            serialize_into(writer, &entry).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        seen.insert((entry._inode, entry._dev), Some(fs::canonicalize(&path)?.to_string_lossy().to_string()));
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
        };

        serialize_into(writer, &entry).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

fn fetch_dir_files(_path: &String, _writer: &mut BufWriter<File>, _seen: &mut HashMap<(u64, u64), String>) -> std::io::Result<()>  {

    let mut paths: Vec<RatEntry>;

    for entry in WalkDir::new(_path).into_iter().filter_map(|e| e.ok()) { // ignores entries the owner of process has no access previlige to it
       
    }

    Ok(())
}

pub fn archive_file(_rat_name: String, _paths: &Vec<String>) -> std::io::Result<()> {

    for path in _paths.iter() {

        let _data = match fs::symlink_metadata(path) {
            Ok(data) => data,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => return Err(io::Error::new(io::ErrorKind::NotFound, format!("{} does not exist!", path))),
                io::ErrorKind::PermissionDenied => return Err(io::Error::new(io::ErrorKind::PermissionDenied, format!("Access denied for {}!", path))),
                io::ErrorKind::InvalidInput => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{} does not exist!", path))),
            },
        },
    };

    let archive = File::create(format!("{}.rat", _rat_name))?;
    let mut writer = BufWriter::new(archive);
    let mut seen: HashMap<(u64, u64), Option<String>> = HashMap::new(); // Checks if two files are the same file or hard-linked by mapping each dev and inode number to an absolute path

    for path in _paths.iter() {

        let data = fs::symlink_metadata(path)?;

        if data.is_dir() {
            fetch_dir_files(path, &mut writer, &mut seen);
        } else { // if its a file or a symbolic link
            serialize_file_entry(path, &data, &mut writer, &mut seen)?;
        }
    }

    writer.flush()?;

    Ok(())
}

pub fn extract_file(_rat_name: String, _path_out: String) -> std::io::Result<()> { Ok(()) }