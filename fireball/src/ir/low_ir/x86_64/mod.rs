//! Submodule for x86_64 architectures.

mod base;
mod r#mut;
mod range;

use crate::ir::low_ir::Register;
use crate::prelude::BitSlice;

macro_rules! generate_register {
    ($name:ident) => {
        fn $name(&self) -> &BitSlice;
    };
}

macro_rules! generate_mutable_register {
    ($name:ident) => {
        fn $name(&mut self) -> &mut BitSlice;
    };
}

macro_rules! generate_range {
    ($name:ident) => {
        fn $name() -> Register;
    };
}

/// Trait for x86_64 CPU registers.
pub trait X64 {
    fn new() -> Self;

    generate_register!(rax);
    generate_register!(eax);
    generate_register!(ax);
    generate_register!(al);
    generate_register!(ah);

    generate_register!(rbx);
    generate_register!(ebx);
    generate_register!(bx);
    generate_register!(bl);
    generate_register!(bh);

    generate_register!(rcx);
    generate_register!(ecx);
    generate_register!(cx);
    generate_register!(cl);
    generate_register!(ch);

    generate_register!(rdx);
    generate_register!(edx);
    generate_register!(dx);
    generate_register!(dl);
    generate_register!(dh);

    generate_register!(rsp);
    generate_register!(esp);
    generate_register!(sp);
    generate_register!(spl);

    generate_register!(rbp);
    generate_register!(ebp);
    generate_register!(bp);
    generate_register!(bpl);

    generate_register!(rsi);
    generate_register!(esi);
    generate_register!(si);
    generate_register!(sil);

    generate_register!(rdi);
    generate_register!(edi);
    generate_register!(di);
    generate_register!(dil);

    generate_register!(r8);
    generate_register!(r8d);
    generate_register!(r8w);
    generate_register!(r8b);

    generate_register!(r9);
    generate_register!(r9d);
    generate_register!(r9w);
    generate_register!(r9b);

    generate_register!(r10);
    generate_register!(r10d);
    generate_register!(r10w);
    generate_register!(r10b);

    generate_register!(r11);
    generate_register!(r11d);
    generate_register!(r11w);
    generate_register!(r11b);

    generate_register!(r12);
    generate_register!(r12d);
    generate_register!(r12w);
    generate_register!(r12b);

    generate_register!(r13);
    generate_register!(r13d);
    generate_register!(r13w);
    generate_register!(r13b);

    generate_register!(r14);
    generate_register!(r14d);
    generate_register!(r14w);
    generate_register!(r14b);

    generate_register!(r15);
    generate_register!(r15d);
    generate_register!(r15w);
    generate_register!(r15b);

    generate_register!(cs);
    generate_register!(ds);
    generate_register!(es);
    generate_register!(fs);
    generate_register!(gs);
    generate_register!(ss);

    generate_register!(rip);
    generate_register!(eip);
    generate_register!(ip);

