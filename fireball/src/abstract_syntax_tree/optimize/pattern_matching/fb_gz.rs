use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

pub(super) fn is_fb_gz_path(path: &str) -> bool {
    path.trim().ends_with(".fb.gz")
}

pub(super) fn encode_source(source: &str) -> Result<Vec<u8>, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(source.as_bytes())
        .map_err(|err| format!("failed to compress .fb.gz payload: {err}"))?;
    encoder
        .finish()
        .map_err(|err| format!("failed to finish .fb.gz payload: {err}"))
}

pub(super) fn decode_source(bytes: &[u8]) -> Result<String, String> {
    let mut decoder = GzDecoder::new(bytes);
    let mut decoded = String::new();
    decoder
        .read_to_string(&mut decoded)
        .map_err(|err| format!("failed to decompress .fb.gz payload: {err}"))?;
    Ok(decoded)
}

pub(super) fn read_source_from_path(path: &str) -> Result<String, String> {
    let bytes = fs::read(path)
        .map_err(|err| format!("failed to read .fb.gz pattern file `{path}`: {err}"))?;
    decode_source(&bytes)
}

pub(super) fn write_source_to_path(path: &Path, source: &str) -> Result<(), String> {
    let path_str = path.to_string_lossy();
    if !is_fb_gz_path(&path_str) {
        return Err(format!(
            "expected an `.fb.gz` output path, got `{}`",
            path.display()
        ));
    }

    let bytes = encode_source(source)?;
    fs::write(path, bytes).map_err(|err| format!("failed to write {}: {err}", path.display()))
}
