use super::fcall::{Stat, Time};
use super::lib_utils::Result;
use log;
use std::collections::HashMap;
use std::fs;
use std::fs::Metadata;

#[cfg(target_os = "linux")]
use std::os::unix::prelude::MetadataExt;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

pub struct VirtualAttributesProvider {
    pub attributes_map: HashMap<String, VirtualAttributes>,
    pub next_inode: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum VAFileType {
    VaFile,
    VaDirectory,
}

#[derive(Debug, Copy, Clone)]
pub struct VirtualAttributes {
    pub inode: u64,
    pub file_size: u64,
    pub file_type: VAFileType,
    pub mode: u32,
    pub creation_time: u64,
    pub access_time: u64,
    pub write_time: u64,
}

impl VirtualAttributesProvider {
    pub fn new() -> VirtualAttributesProvider {
        VirtualAttributesProvider {
            attributes_map: HashMap::new(),
            next_inode: 100,
        }
    }

    pub fn get_or_create_virtual_attributes(
        &mut self,
        file_path: String,
    ) -> Result<VirtualAttributes> {
        match self.attributes_map.get_mut(&file_path) {
            Some(el) => {
                let metadata = fs::metadata(&file_path).map_err(|error| {
                    log::error!("File not found: {}", file_path);
                    error
                })?;

                el.update(metadata);

                Ok(*el)
            }
            None => {
                let metadata = fs::metadata(&file_path).map_err(|error| {
                    log::error!("File not found: {}", file_path);
                    error
                })?;

                let va = VirtualAttributes::new(self.next_inode, &metadata);

                log::debug!(
                    "Created new virtual attributes for file: {} inode: {}",
                    file_path,
                    self.next_inode
                );
                self.next_inode += 1;
                self.attributes_map.insert(file_path, va);
                Ok(va)
            }
        }
    }
    pub fn update_virtual_attributes<F: FnOnce(&mut VirtualAttributes)>(
        &mut self,
        file_path: String,
        f: F,
    ) -> Result<()> {
        if let Some(el) = self.attributes_map.get_mut(&file_path) {
            Ok(f(el))
        } else {
            res!(io_err!(
                Other,
                format!("Virtual attributes not found for directory {}", file_path)
            ))
        }
    }
}

#[cfg(target_os = "windows")]
impl VirtualAttributes {
    pub(crate) fn new(next_inode: u64, metadata: &Metadata) -> VirtualAttributes {
        let (file_type, default_mode) = if metadata.is_dir() {
            (VAFileType::VaDirectory, 16895)
        } else {
            (VAFileType::VaFile, 33279)
        };

        VirtualAttributes {
            inode: next_inode,
            file_type: file_type,
            file_size: metadata.file_size(),
            mode: default_mode,
            creation_time: metadata.creation_time(),
            access_time: metadata.last_access_time(),
            write_time: metadata.last_write_time(),
        }
    }

    fn update(&mut self, metadata: Metadata) {
        let file_type = if metadata.is_dir() {
            VAFileType::VaDirectory
        } else {
            VAFileType::VaFile
        };

        self.file_type = file_type;
        self.file_size = metadata.file_size();
        self.creation_time = metadata.creation_time();
        self.access_time = metadata.last_access_time();
        self.write_time = metadata.last_write_time();
    }
}

#[cfg(target_os = "linux")]
impl VirtualAttributes {
    pub(crate) fn new(_next_inode: u64, metadata: &Metadata) -> VirtualAttributes {
        // TODO: this probably can be better for linux?
        let (file_type, _default_mode) = if metadata.is_dir() {
            (VAFileType::VaDirectory, 16895)
        } else {
            (VAFileType::VaFile, 33279)
        };

        VirtualAttributes {
            inode: metadata.ino(),
            file_type: file_type,
            file_size: metadata.size(),
            // TODO: mode: default_mode?
            mode: metadata.mode(),
            creation_time: metadata.ctime() as u64,
            access_time: metadata.atime() as u64,
            write_time: metadata.mtime() as u64,
        }
    }

    fn update(&mut self, metadata: Metadata) {
        self.file_type = if metadata.is_dir() {
            VAFileType::VaDirectory
        } else {
            VAFileType::VaFile
        };

        // TODO: confirm those values
        self.inode = metadata.ino();
        self.file_size = metadata.size();
        self.creation_time = metadata.ctime() as u64;
        self.access_time = metadata.atime() as u64;
        self.write_time = metadata.mtime() as u64;
    }
}

impl From<VirtualAttributes> for Stat {
    fn from(va: VirtualAttributes) -> Self {
        let stat = Stat {
            mode: va.mode,
            uid: 1000,
            gid: 1000,
            nlink: 1,
            rdev: 0,
            size: va.file_size,
            blksize: 4096,
            blocks: va.file_size / 4096,
            atime: Time {
                sec: va.access_time / 10000000,
                nsec: (va.access_time % 10000000) * 100,
            },
            mtime: Time {
                sec: va.write_time / 10000000,
                nsec: (va.write_time % 10000000) * 100,
            },
            // TODO: Returns the last status change time of the file, in seconds since Unix Epoch.
            // last status change is the same as creation?
            ctime: Time {
                sec: va.creation_time / 10000000,
                nsec: (va.creation_time % 10000000) * 100,
            },
        };
        stat
    }
}