    generate_register!(rflags);
    generate_register!(eflags);
    generate_register!(flags);
    generate_register!(cf);
    generate_register!(pf);
    generate_register!(af);
    generate_register!(zf);
    generate_register!(sf);
    generate_register!(tf);
    generate_register!(r#if);
    generate_register!(df);
    generate_register!(of);
    generate_register!(iopl);
    generate_register!(nt);
    generate_register!(rf);
    generate_register!(vm);
    generate_register!(ac);
    generate_register!(vif);
    generate_register!(vip);
    generate_register!(id);

    generate_register!(fpu_status_word);
    generate_register!(fpu_ie);
    generate_register!(fpu_de);
    generate_register!(fpu_ze);
    generate_register!(fpu_oe);
    generate_register!(fpu_ue);
    generate_register!(fpu_pe);
    generate_register!(fpu_sf);
    generate_register!(fpu_es);
    generate_register!(fpu_c0);
    generate_register!(fpu_c1);
    generate_register!(fpu_c2);
    generate_register!(fpu_top);
    generate_register!(fpu_c3);
    generate_register!(fpu_b);

    generate_register!(st0);
    generate_register!(st1);
    generate_register!(st2);
    generate_register!(st3);
    generate_register!(st4);
    generate_register!(st5);
    generate_register!(st6);
    generate_register!(st7);

    generate_register!(mm0);
    generate_register!(mm1);
    generate_register!(mm2);
    generate_register!(mm3);
    generate_register!(mm4);
    generate_register!(mm5);
    generate_register!(mm6);
    generate_register!(mm7);
    generate_register!(mm8);
    generate_register!(mm9);
    generate_register!(mm10);
    generate_register!(mm11);
    generate_register!(mm12);
    generate_register!(mm13);
    generate_register!(mm14);
    generate_register!(mm15);
    generate_register!(mm16);
    generate_register!(mm17);
    generate_register!(mm18);
    generate_register!(mm19);
    generate_register!(mm20);
    generate_register!(mm21);
    generate_register!(mm22);
    generate_register!(mm23);
    generate_register!(mm24);
    generate_register!(mm25);
    generate_register!(mm26);
    generate_register!(mm27);
    generate_register!(mm28);
    generate_register!(mm29);
    generate_register!(mm30);
    generate_register!(mm31);

    generate_register!(xmm0);
    generate_register!(xmm1);
    generate_register!(xmm2);
    generate_register!(xmm3);
    generate_register!(xmm4);
    generate_register!(xmm5);
    generate_register!(xmm6);
    generate_register!(xmm7);
    generate_register!(xmm8);
    generate_register!(xmm9);
    generate_register!(xmm10);
    generate_register!(xmm11);
    generate_register!(xmm12);
    generate_register!(xmm13);
    generate_register!(xmm14);
    generate_register!(xmm15);
    generate_register!(xmm16);
    generate_register!(xmm17);
    generate_register!(xmm18);
    generate_register!(xmm19);
    generate_register!(xmm20);
    generate_register!(xmm21);
    generate_register!(xmm22);
    generate_register!(xmm23);
    generate_register!(xmm24);
    generate_register!(xmm25);
    generate_register!(xmm26);
    generate_register!(xmm27);
    generate_register!(xmm28);
    generate_register!(xmm29);
    generate_register!(xmm30);
    generate_register!(xmm31);

    generate_register!(ymm0);
    generate_register!(ymm1);
    generate_register!(ymm2);
    generate_register!(ymm3);
    generate_register!(ymm4);
    generate_register!(ymm5);
    generate_register!(ymm6);
    generate_register!(ymm7);
    generate_register!(ymm8);
    generate_register!(ymm9);
    generate_register!(ymm10);
    generate_register!(ymm11);
    generate_register!(ymm12);
    generate_register!(ymm13);
    generate_register!(ymm14);
    generate_register!(ymm15);
    generate_register!(ymm16);
    generate_register!(ymm17);
    generate_register!(ymm18);
    generate_register!(ymm19);
    generate_register!(ymm20);
    generate_register!(ymm21);
    generate_register!(ymm22);
    generate_register!(ymm23);
    generate_register!(ymm24);
    generate_register!(ymm25);
    generate_register!(ymm26);
    generate_register!(ymm27);
    generate_register!(ymm28);
    generate_register!(ymm29);
    generate_register!(ymm30);
    generate_register!(ymm31);

    generate_register!(zmm0);
    generate_register!(zmm1);
    generate_register!(zmm2);
    generate_register!(zmm3);
    generate_register!(zmm4);
    generate_register!(zmm5);
    generate_register!(zmm6);
    generate_register!(zmm7);
    generate_register!(zmm8);
    generate_register!(zmm9);
    generate_register!(zmm10);
    generate_register!(zmm11);
    generate_register!(zmm12);
    generate_register!(zmm13);
    generate_register!(zmm14);
    generate_register!(zmm15);
    generate_register!(zmm16);
    generate_register!(zmm17);
    generate_register!(zmm18);
    generate_register!(zmm19);
    generate_register!(zmm20);
    generate_register!(zmm21);
    generate_register!(zmm22);
    generate_register!(zmm23);
    generate_register!(zmm24);
    generate_register!(zmm25);
    generate_register!(zmm26);
    generate_register!(zmm27);
    generate_register!(zmm28);
    generate_register!(zmm29);
    generate_register!(zmm30);
    generate_register!(zmm31);

    generate_register!(cr0);
    generate_register!(cr1);
    generate_register!(cr2);
    generate_register!(cr3);
    generate_register!(cr4);
    generate_register!(cr5);
    generate_register!(cr6);
    generate_register!(cr7);
    generate_register!(cr8);
    generate_register!(cr9);
    generate_register!(cr10);
    generate_register!(cr11);
    generate_register!(cr12);
    generate_register!(cr13);
    generate_register!(cr14);
    generate_register!(cr15);

    generate_register!(dr0);
    generate_register!(dr1);
    generate_register!(dr2);
    generate_register!(dr3);
    generate_register!(dr4);
    generate_register!(dr5);
    generate_register!(dr6);
    generate_register!(dr7);
    generate_register!(dr8);
    generate_register!(dr9);
    generate_register!(dr10);
    generate_register!(dr11);
    generate_register!(dr12);
    generate_register!(dr13);
    generate_register!(dr14);
    generate_register!(dr15);

    generate_register!(tmp8);
    generate_register!(tmp16);
    generate_register!(tmp32);
    generate_register!(tmp64);
    generate_register!(tmp128);
    generate_register!(tmp256);
    generate_register!(tmp512);

    generate_register!(tmp2_8);
    generate_register!(tmp2_16);
    generate_register!(tmp2_32);
    generate_register!(tmp2_64);
    generate_register!(tmp2_128);
    generate_register!(tmp2_256);
    generate_register!(tmp2_512);

    generate_register!(tmp3_8);
    generate_register!(tmp3_16);
    generate_register!(tmp3_32);
    generate_register!(tmp3_64);
    generate_register!(tmp3_128);
    generate_register!(tmp3_256);
    generate_register!(tmp3_512);

    generate_register!(tmp4_8);
    generate_register!(tmp4_16);
    generate_register!(tmp4_32);
    generate_register!(tmp4_64);
    generate_register!(tmp4_128);
    generate_register!(tmp4_256);
    generate_register!(tmp4_512);
}

/// Trait for mutable x86_64 CPU registers.
pub trait X64Mut {
    generate_mutable_register!(rax);
    generate_mutable_register!(eax);
    generate_mutable_register!(ax);
    generate_mutable_register!(al);
    generate_mutable_register!(ah);

