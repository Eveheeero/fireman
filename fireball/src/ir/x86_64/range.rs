use crate::ir::{
    x86_64::{Range, X64Range},
    Ir,
};

macro_rules! generate_range {
    ($name:ident, $block:literal, $from:literal, $to:literal) => {
        #[inline(always)]
        fn $name() -> Range<usize> {
            ($block * 64 + $from)..($block * 64 + $to)
        }
    };
}

impl X64Range for Ir {
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

    generate_range!(less, 20, 0, 1);
    generate_range!(less_or_equal, 20, 1, 2);
    generate_range!(below_or_equal, 20, 2, 3);

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
    generate_range!(mm1, 33, 0, 64);
    generate_range!(mm2, 34, 0, 64);
    generate_range!(mm3, 35, 0, 64);
    generate_range!(mm4, 36, 0, 64);
    generate_range!(mm5, 37, 0, 64);
    generate_range!(mm6, 38, 0, 64);
    generate_range!(mm7, 39, 0, 64);

    generate_range!(xmm0, 40, 0, 128);
    generate_range!(xmm1, 42, 0, 128);
    generate_range!(xmm2, 44, 0, 128);
    generate_range!(xmm3, 46, 0, 128);
    generate_range!(xmm4, 48, 0, 128);
    generate_range!(xmm5, 50, 0, 128);
    generate_range!(xmm6, 52, 0, 128);
    generate_range!(xmm7, 54, 0, 128);
    generate_range!(xmm8, 56, 0, 128);
    generate_range!(xmm9, 58, 0, 128);
    generate_range!(xmm10, 60, 0, 128);
    generate_range!(xmm11, 62, 0, 128);
    generate_range!(xmm12, 64, 0, 128);
    generate_range!(xmm13, 66, 0, 128);
    generate_range!(xmm14, 68, 0, 128);
    generate_range!(xmm15, 70, 0, 128);

    generate_range!(cr0, 72, 0, 32);
    generate_range!(cr1, 72, 32, 64);
    generate_range!(cr2, 73, 0, 32);
    generate_range!(cr3, 73, 32, 64);
    generate_range!(cr4, 74, 0, 32);
    generate_range!(cr5, 74, 32, 64);
    generate_range!(cr6, 75, 0, 32);
    generate_range!(cr7, 75, 32, 64);
    generate_range!(cr8, 76, 0, 32);
    generate_range!(cr9, 76, 32, 64);
    generate_range!(cr10, 77, 0, 32);
    generate_range!(cr11, 77, 32, 64);
    generate_range!(cr12, 78, 0, 32);
    generate_range!(cr13, 78, 32, 64);
    generate_range!(cr14, 79, 0, 32);
    generate_range!(cr15, 79, 32, 64);

    generate_range!(dr0, 80, 0, 32);
    generate_range!(dr1, 80, 32, 64);
    generate_range!(dr2, 81, 0, 32);
    generate_range!(dr3, 81, 32, 64);
    generate_range!(dr4, 82, 0, 32);
    generate_range!(dr5, 82, 32, 64);
    generate_range!(dr6, 83, 0, 32);
    generate_range!(dr7, 83, 32, 64);
    generate_range!(dr8, 84, 0, 32);
    generate_range!(dr9, 84, 32, 64);
    generate_range!(dr10, 85, 0, 32);
    generate_range!(dr11, 85, 32, 64);
    generate_range!(dr12, 86, 0, 32);
    generate_range!(dr13, 86, 32, 64);
    generate_range!(dr14, 87, 0, 32);
    generate_range!(dr15, 87, 32, 64);

    generate_range!(tmp8, 88, 0, 8);
    generate_range!(tmp16, 88, 0, 16);
    generate_range!(tmp32, 88, 0, 32);
    generate_range!(tmp64, 88, 0, 64);
}
