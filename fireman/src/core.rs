use crate::model::{
    AppliedEditResult, Assembly, AstLine, DecompileRequest, DecompileResult, EditPosition,
    EditRequest, EditorLayer, EditorTarget, Ir as DisplayIr, KnownSectionData, PatchOperation,
    build_optimization_config,
};
use fireball::{
    Fireball,
    abstract_syntax_tree::{
        AstOptimizationConfig,
        pattern_matching::{
            parse_editable_asm_to_ir_statements, parse_editable_ast_statement,
            parse_editable_ir_statement,
        },
    },
    core::{Address, Block, FireRaw},
    ir::{Ir as SemanticIr, IrBlock, statements::IrStatement},
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct FirebatCore {
    path: Option<String>,
    fireball: Option<Fireball>,
    session: Option<EditableSession>,
    patch_operations: Vec<PatchOperation>,
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

enum SessionSelection {
    Assembly(AssemblyRowRef),
    Ir(IrRowRef),
    Ast(usize),
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
                ast_sync_message: None,
            },
        };
        session.refresh_rendered()?;
        Ok(session)
    }

    fn rendered(&self) -> &DecompileResult {
        &self.rendered
    }

    fn patch_target(&self, request: &EditRequest) -> Result<String, String> {
        match request.layer {
            EditorLayer::Assembly => {
                let assembly = self
                    .rendered
                    .assembly
                    .get(request.row)
                    .ok_or_else(|| format!("Assembly row {} is out of range", request.row))?;
                Ok(format!(
                    "assembly[row={},index={},section=0x{:X}]",
                    request.row, assembly.index, assembly.parents_start_address
                ))
            }
            EditorLayer::Ir => {
                let ir = self
                    .rendered
                    .ir
                    .get(request.row)
                    .ok_or_else(|| format!("IR row {} is out of range", request.row))?;
                Ok(format!(
                    "ir[row={},parent_assembly_index={}]",
                    request.row, ir.parents_assembly_index
                ))
            }
            EditorLayer::Ast => {
                let ast = self
                    .rendered
                    .ast
                    .get(request.row)
                    .ok_or_else(|| format!("AST row {} is out of range", request.row))?;
                Ok(format!("ast[row={}]", ast.row))
            }
        }
    }

    fn assembly_row(&self, row: usize) -> Result<AssemblyRowRef, String> {
        self.assembly_rows
            .get(row)
            .copied()
            .ok_or_else(|| format!("Assembly row {} is out of range", row))
    }

    fn ir_row(&self, row: usize) -> Result<IrRowRef, String> {
        self.ir_rows
            .get(row)
            .copied()
            .ok_or_else(|| format!("IR row {} is out of range", row))
    }

    fn refresh_rendered(&mut self) -> Result<(), String> {
        self.assembly_rows.clear();
        self.ir_rows.clear();

        let mut assembly = Vec::new();
        let mut irs = Vec::new();
        let mut assembly_index = 0;

        for (block_index, target_block) in self.target_blocks.iter().enumerate() {
            let start_address = target_block.get_start_address().get_virtual_address();
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
                assembly_index += 1;
                self.assembly_rows.push(assembly_ref);
                assembly.push(Assembly {
                    index: assembly_index,
                    parents_start_address: start_address,
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
                        parents_assembly_index: assembly_index,
                        data: statement.to_string(),
                    });
                }
            }
        }

        let (ast, ast_sync_message) = if let Some(lines) = self.manual_ast_lines.clone() {
            (
                lines,
                Some("AST view contains manual statement edits.".to_string()),
            )
        } else {
            (self.generate_ast_lines()?, None)
        };

        self.rendered = DecompileResult {
            assembly,
            ir: irs,
            ast,
            ast_sync_message,
        };
        Ok(())
    }

    fn generate_ast_lines(&self) -> Result<Vec<AstLine>, String> {
        let ast = fireball::ir::analyze::generate_ast(self.target_blocks.iter().cloned())
            .map_err(|error| error.to_string())?
            .optimize(Some(self.optimization_config.clone()))
            .map_err(|error| error.to_string())?
            .print(None);
        Ok(ast_lines_from_text(&ast))
    }

    fn resolve_selection(&self, selection: SessionSelection) -> Result<EditorTarget, String> {
        match selection {
            SessionSelection::Assembly(row_ref) => self
                .assembly_rows
                .iter()
                .position(|candidate| *candidate == row_ref)
                .map(|row| EditorTarget {
                    layer: EditorLayer::Assembly,
                    row,
                })
                .ok_or_else(|| "Edited assembly row is no longer available".to_string()),
            SessionSelection::Ir(row_ref) => self
                .ir_rows
                .iter()
                .position(|candidate| *candidate == row_ref)
                .map(|row| EditorTarget {
                    layer: EditorLayer::Ir,
                    row,
                })
                .ok_or_else(|| "Edited IR row is no longer available".to_string()),
            SessionSelection::Ast(row) => {
                if row < self.rendered.ast.len() {
                    Ok(EditorTarget {
                        layer: EditorLayer::Ast,
                        row,
                    })
                } else {
                    Err("Edited AST row is no longer available".to_string())
                }
            }
        }
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
        self.patch_operations.clear();
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
        self.patch_operations.clear();
        Ok(result)
    }

    pub fn apply_edit(&mut self, request: EditRequest) -> Result<AppliedEditResult, String> {
        let patch_target = self.patch_target(&request)?;
        let selected_target = {
            let session = self
                .session
                .as_mut()
                .ok_or_else(|| "No active decompilation session".to_string())?;
            let selection = match request.layer {
                EditorLayer::Assembly => apply_assembly_edit(session, &request)?,
                EditorLayer::Ir => apply_ir_edit(session, &request)?,
                EditorLayer::Ast => apply_ast_edit(session, &request)?,
            };
            session.refresh_rendered()?;
            session.resolve_selection(selection)?
        };

        self.patch_operations.push(PatchOperation {
            layer: request.layer,
            position: request.position,
            target: patch_target,
            text: normalize_single_line(&request.text)?,
        });

        let result = self
            .session
            .as_ref()
            .ok_or_else(|| "No active decompilation session".to_string())?
            .rendered()
            .clone();
        Ok(AppliedEditResult {
            result,
            selected_target,
        })
    }

    pub fn export_patch(&self) -> Result<String, String> {
        if self.fireball.is_none() {
            return Err("Open a binary before exporting a patch".to_string());
        }

        #[derive(Serialize)]
        struct PatchExport<'a> {
            path: Option<&'a str>,
            operations: &'a [PatchOperation],
        }

        serde_json::to_string_pretty(&PatchExport {
            path: self.path.as_deref(),
            operations: &self.patch_operations,
        })
        .map_err(|error| error.to_string())
    }

    fn patch_target(&self, request: &EditRequest) -> Result<String, String> {
        self.session
            .as_ref()
            .ok_or_else(|| "No active decompilation session".to_string())?
            .patch_target(request)
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

fn apply_assembly_edit(
    session: &mut EditableSession,
    request: &EditRequest,
) -> Result<SessionSelection, String> {
    if request.position != EditPosition::Replace {
        return Err("Assembly edits currently support replace only".to_string());
    }

    let new_source = normalize_single_line(&request.text)?;
    let new_statements = parse_editable_asm_to_ir_statements(&new_source)
        .map_err(|error| format!("Assembly parse failed: {error}"))?;
    if new_statements.is_empty() {
        return Err("Assembly edit produced no IR statements".to_string());
    }

    let row_ref = session.assembly_row(request.row)?;
    let current_display = session
        .rendered()
        .assembly
        .get(request.row)
        .map(|row| row.data.clone())
        .ok_or_else(|| format!("Assembly row {} is out of range", request.row))?;

    replace_instruction_statements(
        session,
        row_ref.block_index,
        row_ref.instruction_index,
        new_statements,
    )?;
    session.assembly_overrides.insert(
        row_ref,
        with_existing_address_prefix(&current_display, &new_source),
    );
    session.manual_ast_lines = None;
    Ok(SessionSelection::Assembly(row_ref))
}

fn apply_ir_edit(
    session: &mut EditableSession,
    request: &EditRequest,
) -> Result<SessionSelection, String> {
    let new_source = normalize_single_line(&request.text)?;
    let statement = parse_editable_ir_statement(&new_source)
        .map_err(|error| format!("IR parse failed: {error}"))?;
    let row_ref = session.ir_row(request.row)?;

    let selected_statement_index = mutate_block_ir(session, row_ref.block_index, |ir_entries| {
        let ir_entry = ir_entries
            .get_mut(row_ref.instruction_index)
            .ok_or_else(|| "IR instruction index is out of range".to_string())?;
        let mut statements = ir_entry
            .statements
            .as_ref()
            .map(|statements| statements.to_vec())
            .ok_or_else(|| "IR row has no statements to edit".to_string())?;

        let selected_statement_index = match request.position {
            EditPosition::Replace => {
                let target = statements.get_mut(row_ref.statement_index).ok_or_else(|| {
                    format!("IR statement {} is out of range", row_ref.statement_index)
                })?;
                *target = statement;
                row_ref.statement_index
            }
            EditPosition::Before => {
                let row = row_ref.statement_index.min(statements.len());
                statements.insert(row, statement);
                row
            }
            EditPosition::After => {
                let row = row_ref
                    .statement_index
                    .saturating_add(1)
                    .min(statements.len());
                statements.insert(row, statement);
                row
            }
        };

        ir_entry.statements = Some(leak_ir_statements(statements));
        Ok(selected_statement_index)
    })?;

    session.manual_ast_lines = None;
    Ok(SessionSelection::Ir(IrRowRef {
        statement_index: selected_statement_index,
        ..row_ref
    }))
}

fn apply_ast_edit(
    session: &mut EditableSession,
    request: &EditRequest,
) -> Result<SessionSelection, String> {
    let new_source = normalize_single_line(&request.text)?;
    parse_editable_ast_statement(&new_source)
        .map_err(|error| format!("AST parse failed: {error}"))?;

    let mut lines = session
        .manual_ast_lines
        .clone()
        .unwrap_or_else(|| session.rendered().ast.clone());
    let selected_row = match request.position {
        EditPosition::Replace => {
            let row = lines
                .get_mut(request.row)
                .ok_or_else(|| format!("AST row {} is out of range", request.row))?;
            row.data = new_source;
            request.row
        }
        EditPosition::Before => {
            let row = request.row.min(lines.len());
            lines.insert(
                row,
                AstLine {
                    row,
                    data: new_source,
                },
            );
            row
        }
        EditPosition::After => {
            let row = request.row.saturating_add(1).min(lines.len());
            lines.insert(
                row,
                AstLine {
                    row,
                    data: new_source,
                },
            );
            row
        }
    };

    reindex_ast_lines(&mut lines);
    session.manual_ast_lines = Some(lines);
    Ok(SessionSelection::Ast(selected_row))
}

fn replace_instruction_statements(
    session: &EditableSession,
    block_index: usize,
    instruction_index: usize,
    statements: Vec<IrStatement>,
) -> Result<(), String> {
    mutate_block_ir(session, block_index, move |ir_entries| {
        let ir_entry = ir_entries
            .get_mut(instruction_index)
            .ok_or_else(|| "Assembly instruction index is out of range".to_string())?;
        ir_entry.statements = Some(leak_ir_statements(statements));
        Ok(())
    })
}

fn mutate_block_ir<T>(
    session: &EditableSession,
    block_index: usize,
    update: impl FnOnce(&mut Vec<SemanticIr>) -> Result<T, String>,
) -> Result<T, String> {
    let block = session
        .target_blocks
        .get(block_index)
        .ok_or_else(|| format!("Block {} is out of range", block_index))?;
    let ir_guard = block.get_ir();
    let ir = ir_guard
        .as_ref()
        .ok_or_else(|| format!("Block {} has no IR data", block_index))?;
    let mut ir_entries = ir.ir().to_vec();
    let instructions = ir.instructions().clone();
    drop(ir_guard);

    let result = update(&mut ir_entries)?;
    block.set_ir(IrBlock::new(ir_entries, instructions));
    Ok(result)
}

fn leak_ir_statements(statements: Vec<IrStatement>) -> &'static [IrStatement] {
    Box::leak(statements.into_boxed_slice())
}

fn with_existing_address_prefix(original: &str, text: &str) -> String {
    let trimmed = original.trim();
    let Some((head, _tail)) = trimmed.split_once(char::is_whitespace) else {
        return text.to_string();
    };
    let is_hex_address = head.starts_with("0x")
        && head.len() > 2
        && head[2..].chars().all(|ch| ch.is_ascii_hexdigit());
    if is_hex_address {
        format!("{head} {}", text.trim())
    } else {
        text.to_string()
    }
}

fn normalize_single_line(text: &str) -> Result<String, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("Edit text cannot be empty".to_string());
    }
    if trimmed.lines().count() > 1 {
        return Err("Only single-line edits are supported in this editor".to_string());
    }
    Ok(trimmed.to_string())
}

fn ast_lines_from_text(text: &str) -> Vec<AstLine> {
    text.lines()
        .enumerate()
        .map(|(row, line)| AstLine {
            row,
            data: line.to_string(),
        })
        .collect()
}

fn reindex_ast_lines(lines: &mut [AstLine]) {
    for (row, line) in lines.iter_mut().enumerate() {
        line.row = row;
    }
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
