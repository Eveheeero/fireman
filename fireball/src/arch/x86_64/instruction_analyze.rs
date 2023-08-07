//! x86_64 아키텍처 인스트럭션을 IR로 변환하는 함수가 담긴 모듈

use crate::{core::Instruction, ir::statements::*};
use iceball::Statement;
use std::rc::Rc;

mod static_register {
    #![allow(non_upper_case_globals, unused)]
    use crate::ir::{x86_64::X64Range as X64, Ir, Register};
    use once_cell::sync::Lazy;

    macro_rules! static_register {
        ($name:ident) => {
            pub static $name: Lazy<Register> = Lazy::new(|| <Ir as X64>::$name());
        };
    }

    static_register!(rax);
    static_register!(eax);
    static_register!(ax);
    static_register!(al);
    static_register!(ah);

    static_register!(rbx);
    static_register!(ebx);
    static_register!(bx);
    static_register!(bl);
    static_register!(bh);

    static_register!(rcx);
    static_register!(ecx);
    static_register!(cx);
    static_register!(cl);
    static_register!(ch);

    static_register!(rdx);
    static_register!(edx);
    static_register!(dx);
    static_register!(dl);
    static_register!(dh);

    static_register!(rsp);
    static_register!(esp);
    static_register!(sp);
    static_register!(spl);

    static_register!(rbp);
    static_register!(ebp);
    static_register!(bp);
    static_register!(bpl);

    static_register!(rsi);
    static_register!(esi);
    static_register!(si);
    static_register!(sil);

    static_register!(rdi);
    static_register!(edi);
    static_register!(di);
    static_register!(dil);

    static_register!(r8);
    static_register!(r8d);
    static_register!(r8w);
    static_register!(r8b);

    static_register!(r9);
    static_register!(r9d);
    static_register!(r9w);
    static_register!(r9b);

    static_register!(r10);
    static_register!(r10d);
    static_register!(r10w);
    static_register!(r10b);

    static_register!(r11);
    static_register!(r11d);
    static_register!(r11w);
    static_register!(r11b);

    static_register!(r12);
    static_register!(r12d);
    static_register!(r12w);
    static_register!(r12b);

    static_register!(r13);
    static_register!(r13d);
    static_register!(r13w);
    static_register!(r13b);

    static_register!(r14);
    static_register!(r14d);
    static_register!(r14w);
    static_register!(r14b);

    static_register!(r15);
    static_register!(r15d);
    static_register!(r15w);
    static_register!(r15b);

    static_register!(cs);
    static_register!(ds);
    static_register!(es);
    static_register!(fs);
    static_register!(gs);
    static_register!(ss);

    static_register!(rip);
    static_register!(eip);
    static_register!(ip);

