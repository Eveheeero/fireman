use std::cell::UnsafeCell;

use crate::{
    ir::{x86_64::X64, Ir},
    prelude::BitSlice,
};

macro_rules! generate_register {
    ($name:ident, $from:literal, $to:literal) => {
        #[inline(always)]
        fn $name(&self) -> &BitSlice {
            unsafe { &(*self.register.get())[$from..$to] }
        }
    };
}

impl X64 for Ir {
    #[inline(always)]
    fn new() -> Self {
        let mut register = bitvec::prelude::BitVec::new();
        register.resize(5696, false);
        Self {
            register: UnsafeCell::new(register.into_boxed_bitslice()),
        }
    }

    generate_register!(rax, 0, 64);
    generate_register!(eax, 0, 32);
    generate_register!(ax, 0, 16);
    generate_register!(al, 0, 8);
    generate_register!(ah, 8, 16);

    generate_register!(rbx, 64, 128);
    generate_register!(ebx, 64, 96);
    generate_register!(bx, 64, 80);
    generate_register!(bl, 64, 72);
    generate_register!(bh, 72, 80);

    generate_register!(rcx, 128, 192);
    generate_register!(ecx, 128, 160);
    generate_register!(cx, 128, 144);
    generate_register!(cl, 128, 136);
    generate_register!(ch, 136, 144);

    generate_register!(rdx, 192, 256);
    generate_register!(edx, 192, 224);
    generate_register!(dx, 192, 208);
    generate_register!(dl, 192, 200);
    generate_register!(dh, 200, 208);

    generate_register!(rsp, 256, 320);
    generate_register!(esp, 256, 288);
    generate_register!(sp, 256, 272);
    generate_register!(spl, 256, 264);

    generate_register!(rbp, 320, 384);
    generate_register!(ebp, 320, 352);
    generate_register!(bp, 320, 336);
    generate_register!(bpl, 320, 328);

    generate_register!(rsi, 384, 448);
    generate_register!(esi, 384, 416);
    generate_register!(si, 384, 400);
    generate_register!(sil, 384, 392);

    generate_register!(rdi, 448, 512);
    generate_register!(edi, 448, 480);
    generate_register!(di, 448, 464);
    generate_register!(dil, 448, 456);

    generate_register!(r8, 512, 576);
    generate_register!(r8d, 512, 544);
    generate_register!(r8w, 512, 528);
    generate_register!(r8b, 512, 520);

    generate_register!(r9, 576, 640);
    generate_register!(r9d, 576, 608);
    generate_register!(r9w, 576, 592);
    generate_register!(r9b, 576, 584);

    generate_register!(r10, 640, 704);
    generate_register!(r10d, 640, 672);
    generate_register!(r10w, 640, 656);
    generate_register!(r10b, 640, 648);

    generate_register!(r11, 704, 768);
    generate_register!(r11d, 704, 736);
    generate_register!(r11w, 704, 720);
    generate_register!(r11b, 704, 712);

    generate_register!(r12, 768, 832);
    generate_register!(r12d, 768, 800);
    generate_register!(r12w, 768, 784);
    generate_register!(r12b, 768, 776);

    generate_register!(r13, 832, 896);
    generate_register!(r13d, 832, 864);
    generate_register!(r13w, 832, 848);
    generate_register!(r13b, 832, 840);

    generate_register!(r14, 896, 960);
    generate_register!(r14d, 896, 928);
    generate_register!(r14w, 896, 912);
    generate_register!(r14b, 896, 904);

    generate_register!(r15, 960, 1024);
    generate_register!(r15d, 960, 992);
    generate_register!(r15w, 960, 976);
    generate_register!(r15b, 960, 968);

    generate_register!(cs, 1024, 1040);
    generate_register!(ds, 1040, 1056);
    generate_register!(es, 1056, 1072);
    generate_register!(fs, 1072, 1088);
    generate_register!(gs, 1088, 1104);
    generate_register!(ss, 1104, 1120);

    // 최적화를 위해 1152까지 스킵

    generate_register!(rip, 1152, 1216);
    generate_register!(eip, 1152, 1184);
    generate_register!(ip, 1152, 1168);