    generate_mutable_register!(rbx);
    generate_mutable_register!(ebx);
    generate_mutable_register!(bx);
    generate_mutable_register!(bl);
    generate_mutable_register!(bh);

    generate_mutable_register!(rcx);
    generate_mutable_register!(ecx);
    generate_mutable_register!(cx);
    generate_mutable_register!(cl);
    generate_mutable_register!(ch);

    generate_mutable_register!(rdx);
    generate_mutable_register!(edx);
    generate_mutable_register!(dx);
    generate_mutable_register!(dl);
    generate_mutable_register!(dh);

    generate_mutable_register!(rsp);
    generate_mutable_register!(esp);
    generate_mutable_register!(sp);
    generate_mutable_register!(spl);

    generate_mutable_register!(rbp);
    generate_mutable_register!(ebp);
    generate_mutable_register!(bp);
    generate_mutable_register!(bpl);

    generate_mutable_register!(rsi);
    generate_mutable_register!(esi);
    generate_mutable_register!(si);
    generate_mutable_register!(sil);

    generate_mutable_register!(rdi);
    generate_mutable_register!(edi);
    generate_mutable_register!(di);
    generate_mutable_register!(dil);

    generate_mutable_register!(r8);
    generate_mutable_register!(r8d);
    generate_mutable_register!(r8w);
    generate_mutable_register!(r8b);

    generate_mutable_register!(r9);
    generate_mutable_register!(r9d);
    generate_mutable_register!(r9w);
    generate_mutable_register!(r9b);

    generate_mutable_register!(r10);
    generate_mutable_register!(r10d);
    generate_mutable_register!(r10w);
    generate_mutable_register!(r10b);

