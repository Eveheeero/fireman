use clap::{Parser, ValueEnum};
use fireball::{
    Fireball,
    abstract_syntax_tree::{
        AstFunction, AstOptimizationConfig, AstParameterLocation, AstPrintConfig, PrintWithConfig,
        pattern_matching::{
            AstPattern, FbzFunction, FbzParameter, FbzSymbol, FbzVariable, encode_fbz_functions,
        },
    },
    core::FireRaw,
    ir::analyze::generate_ast_with_pre_defined_symbols,
    utils::test_log_subscriber_with_file,
};
use serde::Serialize;
use std::{
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    // Spawn with a larger stack to handle deeply nested AST trees from large binaries.
    const STACK_SIZE: usize = 64 * 1024 * 1024; // 64 MiB
    let result = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked");

    if let Err(err) = result {
        eprintln!("glacier: {err}");
        std::process::exit(1);
    }
}

#[derive(Debug, Parser)]
#[command(name = "glacier")]
#[command(
    about = "Export recovered Fireball analysis as compressed patterns (.fb.gz by default, .fbz or plain .fb optional)"
)]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: PathBuf,
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = OutputFormat::FbGz)]
    format: OutputFormat,
    #[arg(long = "log")]
    log: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    FbGz,
    Fbz,
    Fb,
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::FbGz => "fb.gz",
            Self::Fbz => "fbz",
            Self::Fb => "fb",
        }
    }
}

#[derive(Debug, Clone)]
struct ExportRequest {
    input_path: PathBuf,
    output_path: PathBuf,
    output_format: OutputFormat,
    log_file_path: Option<PathBuf>,
    overwrite: bool,
}

#[derive(Debug, Clone)]
struct ExportSummary {
    output_path: PathBuf,
    log_file_path: Option<PathBuf>,
    function_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedParameter {
    name: String,
    location: String,
    value_type: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedVariable {
    name: String,
    value_type: String,
    const_value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedSymbol {
    address: String,
    name: String,
}

/// Phase-specific metadata emitted in each `do:` block.
#[derive(Debug, Serialize)]
struct PhasePayload {
    phase: String,
    function_name: String,
    function_default_name: String,
    entry_virtual_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_file_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<ExportedParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_variables: Option<Vec<ExportedVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<ExportedSymbol>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ast_text: Option<String>,
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let default_output_path = default_output_path(&cli.input, cli.format);
    let output_path = cli.output.unwrap_or(default_output_path);
    validate_output_path_format(&output_path, cli.format)?;
    let request = ExportRequest {
        input_path: cli.input,
        output_path,
        output_format: cli.format,
        log_file_path: cli.log,
        overwrite: true,
    };

    let summary = export_fb(request)?;
    if let Some(log_file_path) = summary.log_file_path {
        println!(
            "wrote {} functions to {} (log: {})",
            summary.function_count,
            summary.output_path.display(),
            log_file_path.display()
        );
    } else {
        println!(
            "wrote {} functions to {}",
            summary.function_count,
            summary.output_path.display()
        );
    }
    Ok(())
}

fn export_fb(request: ExportRequest) -> Result<ExportSummary, String> {
    if request.output_path.exists() && !request.overwrite {
        return Err(format!(
            "output already exists: {} (pass --overwrite to replace it)",
            request.output_path.display()
        ));
    }

    let log_path = request
        .log_file_path
        .as_ref()
        .map(|path| absolute_path(path))
        .transpose()?;
    if let Some(log_path) = &log_path {
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                format!("failed to create log directory {}: {err}", parent.display())
            })?;
        }

        let log_path_string = path_string(log_path)?;
        let subscriber = test_log_subscriber_with_file(&log_path_string);
        let dispatch = tracing::Dispatch::new(subscriber);
        tracing::dispatcher::with_default(&dispatch, || {
            export_fb_inner(request, Some(log_path.clone()))
        })
    } else {
        export_fb_inner(request, None)
    }
}

