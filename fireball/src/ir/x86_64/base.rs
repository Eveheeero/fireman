use crate::{
    ir::{x86_64::X64, Ir},
    prelude::BitSlice,
};

impl X64 for Ir {
    #[inline(always)]
    fn new() -> Self {
        let mut register = bitvec::prelude::BitVec::new();
        register.resize(5696, false);
        Self {
            register: register.into_boxed_bitslice(),
        }
    }

    #[inline(always)]
    fn rax(&self) -> &BitSlice {
        &self.register[0..64]
    }

    #[inline(always)]
    fn eax(&self) -> &BitSlice {
        &self.register[0..32]
    }

    #[inline(always)]
    fn ax(&self) -> &BitSlice {
        &self.register[0..16]
    }

    #[inline(always)]
    fn al(&self) -> &BitSlice {
        &self.register[0..8]
    }

    #[inline(always)]
    fn ah(&self) -> &BitSlice {
        &self.register[8..16]
    }

    #[inline(always)]
    fn rbx(&self) -> &BitSlice {
        &self.register[64..128]
    }

    #[inline(always)]
    fn ebx(&self) -> &BitSlice {
        &self.register[64..96]
    }

    #[inline(always)]
    fn bx(&self) -> &BitSlice {
        &self.register[64..80]
    }

    #[inline(always)]
    fn bl(&self) -> &BitSlice {
        &self.register[64..72]
    }

    #[inline(always)]
    fn bh(&self) -> &BitSlice {
        &self.register[72..80]
    }

    #[inline(always)]
    fn rcx(&self) -> &BitSlice {
        &self.register[128..192]
    }

    #[inline(always)]
    fn ecx(&self) -> &BitSlice {
        &self.register[128..160]
    }

    #[inline(always)]
    fn cx(&self) -> &BitSlice {
        &self.register[128..144]
    }

    #[inline(always)]
    fn cl(&self) -> &BitSlice {
        &self.register[128..136]
    }

    #[inline(always)]
    fn ch(&self) -> &BitSlice {
        &self.register[136..144]
    }

    #[inline(always)]
    fn rdx(&self) -> &BitSlice {
        &self.register[192..256]
    }

    #[inline(always)]
    fn edx(&self) -> &BitSlice {
        &self.register[192..224]
    }

    #[inline(always)]
    fn dx(&self) -> &BitSlice {
        &self.register[192..208]
    }

    #[inline(always)]
    fn dl(&self) -> &BitSlice {
        &self.register[192..200]
    }

    #[inline(always)]
    fn dh(&self) -> &BitSlice {
        &self.register[200..208]
    }

    #[inline(always)]
    fn rsp(&self) -> &BitSlice {
        &self.register[256..320]
    }

    #[inline(always)]
    fn esp(&self) -> &BitSlice {
        &self.register[256..288]
    }

    #[inline(always)]
    fn sp(&self) -> &BitSlice {
        &self.register[256..272]
    }

    #[inline(always)]
    fn spl(&self) -> &BitSlice {
        &self.register[256..264]
    }

    #[inline(always)]
    fn rbp(&self) -> &BitSlice {
        &self.register[320..384]
    }

    #[inline(always)]
    fn ebp(&self) -> &BitSlice {
        &self.register[320..352]
    }

    #[inline(always)]
    fn bp(&self) -> &BitSlice {
        &self.register[320..336]
    }

    #[inline(always)]
    fn bpl(&self) -> &BitSlice {
        &self.register[320..328]
    }

    #[inline(always)]
    fn rsi(&self) -> &BitSlice {
        &self.register[384..448]
    }

    #[inline(always)]
    fn esi(&self) -> &BitSlice {
        &self.register[384..416]
    }

    #[inline(always)]
    fn si(&self) -> &BitSlice {
        &self.register[384..400]
    }

    #[inline(always)]
    fn sil(&self) -> &BitSlice {
        &self.register[384..392]
    }

    #[inline(always)]
    fn rdi(&self) -> &BitSlice {
        &self.register[448..512]
    }

    #[inline(always)]
    fn edi(&self) -> &BitSlice {
        &self.register[448..480]
    }

    #[inline(always)]
    fn di(&self) -> &BitSlice {
        &self.register[448..464]
    }

