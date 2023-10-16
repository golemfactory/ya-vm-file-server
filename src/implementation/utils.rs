use {
    crate::core::attributes_cache::VirtualAttributes,
    crate::core::fcall::*,
    crate::core::lib_utils::Result,
    std::{fs::Metadata, path::Path},
    tokio::fs,
};

fn file_type_to_byte(file_type: &std::fs::FileType) -> u8 {
    if file_type.is_file() {
        0x08
    } else if file_type.is_dir() {
        0x04
    } else if file_type.is_symlink() {
        0x12
    } else {
        0x00
    }
}

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
        typ: file_type_to_byte(&entry.file_type().await?),
        name: entry.file_name().to_string_lossy().into_owned(),
    })
}