fn export_fb_inner(
    request: ExportRequest,
    log_path: Option<PathBuf>,
) -> Result<ExportSummary, String> {
    let output_path = absolute_path(&request.output_path)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create output directory {}: {err}",
                parent.display()
            )
        })?;
    }

    let input_path_string = path_string(&request.input_path)?;
    let fireball = Fireball::from_path(&input_path_string).map_err(|err| err.to_string())?;
    let blocks = fireball.analyze_all().map_err(|err| err.to_string())?;
    let mut ast = generate_ast_with_pre_defined_symbols(blocks, fireball.get_defined())
        .map_err(|err| err.to_string())?;
    ast = ast.optimize(None).map_err(|err| err.to_string())?;

    let source_path = absolute_path(&request.input_path)?;
    let source_path_string = source_path.display().to_string();
    let functions = ast
        .functions
        .read()
        .map_err(|_| "failed to read AST functions".to_string())?;

    let mut ordered_functions = functions
        .iter()
        .map(|(_, version_map)| version_map.get_last_version())
        .collect::<Vec<_>>();

    ordered_functions.sort_by_key(|function| {
        function
            .ir
            .get_ir()
            .first()
            .map(|ir| ir.address.get_virtual_address())
            .unwrap_or_default()
    });

    let encoded = match request.output_format {
        OutputFormat::FbGz => {
            let output = render_all_rules(
                &source_path_string,
                &ast.pre_defined_symbols,
                &ordered_functions,
            )?;
            AstPattern::fb_gz_bytes_from_source(&output)?
        }
        OutputFormat::Fb => {
            let output = render_all_rules(
                &source_path_string,
                &ast.pre_defined_symbols,
                &ordered_functions,
            )?;
            output.into_bytes()
        }
        OutputFormat::Fbz => {
            let functions = build_fbz_functions(&ast.pre_defined_symbols, &ordered_functions)?;
            encode_fbz_functions(functions)?
        }
    };
    fs::write(&output_path, encoded)
        .map_err(|err| format!("failed to write {}: {err}", output_path.display()))?;

    Ok(ExportSummary {
        output_path,
        log_file_path: log_path,
        function_count: ordered_functions.len(),
    })
}

fn render_all_rules(
    source_path: &str,
    symbols: &hashbrown::HashMap<u64, String>,
    functions: &[&AstFunction],
) -> Result<String, String> {
    let rendered = functions
        .iter()
        .map(|f| render_function_rules(source_path, symbols, f))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rendered.join("\n\n"))
}

fn build_fbz_functions(
    symbols: &hashbrown::HashMap<u64, String>,
    functions: &[&AstFunction],
) -> Result<Vec<FbzFunction>, String> {
    functions
        .iter()
        .map(|f| build_fbz_function(symbols, f))
        .collect()
}

fn build_fbz_function(
    symbols: &hashbrown::HashMap<u64, String>,
    function: &AstFunction,
) -> Result<FbzFunction, String> {
    let entry_ir = function
        .ir
        .get_ir()
        .first()
        .ok_or_else(|| format!("function {} has no IR", function.name()))?;
    let entry_va = entry_ir.address.get_virtual_address();
    let file_offset = entry_ir.address.get_file_offset();

    // Assembly seeds (all instructions)
    let asm_seeds = function
        .ir
        .get_instructions()
        .iter()
        .map(|i| i.inner().to_string())
        .collect::<Vec<_>>();

    // IR seeds (all statements)
    let ir_seeds = function
        .ir
        .get_ir()
        .iter()
        .filter_map(|ir| ir.statements)
        .flat_map(|stmts| stmts.iter().map(ToString::to_string))
        .collect::<Vec<_>>();

    // Parameters
    let parameters = function
        .parameters
        .iter()
        .map(|p| FbzParameter {
            name: p
                .name(&function.variables)
                .unwrap_or_else(|_| "unknown".to_string()),
            location: format_parameter_location(&p.location),
            value_type: p
                .read_type(&function.variables)
                .map(|v| v.to_string_with_config(None))
                .unwrap_or_else(|_| "unknown".to_string()),
        })
        .collect::<Vec<_>>();

    // Variables
    let variables_guard = function
        .variables
        .read()
        .map_err(|_| format!("failed to read variables for {}", function.name()))?;
    let mut variables = variables_guard
        .values()
        .map(|v| FbzVariable {
            name: v.name(),
            value_type: v.var_type.to_string_with_config(None),
            const_value: v
                .const_value
                .as_ref()
                .map(|cv| cv.to_string_with_config(None)),
        })
        .collect::<Vec<_>>();
    variables.sort_by(|a, b| a.name.cmp(&b.name));

    // Symbols in function range
    let function_start = entry_va;
    let function_end = function
        .ir
        .get_ir()
        .last()
        .map(|ir| ir.address.get_virtual_address())
        .unwrap_or(entry_va);
    let mut syms = symbols
        .iter()
        .filter(|(addr, _)| **addr >= function_start && **addr <= function_end)
        .map(|(addr, name)| FbzSymbol {
            address: *addr,
            name: name.clone(),
        })
        .collect::<Vec<_>>();
    syms.sort_by_key(|s| s.address);

    // AST text
    let ast_text = render_function_ast(function).ok();

    // Return type
    let return_type = Some(function.return_type.to_string_with_config(None));

    Ok(FbzFunction {
        name: function.name(),
        default_name: function.id.get_default_name(),
        entry_va,
        file_offset,
        return_type,
        asm_seeds,
        ir_seeds,
        parameters,
        variables,
        symbols: syms,
        ast_text,
        stmt_count: function.body.len(),
    })
}

