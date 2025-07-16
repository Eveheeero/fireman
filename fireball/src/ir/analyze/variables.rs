use crate::{
    ir::{
        analyze::{DataType, KnownDataType},
        data::{AccessSize, DataAccess, DataAccessType, IrData, IrDataOperation, IrIntrinsic},
        operator::BinaryOperator,
        statements::IrStatement,
        utils::{IrStatementDescriptor, IrStatementDescriptorMap},
        IrBlock,
    },
    utils::Aos,
};
pub use private::IrVariable;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

mod private {
    use super::*;
    #[derive(Clone, PartialEq, Eq)]
    pub struct IrVariable {
        pub live_in: Option<u32>,
        pub shown_in: Vec<u32>,
        pub live_out: Option<u32>,
        accesses: IrStatementDescriptorMap<Vec<DataAccess>>,
        pub data_type: DataType,
    }
    impl IrVariable {
        #[inline]
        pub fn new(live_in: Option<u32>, data_type: DataType) -> Self {
            Self {
                live_in,
                shown_in: Vec::new(),
                live_out: None,
                accesses: IrStatementDescriptorMap::new(),
                data_type,
            }
        }
        #[inline]
        pub fn get_data_accesses(&self) -> &IrStatementDescriptorMap<Vec<DataAccess>> {
            &self.accesses
        }
        #[inline]
        pub(crate) fn into_data_accesses(self) -> IrStatementDescriptorMap<Vec<DataAccess>> {
            self.accesses
        }
        #[inline]
        pub fn add_data_access(
            &mut self,
            ir_index: u32,
            statement_index: impl Into<Option<u8>>,
            access: DataAccess,
        ) {
            let key = IrStatementDescriptor::new(ir_index, statement_index);
            self.accesses.insert_checked(key, Vec::new());
            self.accesses.get_mut(key).unwrap().push(access);
        }
        #[inline]
        pub fn get_all_data_accesses(&self) -> Vec<(IrStatementDescriptor, &[DataAccess])> {
            let keys = self.accesses.keys();
            keys.into_iter()
                .map(|key| (key, self.accesses.get(key).unwrap().as_slice()))
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
    instruction_args: &[iceball::Argument],
) {
    match stmt {
        IrStatement::Assignment { to, .. } => {
            let resolved_loc = resolve_operand(to, instruction_args);
            locations_written.insert(resolved_loc);
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
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
    instruction_args: &[iceball::Argument],
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let resolved_to = resolve_operand(to, instruction_args);
            let resolved_from = resolve_operand(from, instruction_args);

            if let Some(src_ids) = resolved_location_to_variable_ids
                .get(&resolved_from)
                .cloned()
            {
                resolved_location_to_variable_ids.insert(resolved_to, src_ids);
            } else {
                resolved_location_to_variable_ids.remove(&resolved_to);
            }
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
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
                let resolved_loc = resolve_operand(&loc, instruction_args);
                merged_map.entry(resolved_loc).or_default().extend(ids);
            }
            *resolved_location_to_variable_ids = merged_map;
        }
        _ => {}
    }
}

pub fn analyze_variables(ir_block: &IrBlock) -> Result<Vec<IrVariable>, &'static str> {
    let mut variables: Vec<IrVariable> = Vec::new();
    let mut resolved_location_to_variable_ids: HashMap<Aos<IrData>, HashSet<usize>> =
        HashMap::new();
    let irs = &ir_block.ir;
    let known_datatypes = ir_block
        .known_datatypes
        .as_ref()
        .ok_or("Datatypes Not Analyzed")?;
    let data_access = ir_block
        .data_access
        .as_ref()
        .ok_or("Data Access Not Analyzed")?;

