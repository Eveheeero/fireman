//! Basic data-dependence graph construction for flattened IR statements.
//!
//! Tracks conservative register-level def-use edges between statements so later
//! naming, slicing, and structuring passes can reuse a pipeline-owned artifact.

use crate::{
    core::Block,
    ir::{
        Register, VirtualMachine,
        data::{IrData, IrDataContainable},
        statements::{IrStatement, IrStatementSpecial},
        x86_64::X64Range,
    },
    prelude::*,
    utils::Aos,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone, Default)]
pub struct DataDependenceGraph {
    pub edges_from_def: HashMap<usize, HashSet<usize>>,
    pub edges_to_use: HashMap<usize, HashSet<usize>>,
}

impl DataDependenceGraph {
    pub fn edge_count(&self) -> usize {
        self.edges_from_def.values().map(HashSet::len).sum()
    }

    pub fn definition_count(&self) -> usize {
        self.edges_from_def.len()
    }

    fn add_edge(&mut self, def_idx: usize, use_idx: usize) {
        self.edges_from_def
            .entry(def_idx)
            .or_default()
            .insert(use_idx);
        self.edges_to_use
            .entry(use_idx)
            .or_default()
            .insert(def_idx);
    }
}

pub fn analyze_data_dependence(blocks: &[Arc<Block>]) -> DataDependenceGraph {
    let statements = flatten_statements(blocks);
    let mut graph = DataDependenceGraph::default();
    let mut last_definition_by_register: HashMap<Register, usize> = HashMap::new();

    for (statement_idx, statement) in &statements {
        for register in read_registers_of_statement(statement) {
            if let Some(definition_idx) = last_definition_by_register.get(&register) {
                graph.add_edge(*definition_idx, *statement_idx);
            }
        }

        for register in written_registers_of_statement(statement) {
            last_definition_by_register.insert(register, *statement_idx);
        }
    }

    graph
}

pub fn log_data_dependence_analysis(graph: &DataDependenceGraph) {
    if graph.edge_count() > 0 {
        debug!(
            "Data-dependence graph: {} definitions, {} def-use edges",
            graph.definition_count(),
            graph.edge_count()
        );
    }
}

fn flatten_statements(blocks: &[Arc<Block>]) -> Vec<(usize, &IrStatement)> {
    let mut flattened = Vec::new();

    for block in blocks {
        let ir_handle = block.get_ir();
        let Some(ir_block) = ir_handle.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            let Some(statements) = ir.statements else {
                continue;
            };

            for statement in statements {
                flatten_statement(statement, &mut flattened);
            }
        }
    }

    flattened
}

fn flatten_statement<'a>(
    statement: &'a IrStatement,
    flattened: &mut Vec<(usize, &'a IrStatement)>,
) {
    let statement_idx = flattened.len();
    flattened.push((statement_idx, statement));

    if let IrStatement::Condition {
        true_branch,
        false_branch,
        ..
    } = statement
    {
        for branch_statement in true_branch.iter() {
            flatten_statement(branch_statement, flattened);
        }
        for branch_statement in false_branch.iter() {
            flatten_statement(branch_statement, flattened);
        }
    }
}

fn read_registers_of_statement(statement: &IrStatement) -> HashSet<Register> {
    match statement {
        IrStatement::Assignment { from, .. } => collect_registers_from_data(from),
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            collect_registers_from_data(target)
        }
        IrStatement::Condition { condition, .. } => collect_registers_from_data(condition),
        IrStatement::Special(special) => read_registers_of_special(special),
        _ => HashSet::new(),
    }
}

fn written_registers_of_statement(statement: &IrStatement) -> HashSet<Register> {
    match statement {
        IrStatement::Assignment { to, .. } => {
            let mut writes = HashSet::new();
            if let IrData::Register(register) = to.as_ref() {
                writes.insert(*register);
            }
            writes
        }
        IrStatement::JumpByCall { .. } => HashSet::from([
            <VirtualMachine as X64Range>::rax(),
            <VirtualMachine as X64Range>::eax(),
        ]),
        _ => HashSet::new(),
    }
}

fn read_registers_of_special(statement: &IrStatementSpecial) -> HashSet<Register> {
    match statement {
        IrStatementSpecial::TypeSpecified { location, .. } => collect_registers_from_data(location),
        IrStatementSpecial::CalcFlagsAutomatically {
            operation, flags, ..
        } => {
            let mut reads = collect_registers_from_data(operation);
            for flag in flags {
                reads.extend(collect_registers_from_data(flag));
            }
            reads
        }
        IrStatementSpecial::Assertion { condition } => collect_registers_from_data(condition),
    }
}

fn collect_registers_from_data(data: &Aos<IrData>) -> HashSet<Register> {
    let mut registers = HashSet::new();

    match data.as_ref() {
        IrData::Register(register) => {
            registers.insert(*register);
        }
        IrData::Dereference(inner) => {
            registers.extend(collect_registers_from_data(inner));
        }
        IrData::Operation(_) => {
            let mut related = Vec::new();
            data.get_related_ir_data(&mut related);
            for related_data in related {
                if let IrData::Register(register) = related_data.as_ref() {
                    registers.insert(*register);
                }
            }
        }
        _ => {}
    }

    registers
}