    generate_register!(rflags, 1216, 1280);
    generate_register!(eflags, 1216, 1248);
    generate_register!(flags, 1216, 1232);
    generate_register!(cf, 1216, 1217);
    generate_register!(pf, 1218, 1219);
    generate_register!(af, 1220, 1221);
    generate_register!(zf, 1222, 1223);
    generate_register!(sf, 1223, 1224);
    generate_register!(tf, 1224, 1225);
    generate_register!(r#if, 1225, 1226);
    generate_register!(df, 1226, 1227);
    generate_register!(of, 1227, 1228);
    generate_register!(iopl, 1228, 1230);
    generate_register!(nt, 1230, 1231);
    generate_register!(rf, 1232, 1233);
    generate_register!(vm, 1233, 1234);
    generate_register!(ac, 1234, 1235);
    generate_register!(vif, 1235, 1236);
    generate_register!(vip, 1236, 1237);
    generate_register!(id, 1237, 1238);

    generate_register!(less, 1280, 1281);
    generate_register!(less_or_equal, 1281, 1282);
    generate_register!(below_or_equal, 1282, 1283);

    // 최적화를 위해 1344까지 스킵

    generate_register!(fpu_status_word, 1344, 1360);
    generate_register!(fpu_ie, 1344, 1345);
    generate_register!(fpu_de, 1345, 1346);
    generate_register!(fpu_ze, 1346, 1347);
    generate_register!(fpu_oe, 1347, 1348);
    generate_register!(fpu_ue, 1348, 1349);
    generate_register!(fpu_pe, 1349, 1350);
    generate_register!(fpu_sf, 1350, 1351);
    generate_register!(fpu_es, 1351, 1352);
    generate_register!(fpu_c0, 1352, 1353);
    generate_register!(fpu_c1, 1353, 1354);
    generate_register!(fpu_c2, 1354, 1355);
    generate_register!(fpu_top, 1355, 1358);
    generate_register!(fpu_c3, 1358, 1359);
    generate_register!(fpu_b, 1359, 1360);

    // 최적화를 위해 1408까지 스킵

    generate_register!(st0, 1408, 1488);
    generate_register!(st1, 1488, 1568);
    generate_register!(st2, 1568, 1648);
    generate_register!(st3, 1648, 1728);
    generate_register!(st4, 1728, 1808);
    generate_register!(st5, 1808, 1888);
    generate_register!(st6, 1888, 1968);
    generate_register!(st7, 1968, 2048);

    generate_register!(mm0, 2048, 2112);
    generate_register!(mm1, 2112, 2176);
    generate_register!(mm2, 2176, 2240);
    generate_register!(mm3, 2240, 2304);
    generate_register!(mm4, 2304, 2368);
    generate_register!(mm5, 2368, 2432);
    generate_register!(mm6, 2432, 2496);
    generate_register!(mm7, 2496, 2560);

    generate_register!(xmm0, 2560, 2688);
    generate_register!(xmm1, 2688, 2816);
    generate_register!(xmm2, 2816, 2944);
    generate_register!(xmm3, 2944, 3072);
    generate_register!(xmm4, 3072, 3200);
    generate_register!(xmm5, 3200, 3328);
    generate_register!(xmm6, 3328, 3456);
    generate_register!(xmm7, 3456, 3584);
    generate_register!(xmm8, 3584, 3712);
    generate_register!(xmm9, 3712, 3840);
    generate_register!(xmm10, 3840, 3968);
    generate_register!(xmm11, 3968, 4096);
    generate_register!(xmm12, 4096, 4224);
    generate_register!(xmm13, 4224, 4352);
    generate_register!(xmm14, 4352, 4480);
    generate_register!(xmm15, 4480, 4608);

    generate_register!(cr0, 4608, 4640);
    generate_register!(cr1, 4640, 4672);
    generate_register!(cr2, 4672, 4704);
    generate_register!(cr3, 4704, 4736);
    generate_register!(cr4, 4736, 4768);
    generate_register!(cr5, 4768, 4800);
    generate_register!(cr6, 4800, 4832);
    generate_register!(cr7, 4832, 4864);
    generate_register!(cr8, 4864, 4896);
    generate_register!(cr9, 4896, 4928);
    generate_register!(cr10, 4928, 4960);
    generate_register!(cr11, 4960, 4992);
    generate_register!(cr12, 4992, 5024);
    generate_register!(cr13, 5024, 5056);
    generate_register!(cr14, 5056, 5088);
    generate_register!(cr15, 5088, 5120);

    generate_register!(dr0, 5120, 5152);
    generate_register!(dr1, 5152, 5184);
    generate_register!(dr2, 5184, 5216);
    generate_register!(dr3, 5216, 5248);
    generate_register!(dr4, 5248, 5280);
    generate_register!(dr5, 5280, 5312);
    generate_register!(dr6, 5312, 5344);
    generate_register!(dr7, 5344, 5376);
    generate_register!(dr8, 5376, 5408);
    generate_register!(dr9, 5408, 5440);
    generate_register!(dr10, 5440, 5472);
    generate_register!(dr11, 5472, 5504);
    generate_register!(dr12, 5504, 5536);
    generate_register!(dr13, 5536, 5568);
    generate_register!(dr14, 5568, 5600);
    generate_register!(dr15, 5600, 5632);

    generate_register!(tmp8, 5632, 5640);
    generate_register!(tmp16, 5632, 5648);
    generate_register!(tmp32, 5632, 5664);
    generate_register!(tmp64, 5632, 5696);
}