    generate_mutable_register!(r11);
    generate_mutable_register!(r11d);
    generate_mutable_register!(r11w);
    generate_mutable_register!(r11b);

    generate_mutable_register!(r12);
    generate_mutable_register!(r12d);
    generate_mutable_register!(r12w);
    generate_mutable_register!(r12b);

    generate_mutable_register!(r13);
    generate_mutable_register!(r13d);
    generate_mutable_register!(r13w);
    generate_mutable_register!(r13b);

    generate_mutable_register!(r14);
    generate_mutable_register!(r14d);
    generate_mutable_register!(r14w);
    generate_mutable_register!(r14b);

    generate_mutable_register!(r15);
    generate_mutable_register!(r15d);
    generate_mutable_register!(r15w);
    generate_mutable_register!(r15b);

    generate_mutable_register!(cs);
    generate_mutable_register!(ds);
    generate_mutable_register!(es);
    generate_mutable_register!(fs);
    generate_mutable_register!(gs);
    generate_mutable_register!(ss);

    generate_mutable_register!(rip);
    generate_mutable_register!(eip);
    generate_mutable_register!(ip);

    generate_mutable_register!(rflags);
    generate_mutable_register!(eflags);
    generate_mutable_register!(flags);
    generate_mutable_register!(cf);
    generate_mutable_register!(pf);
    generate_mutable_register!(af);
    generate_mutable_register!(zf);
    generate_mutable_register!(sf);
    generate_mutable_register!(tf);
    generate_mutable_register!(r#if);
    generate_mutable_register!(df);
    generate_mutable_register!(of);
    generate_mutable_register!(iopl);
    generate_mutable_register!(nt);
    generate_mutable_register!(rf);
    generate_mutable_register!(vm);
    generate_mutable_register!(ac);
    generate_mutable_register!(vif);
    generate_mutable_register!(vip);
    generate_mutable_register!(id);

    generate_mutable_register!(fpu_status_word);
    generate_mutable_register!(fpu_ie);
    generate_mutable_register!(fpu_de);
    generate_mutable_register!(fpu_ze);
    generate_mutable_register!(fpu_oe);
    generate_mutable_register!(fpu_ue);
    generate_mutable_register!(fpu_pe);
    generate_mutable_register!(fpu_sf);
    generate_mutable_register!(fpu_es);
    generate_mutable_register!(fpu_c0);
    generate_mutable_register!(fpu_c1);
    generate_mutable_register!(fpu_c2);
    generate_mutable_register!(fpu_top);
    generate_mutable_register!(fpu_c3);
    generate_mutable_register!(fpu_b);

    generate_mutable_register!(st0);
    generate_mutable_register!(st1);
    generate_mutable_register!(st2);
    generate_mutable_register!(st3);
    generate_mutable_register!(st4);
    generate_mutable_register!(st5);
    generate_mutable_register!(st6);
    generate_mutable_register!(st7);

    generate_mutable_register!(mm0);
    generate_mutable_register!(mm1);
    generate_mutable_register!(mm2);
    generate_mutable_register!(mm3);
    generate_mutable_register!(mm4);
    generate_mutable_register!(mm5);
    generate_mutable_register!(mm6);
    generate_mutable_register!(mm7);
    generate_mutable_register!(mm8);
    generate_mutable_register!(mm9);
    generate_mutable_register!(mm10);
    generate_mutable_register!(mm11);
    generate_mutable_register!(mm12);
    generate_mutable_register!(mm13);
    generate_mutable_register!(mm14);
    generate_mutable_register!(mm15);
    generate_mutable_register!(mm16);
    generate_mutable_register!(mm17);
    generate_mutable_register!(mm18);
    generate_mutable_register!(mm19);
    generate_mutable_register!(mm20);
    generate_mutable_register!(mm21);
    generate_mutable_register!(mm22);
    generate_mutable_register!(mm23);
    generate_mutable_register!(mm24);
    generate_mutable_register!(mm25);
    generate_mutable_register!(mm26);
    generate_mutable_register!(mm27);
    generate_mutable_register!(mm28);
    generate_mutable_register!(mm29);
    generate_mutable_register!(mm30);
    generate_mutable_register!(mm31);

    generate_mutable_register!(xmm0);
    generate_mutable_register!(xmm1);
    generate_mutable_register!(xmm2);
    generate_mutable_register!(xmm3);
    generate_mutable_register!(xmm4);
    generate_mutable_register!(xmm5);
    generate_mutable_register!(xmm6);
    generate_mutable_register!(xmm7);
    generate_mutable_register!(xmm8);
    generate_mutable_register!(xmm9);
    generate_mutable_register!(xmm10);
    generate_mutable_register!(xmm11);
    generate_mutable_register!(xmm12);
    generate_mutable_register!(xmm13);
    generate_mutable_register!(xmm14);
    generate_mutable_register!(xmm15);
    generate_mutable_register!(xmm16);
    generate_mutable_register!(xmm17);
    generate_mutable_register!(xmm18);
    generate_mutable_register!(xmm19);
    generate_mutable_register!(xmm20);
    generate_mutable_register!(xmm21);
    generate_mutable_register!(xmm22);
    generate_mutable_register!(xmm23);
    generate_mutable_register!(xmm24);
    generate_mutable_register!(xmm25);
    generate_mutable_register!(xmm26);
    generate_mutable_register!(xmm27);
    generate_mutable_register!(xmm28);
    generate_mutable_register!(xmm29);
    generate_mutable_register!(xmm30);
    generate_mutable_register!(xmm31);

    generate_mutable_register!(ymm0);
    generate_mutable_register!(ymm1);
    generate_mutable_register!(ymm2);
    generate_mutable_register!(ymm3);
    generate_mutable_register!(ymm4);
    generate_mutable_register!(ymm5);
    generate_mutable_register!(ymm6);
    generate_mutable_register!(ymm7);
    generate_mutable_register!(ymm8);
    generate_mutable_register!(ymm9);
    generate_mutable_register!(ymm10);
    generate_mutable_register!(ymm11);
    generate_mutable_register!(ymm12);
    generate_mutable_register!(ymm13);
    generate_mutable_register!(ymm14);
    generate_mutable_register!(ymm15);
    generate_mutable_register!(ymm16);
    generate_mutable_register!(ymm17);
    generate_mutable_register!(ymm18);
    generate_mutable_register!(ymm19);
    generate_mutable_register!(ymm20);
    generate_mutable_register!(ymm21);
    generate_mutable_register!(ymm22);
    generate_mutable_register!(ymm23);
    generate_mutable_register!(ymm24);
    generate_mutable_register!(ymm25);
    generate_mutable_register!(ymm26);
    generate_mutable_register!(ymm27);
    generate_mutable_register!(ymm28);
    generate_mutable_register!(ymm29);
    generate_mutable_register!(ymm30);
    generate_mutable_register!(ymm31);

    generate_mutable_register!(zmm0);
    generate_mutable_register!(zmm1);
    generate_mutable_register!(zmm2);
    generate_mutable_register!(zmm3);
    generate_mutable_register!(zmm4);
    generate_mutable_register!(zmm5);
    generate_mutable_register!(zmm6);
    generate_mutable_register!(zmm7);
    generate_mutable_register!(zmm8);
    generate_mutable_register!(zmm9);
    generate_mutable_register!(zmm10);
    generate_mutable_register!(zmm11);
    generate_mutable_register!(zmm12);
    generate_mutable_register!(zmm13);
    generate_mutable_register!(zmm14);
    generate_mutable_register!(zmm15);
    generate_mutable_register!(zmm16);
    generate_mutable_register!(zmm17);
    generate_mutable_register!(zmm18);
    generate_mutable_register!(zmm19);
    generate_mutable_register!(zmm20);
    generate_mutable_register!(zmm21);
    generate_mutable_register!(zmm22);
    generate_mutable_register!(zmm23);
    generate_mutable_register!(zmm24);
    generate_mutable_register!(zmm25);
    generate_mutable_register!(zmm26);
    generate_mutable_register!(zmm27);
    generate_mutable_register!(zmm28);
    generate_mutable_register!(zmm29);
    generate_mutable_register!(zmm30);
    generate_mutable_register!(zmm31);

    generate_mutable_register!(cr0);
    generate_mutable_register!(cr1);
    generate_mutable_register!(cr2);
    generate_mutable_register!(cr3);
    generate_mutable_register!(cr4);
    generate_mutable_register!(cr5);
    generate_mutable_register!(cr6);
    generate_mutable_register!(cr7);
    generate_mutable_register!(cr8);
    generate_mutable_register!(cr9);
    generate_mutable_register!(cr10);
    generate_mutable_register!(cr11);
    generate_mutable_register!(cr12);
    generate_mutable_register!(cr13);
    generate_mutable_register!(cr14);
    generate_mutable_register!(cr15);

    generate_mutable_register!(dr0);
    generate_mutable_register!(dr1);
    generate_mutable_register!(dr2);
    generate_mutable_register!(dr3);
    generate_mutable_register!(dr4);
    generate_mutable_register!(dr5);
    generate_mutable_register!(dr6);
    generate_mutable_register!(dr7);
    generate_mutable_register!(dr8);
    generate_mutable_register!(dr9);
    generate_mutable_register!(dr10);
    generate_mutable_register!(dr11);
    generate_mutable_register!(dr12);
    generate_mutable_register!(dr13);
    generate_mutable_register!(dr14);
    generate_mutable_register!(dr15);

    generate_mutable_register!(tmp8);
    generate_mutable_register!(tmp16);
    generate_mutable_register!(tmp32);
    generate_mutable_register!(tmp64);
    generate_mutable_register!(tmp128);
    generate_mutable_register!(tmp256);
    generate_mutable_register!(tmp512);

    generate_mutable_register!(tmp2_8);
    generate_mutable_register!(tmp2_16);
    generate_mutable_register!(tmp2_32);
    generate_mutable_register!(tmp2_64);
    generate_mutable_register!(tmp2_128);
    generate_mutable_register!(tmp2_256);
    generate_mutable_register!(tmp2_512);

    generate_mutable_register!(tmp3_8);
    generate_mutable_register!(tmp3_16);
    generate_mutable_register!(tmp3_32);
    generate_mutable_register!(tmp3_64);
    generate_mutable_register!(tmp3_128);
    generate_mutable_register!(tmp3_256);
    generate_mutable_register!(tmp3_512);

    generate_mutable_register!(tmp4_8);
    generate_mutable_register!(tmp4_16);
    generate_mutable_register!(tmp4_32);
    generate_mutable_register!(tmp4_64);
    generate_mutable_register!(tmp4_128);
    generate_mutable_register!(tmp4_256);
    generate_mutable_register!(tmp4_512);
}

pub(crate) trait X64Range {
    generate_range!(rax);
    generate_range!(eax);
    generate_range!(ax);
    generate_range!(al);
    generate_range!(ah);