/// Collect shared data for a function, then emit three phase-specific `if:do:` blocks.
fn render_function_rules(
    source_path: &str,
    symbols: &hashbrown::HashMap<u64, String>,
    function: &AstFunction,
) -> Result<String, String> {
    let entry_ir = function
        .ir
        .get_ir()
        .first()
        .ok_or_else(|| format!("function {} has no IR", function.name()))?;
    let entry_va = entry_ir.address.get_virtual_address();
    let entry_file_offset = entry_ir.address.get_file_offset();

    let assembly_seeds = function
        .ir
        .get_instructions()
        .iter()
        .map(|instruction| instruction.inner().to_string())
        .collect::<Vec<_>>();

    let ir_seeds = function
        .ir
        .get_ir()
        .iter()
        .filter_map(|ir| ir.statements)
        .flat_map(|statements| statements.iter().map(ToString::to_string))
        .collect::<Vec<_>>();

    let parameters = function
        .parameters
        .iter()
        .map(|parameter| ExportedParameter {
            name: parameter
                .name(&function.variables)
                .unwrap_or_else(|_| "unknown".to_string()),
            location: format_parameter_location(&parameter.location),
            value_type: parameter
                .read_type(&function.variables)
                .map(|value| value.to_string_with_config(None))
                .unwrap_or_else(|_| "unknown".to_string()),
        })
        .collect::<Vec<_>>();

    let variables = function
        .variables
        .read()
        .map_err(|_| format!("failed to read variables for {}", function.name()))?;
    let mut local_variables = variables
        .values()
        .map(|variable| ExportedVariable {
            name: variable.name(),
            value_type: variable.var_type.to_string_with_config(None),
            const_value: variable
                .const_value
                .as_ref()
                .map(|value| value.to_string_with_config(None)),
        })
        .collect::<Vec<_>>();
    local_variables.sort_by(|left, right| left.name.cmp(&right.name));

    let function_start = entry_va;
    let function_end = function
        .ir
        .get_ir()
        .last()
        .map(|ir| ir.address.get_virtual_address())
        .unwrap_or(entry_va);
    let mut relevant_symbols = symbols
        .iter()
        .filter(|(address, _)| **address >= function_start && **address <= function_end)
        .map(|(address, name): (&u64, &String)| ExportedSymbol {
            address: format!("0x{address:x}"),
            name: name.clone(),
        })
        .collect::<Vec<_>>();
    relevant_symbols.sort_by(|left, right| left.address.cmp(&right.address));

    let entry_va_str = format!("0x{entry_va:x}");
    let entry_offset_str = entry_file_offset.map(|offset| format!("0x{offset:x}"));
    let func_name = function.name();
    let func_default_name = function.id.get_default_name();

    // Phase 1: beforeIrAnalyzation — match on asm seeds
    let asm_rule = render_asm_rule(
        &assembly_seeds,
        &func_name,
        &func_default_name,
        &entry_va_str,
        &entry_offset_str,
        &parameters,
        &relevant_symbols,
    )?;

    // Phase 2: afterIrAnalyzation — match on IR seeds
    let ir_rule = render_ir_rule(
        &ir_seeds,
        &func_name,
        &func_default_name,
        &entry_va_str,
        &entry_offset_str,
        &parameters,
    )?;

    // Phase 3: afterOptimization — full metadata with AST
    let ast_rule = render_ast_rule(
        source_path,
        function,
        &assembly_seeds,
        &ir_seeds,
        &func_name,
        &func_default_name,
        &entry_va_str,
        &entry_offset_str,
        &parameters,
        &local_variables,
        &relevant_symbols,
    )?;

    let mut blocks = Vec::new();
    if let Some(rule) = asm_rule {
        blocks.push(rule);
    }
    if let Some(rule) = ir_rule {
        blocks.push(rule);
    }
    blocks.push(ast_rule);

    Ok(blocks.join("\n\n"))
}

