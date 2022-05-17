use super::utils::*;
use crate::core::attributes_cache::*;
use crate::core::lib_utils::Result;
use std::path::PathBuf;
use std::sync::Arc;
use {
    async_trait::async_trait,
    filetime::FileTime,
    tokio::{
        fs,
        io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
        sync::{Mutex, RwLock},
    },
    tokio_stream::{wrappers::ReadDirStream, StreamExt},
};

use crate::core::fcall::*;
use crate::core::srv::Fid;
use crate::core::srv::Filesystem;
use tokio::io::SeekFrom;

#[derive(Default)]
pub struct UnpfsFid {
    realpath: RwLock<PathBuf>,
    file: Mutex<Option<fs::File>>,
}

#[derive(Clone)]
pub struct Unpfs {
    pub realroot: PathBuf,
    pub vap: Arc<Mutex<VirtualAttributesProvider>>,
}

//todo -add feature maybe?
const DEBUG_FLAGS: bool = false;

impl Unpfs {
    async fn get_va_from_realpath(&self, realpath: &PathBuf) -> Result<VirtualAttributes> {
        let my_str = realpath.clone().into_os_string().into_string().unwrap();

        let mut vap = self.vap.lock().await;
        let va = vap.get_or_create_virtual_attributes(my_str);
        va
    }
    /*async fn get_va_from_os_string(&self, os_str: OsString) -> Result<VirtualAttributes> {
        let my_str = os_str.into_string().unwrap();
        let mut vap = self.vap.lock().await;
        let va = vap.get_or_create_virtual_attributes(my_str);
        va
    }*/
    async fn update_permission_mode_va(&self, realpath: &PathBuf, mode: u32) -> Result<()> {
        let my_str = realpath.clone().into_os_string().into_string().unwrap();
        let mut vap = self.vap.lock().await;
        vap.update_virtual_attributes(my_str, |va| {
            log::debug!("Changing attributes from: {} to {}", va.mode, mode);
            va.mode = mode;
        })
    }
}

#[async_trait]
impl Filesystem for Unpfs {
    type Fid = UnpfsFid;

    fn get_mount_point(&self) -> &PathBuf {
        &self.realroot
    }
    async fn rattach(
        &self,
        fid: &Fid<Self::Fid>,
        _afid: Option<&Fid<Self::Fid>>,
        _uname: &str,
        _aname: &str,
        _n_uname: u32,
    ) -> Result<Fcall> {
        let realpath = {
            let mut realpath = fid.aux.realpath.write().await;
            *realpath = PathBuf::from(&self.realroot);
            realpath.clone()
        };

        let va = self.get_va_from_realpath(&realpath).await?;

        Ok(Fcall::Rattach {
            qid: get_qid(&self.realroot, &va).await?,
        })
    }

    async fn rwalk(
        &self,
        fid: &Fid<Self::Fid>,
        newfid: &Fid<Self::Fid>,
        wnames: &[String],
    ) -> Result<Fcall> {
        let mut wqids = Vec::new();
        let mut path = {
            let realpath = fid.aux.realpath.read().await;
            realpath.clone()
        };

        for (i, name) in wnames.iter().enumerate() {
            path.push(name);
            let va = self.get_va_from_realpath(&path).await?;

            let qid = match get_qid(&path, &va).await {
                Ok(qid) => qid,
                Err(e) => {
                    if i == 0 {
                        return Err(e);
                    } else {
                        break;
                    }
                }
            };

            wqids.push(qid);
        }

        {
            let mut new_realpath = newfid.aux.realpath.write().await;
            *new_realpath = path;
        }

        Ok(Fcall::Rwalk { wqids: wqids })
    }

    async fn rgetattr(&self, fid: &Fid<Self::Fid>, req_mask: GetattrMask) -> Result<Fcall> {
        let realpath = { fid.aux.realpath.read().await.clone() };
        let va = self.get_va_from_realpath(&realpath).await?;

        let attr = { fs::symlink_metadata(&*realpath).await? };

        Ok(Fcall::Rgetattr {
            valid: req_mask,
            qid: qid_from_attr(&attr, &va),
            stat: va.into(),
        })
    }

