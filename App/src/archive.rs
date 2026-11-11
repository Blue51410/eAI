use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[allow(dead_code)]
pub struct Entry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[allow(dead_code)]
pub fn list(path: &Path) -> Result<Vec<Entry>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for i in 0..archive.len() {
        let f = archive.by_index(i).map_err(|e| e.to_string())?;
        out.push(Entry {
            name: f.name().map_err(|e| e.to_string())?.to_string(),
            size: f.size(),
            is_dir: f.is_dir(),
        });
    }
    Ok(out)
}

#[allow(dead_code)]
pub fn read_entry(path: &Path, name: &str) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut f = archive.by_name(name).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

#[allow(dead_code)]
pub fn write_entry(path: &Path, name: &str, data: &[u8], out: &Path) -> Result<(), String> {
    // copy archive, replace/add entry
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let out_file = File::create(out).map_err(|e| e.to_string())?;
    let mut writer = zip::ZipWriter::new(out_file);
    let options = zip::write::SimpleFileOptions::default();
    for i in 0..archive.len() {
        let mut f = archive.by_index(i).map_err(|e| e.to_string())?;
        let fname = f.name().map_err(|e| e.to_string())?.to_string();
        if fname == name {
            continue;
        }
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        writer.start_file(fname, options).map_err(|e| e.to_string())?;
        writer.write_all(&buf).map_err(|e| e.to_string())?;
    }
    writer.start_file(name.to_string(), options).map_err(|e| e.to_string())?;
    writer.write_all(data).map_err(|e| e.to_string())?;
    writer.finish().map_err(|e| e.to_string())?;
    Ok(())
}
