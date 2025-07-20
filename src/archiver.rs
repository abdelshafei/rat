mod config;

fn construct_file_entry(path: &String, data: &Metadata, seen: &mut HashMap<(u64, u64), Option<String>>) -> RatEntryFile {
    let entry: RatEntryFile; // Assigning only once to the varibale so we dont need the mut keyword

    if data.is_file() {

        let canonical = Some(fs::canonicalize(path)?.to_string_lossy().to_string());

        if let Some(existing) = seen.get(&(data.ino(), data.dev())) {

            if existing == &canonical { // Same path referencing the same file (non hard link)
                entry = RatEntryFile {
                    path: path.to_string(),
                    content: Some(fs::read(&path)?),
                    size: data.size(),
                    target: None,   
                    inode: data.ino(),      
                    dev: data.dev(),
                    mode: data.mode(),
                    mtime: data.mtime(),
                    type: FileType::RegFile
                };

            } else { // Hard linked
                entry = RatEntryFile {
                    path: path.to_string(),
                    content: None,
                    size: data.size(),
                    target: None,   
                    inode: data.ino(),      
                    dev: data.dev(),
                    mode: data.mode(),
                    mtime: data.mtime(),
                    type: FileType::HrdLnkFile
                };

            }

        } else {
            entry = RatEntryFile {
                path: path.to_string(),
                content: Some(fs::read(&path)?),
                size: data.size(),
                target: None,   
                inode: data.ino(),      
                dev: data.dev(),
                mode: data.mode(),
                mtime: data.mtime(),
                type: FileType::RegFile
            };

        }

        seen.insert((entry.inode, entry.dev), Some(fs::canonicalize(&path)?.to_string_lossy().to_string()));
    } else { // Is symbolic link
        entry = RatEntryFile {
            path: path.to_string(),
            content: None,
            size: data.size(),
            target: Some(fs::read_link(&path)?.to_string_lossy().to_string()),   
            inode: data.ino(),      
            dev: data.dev(),
            mode: data.mode(),
            mtime: data.mtime(),
            type: FileType::SymbFile
        };
    }

    return entry;
}

fn fetch_dir_files(path: &String, writer: &mut BufWriter<File>, seen: &mut HashMap<(u64, u64), String>) ->std::io::Result<(RatEntryDir)>  {

    let mut entries: Vec<RatEntry> = Vec::new();
    let mut dir_entries: HashMap<String, RatEntryFile> = HashMap::new();
    let mut dirs: HashMap<String, RatEntryDir> = HashMap::new();

    for entry in WalkDir::new(path).follow_links(false) { // ignores entries the owner of process has no access previlige to it
       let entry = match entry {
            Ok(e) => e,
            Err(e) => if e.io_error().map_or(false, |e| e.kind() == io::ErrorKind::PermissionDenied) {
                eprintln!("Skipping inaccessible entry: {}", e);
                continue;
            }
       }

       if entry.is_file() {
            if let Some(parent) = entry.path().parent() {
                let parent_str = parent.to_string_lossy().to_string();
                let file_entry = construct_file_entry(&entry.path(), &fs::symlink_metadata(entry.path())?, seen);
                dir_entries.insert(parent_str, file_entry);
            }
        }
    }

    for (parent_path, file_meta) in dir_entries {
        if parent_path == path {
            entries.push(file_meta);
        } else {
            if dirs.contains(parent_path) {
                dirs.get(parent_path).files.push(file_meta)
            } else {
                let mut dir_data = RatEntryDir {
                    path: String::from(parent_path),
                    files: Vec::new(),
                };
                dirs.insert(parent_path, dir_data);
                dir_data.files.push(file_meta);
            }
        }
    }

    for (_, dir_metadata) in dirs {
        entries.push(dir_metadata);
    }

    let mut dir_entry = RatEntryDir {
        path: path,
        files: entries,
    };

    return dir_entry;
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
            serialize_into(writer, &construct_file_entry(&path, &data, &mut seen)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }
    }

    writer.flush()?;

    Ok(())
}