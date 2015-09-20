use std::io;
use std::fs;

use std::fs::ReadDir;
use std::path::PathBuf;

pub struct DirWalker {
    current: Option<ReadDir>,
    dirs: Vec<PathBuf>,
}

pub fn walk_dir<P: Into<PathBuf>>(path: P) -> io::Result<DirWalker> {
    let path_buf = path.into();
    let readdir = try!(fs::read_dir(&path_buf));
    Ok(DirWalker {
        current: Some(readdir),
        dirs: Vec::new()
    })
}

impl Iterator for DirWalker {
    type Item = io::Result<(PathBuf, fs::Metadata)>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            {
                let readdir = match self.current {
                    None => return None,
                    Some(ref mut readdir) => readdir
                };
                let dirs = &mut self.dirs;

                if let Some(readdir_result) = readdir.next() {
                    return Some(readdir_result.and_then(|entry| {
                        let metadata = try!(entry.metadata());
                        let path = entry.path();
                        if metadata.file_type().is_dir() {
                            dirs.push(path.clone());
                        }
                        Ok((entry.path(), metadata))
                    }));
                }
            }

            self.current = None;

            if let Some(next_dir) = self.dirs.pop() {
                match fs::read_dir(&next_dir) {
                    Ok(readdir) => {
                        self.current = Some(readdir);
                    }
                    Err(e) => {
                        self.dirs.push(next_dir);
                        return Some(Err(e));
                    }
                }
            } else {
                return None;
            };
        }
    }
}
