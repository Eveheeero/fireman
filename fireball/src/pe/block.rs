use crate::{
    core::{Address, Block, DestinationType, RelationType},
    pe::Pe,
};
use std::sync::Arc;

impl Pe {
    /// 파일 오프셋을 기준으로 블럭의 범위를 계산한다
    ///
    /// ### Arguments
    /// - `address: &Address` - 블럭의 시작
    ///
    /// ### Returns
    /// - `Arc<Block>` - 해당 주소로부터 계산된 블럭
    pub(crate) fn generate_block_from_address(&self, address: &Address) -> Arc<Block> {
        if let Some(block) = self.blocks.get_by_start_address(address) {
            return block;
        }
        let mut address = address.clone();
        let start_address = address.clone();
        let mut last_instruction_address = None;
        let mut block_size = None;
        loop {
            let inst = self.parse_assem_count(&address, 1);
            if inst.is_err() || inst.as_ref().unwrap().is_empty() {
                break;
            }
            debug_assert_eq!(inst.as_ref().unwrap().len(), 1);
            let inst = &inst.unwrap()[0].inner;
            if inst.statement.is_err() {
                break;
            }
            if inst.is_jcc() || inst.is_jmp() || inst.is_call() || inst.is_ret() {
                last_instruction_address = Some(address);
                break;
            }
            address += inst.bytes.as_ref().unwrap().len() as u64;
        }

        /* 해당 블럭에서 연결된 다른 블럭 찾기 */
        let mut connected_to = Vec::new();
        // 끝 주소가 정해지지 않은 경우 연결된 블럭 없음
        if let Some(last_instruction_address) = &last_instruction_address {
            let inst = &self.parse_assem_count(last_instruction_address, 1).unwrap()[0].inner;
            block_size = Some(
                last_instruction_address - &start_address
                    + inst.bytes.as_ref().unwrap().len() as u64,
            );
            if inst.is_jcc() {
                // 다음 주소
                connected_to.push((
                    Some(last_instruction_address + inst.bytes.as_ref().unwrap().len() as u64),
                    DestinationType::Static,
                    RelationType::Continued,
                ));
            }
            // jcc나 call등에 의해 이동하는 주소
            connected_to
                .push(self.get_connected_address_and_relation_type(last_instruction_address, inst));
        }

        self.blocks
            .generate_block(start_address, block_size, &connected_to, None)
    }

    /// 마지막 인스트럭션을 통해 어떤 주소와 연결되어있는지 파악한다
    fn get_connected_address_and_relation_type(
        &self,
        ip: &Address,
        inst: &iceball::Instruction,
    ) -> (Option<Address>, DestinationType, RelationType) {
        let relation_type = match () {
            _ if inst.is_ret() => RelationType::Return,
            _ if inst.is_jcc() => RelationType::Jcc,
            _ if inst.is_jmp() => RelationType::Jump,
            _ if inst.is_call() => RelationType::Call,
            _ => unreachable!("{:?}", inst),
        };
        if inst.arguments.len() != 1 {
            return (None, DestinationType::Dynamic, relation_type);
        }
        let arg = &inst.arguments[0];
        match arg {
            // only rip is predictable target but we can't get it
            iceball::Argument::Register(_) => (None, DestinationType::Dynamic, relation_type),
            iceball::Argument::Memory(iceball::Memory::AbsoluteAddressing(offset)) => (
                Some(Address::from_virtual_address(&self.sections, *offset)),
                DestinationType::Static,
                relation_type,
            ),
            iceball::Argument::Memory(iceball::Memory::RelativeAddressing(args)) => {
                if args
                    .iter()
                    .filter(|x| matches!(x, iceball::RelativeAddressingArgument::Register(_)))
                    .all(|x| {
                        matches!(
                            x,
                            iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                                iceball::X64Register::Eip,
                            )) | iceball::RelativeAddressingArgument::Register(
                                iceball::Register::X64(iceball::X64Register::Rip,)
                            )
                        )
                    })
                {
                    (
                        Some(self.calc_relative_address_with_ip(ip, args)),
                        DestinationType::Static,
                        relation_type,
                    )
                } else {
                    (None, DestinationType::Dynamic, relation_type)
                }
            }
            iceball::Argument::Constant(arg) => (
                Some(Address::from_virtual_address(&self.sections, *arg)),
                DestinationType::Static,
                relation_type,
            ),
        }
    }
    fn calc_relative_address_with_ip(
        &self,
        ip: &Address,
        args: &[iceball::RelativeAddressingArgument],
    ) -> Address {
        let extract_constant = |arg: &iceball::RelativeAddressingArgument| match arg {
            iceball::RelativeAddressingArgument::Constant(x) => *x,
            _ => unreachable!("{:?}", arg),
        };
        let mut args: Vec<_> = args.into();

        // turn ip to constant
        for i in &mut args {
            match i {
                iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                    iceball::X64Register::Eip,
                ))
                | iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                    iceball::X64Register::Rip,
                )) => {
                    *i = iceball::RelativeAddressingArgument::Constant(
                        ip.get_virtual_address() as i128
                    );
                }
                _ => {}
            }
        }

        // calc mul operator
        while args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Mul,
        )) {
            let operator_index = args
                .iter()
                .position(|x| {
                    matches!(
                        x,
                        iceball::RelativeAddressingArgument::Operator(
                            iceball::AddressingOperator::Mul
                        )
                    )
                })
                .unwrap();
            let arg1 = extract_constant(&args[operator_index - 1]);
            let arg2 = extract_constant(&args[operator_index + 1]);
            args.insert(
                operator_index - 1,
                iceball::RelativeAddressingArgument::Constant(arg1 * arg2),
            );
            args.remove(operator_index);
            args.remove(operator_index);
        }

        // calc add/sub operator
        while args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Add,
        )) || args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Sub,
        )) {
            let operator_index = args
                .iter()
                .position(|x| {
                    matches!(
                        x,
                        iceball::RelativeAddressingArgument::Operator(
                            iceball::AddressingOperator::Add | iceball::AddressingOperator::Sub
                        )
                    )
                })
                .unwrap();
            let arg1 = extract_constant(&args[operator_index - 1]);
            let arg2 = extract_constant(&args[operator_index + 1]);
            args.insert(
                operator_index - 1,
                iceball::RelativeAddressingArgument::Constant(match args[operator_index] {
                    iceball::RelativeAddressingArgument::Operator(
                        iceball::AddressingOperator::Add,
                    ) => arg1 + arg2,
                    iceball::RelativeAddressingArgument::Operator(
                        iceball::AddressingOperator::Sub,
                    ) => arg1 - arg2,
                    _ => unreachable!(),
                }),
            );
            args.remove(operator_index);
            args.remove(operator_index);
            args.remove(operator_index);
        }

        // return
        debug_assert!(
            args.len() == 1,
            "주소 계산이 완전하게 끝나지 않았습니다. {:?}",
            args
        );
        let address = extract_constant(&args[0])
            .try_into()
            .expect("주소 연산 결과가 음수값입니다.");
        Address::from_virtual_address(&self.sections, address)
    }
}