/// Phase 1: `beforeIrAnalyzation` — asm fingerprint sets function name and parameters early.
fn render_asm_rule(
    assembly_seeds: &[String],
    func_name: &str,
    func_default_name: &str,
    entry_va: &str,
    entry_file_offset: &Option<String>,
    parameters: &[ExportedParameter],
    symbols: &[ExportedSymbol],
) -> Result<Option<String>, String> {
    if assembly_seeds.is_empty() {
        return Ok(None);
    }

    let payload = PhasePayload {
        phase: "asm".to_string(),
        function_name: func_name.to_string(),
        function_default_name: func_default_name.to_string(),
        entry_virtual_address: entry_va.to_string(),
        entry_file_offset: entry_file_offset.clone(),
        return_type: None,
        parameters: Some(parameters.to_vec()),
        local_variables: None,
        symbols: Some(symbols.to_vec()),
        ast_text: None,
    };
    let payload_json = serde_json::to_string(&payload).map_err(|err| err.to_string())?;

    let mut rule = String::new();
    let _ = writeln!(rule, "if:");
    let _ = writeln!(rule, "  at beforeIrAnalyzation");
    for asm in assembly_seeds {
        let _ = writeln!(rule, "  asm {}", fb_literal(asm));
    }
    let _ = writeln!(rule, "do:");
    let _ = writeln!(rule, "  info({payload_json})");

    Ok(Some(rule.trim_end().to_string()))
}

/// Phase 2: `afterIrAnalyzation` — IR fingerprint refines function name and parameters.
fn render_ir_rule(
    ir_seeds: &[String],
    func_name: &str,
    func_default_name: &str,
    entry_va: &str,
    entry_file_offset: &Option<String>,
    parameters: &[ExportedParameter],
) -> Result<Option<String>, String> {
    if ir_seeds.is_empty() {
        return Ok(None);
    }

    let payload = PhasePayload {
        phase: "ir".to_string(),
        function_name: func_name.to_string(),
        function_default_name: func_default_name.to_string(),
        entry_virtual_address: entry_va.to_string(),
        entry_file_offset: entry_file_offset.clone(),
        return_type: None,
        parameters: Some(parameters.to_vec()),
        local_variables: None,
        symbols: None,
        ast_text: None,
    };
    let payload_json = serde_json::to_string(&payload).map_err(|err| err.to_string())?;

    let mut rule = String::new();
    let _ = writeln!(rule, "if:");
    let _ = writeln!(rule, "  at afterIrAnalyzation");
    for ir in ir_seeds {
        let _ = writeln!(rule, "  ir {}", fb_literal(ir));
    }
    let _ = writeln!(rule, "do:");
    let _ = writeln!(rule, "  info({payload_json})");

    Ok(Some(rule.trim_end().to_string()))
}

/// Phase 3: `afterOptimization` — full metadata including AST text.
fn render_ast_rule(
    _source_path: &str,
    function: &AstFunction,
    assembly_seeds: &[String],
    ir_seeds: &[String],
    func_name: &str,
    func_default_name: &str,
    entry_va: &str,
    entry_file_offset: &Option<String>,
    parameters: &[ExportedParameter],
    local_variables: &[ExportedVariable],
    symbols: &[ExportedSymbol],
) -> Result<String, String> {
    let ast_text = render_function_ast(function)?;

    let payload = PhasePayload {
        phase: "ast".to_string(),
        function_name: func_name.to_string(),
        function_default_name: func_default_name.to_string(),
        entry_virtual_address: entry_va.to_string(),
        entry_file_offset: entry_file_offset.clone(),
        return_type: Some(function.return_type.to_string_with_config(None)),
        parameters: Some(parameters.to_vec()),
        local_variables: Some(local_variables.to_vec()),
        symbols: Some(symbols.to_vec()),
        ast_text: Some(ast_text),
    };
    let payload_json = serde_json::to_string(&payload).map_err(|err| err.to_string())?;

    let mut rule = String::new();
    let _ = writeln!(rule, "if:");
    let _ = writeln!(rule, "  at afterOptimization");
    // Add IR fingerprint matchers; fall back to asm if no IR seeds available
    if ir_seeds.is_empty() {
        for asm in assembly_seeds {
            let _ = writeln!(rule, "  asm {}", fb_literal(asm));
        }
    }
    for ir in ir_seeds {
        let _ = writeln!(rule, "  ir {}", fb_literal(ir));
    }
    // Add structural script check — stmt_count within ±20% tolerance
    let stmt_count = function.body.len();
    let min_stmts = (stmt_count as f64 * 0.8).floor() as usize;
    let max_stmts = (stmt_count as f64 * 1.2).ceil() as usize;
    let _ = writeln!(
        rule,
        "  script `stmt_count >= {min_stmts} && stmt_count <= {max_stmts}`"
    );
    let _ = writeln!(rule, "do:");
    let _ = writeln!(rule, "  info({payload_json})");

    Ok(rule.trim_end().to_string())
}