    async fn rsetattr(
        &self,
        fid: &Fid<Self::Fid>,
        valid: SetattrMask,
        stat: &SetAttr,
    ) -> Result<Fcall> {
        let filepath = {
            let realpath = fid.aux.realpath.read().await;
            realpath.clone()
        };

        // TODO: add test
        if valid.contains(SetattrMask::MODE) {
            self.update_permission_mode_va(&filepath, stat.mode).await?;
            // fs::set_permissions(&filepath, PermissionsExt::from_mode(stat.mode)).await?;
        }

        if valid.intersects(SetattrMask::UID | SetattrMask::GID) {
            /*let uid = if valid.contains(SetattrMask::UID) {
                None//Some(nix::unistd::Uid::from_raw(stat.uid))
            } else {
                None
            };
            let gid = if valid.contains(SetattrMask::GID) {
            None //   Some(nix::unistd::Gid::from_raw(stat.gid))
            } else {
                None
            };*/
            //  nix::unistd::chown(&filepath, uid, gid)?;
        }

        if valid.contains(SetattrMask::SIZE) {
            let _ = fs::OpenOptions::new()
                .write(true)
                .create(false)
                .open(&filepath)
                .await?
                .set_len(stat.size)
                .await?;
        }

        if valid.intersects(SetattrMask::ATIME_SET | SetattrMask::MTIME_SET) {
            let attr = fs::metadata(&filepath).await?;
            let atime = if valid.contains(SetattrMask::ATIME_SET) {
                FileTime::from_unix_time(stat.atime.sec as i64, stat.atime.nsec as u32)
            } else {
                FileTime::from_last_access_time(&attr)
            };

            let mtime = if valid.contains(SetattrMask::MTIME_SET) {
                FileTime::from_unix_time(stat.mtime.sec as i64, stat.mtime.nsec as u32)
            } else {
                FileTime::from_last_modification_time(&attr)
            };

            let _ = tokio::task::spawn_blocking(move || {
                filetime::set_file_times(filepath, atime, mtime)
            })
            .await;
        }

        Ok(Fcall::Rsetattr)
    }

    async fn rreadlink(&self, fid: &Fid<Self::Fid>) -> Result<Fcall> {
        let link = {
            let realpath = fid.aux.realpath.read().await;
            fs::read_link(&*realpath).await?
        };

        Ok(Fcall::Rreadlink {
            target: link.to_string_lossy().into_owned(),
        })
    }

    async fn rreaddir(&self, fid: &Fid<Self::Fid>, off: u64, count: u32) -> Result<Fcall> {
        let mut dirents = DirEntryData::new();

        let realpath = {
            let realpath = fid.aux.realpath.read().await;
            realpath.clone()
        };

        let offset = if off == 0 {
            let va = self.get_va_from_realpath(&realpath).await?;
            dirents.push(get_dirent_from(".", 0, &va).await?);
            dirents.push(get_dirent_from("..", 1, &va).await?);
            off
        } else {
            off - 1
        } as usize;

        let mut entries = { ReadDirStream::new(fs::read_dir(&*realpath).await?).skip(offset) };

        let mut i = offset;
        while let Some(entry) = entries.next().await {
            let entr = entry?;
            let realpath = realpath.join(entr.file_name());
            let va = self.get_va_from_realpath(&realpath).await?;
            let dirent = get_dirent(&entr, 2 + i as u64, &va).await?;
            if dirents.size() + dirent.size() > count {
                break;
            }
            dirents.push(dirent);
            i += 1;
        }

        Ok(Fcall::Rreaddir { data: dirents })
    }

    async fn rlopen(&self, fid: &Fid<Self::Fid>, flags: u32) -> Result<Fcall> {
        let realpath = {
            let realpath = fid.aux.realpath.read().await;
            realpath.clone()
        };
        let va = self.get_va_from_realpath(&realpath).await?;
        let fmode = FileOpenMode::from_bits_truncate(flags);

        if DEBUG_FLAGS {
            print!(
                "Rlopen {} flags: {} (0x{:x}) bits:(",
                realpath.clone().into_os_string().into_string().unwrap(),
                flags,
                flags,
            );
            let mut u32_val: u32 = 0x1;
            let mut first = true;
            for i in 0..32 {
                if flags & u32_val != 0 {
                    if !first {
                        print!(", ");
                    }
                    first = false;
                    print!("0x{:x}=1<<{}", u32_val, i);
                }
                u32_val <<= 1;
            }
            print!(")\n");
            println!("{:?}", fmode);
        }

        let qid = get_qid(&realpath, &va).await?;
        if !qid.typ.contains(QidType::DIR) {
            let fd = if fmode.intersects(FileOpenMode::P9_DOTL_WRONLY) {
                tokio::fs::OpenOptions::new()
                    .write(true)
                    .open(&realpath)
                    .await?
            } else if fmode.intersects(FileOpenMode::P9_DOTL_RDWR) {
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&realpath)
                    .await?
            } else {
                fs::File::open(&realpath).await?
            };

            {
                let mut file = fid.aux.file.lock().await;
                *file = Some(fd);
            }
        }