    for (ir_index, ir) in irs.iter().enumerate() {
        if ir.statements.is_none() {
            continue;
        }
        let instruction = &ir_block.instructions().as_ref()[ir_index].inner;
        let ir_index = ir_index as u32;
        let statements = ir.statements.as_ref().unwrap();
        let instruction_args = &instruction.arguments;
        let known_datatypes_at_ir_resolved = resolve_known_datatypes(
            &known_datatypes
                .iter()
                .filter(|(key, _)| key.ir_index() == ir_index)
                .flat_map(|(_, value)| value)
                .collect::<Vec<_>>(),
            instruction_args,
        );
        let data_access_at_ir_resolved =
            resolve_data_accesses(data_access, ir_index, instruction_args);

        // --- Step 1: Identify all locations written within this IR (including nested statements) ---
        let mut locations_written_this_ir: HashSet<Aos<IrData>> = HashSet::new();
        for da in data_access_at_ir_resolved.iter() {
            if *da.1.access_type() == DataAccessType::Write {
                let resolved_loc = da.1.location();
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
            let resolved_loc = da.1.location().clone();
            let access_type = da.1.access_type();
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
                variables[var_id].add_data_access(ir_index, *da.0.statement_index(), da.1.clone());

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

    Ok(variables)
}

fn resolve_access_size(
    access_size: &AccessSize,
    instruction_args: &[iceball::Argument],
) -> AccessSize {
    match access_size {
        AccessSize::ResultOfBit(data) => {
            AccessSize::ResultOfBit(resolve_operand(data, instruction_args))
        }
        AccessSize::ResultOfByte(data) => {
            AccessSize::ResultOfByte(resolve_operand(data, instruction_args))
        }
        AccessSize::RelativeWith(data) => {
            AccessSize::RelativeWith(resolve_operand(data, instruction_args))
        }
        AccessSize::ArchitectureSize | AccessSize::Unlimited => access_size.clone(),
    }
}

fn resolve_ir_intrinsic(
    intrinsic: &IrIntrinsic,
    instruction_args: &[iceball::Argument],
) -> IrIntrinsic {
    match intrinsic {
        IrIntrinsic::SignedMax(size) => {
            IrIntrinsic::SignedMax(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::SignedMin(size) => {
            IrIntrinsic::SignedMin(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::UnsignedMax(size) => {
            IrIntrinsic::UnsignedMax(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::UnsignedMin(size) => {
            IrIntrinsic::UnsignedMin(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::BitOnes(size) => {
            IrIntrinsic::BitOnes(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::BitZeros(size) => {
            IrIntrinsic::BitZeros(resolve_access_size(size, instruction_args))
        }
        IrIntrinsic::ByteSizeOf(data) => {
            IrIntrinsic::ByteSizeOf(resolve_operand(data, instruction_args))
        }
        IrIntrinsic::BitSizeOf(data) => {
            IrIntrinsic::BitSizeOf(resolve_operand(data, instruction_args))
        }
        IrIntrinsic::Sized(data, size) => IrIntrinsic::Sized(
            resolve_operand(data, instruction_args),
            resolve_access_size(size, instruction_args),
        ),
        IrIntrinsic::OperandExists(_) => intrinsic.clone(),
        IrIntrinsic::Unknown
        | IrIntrinsic::Undefined
        | IrIntrinsic::ArchitectureByteSize
        | IrIntrinsic::ArchitectureBitSize
        | IrIntrinsic::ArchitectureBitPerByte
        | IrIntrinsic::InstructionByteSize
        | IrIntrinsic::ArchitectureByteSizeCondition(..) => intrinsic.clone(),
    }
}

fn resolve_binary_operator(
    op: &BinaryOperator,
    instruction_args: &[iceball::Argument],
) -> BinaryOperator {
    match op {
        BinaryOperator::Equal(s) => BinaryOperator::Equal(resolve_access_size(s, instruction_args)),
        BinaryOperator::SignedLess(s) => {
            BinaryOperator::SignedLess(resolve_access_size(s, instruction_args))
        }
        BinaryOperator::SignedLessOrEqual(s) => {
            BinaryOperator::SignedLessOrEqual(resolve_access_size(s, instruction_args))
        }
        BinaryOperator::UnsignedLess(s) => {
            BinaryOperator::UnsignedLess(resolve_access_size(s, instruction_args))
        }
        BinaryOperator::UnsignedLessOrEqual(s) => {
            BinaryOperator::UnsignedLessOrEqual(resolve_access_size(s, instruction_args))
        }
        BinaryOperator::And
        | BinaryOperator::Or
        | BinaryOperator::Xor
        | BinaryOperator::Shl
        | BinaryOperator::Shr
        | BinaryOperator::Sar
        | BinaryOperator::Add
        | BinaryOperator::Sub
        | BinaryOperator::Mul
        | BinaryOperator::SignedDiv
        | BinaryOperator::SignedRem
        | BinaryOperator::UnsignedDiv
        | BinaryOperator::UnsignedRem => op.clone(),
    }
}

pub fn resolve_operand(data: &Aos<IrData>, instruction_args: &[iceball::Argument]) -> Aos<IrData> {
    match data.as_ref() {
        IrData::Operand(op_num) => {
            if instruction_args.len() < op_num.get() as usize {
                /* Fallback if `operand_exists` based routine */
                static UNDEFINED: LazyLock<Aos<IrData>> =
                    LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::Undefined)));
                return UNDEFINED.clone();
            }
            let result = (&instruction_args[op_num.get() as usize - 1]).into();
            return result;
        }
        _ => {}
    }

    match data.as_ref() {
        IrData::Intrinsic(intrinsic) => Aos::new(IrData::Intrinsic(resolve_ir_intrinsic(
            intrinsic,
            instruction_args,
        ))),
        IrData::Dereference(inner_data) => {
            let resolved_inner = resolve_operand(inner_data, instruction_args);
            Aos::new(IrData::Dereference(resolved_inner))
        }
        IrData::Operation(operation) => match operation {
            IrDataOperation::Unary { operator, arg } => {
                let resolved_arg = resolve_operand(arg, instruction_args);
                Aos::new(IrData::Operation(IrDataOperation::Unary {
                    operator: *operator,
                    arg: resolved_arg,
                }))
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                let resolved_arg1 = resolve_operand(arg1, instruction_args);
                let resolved_arg2 = resolve_operand(arg2, instruction_args);
                let resolved_op = resolve_binary_operator(operator, instruction_args);
                Aos::new(IrData::Operation(IrDataOperation::Binary {
                    operator: resolved_op,
                    arg1: resolved_arg1,
                    arg2: resolved_arg2,
                }))
            }
        },
        IrData::Operand(_) => unreachable!(),
        IrData::Constant(_) | IrData::Register(_) => data.clone(),
    }
}

fn resolve_data_accesses(
    data_access: &IrStatementDescriptorMap<Vec<DataAccess>>,
    ir_index: u32,
    instruction_args: &[iceball::Argument],
) -> Vec<(IrStatementDescriptor, DataAccess)> {
    data_access
        .iter()
        .filter(|(k, _v)| k.ir_index() == ir_index)
        .flat_map(|(k, da)| {
            da.iter().map(move |da| {
                let resolved_loc = resolve_operand(da.location(), instruction_args);
                let resolved_size = resolve_access_size(da.size(), instruction_args);
                (
                    k,
                    DataAccess::new(resolved_loc, *da.access_type(), resolved_size),
                )
            })
        })
        .collect()
}

fn resolve_known_datatypes(
    known_datatype: &[&KnownDataType],
    instruction_args: &[iceball::Argument],
) -> Vec<KnownDataType> {
    known_datatype
        .iter()
        .map(|known_datatype| {
            let resolved_location = resolve_operand(&known_datatype.location, instruction_args);
            let resolved_data_size =
                resolve_access_size(&known_datatype.data_size, instruction_args);

            KnownDataType {
                location: resolved_location,
                data_type: known_datatype.data_type,
                data_size: resolved_data_size,
            }
        })
        .collect()
}
