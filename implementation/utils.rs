use {
    crate::core::attributes_cache::VirtualAttributes,
    crate::core::fcall::*,
    crate::core::lib_utils::Result,
    std::{fs::Metadata, /*os::unix::prelude::*,*/ path::Path},
    tokio::fs,
};

pub fn create_buffer(size: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(size);
    unsafe {
        buffer.set_len(size);
    }
    buffer
}

pub async fn get_qid<T: AsRef<Path> + ?Sized>(path: &T, va: &VirtualAttributes) -> Result<Qid> {
    Ok(qid_from_attr(
        &fs::symlink_metadata(path.as_ref()).await?,
        va,
    ))
}

pub fn qid_from_attr(attr: &Metadata, va: &VirtualAttributes) -> Qid {
    Qid {
        typ: From::from(attr.file_type()),
        version: 0,
        path: va.inode,
    }
}

pub async fn get_dirent_from<P: AsRef<Path> + ?Sized>(
    p: &P,
    offset: u64,
    va: &VirtualAttributes,
) -> Result<DirEntry> {
    Ok(DirEntry {
        qid: get_qid(p, va).await?,
        offset: offset,
        typ: 0,
        name: p.as_ref().to_string_lossy().into_owned(),
    })
}

pub async fn get_dirent(
    entry: &fs::DirEntry,
    offset: u64,
    va: &VirtualAttributes,
) -> Result<DirEntry> {
    Ok(DirEntry {
        qid: qid_from_attr(&entry.metadata().await?, va),
        offset: offset,
        typ: 0,
        name: entry.file_name().to_string_lossy().into_owned(),
    })
}
