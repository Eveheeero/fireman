use crate::model::{
    Assembly, AstLine, DecompileRequest, DecompileResult, Ir as DisplayIr, KnownSectionData,
    OptimizeAstRequest, OptimizeAstResult, build_optimization_config,
};
use fireball::{
    Fireball,
    abstract_syntax_tree::AstOptimizationConfig,
    core::{Address, Block, FireRaw},
};
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct FirebatCore {
    path: Option<String>,
    fireball: Option<Fireball>,
    session: Option<EditableSession>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct AssemblyRowRef {
    block_index: usize,
    instruction_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct IrRowRef {
    block_index: usize,
    instruction_index: usize,
    statement_index: usize,
}

struct EditableSession {
    target_blocks: Vec<Arc<Block>>,
    assembly_rows: Vec<AssemblyRowRef>,
    ir_rows: Vec<IrRowRef>,
    assembly_overrides: HashMap<AssemblyRowRef, String>,
    manual_ast_lines: Option<Vec<AstLine>>,
    optimization_config: AstOptimizationConfig,
    rendered: DecompileResult,
}

impl EditableSession {
    fn from_blocks(
        target_blocks: Vec<Arc<Block>>,
        optimization_config: AstOptimizationConfig,
    ) -> Result<Self, String> {
        let mut session = Self {
            target_blocks,
            assembly_rows: Vec::new(),
            ir_rows: Vec::new(),
            assembly_overrides: HashMap::new(),
            manual_ast_lines: None,
            optimization_config,
            rendered: DecompileResult {
                assembly: Vec::new(),
                ir: Vec::new(),
                ast: Vec::new(),
                ast_object: None,
            },
        };
        session.refresh_rendered()?;
        Ok(session)
    }

    fn rendered(&self) -> &DecompileResult {
        &self.rendered
    }

    fn refresh_rendered(&mut self) -> Result<(), String> {
        self.assembly_rows.clear();
        self.ir_rows.clear();

        let mut assembly = Vec::new();
        let mut irs = Vec::new();
        for (block_index, target_block) in self.target_blocks.iter().enumerate() {
            let ir = target_block.get_ir();
            let Some(ir) = ir.as_ref() else {
                continue;
            };

            for (instruction_index, (instruction, ir_entry)) in
                ir.instructions().iter().zip(ir.ir()).enumerate()
            {
                let assembly_ref = AssemblyRowRef {
                    block_index,
                    instruction_index,
                };
                self.assembly_rows.push(assembly_ref);
                assembly.push(Assembly {
                    data: self
                        .assembly_overrides
                        .get(&assembly_ref)
                        .cloned()
                        .unwrap_or_else(|| instruction.to_string()),
                });

                let Some(statements) = ir_entry.statements.as_ref() else {
                    continue;
                };
                for (statement_index, statement) in statements.iter().enumerate() {
                    self.ir_rows.push(IrRowRef {
                        block_index,
                        instruction_index,
                        statement_index,
                    });
                    irs.push(DisplayIr {
                        data: statement.to_string(),
                    });
                }
            }
        }

        let (ast, ast_object) = if let Some(lines) = self.manual_ast_lines.clone() {
            (lines, None)
        } else {
            let (lines, ast_obj) = self.generate_ast()?;
            (lines, Some(ast_obj))
        };

        self.rendered = DecompileResult {
            assembly,
            ir: irs,
            ast,
            ast_object,
        };
        Ok(())
    }

    fn generate_ast(
        &self,
    ) -> Result<
        (
            Vec<AstLine>,
            std::sync::Arc<fireball::abstract_syntax_tree::Ast>,
        ),
        String,
    > {
        let ast = fireball::ir::analyze::generate_ast(self.target_blocks.iter().cloned())
            .map_err(|error| error.to_string())?
            .optimize(Some(self.optimization_config.clone()))
            .map_err(|error| error.to_string())?;
        let ast_text = ast.print(None);
        let lines = ast_lines_from_text(&ast_text);
        Ok((lines, std::sync::Arc::new(ast)))
    }
}

impl FirebatCore {
    fn fireball(&self) -> Result<&Fireball, String> {
        self.fireball
            .as_ref()
            .ok_or_else(|| "Fireball is None".to_string())
    }

    pub fn open_file(&mut self, path: &str) -> Result<(), String> {
        self.path = Some(path.to_owned());
        let fireball = Fireball::from_path(path).map_err(|error| error.to_string())?;
        self.fireball = Some(fireball);
        self.session = None;
        Ok(())
    }

    pub fn analyze_section(&self, address: &str) -> Result<Vec<KnownSectionData>, String> {
        if address.is_empty() {
            return self.analyze_section_from_entry();
        }
        let parsed_address = parse_address(address)?;
        self.analyze_section_from_address(parsed_address)
    }

    pub fn analyze_all_sections(&self) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let analyzed = fireball.analyze_all().map_err(|error| error.to_string())?;
        Ok(block_to_result(analyzed))
    }

    fn analyze_section_from_address(&self, address: u64) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let result = fireball
            .analyze_from_virtual_address(address)
            .map_err(|error| error.to_string())?;
        Ok(block_to_result([result]))
    }

    fn analyze_section_from_entry(&self) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let result = fireball
            .analyze_from_entry()
            .map_err(|error| error.to_string())?;
        Ok(block_to_result([result]))
    }

    pub fn decompile_sections(
        &mut self,
        request: DecompileRequest,
    ) -> Result<DecompileResult, String> {
        let optimization_config = build_optimization_config(
            &request.settings,
            &request.script_paths,
            request.buffer_script.as_deref(),
        )?;
        let session = self.build_session(&request.start_addresses, optimization_config)?;
        let result = session.rendered().clone();
        self.session = Some(session);
        Ok(result)
    }

    pub fn optimize_ast(&self, request: OptimizeAstRequest) -> Result<OptimizeAstResult, String> {
        let config = build_optimization_config(
            &request.settings,
            &request.script_paths,
            request.buffer_script.as_deref(),
        )?;
        let optimized = if is_config_none(&config) {
            request.ast
        } else {
            let function_ids: Vec<_> = request.ast.function_versions.keys().cloned().collect();
            request
                .ast
                .optimize_functions(&function_ids, Some(config))
                .map_err(|error| error.to_string())?
        };
        let lines = ast_lines_from_text(&optimized.print(None));
        Ok(OptimizeAstResult {
            ast: Arc::new(optimized),
            ast_lines: lines,
        })
    }

    fn build_session(
        &self,
        start_addresses: &[u64],
        optimization_config: AstOptimizationConfig,
    ) -> Result<EditableSession, String> {
        let fireball = self.fireball()?;
        let blocks = fireball.get_blocks();
        let sections = fireball.get_sections();
        let target_blocks = start_addresses
            .iter()
            .map(|&address| Address::from_virtual_address(&sections, address))
            .filter_map(|address| blocks.get_by_start_address(&address))
            .collect::<Vec<_>>();
        EditableSession::from_blocks(target_blocks, optimization_config)
    }
}

