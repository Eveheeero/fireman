use bitcode::{Decode, Encode};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fmt::Write as _,
    fs,
    io::{Read, Write},
    path::Path,
};

const FBZ_MAGIC: &str = "fbz";
const FBZ_VERSION: u32 = 3;

#[derive(Debug, Encode, Decode)]
struct FbzPayload {
    magic: String,
    version: u32,
    functions: Vec<FbzFunction>,
}

/// Exported function data for efficient binary serialization.
/// Field names are NOT stored by bitcode — only values in order.
#[derive(Debug, Clone, Encode, Decode)]
pub struct FbzFunction {
    pub name: String,
    pub default_name: String,
    pub entry_va: u64,
    pub file_offset: Option<u64>,
    pub return_type: Option<String>,
    pub asm_seeds: Vec<String>,
    pub ir_seeds: Vec<String>,
    pub parameters: Vec<FbzParameter>,
    pub variables: Vec<FbzVariable>,
    pub symbols: Vec<FbzSymbol>,
    pub ast_text: Option<String>,
    pub stmt_count: usize,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct FbzParameter {
    pub name: String,
    pub location: String,
    pub value_type: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct FbzVariable {
    pub name: String,
    pub value_type: String,
    pub const_value: Option<String>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct FbzSymbol {
    pub address: u64,
    pub name: String,
}

pub(super) fn is_fbz_path(path: &str) -> bool {
    path.trim().ends_with(".fbz")
}

/// Encode structured function data into .fbz bytes (bitcode + gzip).
pub fn encode_functions(functions: Vec<FbzFunction>) -> Result<Vec<u8>, String> {
    let payload = FbzPayload {
        magic: FBZ_MAGIC.to_string(),
        version: FBZ_VERSION,
        functions,
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

/// Legacy: encode source text by wrapping it as a single function with ast_text.
/// Kept for backward compatibility with tests that encode raw source.
pub(super) fn encode_source(source: &str) -> Result<Vec<u8>, String> {
    // For source-text encoding, store as a single pseudo-function with the source as ast_text.
    // This preserves the existing API but uses the v3 payload structure.
    let functions = vec![FbzFunction {
        name: String::new(),
        default_name: String::new(),
        entry_va: 0,
        file_offset: None,
        return_type: None,
        asm_seeds: Vec::new(),
        ir_seeds: Vec::new(),
        parameters: Vec::new(),
        variables: Vec::new(),
        symbols: Vec::new(),
        ast_text: Some(source.to_string()),
        stmt_count: 0,
    }];
    encode_functions(functions)
}

/// Decode .fbz bytes → reconstructed .fb source text.
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

    // Check if this is a legacy source-text wrapper (single function with only ast_text)
    if payload.functions.len() == 1 {
        let f = &payload.functions[0];
        if f.name.is_empty()
            && f.asm_seeds.is_empty()
            && f.ir_seeds.is_empty()
            && f.parameters.is_empty()
        {
            if let Some(source) = &f.ast_text {
                return Ok(source.clone());
            }
        }
    }

    // Reconstruct .fb source from structured data
    reconstruct_source(&payload.functions)
}

/// Reconstruct .fb DSL source text from structured function data.
fn reconstruct_source(functions: &[FbzFunction]) -> Result<String, String> {
    let mut output = String::new();
    for (i, func) in functions.iter().enumerate() {
        if i > 0 {
            output.push_str("\n\n");
        }
        reconstruct_function_rules(&mut output, func)?;
    }
    Ok(output)
}

fn reconstruct_function_rules(out: &mut String, func: &FbzFunction) -> Result<(), String> {
    let entry_va_str = format!("0x{:x}", func.entry_va);
    let entry_offset_str = func.file_offset.map(|o| format!("0x{o:x}"));
    let mut blocks: Vec<String> = Vec::new();

    // Phase 1: beforeIrAnalyzation (asm fingerprint)
    if !func.asm_seeds.is_empty() {
        let mut rule = String::new();
        let _ = writeln!(rule, "if:");
        let _ = writeln!(rule, "  at beforeIrAnalyzation");
        for asm in &func.asm_seeds {
            let _ = writeln!(rule, "  asm `{}`", asm.replace('`', "\\`"));
        }
        let _ = writeln!(rule, "do:");
        let payload = build_info_json(
            "asm",
            &func.name,
            &func.default_name,
            &entry_va_str,
            &entry_offset_str,
            None,
            Some(&func.parameters),
            None,
            Some(&func.symbols),
            None,
        );
        let _ = writeln!(rule, "  info({payload})");
        blocks.push(rule.trim_end().to_string());
    }

    // Phase 2: afterIrAnalyzation (IR fingerprint)
    if !func.ir_seeds.is_empty() {
        let mut rule = String::new();
        let _ = writeln!(rule, "if:");
        let _ = writeln!(rule, "  at afterIrAnalyzation");
        for ir in &func.ir_seeds {
            let _ = writeln!(rule, "  ir `{}`", ir.replace('`', "\\`"));
        }
        let _ = writeln!(rule, "do:");
        let payload = build_info_json(
            "ir",
            &func.name,
            &func.default_name,
            &entry_va_str,
            &entry_offset_str,
            None,
            Some(&func.parameters),
            None,
            None,
            None,
        );
        let _ = writeln!(rule, "  info({payload})");
        blocks.push(rule.trim_end().to_string());
    }

    // Phase 3: afterOptimization (full metadata)
    {
        let mut rule = String::new();
        let _ = writeln!(rule, "if:");
        let _ = writeln!(rule, "  at afterOptimization");
        for asm in &func.asm_seeds {
            let _ = writeln!(rule, "  asm `{}`", asm.replace('`', "\\`"));
        }
        for ir in &func.ir_seeds {
            let _ = writeln!(rule, "  ir `{}`", ir.replace('`', "\\`"));
        }
        if func.stmt_count > 0 {
            let min_stmts = (func.stmt_count as f64 * 0.8).floor() as usize;
            let max_stmts = (func.stmt_count as f64 * 1.2).ceil() as usize;
            let _ = writeln!(
                rule,
                "  script `stmt_count >= {min_stmts} && stmt_count <= {max_stmts}`"
            );
        }
        let _ = writeln!(rule, "do:");
        let payload = build_info_json(
            "ast",
            &func.name,
            &func.default_name,
            &entry_va_str,
            &entry_offset_str,
            func.return_type.as_deref(),
            Some(&func.parameters),
            Some(&func.variables),
            Some(&func.symbols),
            func.ast_text.as_deref(),
        );
        let _ = writeln!(rule, "  info({payload})");
        blocks.push(rule.trim_end().to_string());
    }

    out.push_str(&blocks.join("\n\n"));
    Ok(())
}

/// Build a JSON string matching the PhasePayload format from dryice.
fn build_info_json(
    phase: &str,
    function_name: &str,
    function_default_name: &str,
    entry_va: &str,
    entry_file_offset: &Option<String>,
    return_type: Option<&str>,
    parameters: Option<&[FbzParameter]>,
    local_variables: Option<&[FbzVariable]>,
    symbols: Option<&[FbzSymbol]>,
    ast_text: Option<&str>,
) -> String {
    // Build JSON manually to match serde_json output format exactly
    let mut json = String::from("{");
    json.push_str(&format!("\"phase\":\"{phase}\""));
    json.push_str(&format!(
        ",\"function_name\":\"{}\"",
        escape_json(function_name)
    ));
    json.push_str(&format!(
        ",\"function_default_name\":\"{}\"",
        escape_json(function_default_name)
    ));
    json.push_str(&format!(",\"entry_virtual_address\":\"{entry_va}\""));
    if let Some(offset) = entry_file_offset {
        json.push_str(&format!(",\"entry_file_offset\":\"{offset}\""));
    }
    if let Some(rt) = return_type {
        json.push_str(&format!(",\"return_type\":\"{}\"", escape_json(rt)));
    }
    if let Some(params) = parameters {
        json.push_str(",\"parameters\":[");
        for (i, p) in params.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                "{{\"name\":\"{}\",\"location\":\"{}\",\"value_type\":\"{}\"}}",
                escape_json(&p.name),
                escape_json(&p.location),
                escape_json(&p.value_type)
            ));
        }
        json.push(']');
    }
    if let Some(vars) = local_variables {
        json.push_str(",\"local_variables\":[");
        for (i, v) in vars.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                "{{\"name\":\"{}\",\"value_type\":\"{}\"",
                escape_json(&v.name),
                escape_json(&v.value_type)
            ));
            if let Some(cv) = &v.const_value {
                json.push_str(&format!(",\"const_value\":\"{}\"", escape_json(cv)));
            }
            json.push_str("}}");
        }
        json.push(']');
    }
    if let Some(syms) = symbols {
        json.push_str(",\"symbols\":[");
        for (i, s) in syms.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                "{{\"address\":\"0x{:x}\",\"name\":\"{}\"}}",
                s.address,
                escape_json(&s.name)
            ));
        }
        json.push(']');
    }
    if let Some(ast) = ast_text {
        json.push_str(&format!(",\"ast_text\":\"{}\"", escape_json(ast)));
    }
    json.push('}');
    json
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
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
