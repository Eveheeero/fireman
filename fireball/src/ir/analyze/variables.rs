use crate::{
    ir::{
        analyze::DataType,
        data::{DataAccess, DataAccessType, IrData},
        statements::{IrStatement, IrStatementSpecial},
        IrBlock,
    },
    utils::Aos,
};
pub use private::IrVariable;
use std::collections::{HashMap, HashSet};

mod private {
    use super::*;
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct IrVariable {
        /// Index of Ir in IrBlock, None if passed last IrBlock
        pub live_in: Option<usize>,
        /// Index of Ir in IrBlock
        pub shown_in: Vec<usize>,
        /// Index of Ir in IrBlock
        pub live_out: Option<usize>,
        /// Index of Ir in IrBlock
        accesses: Vec<Option<Vec<DataAccess>>>,
        pub data_type: DataType,
    }
    impl IrVariable {
        #[inline]
        pub fn new(live_in_ir_index: Option<usize>, data_type: DataType) -> Self {
            Self {
                live_in: live_in_ir_index,
                shown_in: Vec::new(),
                live_out: None,
                accesses: Vec::new(),
                data_type,
            }
        }
        #[inline]
        pub fn get_data_accesses(&self, ir_index: usize) -> &[DataAccess] {
            self.accesses
                .get(ir_index)
                .unwrap_or(&None)
                .as_ref()
                .map(Vec::as_slice)
                .unwrap_or(&[])
        }
        #[inline]
        pub fn add_data_access(&mut self, ir_index: usize, access: DataAccess) {
            if self.accesses.len() <= ir_index {
                self.accesses.resize_with(ir_index + 1, || None);
            }
            if self.accesses[ir_index].is_none() {
                self.accesses[ir_index] = Some(Vec::new());
            }
            self.accesses[ir_index].as_mut().unwrap().push(access);
        }
        #[inline]
        pub fn get_all_data_accesses(&self) -> Vec<(usize, &[DataAccess])> {
            self.accesses
                .iter()
                .enumerate()
                .filter_map(|(ir_index, access)| {
                    access.as_ref().map(|access| (ir_index, access.as_slice()))
                })
                .collect()
        }
    }
    impl std::fmt::Debug for IrVariable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IrVariable")
                .field("live_in", &self.live_in)
                .field("shown_in", &self.shown_in)
                .field("live_out", &self.live_out)
                .field("accesses", &self.get_all_data_accesses())
                .field("data_type", &self.data_type)
                .finish()
        }
    }
}

fn collect_written_locations_recursive(
    stmt: &IrStatement,
    locations_written: &mut HashSet<Aos<IrData>>,
) {
    match stmt {
        IrStatement::Assignment { to, .. } => {
            locations_written.insert(to.clone());
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        }
        | IrStatement::Special(IrStatementSpecial::ArchitectureByteSizeCondition {
            true_branch,
            false_branch,
            ..
        }) => {
            for s in true_branch.iter().chain(false_branch.iter()) {
                collect_written_locations_recursive(s, locations_written);
            }
        }
        _ => {}
    }
}

fn update_location_mapping_recursive(
    stmt: &IrStatement,
    location_to_variable_ids: &mut HashMap<Aos<IrData>, HashSet<usize>>,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            if let Some(src_ids) = location_to_variable_ids.get(from).cloned() {
                location_to_variable_ids.insert(to.clone(), src_ids);
            } else {
                location_to_variable_ids.remove(to);
            }
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        }
        | IrStatement::Special(IrStatementSpecial::ArchitectureByteSizeCondition {
            true_branch,
            false_branch,
            ..
        }) => {
            let mut true_map = location_to_variable_ids.clone();
            for s in true_branch.iter() {
                update_location_mapping_recursive(s, &mut true_map);
            }
            let mut false_map = location_to_variable_ids.clone();
            for s in false_branch.iter() {
                update_location_mapping_recursive(s, &mut false_map);
            }

            let mut merged_map: HashMap<Aos<IrData>, HashSet<usize>> = HashMap::new();
            for (loc, ids) in true_map.into_iter().chain(false_map.into_iter()) {
                merged_map.entry(loc).or_default().extend(ids);
            }

            *location_to_variable_ids = merged_map;
        }
        _ => {}
    }
}

