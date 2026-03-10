use bitcode::{Decode, Encode};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

const FBZ_MAGIC: &str = "fbz";
const FBZ_VERSION: u32 = 1;

#[derive(Debug, Encode, Decode)]
struct FbzPayload {
    magic: String,
    version: u32,
    source: String,
}

pub(super) fn is_fbz_path(path: &str) -> bool {
    path.trim().ends_with(".fbz")
}

pub(super) fn encode_source(source: &str) -> Result<Vec<u8>, String> {
    let payload = FbzPayload {
        magic: FBZ_MAGIC.to_string(),
        version: FBZ_VERSION,
        source: source.to_string(),
    };
    let encoded = bitcode::encode(&payload);
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(&encoded)
        .map_err(|err| format!("failed to compress .fbz payload: {err}"))?;
    encoder
        .finish()
        .map_err(|err| format!("failed to finish .fbz payload: {err}"))
}

pub(super) fn decode_source(bytes: &[u8]) -> Result<String, String> {
    let mut decoder = GzDecoder::new(bytes);
    let mut decoded = Vec::new();
    decoder
        .read_to_end(&mut decoded)
        .map_err(|err| format!("failed to decompress .fbz payload: {err}"))?;
    let payload = bitcode::decode::<FbzPayload>(&decoded)
        .map_err(|err| format!("failed to decode .fbz payload: {err}"))?;
    if payload.magic != FBZ_MAGIC {
        return Err(format!("invalid .fbz magic `{}`", payload.magic));
    }
    if payload.version != FBZ_VERSION {
        return Err(format!(
            "unsupported .fbz version `{}` (expected `{FBZ_VERSION}`)",
            payload.version
        ));
    }
    Ok(payload.source)
}

pub(super) fn read_source_from_path(path: &str) -> Result<String, String> {
    if is_fbz_path(path) {
        let bytes = fs::read(path)
            .map_err(|err| format!("failed to read .fbz pattern file `{path}`: {err}"))?;
        decode_source(&bytes)
    } else {
        fs::read_to_string(path)
            .map_err(|err| format!("failed to read pattern file `{path}`: {err}"))
    }
}

pub(super) fn write_source_to_path(path: &Path, source: &str) -> Result<(), String> {
    let path_str = path.to_string_lossy();
    if !is_fbz_path(&path_str) {
        return Err(format!(
            "expected an `.fbz` output path, got `{}`",
            path.display()
        ));
    }
    let bytes = encode_source(source)?;
    fs::write(path, bytes).map_err(|err| format!("failed to write {}: {err}", path.display()))
}
