//! Module that converts x86_64 architecture instructions into IR

pub mod atomic;
pub mod fpu_stack;
pub mod instruction_analyze;
pub mod lifter;
mod lifter_types;

mod static_register {
    #![allow(non_upper_case_globals, unused)]
    use crate::{
        ir::{VirtualMachine, data::IrData, x86_64::X64Range as X64},
        utils::Aos,
    };
    use std::sync::LazyLock;

    macro_rules! static_register {
        ($name:ident) => {
            pub(crate) static $name: LazyLock<Aos<IrData>> = LazyLock::new(|| {
                Aos::new_static(IrData::Register(<VirtualMachine as X64>::$name()))
            });
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
    /// Carry Flag, true if over/underflow during unsigned operation.
    static_register!(cf);
    /// Parity Flag, true if number of 1 bits is even. (for error detection)
    static_register!(pf);
    /// Adjust Flag, true if result of operation is adjusted.
    static_register!(af);
    /// Zero Flag, true if result is zero.
    static_register!(zf);
    /// Sign Flag, true if result is negative.
    static_register!(sf);
    /// Trap Flag, if true, break(int1) every step
    static_register!(tf);
    /// Interrupt Enable Flag, if true, interrupt is enabled.
    static_register!(r#if);
    /// Direction Flag, true if string operation is backward.
    static_register!(df);
    /// Overflow Flag, true if overflow/underflow during signed operation.
    static_register!(of);
    /// I/O Privilege Level, 0 for kernel, 3 for user.
    static_register!(iopl);
    /// Nested Task Flag, true if nested task.
    static_register!(nt);
    /// Resume Flag, true if resume from exception.
    static_register!(rf);
    /// Virtual 8086 Mode, true if in 8086 mode.
    static_register!(vm);
    /// Alignment Check Flag.
    static_register!(ac);
    /// Virtual Interrupt Flag, true if virtual interrupt.
    static_register!(vif);
    /// Virtual Interrupt Pending Flag, true if virtual interrupt pending.
    static_register!(vip);
    /// ID Flag, true if CPUID is supported.
    static_register!(id);

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

pub(crate) fn str_to_x64_register(data: &str) -> crate::utils::Aos<crate::ir::data::IrData> {
    let data = data.to_ascii_lowercase();
    macro_rules! str_to_reg {
        ($name:ident) => {
            if data == stringify!($name) {
                return static_register::$name.clone();
            }
        };
    }

    str_to_reg!(rax);
    str_to_reg!(eax);
    str_to_reg!(ax);
    str_to_reg!(al);
    str_to_reg!(ah);

    str_to_reg!(rbx);
    str_to_reg!(ebx);
    str_to_reg!(bx);
    str_to_reg!(bl);
    str_to_reg!(bh);

    str_to_reg!(rcx);
    str_to_reg!(ecx);
    str_to_reg!(cx);
    str_to_reg!(cl);
    str_to_reg!(ch);

    str_to_reg!(rdx);
    str_to_reg!(edx);
    str_to_reg!(dx);
    str_to_reg!(dl);
    str_to_reg!(dh);

    str_to_reg!(rsp);
    str_to_reg!(esp);
    str_to_reg!(sp);
    str_to_reg!(spl);

    str_to_reg!(rbp);
    str_to_reg!(ebp);
    str_to_reg!(bp);
    str_to_reg!(bpl);

    str_to_reg!(rsi);
    str_to_reg!(esi);
    str_to_reg!(si);
    str_to_reg!(sil);

    str_to_reg!(rdi);
    str_to_reg!(edi);
    str_to_reg!(di);
    str_to_reg!(dil);

    str_to_reg!(r8);
    str_to_reg!(r8d);
    str_to_reg!(r8w);
    str_to_reg!(r8b);

    str_to_reg!(r9);
    str_to_reg!(r9d);
    str_to_reg!(r9w);
    str_to_reg!(r9b);

    str_to_reg!(r10);
    str_to_reg!(r10d);
    str_to_reg!(r10w);
    str_to_reg!(r10b);

    str_to_reg!(r11);
    str_to_reg!(r11d);
    str_to_reg!(r11w);
    str_to_reg!(r11b);

    str_to_reg!(r12);
    str_to_reg!(r12d);
    str_to_reg!(r12w);
    str_to_reg!(r12b);

    str_to_reg!(r13);
    str_to_reg!(r13d);
    str_to_reg!(r13w);
    str_to_reg!(r13b);

    str_to_reg!(r14);
    str_to_reg!(r14d);
    str_to_reg!(r14w);
    str_to_reg!(r14b);

    str_to_reg!(r15);
    str_to_reg!(r15d);
    str_to_reg!(r15w);
    str_to_reg!(r15b);

    str_to_reg!(cs);
    str_to_reg!(ds);
    str_to_reg!(es);
    str_to_reg!(fs);
    str_to_reg!(gs);
    str_to_reg!(ss);

    str_to_reg!(rip);
    str_to_reg!(eip);
    str_to_reg!(ip);

    str_to_reg!(rflags);
    str_to_reg!(eflags);
    str_to_reg!(flags);
    str_to_reg!(cf);
    str_to_reg!(pf);
    str_to_reg!(af);
    str_to_reg!(zf);
    str_to_reg!(sf);
    str_to_reg!(tf);
    str_to_reg!(r#if);
    str_to_reg!(df);
    str_to_reg!(of);
    str_to_reg!(iopl);
    str_to_reg!(nt);
    str_to_reg!(rf);
    str_to_reg!(vm);
    str_to_reg!(ac);
    str_to_reg!(vif);
    str_to_reg!(vip);
    str_to_reg!(id);

    str_to_reg!(fpu_status_word);
    str_to_reg!(fpu_ie);
    str_to_reg!(fpu_de);
    str_to_reg!(fpu_ze);
    str_to_reg!(fpu_oe);
    str_to_reg!(fpu_ue);
    str_to_reg!(fpu_pe);
    str_to_reg!(fpu_sf);
    str_to_reg!(fpu_es);
    str_to_reg!(fpu_c0);
    str_to_reg!(fpu_c1);
    str_to_reg!(fpu_c2);
    str_to_reg!(fpu_top);
    str_to_reg!(fpu_c3);
    str_to_reg!(fpu_b);

    str_to_reg!(st0);
    str_to_reg!(st1);
    str_to_reg!(st2);
    str_to_reg!(st3);
    str_to_reg!(st4);
    str_to_reg!(st5);
    str_to_reg!(st6);
    str_to_reg!(st7);

    str_to_reg!(mm0);
    str_to_reg!(mm1);
    str_to_reg!(mm2);
    str_to_reg!(mm3);
    str_to_reg!(mm4);
    str_to_reg!(mm5);
    str_to_reg!(mm6);
    str_to_reg!(mm7);
    str_to_reg!(mm8);
    str_to_reg!(mm9);
    str_to_reg!(mm10);
    str_to_reg!(mm11);
    str_to_reg!(mm12);
    str_to_reg!(mm13);
    str_to_reg!(mm14);
    str_to_reg!(mm15);
    str_to_reg!(mm16);
    str_to_reg!(mm17);
    str_to_reg!(mm18);
    str_to_reg!(mm19);
    str_to_reg!(mm20);
    str_to_reg!(mm21);
    str_to_reg!(mm22);
    str_to_reg!(mm23);
    str_to_reg!(mm24);
    str_to_reg!(mm25);
    str_to_reg!(mm26);
    str_to_reg!(mm27);
    str_to_reg!(mm28);
    str_to_reg!(mm29);
    str_to_reg!(mm30);
    str_to_reg!(mm31);

    str_to_reg!(xmm0);
    str_to_reg!(xmm1);
    str_to_reg!(xmm2);
    str_to_reg!(xmm3);
    str_to_reg!(xmm4);
    str_to_reg!(xmm5);
    str_to_reg!(xmm6);
    str_to_reg!(xmm7);
    str_to_reg!(xmm8);
    str_to_reg!(xmm9);
    str_to_reg!(xmm10);
    str_to_reg!(xmm11);
    str_to_reg!(xmm12);
    str_to_reg!(xmm13);
    str_to_reg!(xmm14);
    str_to_reg!(xmm15);
    str_to_reg!(xmm16);
    str_to_reg!(xmm17);
    str_to_reg!(xmm18);
    str_to_reg!(xmm19);
    str_to_reg!(xmm20);
    str_to_reg!(xmm21);
    str_to_reg!(xmm22);
    str_to_reg!(xmm23);
    str_to_reg!(xmm24);
    str_to_reg!(xmm25);
    str_to_reg!(xmm26);
    str_to_reg!(xmm27);
    str_to_reg!(xmm28);
    str_to_reg!(xmm29);
    str_to_reg!(xmm30);
    str_to_reg!(xmm31);

    str_to_reg!(ymm0);
    str_to_reg!(ymm1);
    str_to_reg!(ymm2);
    str_to_reg!(ymm3);
    str_to_reg!(ymm4);
    str_to_reg!(ymm5);
    str_to_reg!(ymm6);
    str_to_reg!(ymm7);
    str_to_reg!(ymm8);
    str_to_reg!(ymm9);
    str_to_reg!(ymm10);
    str_to_reg!(ymm11);
    str_to_reg!(ymm12);
    str_to_reg!(ymm13);
    str_to_reg!(ymm14);
    str_to_reg!(ymm15);
    str_to_reg!(ymm16);
    str_to_reg!(ymm17);
    str_to_reg!(ymm18);
    str_to_reg!(ymm19);
    str_to_reg!(ymm20);
    str_to_reg!(ymm21);
    str_to_reg!(ymm22);
    str_to_reg!(ymm23);
    str_to_reg!(ymm24);
    str_to_reg!(ymm25);
    str_to_reg!(ymm26);
    str_to_reg!(ymm27);
    str_to_reg!(ymm28);
    str_to_reg!(ymm29);
    str_to_reg!(ymm30);
    str_to_reg!(ymm31);

    str_to_reg!(zmm0);
    str_to_reg!(zmm1);
    str_to_reg!(zmm2);
    str_to_reg!(zmm3);
    str_to_reg!(zmm4);
    str_to_reg!(zmm5);
    str_to_reg!(zmm6);
    str_to_reg!(zmm7);
    str_to_reg!(zmm8);
    str_to_reg!(zmm9);
    str_to_reg!(zmm10);
    str_to_reg!(zmm11);
    str_to_reg!(zmm12);
    str_to_reg!(zmm13);
    str_to_reg!(zmm14);
    str_to_reg!(zmm15);
    str_to_reg!(zmm16);
    str_to_reg!(zmm17);
    str_to_reg!(zmm18);
    str_to_reg!(zmm19);
    str_to_reg!(zmm20);
    str_to_reg!(zmm21);
    str_to_reg!(zmm22);
    str_to_reg!(zmm23);
    str_to_reg!(zmm24);
    str_to_reg!(zmm25);
    str_to_reg!(zmm26);
    str_to_reg!(zmm27);
    str_to_reg!(zmm28);
    str_to_reg!(zmm29);
    str_to_reg!(zmm30);
    str_to_reg!(zmm31);

    str_to_reg!(cr0);
    str_to_reg!(cr1);
    str_to_reg!(cr2);
    str_to_reg!(cr3);
    str_to_reg!(cr4);
    str_to_reg!(cr5);
    str_to_reg!(cr6);
    str_to_reg!(cr7);
    str_to_reg!(cr8);
    str_to_reg!(cr9);
    str_to_reg!(cr10);
    str_to_reg!(cr11);
    str_to_reg!(cr12);
    str_to_reg!(cr13);
    str_to_reg!(cr14);
    str_to_reg!(cr15);

    str_to_reg!(dr0);
    str_to_reg!(dr1);
    str_to_reg!(dr2);
    str_to_reg!(dr3);
    str_to_reg!(dr4);
    str_to_reg!(dr5);
    str_to_reg!(dr6);
    str_to_reg!(dr7);
    str_to_reg!(dr8);
    str_to_reg!(dr9);
    str_to_reg!(dr10);
    str_to_reg!(dr11);
    str_to_reg!(dr12);
    str_to_reg!(dr13);
    str_to_reg!(dr14);
    str_to_reg!(dr15);

    unreachable!("{}", data)
}