    #[inline(always)]
    fn dil(&self) -> &BitSlice {
        &self.register[448..456]
    }

    #[inline(always)]
    fn r8(&self) -> &BitSlice {
        &self.register[512..576]
    }

    #[inline(always)]
    fn r8d(&self) -> &BitSlice {
        &self.register[512..544]
    }

    #[inline(always)]
    fn r8w(&self) -> &BitSlice {
        &self.register[512..528]
    }

    #[inline(always)]
    fn r8b(&self) -> &BitSlice {
        &self.register[512..520]
    }

    #[inline(always)]
    fn r9(&self) -> &BitSlice {
        &self.register[576..640]
    }

    #[inline(always)]
    fn r9d(&self) -> &BitSlice {
        &self.register[576..608]
    }

    #[inline(always)]
    fn r9w(&self) -> &BitSlice {
        &self.register[576..592]
    }

    #[inline(always)]
    fn r9b(&self) -> &BitSlice {
        &self.register[576..584]
    }

    #[inline(always)]
    fn r10(&self) -> &BitSlice {
        &self.register[640..704]
    }

    #[inline(always)]
    fn r10d(&self) -> &BitSlice {
        &self.register[640..672]
    }

    #[inline(always)]
    fn r10w(&self) -> &BitSlice {
        &self.register[640..656]
    }

    #[inline(always)]
    fn r10b(&self) -> &BitSlice {
        &self.register[640..648]
    }

    #[inline(always)]
    fn r11(&self) -> &BitSlice {
        &self.register[704..768]
    }

    #[inline(always)]
    fn r11d(&self) -> &BitSlice {
        &self.register[704..736]
    }

    #[inline(always)]
    fn r11w(&self) -> &BitSlice {
        &self.register[704..720]
    }

    #[inline(always)]
    fn r11b(&self) -> &BitSlice {
        &self.register[704..712]
    }

    #[inline(always)]
    fn r12(&self) -> &BitSlice {
        &self.register[768..832]
    }

    #[inline(always)]
    fn r12d(&self) -> &BitSlice {
        &self.register[768..800]
    }

    #[inline(always)]
    fn r12w(&self) -> &BitSlice {
        &self.register[768..784]
    }

    #[inline(always)]
    fn r12b(&self) -> &BitSlice {
        &self.register[768..776]
    }

    #[inline(always)]
    fn r13(&self) -> &BitSlice {
        &self.register[832..896]
    }

    #[inline(always)]
    fn r13d(&self) -> &BitSlice {
        &self.register[832..864]
    }

    #[inline(always)]
    fn r13w(&self) -> &BitSlice {
        &self.register[832..848]
    }

    #[inline(always)]
    fn r13b(&self) -> &BitSlice {
        &self.register[832..840]
    }

    #[inline(always)]
    fn r14(&self) -> &BitSlice {
        &self.register[896..960]
    }

    #[inline(always)]
    fn r14d(&self) -> &BitSlice {
        &self.register[896..928]
    }

    #[inline(always)]
    fn r14w(&self) -> &BitSlice {
        &self.register[896..912]
    }

    #[inline(always)]
    fn r14b(&self) -> &BitSlice {
        &self.register[896..904]
    }

    #[inline(always)]
    fn r15(&self) -> &BitSlice {
        &self.register[960..1024]
    }

    #[inline(always)]
    fn r15d(&self) -> &BitSlice {
        &self.register[960..992]
    }

    #[inline(always)]
    fn r15w(&self) -> &BitSlice {
        &self.register[960..976]
    }

    #[inline(always)]
    fn r15b(&self) -> &BitSlice {
        &self.register[960..968]
    }

    #[inline(always)]
    fn cs(&self) -> &BitSlice {
        &self.register[1024..1040]
    }

    #[inline(always)]
    fn ds(&self) -> &BitSlice {
        &self.register[1040..1056]
    }

    #[inline(always)]
    fn es(&self) -> &BitSlice {
        &self.register[1056..1072]
    }

    #[inline(always)]
    fn fs(&self) -> &BitSlice {
        &self.register[1072..1088]
    }

    #[inline(always)]
    fn gs(&self) -> &BitSlice {
        &self.register[1088..1104]
    }

    #[inline(always)]
    fn ss(&self) -> &BitSlice {
        &self.register[1104..1120]
    }