    static_register!(rflags);
    static_register!(eflags);
    static_register!(flags);
    static_register!(cf);
    static_register!(pf);
    static_register!(af);
    static_register!(zf);
    static_register!(sf);
    static_register!(tf);
    static_register!(r#if);
    static_register!(df);
    static_register!(of);
    static_register!(iopl);
    static_register!(nt);
    static_register!(rf);
    static_register!(vm);
    static_register!(ac);
    static_register!(vif);
    static_register!(vip);
    static_register!(id);

    static_register!(less);
    static_register!(less_or_equal);
    static_register!(below_or_equal);

    static_register!(fpu_status_word);
    static_register!(fpu_ie);
    static_register!(fpu_de);
    static_register!(fpu_ze);
    static_register!(fpu_oe);
    static_register!(fpu_ue);
    static_register!(fpu_pe);
    static_register!(fpu_sf);
    static_register!(fpu_es);
    static_register!(fpu_c0);
    static_register!(fpu_c1);
    static_register!(fpu_c2);
    static_register!(fpu_top);
    static_register!(fpu_c3);
    static_register!(fpu_b);

    static_register!(st0);
    static_register!(st1);
    static_register!(st2);
    static_register!(st3);
    static_register!(st4);
    static_register!(st5);
    static_register!(st6);
    static_register!(st7);

    static_register!(mm0);
    static_register!(mm1);
    static_register!(mm2);
    static_register!(mm3);
    static_register!(mm4);
    static_register!(mm5);
    static_register!(mm6);
    static_register!(mm7);
    static_register!(mm8);
    static_register!(mm9);
    static_register!(mm10);
    static_register!(mm11);
    static_register!(mm12);
    static_register!(mm13);
    static_register!(mm14);
    static_register!(mm15);
    static_register!(mm16);
    static_register!(mm17);
    static_register!(mm18);
    static_register!(mm19);
    static_register!(mm20);
    static_register!(mm21);
    static_register!(mm22);
    static_register!(mm23);
    static_register!(mm24);
    static_register!(mm25);
    static_register!(mm26);
    static_register!(mm27);
    static_register!(mm28);
    static_register!(mm29);
    static_register!(mm30);
    static_register!(mm31);

    static_register!(xmm0);
    static_register!(xmm1);
    static_register!(xmm2);
    static_register!(xmm3);
    static_register!(xmm4);
    static_register!(xmm5);
    static_register!(xmm6);
    static_register!(xmm7);
    static_register!(xmm8);
    static_register!(xmm9);
    static_register!(xmm10);
    static_register!(xmm11);
    static_register!(xmm12);
    static_register!(xmm13);
    static_register!(xmm14);
    static_register!(xmm15);
    static_register!(xmm16);
    static_register!(xmm17);
    static_register!(xmm18);
    static_register!(xmm19);
    static_register!(xmm20);
    static_register!(xmm21);
    static_register!(xmm22);
    static_register!(xmm23);
    static_register!(xmm24);
    static_register!(xmm25);
    static_register!(xmm26);
    static_register!(xmm27);
    static_register!(xmm28);
    static_register!(xmm29);
    static_register!(xmm30);
    static_register!(xmm31);

    static_register!(ymm0);
    static_register!(ymm1);
    static_register!(ymm2);
    static_register!(ymm3);
    static_register!(ymm4);
    static_register!(ymm5);
    static_register!(ymm6);
    static_register!(ymm7);
    static_register!(ymm8);
    static_register!(ymm9);
    static_register!(ymm10);
    static_register!(ymm11);
    static_register!(ymm12);
    static_register!(ymm13);
    static_register!(ymm14);
    static_register!(ymm15);
    static_register!(ymm16);
    static_register!(ymm17);
    static_register!(ymm18);
    static_register!(ymm19);
    static_register!(ymm20);
    static_register!(ymm21);
    static_register!(ymm22);
    static_register!(ymm23);
    static_register!(ymm24);
    static_register!(ymm25);
    static_register!(ymm26);
    static_register!(ymm27);
    static_register!(ymm28);
    static_register!(ymm29);
    static_register!(ymm30);
    static_register!(ymm31);

    static_register!(zmm0);
    static_register!(zmm1);
    static_register!(zmm2);
    static_register!(zmm3);
    static_register!(zmm4);
    static_register!(zmm5);
    static_register!(zmm6);
    static_register!(zmm7);
    static_register!(zmm8);
    static_register!(zmm9);
    static_register!(zmm10);
    static_register!(zmm11);
    static_register!(zmm12);
    static_register!(zmm13);
    static_register!(zmm14);
    static_register!(zmm15);
    static_register!(zmm16);
    static_register!(zmm17);
    static_register!(zmm18);
    static_register!(zmm19);
    static_register!(zmm20);
    static_register!(zmm21);
    static_register!(zmm22);
    static_register!(zmm23);
    static_register!(zmm24);
    static_register!(zmm25);
    static_register!(zmm26);
    static_register!(zmm27);
    static_register!(zmm28);
    static_register!(zmm29);
    static_register!(zmm30);
    static_register!(zmm31);

    static_register!(cr0);
    static_register!(cr1);
    static_register!(cr2);
    static_register!(cr3);
    static_register!(cr4);
    static_register!(cr5);
    static_register!(cr6);
    static_register!(cr7);
    static_register!(cr8);
    static_register!(cr9);
    static_register!(cr10);
    static_register!(cr11);
    static_register!(cr12);
    static_register!(cr13);
    static_register!(cr14);
    static_register!(cr15);

    static_register!(dr0);
    static_register!(dr1);
    static_register!(dr2);
    static_register!(dr3);
    static_register!(dr4);
    static_register!(dr5);
    static_register!(dr6);
    static_register!(dr7);
    static_register!(dr8);
    static_register!(dr9);
    static_register!(dr10);
    static_register!(dr11);
    static_register!(dr12);
    static_register!(dr13);
    static_register!(dr14);
    static_register!(dr15);

    static_register!(tmp8);
    static_register!(tmp16);
    static_register!(tmp32);
    static_register!(tmp64);
    static_register!(tmp128);
    static_register!(tmp256);
    static_register!(tmp512);

    static_register!(tmp2_8);
    static_register!(tmp2_16);
    static_register!(tmp2_32);
    static_register!(tmp2_64);
    static_register!(tmp2_128);
    static_register!(tmp2_256);
    static_register!(tmp2_512);

    static_register!(tmp3_8);
    static_register!(tmp3_16);
    static_register!(tmp3_32);
    static_register!(tmp3_64);
    static_register!(tmp3_128);
    static_register!(tmp3_256);
    static_register!(tmp3_512);

    static_register!(tmp4_8);
    static_register!(tmp4_16);
    static_register!(tmp4_32);
    static_register!(tmp4_64);
    static_register!(tmp4_128);
    static_register!(tmp4_256);
    static_register!(tmp4_512);
}

/// 어셈블리 인스트럭션을 받아 IR 명령으로 변환한다.
///
/// ### Arguments
/// - `instruction` : 어셈블리 인스트럭션
///
/// ### Returns
/// `Rc<Vec<IRStatement>>` : IR 명령 배열
#[allow(unused)]
fn create_ir_statement(instruction: &Instruction) -> Rc<Vec<IRStatement>> {
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return Rc::new(
            [IRStatement::Unknown(IRStatementUnknown::Instruction(
                instruction.clone(),
            ))]
            .to_vec(),
        );
    };