        Ok(Fcall::Rlopen {
            qid: qid,
            iounit: 0,
        })
    }

    async fn rlcreate(
        &self,
        fid: &Fid<Self::Fid>,
        name: &str,
        flags: u32,
        mode: u32,
        _gid: u32,
    ) -> Result<Fcall> {
        let path = {
            let realpath = fid.aux.realpath.read().await;
            realpath.join(name)
        };
        let fmode = FileOpenMode::from_bits_truncate(flags);
        // let oflags = 0;//nix::fcntl::OFlag::from_bits_truncate((flags & UNIX_FLAGS) as i32);
        // let omode = 0;//nix::sys::stat::Mode::from_bits_truncate(mode);
        //let fd = 0;//nix::fcntl::open(&path, oflags, omode)?;
        if DEBUG_FLAGS {
            print!(
                "Create {} flags: {} (0x{:x}) mode: {} (0x{:x}) bits:(",
                path.clone().into_os_string().into_string().unwrap(),
                flags,
                flags,
                mode,
                mode
            );
            let mut u32_val: u32 = 0x1;
            let mut first = true;
            for i in 0..32 {
                if flags & u32_val != 0 {
                    if !first {
                        print!(", ");
                    }
                    first = false;
                    print!("0x{:x}=1<<{}", u32_val, i);
                }
                u32_val <<= 1;
            }
            print!(")\n");
            println!("{:?}", fmode);
        }

        let fd = if fmode.intersects(FileOpenMode::P9_DOTL_WRONLY) {
            tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .await?
        } else if fmode.intersects(FileOpenMode::P9_DOTL_RDWR) {
            tokio::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&path)
                .await?
        } else {
            tokio::fs::OpenOptions::new()
                .create(true)
                .read(true)
                .open(&path)
                .await?
        };

        let va = self.get_va_from_realpath(&path).await?;
        let qid = get_qid(&path, &va).await?;
        {
            let mut realpath = fid.aux.realpath.write().await;
            *realpath = path;
        }
        {
            let mut file = fid.aux.file.lock().await;
            *file = Some(fd);
        }

        Ok(Fcall::Rlcreate { qid, iounit: 0 })
    }

    async fn rread(&self, fid: &Fid<Self::Fid>, offset: u64, count: u32) -> Result<Fcall> {
        let buf = {
            let mut file = fid.aux.file.lock().await;
            let file = file.as_mut().ok_or(io_err!(InvalidInput, "Invalid fid"))?;
            file.seek(SeekFrom::Start(offset)).await?;

            let mut buf = create_buffer(count as usize);
            let bytes = file.read_exact(&mut buf[..]).await?;

            buf.truncate(bytes);
            buf
        };

        Ok(Fcall::Rread { data: Data(buf) })
    }

    async fn rwrite(&self, fid: &Fid<Self::Fid>, offset: u64, data: &Data) -> Result<Fcall> {
        let count = {
            let mut file = fid.aux.file.lock().await;
            let file = file.as_mut().ok_or(io_err!(InvalidInput, "Invalid fid"))?;
            file.seek(SeekFrom::Start(offset)).await?;

            file.write_all(&data.0).await?;
            data.0.len() as u32
        };

        Ok(Fcall::Rwrite { count })
    }

    async fn rmkdir(
        &self,
        dfid: &Fid<Self::Fid>,
        name: &str,
        _mode: u32,
        _gid: u32,
    ) -> Result<Fcall> {
        let path = {
            let realpath = dfid.aux.realpath.read().await;
            realpath.join(name)
        };

        fs::create_dir(&path).await?;

        let va = self.get_va_from_realpath(&path).await?;
        Ok(Fcall::Rmkdir {
            qid: get_qid(&path, &va).await?,
        })
    }

    async fn rrenameat(
        &self,
        olddir: &Fid<Self::Fid>,
        oldname: &str,
        newdir: &Fid<Self::Fid>,
        newname: &str,
    ) -> Result<Fcall> {
        let oldpath = {
            let realpath = olddir.aux.realpath.read().await;
            realpath.join(oldname)
        };

        let newpath = {
            let realpath = newdir.aux.realpath.read().await;
            realpath.join(newname)
        };

        fs::rename(&oldpath, &newpath).await?;

        Ok(Fcall::Rrenameat)
    }

    async fn runlinkat(&self, dirfid: &Fid<Self::Fid>, name: &str, _flags: u32) -> Result<Fcall> {
        let path = {
            let realpath = dirfid.aux.realpath.read().await;
            realpath.join(name)
        };

        let attr = fs::symlink_metadata(&path).await?;

        if attr.is_dir() {
            fs::remove_dir(&path).await?;
        } else {
            fs::remove_file(&path).await?;
        }

        Ok(Fcall::Runlinkat)
    }

    async fn rfsync(&self, fid: &Fid<Self::Fid>) -> Result<Fcall> {
        {
            let mut file = fid.aux.file.lock().await;
            file.as_mut()
                .ok_or(io_err!(InvalidInput, "Invalid fid"))?
                .sync_all()
                .await?;
        }

        Ok(Fcall::Rfsync)
    }

    async fn rclunk(&self, _: &Fid<Self::Fid>) -> Result<Fcall> {
        Ok(Fcall::Rclunk)
    }

    async fn rstatfs(&self, _fid: &Fid<Self::Fid>) -> Result<Fcall> {
        log::error!("rstatfs not implemented");
        /*let path = {
            let realpath = fid.aux.realpath.read().await;
            realpath.clone()
        };*/

        //let fs = nix::sys::statvfs::statvfs(&path)?;
        /* let fs = tokio::task::spawn_blocking(move || nix::sys::statvfs::statvfs(&path))
                    .await
                    .unwrap()?;
        */
        /*Ok(Fcall::Rstatfs {
            statfs: From::from(fs),
        })*/
        return res!(io_err!(Other, std::format!("rstatfs not implemented")));
    }
}