fn render_function_ast(function: &AstFunction) -> Result<String, String> {
    let params = function
        .parameters
        .iter()
        .map(|parameter| {
            let name = parameter
                .name(&function.variables)
                .unwrap_or_else(|_| "unknown".to_string());
            let value_type = parameter
                .read_type(&function.variables)
                .map(|value| value.to_string_with_config(None))
                .unwrap_or_else(|_| "unknown".to_string());
            format!("{value_type} {name}")
        })
        .collect::<Vec<_>>()
        .join(", ");

    let mut rendered = String::new();
    let _ = writeln!(
        rendered,
        "{} {}({}) {{",
        function.return_type.to_string_with_config(None),
        function.name(),
        params
    );

    for statement in function.body.iter() {
        for line in statement
            .to_string_with_config(Some(AstPrintConfig::default()))
            .lines()
        {
            let _ = writeln!(rendered, "    {line}");
        }
    }

    let _ = write!(rendered, "}}");
    Ok(rendered)
}

fn format_parameter_location(location: &AstParameterLocation) -> String {
    match location {
        AstParameterLocation::Register(data) => format!("register {}", data.to_string()),
        AstParameterLocation::Stack(offset) => format!("stack {offset:+#x}"),
    }
}

fn fb_literal(value: &str) -> String {
    let escaped = value.replace('`', "\\`");
    format!("`{escaped}`")
}

fn default_output_path(input: &Path, format: OutputFormat) -> PathBuf {
    input.with_extension(format.extension())
}

fn validate_output_path_format(path: &Path, format: OutputFormat) -> Result<(), String> {
    let expected_extension = format.extension();
    let rendered = path.to_string_lossy();
    if rendered.ends_with(expected_extension) {
        return Ok(());
    }

    Err(format!(
        "output path `{}` does not match selected format `.{expected_extension}`",
        path.display()
    ))
}

fn path_string(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("path is not valid UTF-8: {}", path.display()))
}

fn absolute_path(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let cwd = std::env::current_dir().map_err(|err| err.to_string())?;
        Ok(cwd.join(path))
    }
}

fn _validate_generated_pattern_file_for_debug(
    input_path: &Path,
    pattern_path: &Path,
) -> Result<(), String> {
    let input_path_string = path_string(input_path)?;
    let fireball = Fireball::from_path(&input_path_string).map_err(|err| err.to_string())?;
    let blocks = fireball.analyze_all().map_err(|err| err.to_string())?;
    let ast = generate_ast_with_pre_defined_symbols(blocks, fireball.get_defined())
        .map_err(|err| err.to_string())?;
    let pattern_path_string = path_string(pattern_path)?;
    let pattern = AstPattern::from_file(pattern_path_string);
    let result = ast.optimize(Some(
        AstOptimizationConfig::NONE
            .pattern_matching_enabled(true)
            .pattern_matching(vec![pattern])
            .max_pass_iterations(1),
    ));
    result.map(|_| ()).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_output_path_uses_fb_gz_for_default_format() {
        let input = Path::new("sample.exe");
        assert_eq!(
            default_output_path(input, OutputFormat::FbGz),
            PathBuf::from("sample.fb.gz")
        );
        assert_eq!(
            default_output_path(input, OutputFormat::Fbz),
            PathBuf::from("sample.fbz")
        );
    }

    #[test]
    fn validate_output_path_format_rejects_extension_mismatch() {
        assert!(validate_output_path_format(Path::new("out.fb.gz"), OutputFormat::FbGz).is_ok());
        assert!(validate_output_path_format(Path::new("out.fbz"), OutputFormat::Fbz).is_ok());
        assert!(validate_output_path_format(Path::new("out.fbz"), OutputFormat::FbGz).is_err());
        assert!(validate_output_path_format(Path::new("out.fb.gz"), OutputFormat::Fbz).is_err());
    }
}
