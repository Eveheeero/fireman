use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct RegKey {
    architecture: Architecture,
    bit_start: usize,
}

impl RegKey {
    #[inline]
    pub(super) fn from_register(reg: &Register) -> Self {
        Self {
            architecture: reg.architecture(),
            bit_start: reg.bit_range().start,
        }
    }

    #[inline]
    pub(super) fn sort_key(&self) -> (u8, usize) {
        (architecture_sort_key(self.architecture), self.bit_start)
    }
}

#[inline]
const fn architecture_sort_key(architecture: Architecture) -> u8 {
    match architecture {
        Architecture::X64 => 0,
    }
}

pub(super) type RegName = RegKey;
pub(super) type RegNameToVarMap = HashMap<RegName, AstVariableId>;
pub(super) type RegNameExprMap = HashMap<RegName, Wrapped<AstExpression>>;
pub(super) type RegNameSet = HashSet<RegName>;

pub(super) fn get_function_parameters(
    ast: &Ast,
    function_id: AstFunctionId,
) -> Option<Vec<crate::abstract_syntax_tree::AstParameter>> {
    let functions = ast.functions.read().unwrap();
    let version = *ast.function_versions.get(&function_id)?;
    let func = functions.get(&function_id)?.get(&version)?;
    Some(func.parameters.clone())
}

pub(super) fn build_register_name_to_var_map(variables: &ArcAstVariableMap) -> RegNameToVarMap {
    let mut map: RegNameToVarMap = HashMap::new();
    let vars = variables.read().unwrap();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                let IrData::Register(reg) = da.location().as_ref() else {
                    continue;
                };
                map.entry(RegKey::from_register(reg)).or_insert(*var_id);
            }
        }
    }
    map
}

pub(super) fn build_var_id_to_register_name_map(
    variables: &ArcAstVariableMap,
) -> HashMap<AstVariableId, Register> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, Register> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        let mut any_reg: Option<Register> = None;
        let mut written_reg: Option<Register> = None;
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                let IrData::Register(reg) = da.location().as_ref() else {
                    continue;
                };
                if any_reg.is_none() {
                    any_reg = Some(reg.clone());
                }
                if *da.access_type() == IrDataAccessType::Write {
                    written_reg = Some(reg.clone());
                    break;
                }
            }
            if written_reg.is_some() {
                break;
            }
        }
        if let Some(reg) = written_reg.or(any_reg) {
            map.insert(*var_id, reg);
        }
    }
    map
}

pub(super) fn build_data_location_to_var_map(
    variables: &ArcAstVariableMap,
) -> HashMap<IrData, AstVariableId> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<IrData, AstVariableId> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                map.entry(da.location().as_ref().clone()).or_insert(*var_id);
            }
        }
    }
    map
}

pub(super) fn build_var_id_to_rsp_offset_map(
    variables: &ArcAstVariableMap,
) -> HashMap<AstVariableId, isize> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, isize> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            let mut found = false;
            for da in accesses.iter() {
                let Some(off) = rsp_offset_from_location(da.location().as_ref()) else {
                    continue;
                };
                map.entry(*var_id).or_insert(off);
                found = true;
                break;
            }
            if found {
                break;
            }
        }
    }
    map
}

pub(super) fn build_rsp_offset_to_var_map(
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
) -> HashMap<isize, AstVariableId> {
    let mut map: HashMap<isize, AstVariableId> = HashMap::new();
    let mut entries: Vec<_> = var_id_to_rsp_offset
        .iter()
        .map(|(var_id, off)| (*off, *var_id))
        .collect();
    entries.sort_unstable_by_key(|(off, var_id)| (*off, var_id.index));
    for (off, var_id) in entries {
        map.entry(off).or_insert(var_id);
    }
    map
}

pub(super) fn rsp_offset_from_location(loc: &IrData) -> Option<isize> {
    let IrData::Dereference(inner) = loc else {
        return None;
    };
    rsp_offset_from_expr(inner.as_ref())
}

fn rsp_offset_from_expr(expr: &IrData) -> Option<isize> {
    fn is_sp_expr(data: &IrData) -> bool {
        matches!(data, IrData::Register(r) if r.is_sp())
    }
    fn signed_const(data: &IrData) -> Option<isize> {
        match data {
            IrData::Constant(v) => Some(*v as isize),
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::Negation,
                arg,
            }) => arg.constant().map(|v| 0 - v as isize),
            _ => None,
        }
    }

    if is_sp_expr(expr) {
        return Some(0);
    }

    match expr {
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Add,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()),
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Sub,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()).map(|v| 0 - v),
        _ => None,
    }
}

pub(super) fn ir_register(data: &IrData) -> Option<Register> {
    match data {
        IrData::Register(r) => Some(r.clone()),
        _ => None,
    }
}

pub(super) fn wrap_var_expr(
    var_map: ArcAstVariableMap,
    var_id: AstVariableId,
) -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Variable(var_map, var_id),
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

pub(super) fn wrap_unknown_expr() -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Unknown,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

pub(super) fn is_unknown_expr(expr: &Wrapped<AstExpression>) -> bool {
    matches!(expr.item, AstExpression::Unknown)
}
