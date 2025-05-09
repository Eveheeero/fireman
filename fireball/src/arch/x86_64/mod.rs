//! x86_64 아키텍처 어셈블리를 IR로 변환하는 모듈

pub mod instruction_analyze;

mod static_register {
    #![allow(non_upper_case_globals, unused)]
    use crate::{
        ir::{data::IrData, x86_64::X64Range as X64, Register, VirtualMachine},
        utils::Aos,
    };
    use std::sync::LazyLock;

    macro_rules! static_register {
        ($name:ident) => {
            pub(crate) static $name: LazyLock<Aos<IrData>> =
                LazyLock::new(|| IrData::Register(<VirtualMachine as X64>::$name()).into());
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

pub(crate) fn str_to_x64_range(data: &str) -> crate::utils::Aos<crate::ir::data::IrData> {
    let data = data.to_ascii_lowercase();
    macro_rules! ww {
        ($name:ident) => {
            if data == "$name" {
                return static_register::$name.clone();
            }
        };
    }

    ww!(rax);
    ww!(eax);
    ww!(ax);
    ww!(al);
    ww!(ah);

    ww!(rbx);
    ww!(ebx);
    ww!(bx);
    ww!(bl);
    ww!(bh);

    ww!(rcx);
    ww!(ecx);
    ww!(cx);
    ww!(cl);
    ww!(ch);

    ww!(rdx);
    ww!(edx);
    ww!(dx);
    ww!(dl);
    ww!(dh);

    ww!(rsp);
    ww!(esp);
    ww!(sp);
    ww!(spl);

    ww!(rbp);
    ww!(ebp);
    ww!(bp);
    ww!(bpl);

    ww!(rsi);
    ww!(esi);
    ww!(si);
    ww!(sil);

    ww!(rdi);
    ww!(edi);
    ww!(di);
    ww!(dil);

    ww!(r8);
    ww!(r8d);
    ww!(r8w);
    ww!(r8b);

    ww!(r9);
    ww!(r9d);
    ww!(r9w);
    ww!(r9b);

    ww!(r10);
    ww!(r10d);
    ww!(r10w);
    ww!(r10b);

    ww!(r11);
    ww!(r11d);
    ww!(r11w);
    ww!(r11b);

    ww!(r12);
    ww!(r12d);
    ww!(r12w);
    ww!(r12b);

    ww!(r13);
    ww!(r13d);
    ww!(r13w);
    ww!(r13b);

    ww!(r14);
    ww!(r14d);
    ww!(r14w);
    ww!(r14b);

    ww!(r15);
    ww!(r15d);
    ww!(r15w);
    ww!(r15b);

    ww!(cs);
    ww!(ds);
    ww!(es);
    ww!(fs);
    ww!(gs);
    ww!(ss);

    ww!(rip);
    ww!(eip);
    ww!(ip);

    ww!(rflags);
    ww!(eflags);
    ww!(flags);
    ww!(cf);
    ww!(pf);
    ww!(af);
    ww!(zf);
    ww!(sf);
    ww!(tf);
    ww!(r#if);
    ww!(df);
    ww!(of);
    ww!(iopl);
    ww!(nt);
    ww!(rf);
    ww!(vm);
    ww!(ac);
    ww!(vif);
    ww!(vip);
    ww!(id);

    ww!(fpu_status_word);
    ww!(fpu_ie);
    ww!(fpu_de);
    ww!(fpu_ze);
    ww!(fpu_oe);
    ww!(fpu_ue);
    ww!(fpu_pe);
    ww!(fpu_sf);
    ww!(fpu_es);
    ww!(fpu_c0);
    ww!(fpu_c1);
    ww!(fpu_c2);
    ww!(fpu_top);
    ww!(fpu_c3);
    ww!(fpu_b);

    ww!(st0);
    ww!(st1);
    ww!(st2);
    ww!(st3);
    ww!(st4);
    ww!(st5);
    ww!(st6);
    ww!(st7);

    ww!(mm0);
    ww!(mm1);
    ww!(mm2);
    ww!(mm3);
    ww!(mm4);
    ww!(mm5);
    ww!(mm6);
    ww!(mm7);
    ww!(mm8);
    ww!(mm9);
    ww!(mm10);
    ww!(mm11);
    ww!(mm12);
    ww!(mm13);
    ww!(mm14);
    ww!(mm15);
    ww!(mm16);
    ww!(mm17);
    ww!(mm18);
    ww!(mm19);
    ww!(mm20);
    ww!(mm21);
    ww!(mm22);
    ww!(mm23);
    ww!(mm24);
    ww!(mm25);
    ww!(mm26);
    ww!(mm27);
    ww!(mm28);
    ww!(mm29);
    ww!(mm30);
    ww!(mm31);

    ww!(xmm0);
    ww!(xmm1);
    ww!(xmm2);
    ww!(xmm3);
    ww!(xmm4);
    ww!(xmm5);
    ww!(xmm6);
    ww!(xmm7);
    ww!(xmm8);
    ww!(xmm9);
    ww!(xmm10);
    ww!(xmm11);
    ww!(xmm12);
    ww!(xmm13);
    ww!(xmm14);
    ww!(xmm15);
    ww!(xmm16);
    ww!(xmm17);
    ww!(xmm18);
    ww!(xmm19);
    ww!(xmm20);
    ww!(xmm21);
    ww!(xmm22);
    ww!(xmm23);
    ww!(xmm24);
    ww!(xmm25);
    ww!(xmm26);
    ww!(xmm27);
    ww!(xmm28);
    ww!(xmm29);
    ww!(xmm30);
    ww!(xmm31);

    ww!(ymm0);
    ww!(ymm1);
    ww!(ymm2);
    ww!(ymm3);
    ww!(ymm4);
    ww!(ymm5);
    ww!(ymm6);
    ww!(ymm7);
    ww!(ymm8);
    ww!(ymm9);
    ww!(ymm10);
    ww!(ymm11);
    ww!(ymm12);
    ww!(ymm13);
    ww!(ymm14);
    ww!(ymm15);
    ww!(ymm16);
    ww!(ymm17);
    ww!(ymm18);
    ww!(ymm19);
    ww!(ymm20);
    ww!(ymm21);
    ww!(ymm22);
    ww!(ymm23);
    ww!(ymm24);
    ww!(ymm25);
    ww!(ymm26);
    ww!(ymm27);
    ww!(ymm28);
    ww!(ymm29);
    ww!(ymm30);
    ww!(ymm31);

    ww!(zmm0);
    ww!(zmm1);
    ww!(zmm2);
    ww!(zmm3);
    ww!(zmm4);
    ww!(zmm5);
    ww!(zmm6);
    ww!(zmm7);
    ww!(zmm8);
    ww!(zmm9);
    ww!(zmm10);
    ww!(zmm11);
    ww!(zmm12);
    ww!(zmm13);
    ww!(zmm14);
    ww!(zmm15);
    ww!(zmm16);
    ww!(zmm17);
    ww!(zmm18);
    ww!(zmm19);
    ww!(zmm20);
    ww!(zmm21);
    ww!(zmm22);
    ww!(zmm23);
    ww!(zmm24);
    ww!(zmm25);
    ww!(zmm26);
    ww!(zmm27);
    ww!(zmm28);
    ww!(zmm29);
    ww!(zmm30);
    ww!(zmm31);

    ww!(cr0);
    ww!(cr1);
    ww!(cr2);
    ww!(cr3);
    ww!(cr4);
    ww!(cr5);
    ww!(cr6);
    ww!(cr7);
    ww!(cr8);
    ww!(cr9);
    ww!(cr10);
    ww!(cr11);
    ww!(cr12);
    ww!(cr13);
    ww!(cr14);
    ww!(cr15);

    ww!(dr0);
    ww!(dr1);
    ww!(dr2);
    ww!(dr3);
    ww!(dr4);
    ww!(dr5);
    ww!(dr6);
    ww!(dr7);
    ww!(dr8);
    ww!(dr9);
    ww!(dr10);
    ww!(dr11);
    ww!(dr12);
    ww!(dr13);
    ww!(dr14);
    ww!(dr15);

    ww!(tmp8);
    ww!(tmp16);
    ww!(tmp32);
    ww!(tmp64);
    ww!(tmp128);
    ww!(tmp256);
    ww!(tmp512);

    ww!(tmp2_8);
    ww!(tmp2_16);
    ww!(tmp2_32);
    ww!(tmp2_64);
    ww!(tmp2_128);
    ww!(tmp2_256);
    ww!(tmp2_512);

    ww!(tmp3_8);
    ww!(tmp3_16);
    ww!(tmp3_32);
    ww!(tmp3_64);
    ww!(tmp3_128);
    ww!(tmp3_256);
    ww!(tmp3_512);

    ww!(tmp4_8);
    ww!(tmp4_16);
    ww!(tmp4_32);
    ww!(tmp4_64);
    ww!(tmp4_128);
    ww!(tmp4_256);
    ww!(tmp4_512);

    unreachable!()
}