    generate_range!(rbx);
    generate_range!(ebx);
    generate_range!(bx);
    generate_range!(bl);
    generate_range!(bh);

    generate_range!(rcx);
    generate_range!(ecx);
    generate_range!(cx);
    generate_range!(cl);
    generate_range!(ch);

    generate_range!(rdx);
    generate_range!(edx);
    generate_range!(dx);
    generate_range!(dl);
    generate_range!(dh);

    generate_range!(rsp);
    generate_range!(esp);
    generate_range!(sp);
    generate_range!(spl);

    generate_range!(rbp);
    generate_range!(ebp);
    generate_range!(bp);
    generate_range!(bpl);

    generate_range!(rsi);
    generate_range!(esi);
    generate_range!(si);
    generate_range!(sil);

    generate_range!(rdi);
    generate_range!(edi);
    generate_range!(di);
    generate_range!(dil);

    generate_range!(r8);
    generate_range!(r8d);
    generate_range!(r8w);
    generate_range!(r8b);

    generate_range!(r9);
    generate_range!(r9d);
    generate_range!(r9w);
    generate_range!(r9b);

    generate_range!(r10);
    generate_range!(r10d);
    generate_range!(r10w);
    generate_range!(r10b);

    generate_range!(r11);
    generate_range!(r11d);
    generate_range!(r11w);
    generate_range!(r11b);