fn ast_lines_from_text(text: &str) -> Vec<AstLine> {
    text.lines()
        .enumerate()
        .map(|(_row, line)| AstLine {
            data: line.to_string(),
        })
        .collect()
}

pub fn parse_address(address: &str) -> Result<u64, String> {
    let address = address.trim();
    if let Ok(address) = address.parse::<u64>() {
        return Ok(address);
    }

    let address = if address.starts_with("0x") || address.starts_with("0X") {
        &address[2..]
    } else {
        address
    };

    if let Ok(address) = u64::from_str_radix(address, 16) {
        return Ok(address);
    }

    Err("Invalid Address".to_string())
}

fn block_to_result(blocks: impl IntoIterator<Item = Arc<Block>>) -> Vec<KnownSectionData> {
    let mut result = Vec::new();
    for block in blocks {
        let start_address = block.get_start_address().get_virtual_address();
        let known = KnownSectionData {
            end_address: block.get_block_size().map(|size| start_address + size),
            start_address,
            analyzed: true,
        };
        result.retain(|item: &KnownSectionData| item.start_address != known.start_address);
        result.push(known);

        let reader = block.get_connected_to();
        for relation in reader.iter() {
            let Some(to) = relation.to() else {
                continue;
            };
            let target = to.get_virtual_address();
            if result.iter().any(|item| item.start_address == target) {
                continue;
            }

            result.push(KnownSectionData {
                start_address: target,
                end_address: None,
                analyzed: false,
            });
        }
    }
    result
}

fn is_config_none(config: &AstOptimizationConfig) -> bool {
    !config.ir_analyzation
        && !config.parameter_analyzation
        && !config.call_argument_analyzation
        && !config.constant_folding
        && !config.control_flow_cleanup
        && !config.collapse_unused_varaible
        && !config.dead_store_elimination
        && !config.pattern_matching_enabled
        && !config.loop_analyzation
        && !config.copy_propagation
        && !config.expression_inlining
        && !config.operator_canonicalization
        && !config.magic_division_recovery
        && !config.identity_simplification
        && !config.bit_trick_recognition
        && !config.cast_minimization
        && !config.ternary_recovery
        && !config.boolean_recovery
        && !config.assertion_recovery
        && !config.do_while_recovery
        && !config.clamp_recovery
        && !config.loop_cleanup
        && !config.if_conversion_reversal
        && !config.switch_reconstruction
        && !config.lifetime_scoping
        && !config.signedness_inference
        && !config.name_recovery
        && !config.early_return_normalization
        && !config.anti_debug_ast_suppression
        && !config.logging_suppression
        && !config.static_guard_suppression
        && !config.security_scaffold_suppression
}