    // 최적화를 위해 1252까지 스킵

    #[inline(always)]
    fn rip(&self) -> &BitSlice {
        &self.register[1152..1216]
    }

    #[inline(always)]
    fn eip(&self) -> &BitSlice {
        &self.register[1152..1184]
    }

    #[inline(always)]
    fn ip(&self) -> &BitSlice {
        &self.register[1152..1168]
    }

    #[inline(always)]
    fn rflags(&self) -> &BitSlice {
        &self.register[1216..1280]
    }

    #[inline(always)]
    fn eflags(&self) -> &BitSlice {
        &self.register[1216..1248]
    }

    #[inline(always)]
    fn flags(&self) -> &BitSlice {
        &self.register[1216..1232]
    }

    #[inline(always)]
    fn cf(&self) -> &BitSlice {
        &self.register[1216..1217]
    }

    #[inline(always)]
    fn pf(&self) -> &BitSlice {
        &self.register[1218..1219]
    }

    #[inline(always)]
    fn af(&self) -> &BitSlice {
        &self.register[1220..1221]
    }

    #[inline(always)]
    fn zf(&self) -> &BitSlice {
        &self.register[1222..1223]
    }

    #[inline(always)]
    fn sf(&self) -> &BitSlice {
        &self.register[1223..1224]
    }

    #[inline(always)]
    fn tf(&self) -> &BitSlice {
        &self.register[1224..1225]
    }

