use super::fcall::{Stat, Time};
use super::lib_utils::Result;
use log;
use std::collections::HashMap;
use std::fs;
use std::fs::Metadata;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::time::Duration;

#[cfg(any(target_os = "linux", target_os = "macos"))]
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
    // TODO: filetype is unused
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
                    #[cfg(feature = "debug-msg")]
                    log::error!("File not found despite existing attributes: {}", file_path);
                    error
                })?;

                el.update(metadata);

                Ok(*el)
            }
            None => {
                let metadata = fs::metadata(&file_path).map_err(|error| {
                    #[cfg(feature = "debug-msg")]
                    log::debug!("File not found: {}", file_path);
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
        // The returned 64-bit value is equivalent to a FILETIME struct, which represents the number of
        //  100-nanosecond intervals since January 1, 1601 (UTC).
        self.access_time = metadata.last_access_time();
        self.write_time = metadata.last_write_time();
    }
}

#[cfg(target_os = "windows")]
impl From<VirtualAttributes> for Stat {
    fn from(va: VirtualAttributes) -> Self {
        // Number of seconds that passed between win32 epoch (01-01-1601) and unix epoch (01-01-1970)
        const UNIX_WIN32_EPOCH_DIFF: u64 = 11644473600;

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
                sec: (va.access_time / 10000000) - UNIX_WIN32_EPOCH_DIFF,
                nsec: (va.access_time % 10000000) * 100,
            },
            mtime: Time {
                sec: (va.write_time / 10000000) - UNIX_WIN32_EPOCH_DIFF,
                nsec: (va.write_time % 10000000) * 100,
            },
            ctime: Time {
                sec: (va.creation_time / 10000000) - UNIX_WIN32_EPOCH_DIFF,
                nsec: (va.creation_time % 10000000) * 100,
            },
        };
        stat
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl VirtualAttributes {
    pub(crate) fn new(_next_inode: u64, metadata: &Metadata) -> VirtualAttributes {
        // TODO: this probably can be better for linux?
        let file_type = if metadata.is_dir() {
            VAFileType::VaDirectory
        } else {
            VAFileType::VaFile
        };

        VirtualAttributes {
            inode: metadata.ino(),
            file_type: file_type,
            file_size: metadata.size(),

            mode: metadata.mode(),

            creation_time: Duration::new(metadata.ctime() as u64, metadata.ctime_nsec() as u32)
                .as_nanos() as u64,
            access_time: Duration::new(metadata.atime() as u64, metadata.atime_nsec() as u32)
                .as_nanos() as u64,
            write_time: Duration::new(metadata.mtime() as u64, metadata.mtime_nsec() as u32)
                .as_nanos() as u64,
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
        self.creation_time =
            Duration::new(metadata.ctime() as u64, metadata.ctime_nsec() as u32).as_nanos() as u64;
        self.access_time =
            Duration::new(metadata.atime() as u64, metadata.atime_nsec() as u32).as_nanos() as u64;
        self.write_time =
            Duration::new(metadata.mtime() as u64, metadata.mtime_nsec() as u32).as_nanos() as u64;
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl From<VirtualAttributes> for Stat {
    fn from(va: VirtualAttributes) -> Self {
        let access = Duration::from_nanos(va.access_time);
        let write = Duration::from_nanos(va.write_time);
        let change = Duration::from_nanos(va.creation_time);

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
                sec: access.as_secs(),
                nsec: access.subsec_nanos() as u64,
            },
            mtime: Time {
                sec: write.as_secs(),
                nsec: write.subsec_nanos() as u64,
            },
            ctime: Time {
                sec: change.as_secs(),
                nsec: change.subsec_nanos() as u64,
            },
        };

        stat
    }
}
