use crate::{
    core::Address,
    ir::{data::DataAccess, statements::IrStatement, Ir},
};

pub fn analyze_data_access(ir: &Ir) -> Vec<DataAccess> {
    if ir.statements.is_right() {
        return Vec::new();
    }
    let mut result = Vec::new();
    let address = &ir.address;
    for statement in ir.statements.as_ref().unwrap_left().iter() {
        analyze_data_access_raw(&mut result, &address, statement);
    }
    result
}

pub fn analyze_data_access_raw(
    v: &mut Vec<DataAccess>,
    address: &Address,
    statement: &IrStatement,
) {
    todo!()
}