    use crate::ir::{data::*, operator::*, statements::*, x86_64::X64Range as X64, Ir};
    use iceball::X64Statement;
    use static_register::*;

    Rc::new(match op {
        X64Statement::Aaa => {
            // TODO 64모드일때에 대한 처리
            let al_and_0fh = IRData::Operator(IRDataOperator::Binary(
                BinaryOperator::And,
                Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
                    IRData::Register(al.clone()),
                )))),
                Box::new(IRData::Constant(0x0f)),
            ));
            let al_and_0fh_lt_9 = IRData::Operator(IRDataOperator::Binary(
                BinaryOperator::UnsignedLess,
                Box::new(IRData::Constant(9)),
                Box::new(al_and_0fh),
            ));
            let then = [
                IRStatement::Assignment {
                    from: IRData::Operator(IRDataOperator::Binary(
                        BinaryOperator::Add,
                        Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
                            IRData::Register(ax.clone()),
                        )))),
                        Box::new(IRData::Constant(0x106)),
                    )),
                    to: IRData::Register(ax.clone()),
                },
                IRStatement::Assignment {
                    from: IRData::Constant(1),
                    to: IRData::Register(af.clone()),
                },
                IRStatement::Assignment {
                    from: IRData::Constant(1),
                    to: IRData::Register(cf.clone()),
                },
            ];
            let r#else = [
                IRStatement::Assignment {
                    from: IRData::Constant(0),
                    to: IRData::Register(af.clone()),
                },
                IRStatement::Assignment {
                    from: IRData::Constant(0),
                    to: IRData::Register(cf.clone()),
                },
            ];
            let after = IRStatement::Assignment {
                from: IRData::Operator(IRDataOperator::Binary(
                    BinaryOperator::And,
                    Box::new(IRData::Register(al.clone())),
                    Box::new(IRData::Constant(0)),
                )),
                to: IRData::Register(al.clone()),
            };

            [
                IRStatement::Condition {
                    condition: al_and_0fh_lt_9,
                    true_branch: then.to_vec(),
                    false_branch: r#else.to_vec(),
                },
                after,
            ]
            .to_vec()
        }
        // X64Statement::Aad => [IRStatement::Touch],
        _ => [IRStatement::Unknown(IRStatementUnknown::Instruction(
            instruction.clone(),
        ))]
        .to_vec(),
    })
}