    #[inline(always)]
    fn r#if(&self) -> &BitSlice {
        &self.register[1225..1226]
    }

    #[inline(always)]
    fn df(&self) -> &BitSlice {
        &self.register[1226..1227]
    }

    #[inline(always)]
    fn of(&self) -> &BitSlice {
        &self.register[1227..1228]
    }

    #[inline(always)]
    fn iopl(&self) -> &BitSlice {
        &self.register[1228..1230]
    }

    #[inline(always)]
    fn nt(&self) -> &BitSlice {
        &self.register[1230..1231]
    }

    #[inline(always)]
    fn rf(&self) -> &BitSlice {
        &self.register[1232..1233]
    }

    #[inline(always)]
    fn vm(&self) -> &BitSlice {
        &self.register[1233..1234]
    }

    #[inline(always)]
    fn ac(&self) -> &BitSlice {
        &self.register[1234..1235]
    }

    #[inline(always)]
    fn vif(&self) -> &BitSlice {
        &self.register[1235..1236]
    }

    #[inline(always)]
    fn vip(&self) -> &BitSlice {
        &self.register[1236..1237]
    }

    #[inline(always)]
    fn id(&self) -> &BitSlice {
        &self.register[1237..1238]
    }

    #[inline(always)]
    fn less(&self) -> &BitSlice {
        &self.register[1280..1281]
    }

    #[inline(always)]
    fn less_or_equal(&self) -> &BitSlice {
        &self.register[1281..1282]
    }

    #[inline(always)]
    fn below_or_equal(&self) -> &BitSlice {
        &self.register[1282..1283]
    }

    // 최적화를 위해 1344까지 스킵

    #[inline(always)]
    fn fpu_status_word(&self) -> &BitSlice {
        &self.register[1344..1360]
    }

    #[inline(always)]
    fn fpu_ie(&self) -> &BitSlice {
        &self.register[1344..1345]
    }

    #[inline(always)]
    fn fpu_de(&self) -> &BitSlice {
        &self.register[1345..1346]
    }

    #[inline(always)]
    fn fpu_ze(&self) -> &BitSlice {
        &self.register[1346..1347]
    }

    #[inline(always)]
    fn fpu_oe(&self) -> &BitSlice {
        &self.register[1347..1348]
    }

    #[inline(always)]
    fn fpu_ue(&self) -> &BitSlice {
        &self.register[1348..1349]
    }

    #[inline(always)]
    fn fpu_pe(&self) -> &BitSlice {
        &self.register[1349..1350]
    }

    #[inline(always)]
    fn fpu_sf(&self) -> &BitSlice {
        &self.register[1350..1351]
    }

    #[inline(always)]
    fn fpu_es(&self) -> &BitSlice {
        &self.register[1351..1352]
    }

    #[inline(always)]
    fn fpu_c0(&self) -> &BitSlice {
        &self.register[1352..1353]
    }

    #[inline(always)]
    fn fpu_c1(&self) -> &BitSlice {
        &self.register[1353..1354]
    }

    #[inline(always)]
    fn fpu_c2(&self) -> &BitSlice {
        &self.register[1354..1355]
    }

    #[inline(always)]
    fn fpu_top(&self) -> &BitSlice {
        &self.register[1355..1358]
    }

    #[inline(always)]
    fn fpu_c3(&self) -> &BitSlice {
        &self.register[1358..1359]
    }

    #[inline(always)]
    fn fpu_b(&self) -> &BitSlice {
        &self.register[1359..1360]
    }

    // 최적화를 위해 1408까지 스킵

    #[inline(always)]
    fn st0(&self) -> &BitSlice {
        &self.register[1408..1488]
    }

    #[inline(always)]
    fn st1(&self) -> &BitSlice {
        &self.register[1488..1568]
    }

    #[inline(always)]
    fn st2(&self) -> &BitSlice {
        &self.register[1568..1648]
    }

    #[inline(always)]
    fn st3(&self) -> &BitSlice {
        &self.register[1648..1728]
    }

    #[inline(always)]
    fn st4(&self) -> &BitSlice {
        &self.register[1728..1808]
    }

    #[inline(always)]
    fn st5(&self) -> &BitSlice {
        &self.register[1808..1888]
    }

    #[inline(always)]
    fn st6(&self) -> &BitSlice {
        &self.register[1888..1968]
    }

    #[inline(always)]
    fn st7(&self) -> &BitSlice {
        &self.register[1968..2048]
    }

    #[inline(always)]
    fn mm0(&self) -> &BitSlice {
        &self.register[2048..2112]
    }

    #[inline(always)]
    fn mm1(&self) -> &BitSlice {
        &self.register[2112..2176]
    }

    #[inline(always)]
    fn mm2(&self) -> &BitSlice {
        &self.register[2176..2240]
    }

    #[inline(always)]
    fn mm3(&self) -> &BitSlice {
        &self.register[2240..2304]
    }

    #[inline(always)]
    fn mm4(&self) -> &BitSlice {
        &self.register[2304..2368]
    }

    #[inline(always)]
    fn mm5(&self) -> &BitSlice {
        &self.register[2368..2432]
    }

    #[inline(always)]
    fn mm6(&self) -> &BitSlice {
        &self.register[2432..2496]
    }

    #[inline(always)]
    fn mm7(&self) -> &BitSlice {
        &self.register[2496..2560]
    }

    #[inline(always)]
    fn xmm0(&self) -> &BitSlice {
        &self.register[2560..2688]
    }

    #[inline(always)]
    fn xmm1(&self) -> &BitSlice {
        &self.register[2688..2816]
    }

    #[inline(always)]
    fn xmm2(&self) -> &BitSlice {
        &self.register[2816..2944]
    }

    #[inline(always)]
    fn xmm3(&self) -> &BitSlice {
        &self.register[2944..3072]
    }

    #[inline(always)]
    fn xmm4(&self) -> &BitSlice {
        &self.register[3072..3200]
    }

    #[inline(always)]
    fn xmm5(&self) -> &BitSlice {
        &self.register[3200..3328]
    }

    #[inline(always)]
    fn xmm6(&self) -> &BitSlice {
        &self.register[3328..3456]
    }

    #[inline(always)]
    fn xmm7(&self) -> &BitSlice {
        &self.register[3456..3584]
    }

    #[inline(always)]
    fn xmm8(&self) -> &BitSlice {
        &self.register[3584..3712]
    }

    #[inline(always)]
    fn xmm9(&self) -> &BitSlice {
        &self.register[3712..3840]
    }

    #[inline(always)]
    fn xmm10(&self) -> &BitSlice {
        &self.register[3840..3968]
    }

    #[inline(always)]
    fn xmm11(&self) -> &BitSlice {
        &self.register[3968..4096]
    }

    #[inline(always)]
    fn xmm12(&self) -> &BitSlice {
        &self.register[4096..4224]
    }

    #[inline(always)]
    fn xmm13(&self) -> &BitSlice {
        &self.register[4224..4352]
    }

    #[inline(always)]
    fn xmm14(&self) -> &BitSlice {
        &self.register[4352..4480]
    }

    #[inline(always)]
    fn xmm15(&self) -> &BitSlice {
        &self.register[4480..4608]
    }

    #[inline(always)]
    fn cr0(&self) -> &BitSlice {
        &self.register[4608..4640]
    }

    #[inline(always)]
    fn cr1(&self) -> &BitSlice {
        &self.register[4640..4672]
    }

    #[inline(always)]
    fn cr2(&self) -> &BitSlice {
        &self.register[4672..4704]
    }

    #[inline(always)]
    fn cr3(&self) -> &BitSlice {
        &self.register[4704..4736]
    }

    #[inline(always)]
    fn cr4(&self) -> &BitSlice {
        &self.register[4736..4768]
    }

    #[inline(always)]
    fn cr5(&self) -> &BitSlice {
        &self.register[4768..4800]
    }

    #[inline(always)]
    fn cr6(&self) -> &BitSlice {
        &self.register[4800..4832]
    }

    #[inline(always)]
    fn cr7(&self) -> &BitSlice {
        &self.register[4832..4864]
    }

    #[inline(always)]
    fn cr8(&self) -> &BitSlice {
        &self.register[4864..4896]
    }

    #[inline(always)]
    fn cr9(&self) -> &BitSlice {
        &self.register[4896..4928]
    }

    #[inline(always)]
    fn cr10(&self) -> &BitSlice {
        &self.register[4928..4960]
    }

    #[inline(always)]
    fn cr11(&self) -> &BitSlice {
        &self.register[4960..4992]
    }

    #[inline(always)]
    fn cr12(&self) -> &BitSlice {
        &self.register[4992..5024]
    }

    #[inline(always)]
    fn cr13(&self) -> &BitSlice {
        &self.register[5024..5056]
    }

    #[inline(always)]
    fn cr14(&self) -> &BitSlice {
        &self.register[5056..5088]
    }

    #[inline(always)]
    fn cr15(&self) -> &BitSlice {
        &self.register[5088..5120]
    }

    #[inline(always)]
    fn dr0(&self) -> &BitSlice {
        &self.register[5120..5152]
    }

    #[inline(always)]
    fn dr1(&self) -> &BitSlice {
        &self.register[5152..5184]
    }

    #[inline(always)]
    fn dr2(&self) -> &BitSlice {
        &self.register[5184..5216]
    }

    #[inline(always)]
    fn dr3(&self) -> &BitSlice {
        &self.register[5216..5248]
    }

    #[inline(always)]
    fn dr4(&self) -> &BitSlice {
        &self.register[5248..5280]
    }

    #[inline(always)]
    fn dr5(&self) -> &BitSlice {
        &self.register[5280..5312]
    }

    #[inline(always)]
    fn dr6(&self) -> &BitSlice {
        &self.register[5312..5344]
    }

    #[inline(always)]
    fn dr7(&self) -> &BitSlice {
        &self.register[5344..5376]
    }

    #[inline(always)]
    fn dr8(&self) -> &BitSlice {
        &self.register[5376..5408]
    }

    #[inline(always)]
    fn dr9(&self) -> &BitSlice {
        &self.register[5408..5440]
    }

    #[inline(always)]
    fn dr10(&self) -> &BitSlice {
        &self.register[5440..5472]
    }

    #[inline(always)]
    fn dr11(&self) -> &BitSlice {
        &self.register[5472..5504]
    }

    #[inline(always)]
    fn dr12(&self) -> &BitSlice {
        &self.register[5504..5536]
    }

    #[inline(always)]
    fn dr13(&self) -> &BitSlice {
        &self.register[5536..5568]
    }

    #[inline(always)]
    fn dr14(&self) -> &BitSlice {
        &self.register[5568..5600]
    }

    #[inline(always)]
    fn dr15(&self) -> &BitSlice {
        &self.register[5600..5632]
    }

    #[inline(always)]
    fn tmp8(&self) -> &BitSlice {
        &self.register[5632..5640]
    }

    #[inline(always)]
    fn tmp16(&self) -> &BitSlice {
        &self.register[5632..5648]
    }

    #[inline(always)]
    fn tmp32(&self) -> &BitSlice {
        &self.register[5632..5664]
    }

    #[inline(always)]
    fn tmp64(&self) -> &BitSlice {
        &self.register[5632..5696]
    }
}
