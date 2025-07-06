use crate::ir::low_ir::{Register, VirtualMachine, x86_64::X64Range};

macro_rules! generate_range {
    ($name:ident, $block:literal, $from:literal, $to:literal) => {
        #[inline(always)]
        fn $name() -> Register {
            Register::new(
                stringify!($name),
                (($block * 64 + $from)..($block * 64 + $to)),
            )
        }
    };
}

impl X64Range for VirtualMachine {
    generate_range!(rax, 0, 0, 64);
    generate_range!(eax, 0, 0, 32);
    generate_range!(ax, 0, 0, 16);
    generate_range!(al, 0, 0, 8);
    generate_range!(ah, 0, 8, 16);

    generate_range!(rbx, 1, 0, 64);
    generate_range!(ebx, 1, 0, 32);
    generate_range!(bx, 1, 0, 16);
    generate_range!(bl, 1, 0, 8);
    generate_range!(bh, 1, 8, 16);

    generate_range!(rcx, 2, 0, 64);
    generate_range!(ecx, 2, 0, 32);
    generate_range!(cx, 2, 0, 16);
    generate_range!(cl, 2, 0, 8);
    generate_range!(ch, 2, 8, 16);

    generate_range!(rdx, 3, 0, 64);
    generate_range!(edx, 3, 0, 32);
    generate_range!(dx, 3, 0, 16);
    generate_range!(dl, 3, 0, 8);
    generate_range!(dh, 3, 8, 16);

    generate_range!(rsp, 4, 0, 64);
    generate_range!(esp, 4, 0, 32);
    generate_range!(sp, 4, 0, 16);
    generate_range!(spl, 4, 0, 8);

    generate_range!(rbp, 5, 0, 64);
    generate_range!(ebp, 5, 0, 32);
    generate_range!(bp, 5, 0, 16);
    generate_range!(bpl, 5, 0, 8);

    generate_range!(rsi, 6, 0, 64);
    generate_range!(esi, 6, 0, 32);
    generate_range!(si, 6, 0, 16);
    generate_range!(sil, 6, 0, 8);

    generate_range!(rdi, 7, 0, 64);
    generate_range!(edi, 7, 0, 32);
    generate_range!(di, 7, 0, 16);
    generate_range!(dil, 7, 0, 8);

    generate_range!(r8, 8, 0, 64);
    generate_range!(r8d, 8, 0, 32);
    generate_range!(r8w, 8, 0, 16);
    generate_range!(r8b, 8, 0, 8);

    generate_range!(r9, 9, 0, 64);
    generate_range!(r9d, 9, 0, 32);
    generate_range!(r9w, 9, 0, 16);
    generate_range!(r9b, 9, 0, 8);

    generate_range!(r10, 10, 0, 64);
    generate_range!(r10d, 10, 0, 32);
    generate_range!(r10w, 10, 0, 16);
    generate_range!(r10b, 10, 0, 8);

    generate_range!(r11, 11, 0, 64);
    generate_range!(r11d, 11, 0, 32);
    generate_range!(r11w, 11, 0, 16);
    generate_range!(r11b, 11, 0, 8);

    generate_range!(r12, 12, 0, 64);
    generate_range!(r12d, 12, 0, 32);
    generate_range!(r12w, 12, 0, 16);
    generate_range!(r12b, 12, 0, 8);

    generate_range!(r13, 13, 0, 64);
    generate_range!(r13d, 13, 0, 32);
    generate_range!(r13w, 13, 0, 16);
    generate_range!(r13b, 13, 0, 8);

    generate_range!(r14, 14, 0, 64);
    generate_range!(r14d, 14, 0, 32);
    generate_range!(r14w, 14, 0, 16);
    generate_range!(r14b, 14, 0, 8);

    generate_range!(r15, 15, 0, 64);
    generate_range!(r15d, 15, 0, 32);
    generate_range!(r15w, 15, 0, 16);
    generate_range!(r15b, 15, 0, 8);

    generate_range!(cs, 16, 0, 16);
    generate_range!(ds, 16, 16, 32);
    generate_range!(es, 16, 32, 48);
    generate_range!(fs, 16, 48, 64);
    generate_range!(gs, 17, 0, 16);
    generate_range!(ss, 17, 16, 32);

