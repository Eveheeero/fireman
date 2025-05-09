use crate::{
    ir::{
        analyze::{DataType, KnownDataType},
        data::{DataAccess, DataAccessType, IrData, IrDataContainable},
        statements::{IrStatement, IrStatementSpecial},
        IrBlock,
    },
    utils::Aos,
};
pub use private::IrVariable;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
    sync::LazyLock,
};

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

static O1: LazyLock<Aos<IrData>> =
    LazyLock::new(|| Aos::new_static(IrData::Operand(NonZeroU8::new(1).unwrap())));
static O2: LazyLock<Aos<IrData>> =
    LazyLock::new(|| Aos::new_static(IrData::Operand(NonZeroU8::new(2).unwrap())));
static O3: LazyLock<Aos<IrData>> =
    LazyLock::new(|| Aos::new_static(IrData::Operand(NonZeroU8::new(3).unwrap())));
static O4: LazyLock<Aos<IrData>> =
    LazyLock::new(|| Aos::new_static(IrData::Operand(NonZeroU8::new(4).unwrap())));

fn collect_written_locations_recursive(
    stmt: &IrStatement,
    locations_written: &mut HashSet<Aos<IrData>>,
    instruction_args: &Box<[iceball::Argument]>,
) {
    match stmt {
        IrStatement::Assignment { to, .. } => {
            let resolved_loc = resolve_operand::<true>(to, instruction_args);
            locations_written.insert(resolved_loc);
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
                collect_written_locations_recursive(s, locations_written, instruction_args);
            }
        }
        _ => {}
    }
}

fn update_location_mapping_recursive(
    stmt: &IrStatement,
    resolved_location_to_variable_ids: &mut HashMap<Aos<IrData>, HashSet<usize>>,
    instruction_args: &Box<[iceball::Argument]>,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            if let Some(src_ids) = resolved_location_to_variable_ids.get(from).cloned() {
                resolved_location_to_variable_ids.insert(to.clone(), src_ids);
            } else {
                resolved_location_to_variable_ids.remove(to);
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
            let mut true_map = resolved_location_to_variable_ids.clone();
            for s in true_branch.iter() {
                update_location_mapping_recursive(s, &mut true_map, instruction_args);
            }
            let mut false_map = resolved_location_to_variable_ids.clone();
            for s in false_branch.iter() {
                update_location_mapping_recursive(s, &mut false_map, instruction_args);
            }

            let mut merged_map: HashMap<Aos<IrData>, HashSet<usize>> = HashMap::new();
            for (loc, ids) in true_map.into_iter().chain(false_map.into_iter()) {
                let resolved_loc = resolve_operand::<true>(&loc, instruction_args);
                merged_map.entry(resolved_loc).or_default().extend(ids);
            }

            *resolved_location_to_variable_ids = merged_map;
        }
        _ => {}
    }
}

pub fn analyze_variables(ir_block: &IrBlock) -> Result<HashSet<IrVariable>, &'static str> {
    let mut variables: Vec<IrVariable> = Vec::new();
    let mut resolved_location_to_variable_ids: HashMap<Aos<IrData>, HashSet<usize>> =
        HashMap::new();
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
        if ir.statements.is_none() {
            continue;
        }
        let statements = ir.statements.as_ref().unwrap();
        let instruction = &ir.instruction.as_ref().inner;
        let instruction_args = &instruction.arguments;
        let known_datatypes_at_ir_resolved =
            resolve_known_datatypes(&known_datatypes_per_ir[ir_index], instruction_args);
        let data_access_at_ir_resolved =
            resolve_data_accesses(&data_access_per_ir[ir_index], instruction_args);

        // --- Step 1: Identify all locations written within this IR (including nested statements) ---
        let mut locations_written_this_ir: HashSet<Aos<IrData>> = HashSet::new();
        for da in data_access_at_ir_resolved.iter() {
            if *da.access_type() == DataAccessType::Write {
                let resolved_loc = da.location();
                locations_written_this_ir.insert(resolved_loc.clone());
            }
        }
        for stmt in statements.iter() {
            collect_written_locations_recursive(
                stmt,
                &mut locations_written_this_ir,
                instruction_args,
            );
        }

        // --- Step 2: Tentatively kill variables whose locations are overwritten ---
        for resolved_loc in &locations_written_this_ir {
            if let Some(old_ids) = resolved_location_to_variable_ids.remove(resolved_loc) {
                for id in old_ids {
                    if variables[id].live_out.is_none() {
                        variables[id].live_out = Some(ir_index);
                    }
                }
            }
        }

        // --- Step 3: Process Data Accesses (Reads and Writes) ---
        for da in data_access_at_ir_resolved.iter() {
            let resolved_loc = da.location().clone();
            let access_type = da.access_type();
            let ids = resolved_location_to_variable_ids
                .entry(resolved_loc.clone())
                .or_default();

            if ids.is_empty() {
                let new_id = variables.len();
                let live_in = match access_type {
                    DataAccessType::Write => Some(ir_index),
                    DataAccessType::Read => None, // Live-in from block start (simplified assumption)
                };

                let data_type = known_datatypes_at_ir_resolved
                    .iter()
                    .filter(|x| x.location.as_ref() == resolved_loc.as_ref())
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
            update_location_mapping_recursive(
                stmt,
                &mut resolved_location_to_variable_ids,
                instruction_args,
            );
        }
        resolved_location_to_variable_ids.retain(|_, ids| !ids.is_empty());
    }

    Ok(variables.into_iter().collect())
}

fn resolve_operand<const RECURSIVE: bool>(
    data: &Aos<IrData>,
    instruction_args: &Box<[iceball::Argument]>,
) -> Aos<IrData> {
    if *data == *O1 {
        todo!()
    } else if *data == *O2 {
        todo!()
    } else if *data == *O3 {
        todo!()
    } else if *data == *O4 {
        todo!()
    }

    if !RECURSIVE {
        return data.clone();
    }
    let mut related_data = Vec::new();
    data.get_related_ir_data(&mut related_data);
    todo!()
}

fn resolve_data_accesses(
    data: &Vec<DataAccess>,
    instruction_args: &Box<[iceball::Argument]>,
) -> Vec<DataAccess> {
    let mut result = Vec::new();
    for data in data {
        let mut related_data = Vec::new();
        data.get_related_ir_data(&mut related_data);
        if related_data
            .iter()
            .all(|x| resolve_operand::<false>(x, instruction_args) == **x)
        {
            result.push(data.clone());
            continue;
        }
        let loc = data.location();
        let access_type = data.access_type();
        let size = data.size();
        todo!()
    }
    result
}

fn resolve_known_datatypes(
    data: &Vec<KnownDataType>,
    instruction_args: &Box<[iceball::Argument]>,
) -> Vec<KnownDataType> {
    let mut result = Vec::new();
    for data in data {
        let mut related_data = Vec::new();
        data.get_related_ir_data(&mut related_data);
        if related_data
            .iter()
            .all(|x| resolve_operand::<false>(x, instruction_args) == **x)
        {
            result.push(data.clone());
            continue;
        }
        let &KnownDataType {
            location,
            data_type,
            data_size,
        } = &data;
        todo!()
    }
    result
}