pub fn analyze_variables(ir_block: &IrBlock) -> Result<HashSet<IrVariable>, &'static str> {
    let mut variables: Vec<IrVariable> = Vec::new();
    let mut location_to_variable_ids: HashMap<Aos<IrData>, HashSet<usize>> = HashMap::new();
    let irs = &ir_block.ir;
    let known_datatypes_per_ir = ir_block
        .known_datatypes_per_ir
        .as_ref()
        .ok_or("Datatypes Not Analyzed")?;
    let data_access_per_ir = ir_block
        .data_access_per_ir
        .as_ref()
        .ok_or("Data Access Not Analyzed")?;

    for (ir_index, ir) in irs.iter().enumerate() {
        if ir.statements.is_right() {
            continue;
        }
        let statements = ir.statements.as_ref().unwrap_left();
        let known_datatypes_at_ir = &known_datatypes_per_ir[ir_index];
        let data_access_at_ir = &data_access_per_ir[ir_index];

        // --- Step 1: Identify all locations written within this IR (including nested statements) ---
        let mut locations_written_this_ir: HashSet<Aos<IrData>> = HashSet::new();
        for da in data_access_at_ir.iter() {
            if *da.access_type() == DataAccessType::Write {
                locations_written_this_ir.insert(da.location().clone());
            }
        }
        for stmt in statements.iter() {
            collect_written_locations_recursive(stmt, &mut locations_written_this_ir);
        }

        // --- Step 2: Tentatively kill variables whose locations are overwritten ---
        for loc in &locations_written_this_ir {
            if let Some(old_ids) = location_to_variable_ids.remove(loc) {
                for id in old_ids {
                    if variables[id].live_out.is_none() {
                        variables[id].live_out = Some(ir_index);
                    }
                }
            }
        }

        // --- Step 3: Process Data Accesses (Reads and Writes) ---
        for da in data_access_at_ir.iter() {
            let loc = da.location().clone();
            let access_type = da.access_type();
            let ids = location_to_variable_ids.entry(loc.clone()).or_default();

            if ids.is_empty() {
                let new_id = variables.len();
                let live_in = match access_type {
                    DataAccessType::Write => Some(ir_index),
                    DataAccessType::Read => None, // Live-in from block start (simplified assumption)
                };

                let data_type = known_datatypes_at_ir
                    .iter()
                    .filter(|x| x.location.as_ref() == loc.as_ref())
                    .map(|x| x.data_type)
                    .find(|x| x != &DataType::Unknown)
                    .unwrap_or(DataType::Unknown);

                let mut new_var = IrVariable::new(live_in, data_type);
                if !new_var.shown_in.contains(&ir_index) {
                    new_var.shown_in.push(ir_index);
                }

                variables.push(new_var);
                ids.insert(new_id);
            }

            let current_ids_for_loc = &ids;
            for &var_id in current_ids_for_loc.iter() {
                if !variables[var_id].shown_in.contains(&ir_index) {
                    variables[var_id].shown_in.push(ir_index);
                }
                variables[var_id].add_data_access(ir_index, da.clone());

                if *access_type == DataAccessType::Read
                    && variables[var_id].live_out == Some(ir_index)
                {
                    variables[var_id].live_out = None;
                }
            }
        }

        // --- Step 4: Update location mapping based on assignments (recursively) ---
        for stmt in statements.iter() {
            update_location_mapping_recursive(stmt, &mut location_to_variable_ids);
        }
        location_to_variable_ids.retain(|_, ids| !ids.is_empty());
    }

    Ok(variables.into_iter().collect())
}