    generate_range!(rip, 18, 0, 64);
    generate_range!(eip, 18, 0, 32);
    generate_range!(ip, 18, 0, 16);

    generate_range!(rflags, 19, 0, 64);
    generate_range!(eflags, 19, 0, 32);
    generate_range!(flags, 19, 0, 16);
    generate_range!(cf, 19, 0, 1);
    generate_range!(pf, 19, 2, 3);
    generate_range!(af, 19, 4, 5);
    generate_range!(zf, 19, 6, 7);
    generate_range!(sf, 19, 7, 8);
    generate_range!(tf, 19, 8, 9);
    generate_range!(r#if, 19, 9, 10);
    generate_range!(df, 19, 10, 11);
    generate_range!(of, 19, 11, 12);
    generate_range!(iopl, 19, 12, 14);
    generate_range!(nt, 19, 14, 15);
    generate_range!(rf, 19, 16, 17);
    generate_range!(vm, 19, 17, 18);
    generate_range!(ac, 19, 18, 19);
    generate_range!(vif, 19, 19, 20);
    generate_range!(vip, 19, 20, 21);
    generate_range!(id, 19, 21, 22);

    /* 20 is blank */

    generate_range!(fpu_status_word, 21, 0, 16);
    generate_range!(fpu_ie, 21, 0, 1);
    generate_range!(fpu_de, 21, 1, 2);
    generate_range!(fpu_ze, 21, 2, 3);
    generate_range!(fpu_oe, 21, 3, 4);
    generate_range!(fpu_ue, 21, 4, 5);
    generate_range!(fpu_pe, 21, 5, 6);
    generate_range!(fpu_sf, 21, 6, 7);
    generate_range!(fpu_es, 21, 7, 8);
    generate_range!(fpu_c0, 21, 8, 9);
    generate_range!(fpu_c1, 21, 9, 10);
    generate_range!(fpu_c2, 21, 10, 11);
    generate_range!(fpu_top, 21, 11, 14);
    generate_range!(fpu_c3, 21, 14, 15);
    generate_range!(fpu_b, 21, 15, 16);

    generate_range!(st0, 22, 0, 80);
    generate_range!(st1, 22, 80, 160);
    generate_range!(st2, 22, 160, 240);
    generate_range!(st3, 22, 240, 320);
    generate_range!(st4, 22, 320, 400);
    generate_range!(st5, 22, 400, 480);
    generate_range!(st6, 22, 480, 560);
    generate_range!(st7, 22, 560, 640);

    generate_range!(mm0, 32, 0, 64);
    generate_range!(mm1, 36, 0, 64);
    generate_range!(mm2, 40, 0, 64);
    generate_range!(mm3, 44, 0, 64);
    generate_range!(mm4, 48, 0, 64);
    generate_range!(mm5, 52, 0, 64);
    generate_range!(mm6, 56, 0, 64);
    generate_range!(mm7, 60, 0, 64);
    generate_range!(mm8, 64, 0, 64);
    generate_range!(mm9, 68, 0, 64);
    generate_range!(mm10, 72, 0, 64);
    generate_range!(mm11, 76, 0, 64);
    generate_range!(mm12, 80, 0, 64);
    generate_range!(mm13, 84, 0, 64);
    generate_range!(mm14, 88, 0, 64);
    generate_range!(mm15, 92, 0, 64);
    generate_range!(mm16, 96, 0, 64);
    generate_range!(mm17, 100, 0, 64);
    generate_range!(mm18, 104, 0, 64);
    generate_range!(mm19, 108, 0, 64);
    generate_range!(mm20, 112, 0, 64);
    generate_range!(mm21, 116, 0, 64);
    generate_range!(mm22, 120, 0, 64);
    generate_range!(mm23, 124, 0, 64);
    generate_range!(mm24, 128, 0, 64);
    generate_range!(mm25, 132, 0, 64);
    generate_range!(mm26, 136, 0, 64);
    generate_range!(mm27, 140, 0, 64);
    generate_range!(mm28, 144, 0, 64);
    generate_range!(mm29, 148, 0, 64);
    generate_range!(mm30, 152, 0, 64);
    generate_range!(mm31, 156, 0, 64);

    generate_range!(xmm0, 32, 0, 128);
    generate_range!(xmm1, 36, 0, 128);
    generate_range!(xmm2, 40, 0, 128);
    generate_range!(xmm3, 44, 0, 128);
    generate_range!(xmm4, 48, 0, 128);
    generate_range!(xmm5, 52, 0, 128);
    generate_range!(xmm6, 56, 0, 128);
    generate_range!(xmm7, 60, 0, 128);
    generate_range!(xmm8, 64, 0, 128);
    generate_range!(xmm9, 68, 0, 128);
    generate_range!(xmm10, 72, 0, 128);
    generate_range!(xmm11, 76, 0, 128);
    generate_range!(xmm12, 80, 0, 128);
    generate_range!(xmm13, 84, 0, 128);
    generate_range!(xmm14, 88, 0, 128);
    generate_range!(xmm15, 92, 0, 128);
    generate_range!(xmm16, 96, 0, 128);
    generate_range!(xmm17, 100, 0, 128);
    generate_range!(xmm18, 104, 0, 128);
    generate_range!(xmm19, 108, 0, 128);
    generate_range!(xmm20, 112, 0, 128);
    generate_range!(xmm21, 116, 0, 128);
    generate_range!(xmm22, 120, 0, 128);
    generate_range!(xmm23, 124, 0, 128);
    generate_range!(xmm24, 128, 0, 128);
    generate_range!(xmm25, 132, 0, 128);
    generate_range!(xmm26, 136, 0, 128);
    generate_range!(xmm27, 140, 0, 128);
    generate_range!(xmm28, 144, 0, 128);
    generate_range!(xmm29, 148, 0, 128);
    generate_range!(xmm30, 152, 0, 128);
    generate_range!(xmm31, 156, 0, 128);

    generate_range!(ymm0, 32, 0, 256);
    generate_range!(ymm1, 36, 0, 256);
    generate_range!(ymm2, 40, 0, 256);
    generate_range!(ymm3, 44, 0, 256);
    generate_range!(ymm4, 48, 0, 256);
    generate_range!(ymm5, 52, 0, 256);
    generate_range!(ymm6, 56, 0, 256);
    generate_range!(ymm7, 60, 0, 256);
    generate_range!(ymm8, 64, 0, 256);
    generate_range!(ymm9, 68, 0, 256);
    generate_range!(ymm10, 72, 0, 256);
    generate_range!(ymm11, 76, 0, 256);
    generate_range!(ymm12, 80, 0, 256);
    generate_range!(ymm13, 84, 0, 256);
    generate_range!(ymm14, 88, 0, 256);
    generate_range!(ymm15, 92, 0, 256);
    generate_range!(ymm16, 96, 0, 256);
    generate_range!(ymm17, 100, 0, 256);
    generate_range!(ymm18, 104, 0, 256);
    generate_range!(ymm19, 108, 0, 256);
    generate_range!(ymm20, 112, 0, 256);
    generate_range!(ymm21, 116, 0, 256);
    generate_range!(ymm22, 120, 0, 256);
    generate_range!(ymm23, 124, 0, 256);
    generate_range!(ymm24, 128, 0, 256);
    generate_range!(ymm25, 132, 0, 256);
    generate_range!(ymm26, 136, 0, 256);
    generate_range!(ymm27, 140, 0, 256);
    generate_range!(ymm28, 144, 0, 256);
    generate_range!(ymm29, 148, 0, 256);
    generate_range!(ymm30, 152, 0, 256);
    generate_range!(ymm31, 156, 0, 256);

    generate_range!(zmm0, 32, 0, 512);
    generate_range!(zmm1, 36, 0, 512);
    generate_range!(zmm2, 40, 0, 512);
    generate_range!(zmm3, 44, 0, 512);
    generate_range!(zmm4, 48, 0, 512);
    generate_range!(zmm5, 52, 0, 512);
    generate_range!(zmm6, 56, 0, 512);
    generate_range!(zmm7, 60, 0, 512);
    generate_range!(zmm8, 64, 0, 512);
    generate_range!(zmm9, 68, 0, 512);
    generate_range!(zmm10, 72, 0, 512);
    generate_range!(zmm11, 76, 0, 512);
    generate_range!(zmm12, 80, 0, 512);
    generate_range!(zmm13, 84, 0, 512);
    generate_range!(zmm14, 88, 0, 512);
    generate_range!(zmm15, 92, 0, 512);
    generate_range!(zmm16, 96, 0, 512);
    generate_range!(zmm17, 100, 0, 512);
    generate_range!(zmm18, 104, 0, 512);
    generate_range!(zmm19, 108, 0, 512);
    generate_range!(zmm20, 112, 0, 512);
    generate_range!(zmm21, 116, 0, 512);
    generate_range!(zmm22, 120, 0, 512);
    generate_range!(zmm23, 124, 0, 512);
    generate_range!(zmm24, 128, 0, 512);
    generate_range!(zmm25, 132, 0, 512);
    generate_range!(zmm26, 136, 0, 512);
    generate_range!(zmm27, 140, 0, 512);
    generate_range!(zmm28, 144, 0, 512);
    generate_range!(zmm29, 148, 0, 512);
    generate_range!(zmm30, 152, 0, 512);
    generate_range!(zmm31, 156, 0, 512);

    generate_range!(cr0, 160, 0, 32);
    generate_range!(cr1, 160, 32, 64);
    generate_range!(cr2, 161, 0, 32);
    generate_range!(cr3, 161, 32, 64);
    generate_range!(cr4, 162, 0, 32);
    generate_range!(cr5, 162, 32, 64);
    generate_range!(cr6, 163, 0, 32);
    generate_range!(cr7, 163, 32, 64);
    generate_range!(cr8, 164, 0, 32);
    generate_range!(cr9, 164, 32, 64);
    generate_range!(cr10, 165, 0, 32);
    generate_range!(cr11, 165, 32, 64);
    generate_range!(cr12, 166, 0, 32);
    generate_range!(cr13, 166, 32, 64);
    generate_range!(cr14, 167, 0, 32);
    generate_range!(cr15, 167, 32, 64);

    generate_range!(dr0, 168, 0, 32);
    generate_range!(dr1, 168, 32, 64);
    generate_range!(dr2, 169, 0, 32);
    generate_range!(dr3, 169, 32, 64);
    generate_range!(dr4, 170, 0, 32);
    generate_range!(dr5, 170, 32, 64);
    generate_range!(dr6, 171, 0, 32);
    generate_range!(dr7, 171, 32, 64);
    generate_range!(dr8, 172, 0, 32);
    generate_range!(dr9, 172, 32, 64);
    generate_range!(dr10, 173, 0, 32);
    generate_range!(dr11, 173, 32, 64);
    generate_range!(dr12, 174, 0, 32);
    generate_range!(dr13, 174, 32, 64);
    generate_range!(dr14, 175, 0, 32);
    generate_range!(dr15, 175, 32, 64);

    generate_range!(tmp8, 176, 0, 8);
    generate_range!(tmp16, 176, 0, 16);
    generate_range!(tmp32, 176, 0, 32);
    generate_range!(tmp64, 176, 0, 64);
    generate_range!(tmp128, 176, 0, 128);
    generate_range!(tmp256, 176, 0, 256);
    generate_range!(tmp512, 176, 0, 512);

    generate_range!(tmp2_8, 180, 0, 8);
    generate_range!(tmp2_16, 180, 0, 16);
    generate_range!(tmp2_32, 180, 0, 32);
    generate_range!(tmp2_64, 180, 0, 64);
    generate_range!(tmp2_128, 180, 0, 128);
    generate_range!(tmp2_256, 180, 0, 256);
    generate_range!(tmp2_512, 180, 0, 512);

    generate_range!(tmp3_8, 184, 0, 8);
    generate_range!(tmp3_16, 184, 0, 16);
    generate_range!(tmp3_32, 184, 0, 32);
    generate_range!(tmp3_64, 184, 0, 64);
    generate_range!(tmp3_128, 184, 0, 128);
    generate_range!(tmp3_256, 184, 0, 256);
    generate_range!(tmp3_512, 184, 0, 512);

    generate_range!(tmp4_8, 188, 0, 8);
    generate_range!(tmp4_16, 188, 0, 16);
    generate_range!(tmp4_32, 188, 0, 32);
    generate_range!(tmp4_64, 188, 0, 64);
    generate_range!(tmp4_128, 188, 0, 128);
    generate_range!(tmp4_256, 188, 0, 256);
    generate_range!(tmp4_512, 188, 0, 512);
}