    generate_range!(r12);
    generate_range!(r12d);
    generate_range!(r12w);
    generate_range!(r12b);

    generate_range!(r13);
    generate_range!(r13d);
    generate_range!(r13w);
    generate_range!(r13b);

    generate_range!(r14);
    generate_range!(r14d);
    generate_range!(r14w);
    generate_range!(r14b);

    generate_range!(r15);
    generate_range!(r15d);
    generate_range!(r15w);
    generate_range!(r15b);

    generate_range!(cs);
    generate_range!(ds);
    generate_range!(es);
    generate_range!(fs);
    generate_range!(gs);
    generate_range!(ss);

    generate_range!(rip);
    generate_range!(eip);
    generate_range!(ip);

    generate_range!(rflags);
    generate_range!(eflags);
    generate_range!(flags);
    generate_range!(cf);
    generate_range!(pf);
    generate_range!(af);
    generate_range!(zf);
    generate_range!(sf);
    generate_range!(tf);
    generate_range!(r#if);
    generate_range!(df);
    generate_range!(of);
    generate_range!(iopl);
    generate_range!(nt);
    generate_range!(rf);
    generate_range!(vm);
    generate_range!(ac);
    generate_range!(vif);
    generate_range!(vip);
    generate_range!(id);

    generate_range!(fpu_status_word);
    generate_range!(fpu_ie);
    generate_range!(fpu_de);
    generate_range!(fpu_ze);
    generate_range!(fpu_oe);
    generate_range!(fpu_ue);
    generate_range!(fpu_pe);
    generate_range!(fpu_sf);
    generate_range!(fpu_es);
    generate_range!(fpu_c0);
    generate_range!(fpu_c1);
    generate_range!(fpu_c2);
    generate_range!(fpu_top);
    generate_range!(fpu_c3);
    generate_range!(fpu_b);

    generate_range!(st0);
    generate_range!(st1);
    generate_range!(st2);
    generate_range!(st3);
    generate_range!(st4);
    generate_range!(st5);
    generate_range!(st6);
    generate_range!(st7);

    generate_range!(mm0);
    generate_range!(mm1);
    generate_range!(mm2);
    generate_range!(mm3);
    generate_range!(mm4);
    generate_range!(mm5);
    generate_range!(mm6);
    generate_range!(mm7);
    generate_range!(mm8);
    generate_range!(mm9);
    generate_range!(mm10);
    generate_range!(mm11);
    generate_range!(mm12);
    generate_range!(mm13);
    generate_range!(mm14);
    generate_range!(mm15);
    generate_range!(mm16);
    generate_range!(mm17);
    generate_range!(mm18);
    generate_range!(mm19);
    generate_range!(mm20);
    generate_range!(mm21);
    generate_range!(mm22);
    generate_range!(mm23);
    generate_range!(mm24);
    generate_range!(mm25);
    generate_range!(mm26);
    generate_range!(mm27);
    generate_range!(mm28);
    generate_range!(mm29);
    generate_range!(mm30);
    generate_range!(mm31);

    generate_range!(xmm0);
    generate_range!(xmm1);
    generate_range!(xmm2);
    generate_range!(xmm3);
    generate_range!(xmm4);
    generate_range!(xmm5);
    generate_range!(xmm6);
    generate_range!(xmm7);
    generate_range!(xmm8);
    generate_range!(xmm9);
    generate_range!(xmm10);
    generate_range!(xmm11);
    generate_range!(xmm12);
    generate_range!(xmm13);
    generate_range!(xmm14);
    generate_range!(xmm15);
    generate_range!(xmm16);
    generate_range!(xmm17);
    generate_range!(xmm18);
    generate_range!(xmm19);
    generate_range!(xmm20);
    generate_range!(xmm21);
    generate_range!(xmm22);
    generate_range!(xmm23);
    generate_range!(xmm24);
    generate_range!(xmm25);
    generate_range!(xmm26);
    generate_range!(xmm27);
    generate_range!(xmm28);
    generate_range!(xmm29);
    generate_range!(xmm30);
    generate_range!(xmm31);

    generate_range!(ymm0);
    generate_range!(ymm1);
    generate_range!(ymm2);
    generate_range!(ymm3);
    generate_range!(ymm4);
    generate_range!(ymm5);
    generate_range!(ymm6);
    generate_range!(ymm7);
    generate_range!(ymm8);
    generate_range!(ymm9);
    generate_range!(ymm10);
    generate_range!(ymm11);
    generate_range!(ymm12);
    generate_range!(ymm13);
    generate_range!(ymm14);
    generate_range!(ymm15);
    generate_range!(ymm16);
    generate_range!(ymm17);
    generate_range!(ymm18);
    generate_range!(ymm19);
    generate_range!(ymm20);
    generate_range!(ymm21);
    generate_range!(ymm22);
    generate_range!(ymm23);
    generate_range!(ymm24);
    generate_range!(ymm25);
    generate_range!(ymm26);
    generate_range!(ymm27);
    generate_range!(ymm28);
    generate_range!(ymm29);
    generate_range!(ymm30);
    generate_range!(ymm31);

    generate_range!(zmm0);
    generate_range!(zmm1);
    generate_range!(zmm2);
    generate_range!(zmm3);
    generate_range!(zmm4);
    generate_range!(zmm5);
    generate_range!(zmm6);
    generate_range!(zmm7);
    generate_range!(zmm8);
    generate_range!(zmm9);
    generate_range!(zmm10);
    generate_range!(zmm11);
    generate_range!(zmm12);
    generate_range!(zmm13);
    generate_range!(zmm14);
    generate_range!(zmm15);
    generate_range!(zmm16);
    generate_range!(zmm17);
    generate_range!(zmm18);
    generate_range!(zmm19);
    generate_range!(zmm20);
    generate_range!(zmm21);
    generate_range!(zmm22);
    generate_range!(zmm23);
    generate_range!(zmm24);
    generate_range!(zmm25);
    generate_range!(zmm26);
    generate_range!(zmm27);
    generate_range!(zmm28);
    generate_range!(zmm29);
    generate_range!(zmm30);
    generate_range!(zmm31);

    generate_range!(cr0);
    generate_range!(cr1);
    generate_range!(cr2);
    generate_range!(cr3);
    generate_range!(cr4);
    generate_range!(cr5);
    generate_range!(cr6);
    generate_range!(cr7);
    generate_range!(cr8);
    generate_range!(cr9);
    generate_range!(cr10);
    generate_range!(cr11);
    generate_range!(cr12);
    generate_range!(cr13);
    generate_range!(cr14);
    generate_range!(cr15);

    generate_range!(dr0);
    generate_range!(dr1);
    generate_range!(dr2);
    generate_range!(dr3);
    generate_range!(dr4);
    generate_range!(dr5);
    generate_range!(dr6);
    generate_range!(dr7);
    generate_range!(dr8);
    generate_range!(dr9);
    generate_range!(dr10);
    generate_range!(dr11);
    generate_range!(dr12);
    generate_range!(dr13);
    generate_range!(dr14);
    generate_range!(dr15);

    generate_range!(tmp8);
    generate_range!(tmp16);
    generate_range!(tmp32);
    generate_range!(tmp64);
    generate_range!(tmp128);
    generate_range!(tmp256);
    generate_range!(tmp512);

    generate_range!(tmp2_8);
    generate_range!(tmp2_16);
    generate_range!(tmp2_32);
    generate_range!(tmp2_64);
    generate_range!(tmp2_128);
    generate_range!(tmp2_256);
    generate_range!(tmp2_512);

    generate_range!(tmp3_8);
    generate_range!(tmp3_16);
    generate_range!(tmp3_32);
    generate_range!(tmp3_64);
    generate_range!(tmp3_128);
    generate_range!(tmp3_256);
    generate_range!(tmp3_512);

    generate_range!(tmp4_8);
    generate_range!(tmp4_16);
    generate_range!(tmp4_32);
    generate_range!(tmp4_64);
    generate_range!(tmp4_128);
    generate_range!(tmp4_256);
    generate_range!(tmp4_512);
}
