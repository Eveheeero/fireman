use crate::{
    ir::{x86_64::X64, Ir},
    prelude::BitSlice,
};

impl X64 for Ir {
    fn new() -> Self {
        let mut register = bitvec::prelude::BitVec::new();
        register.resize(5696, false);
        Self {
            register: register.into_boxed_bitslice(),
        }
    }

    fn rax(&self) -> &BitSlice {
        &self.register[0..64]
    }

    fn eax(&self) -> &BitSlice {
        &self.register[0..32]
    }

    fn ax(&self) -> &BitSlice {
        &self.register[0..16]
    }

    fn al(&self) -> &BitSlice {
        &self.register[0..8]
    }

    fn ah(&self) -> &BitSlice {
        &self.register[8..16]
    }

    fn rbx(&self) -> &BitSlice {
        &self.register[64..128]
    }

    fn ebx(&self) -> &BitSlice {
        &self.register[64..96]
    }

    fn bx(&self) -> &BitSlice {
        &self.register[64..80]
    }

    fn bl(&self) -> &BitSlice {
        &self.register[64..72]
    }

    fn bh(&self) -> &BitSlice {
        &self.register[72..80]
    }

    fn rcx(&self) -> &BitSlice {
        &self.register[128..192]
    }

    fn ecx(&self) -> &BitSlice {
        &self.register[128..160]
    }

    fn cx(&self) -> &BitSlice {
        &self.register[128..144]
    }

    fn cl(&self) -> &BitSlice {
        &self.register[128..136]
    }

    fn ch(&self) -> &BitSlice {
        &self.register[136..144]
    }

    fn rdx(&self) -> &BitSlice {
        &self.register[192..256]
    }

    fn edx(&self) -> &BitSlice {
        &self.register[192..224]
    }

    fn dx(&self) -> &BitSlice {
        &self.register[192..208]
    }

    fn dl(&self) -> &BitSlice {
        &self.register[192..200]
    }

    fn dh(&self) -> &BitSlice {
        &self.register[200..208]
    }

    fn rsp(&self) -> &BitSlice {
        &self.register[256..320]
    }

    fn esp(&self) -> &BitSlice {
        &self.register[256..288]
    }

    fn sp(&self) -> &BitSlice {
        &self.register[256..272]
    }

    fn spl(&self) -> &BitSlice {
        &self.register[256..264]
    }

    fn rbp(&self) -> &BitSlice {
        &self.register[320..384]
    }

    fn ebp(&self) -> &BitSlice {
        &self.register[320..352]
    }

    fn bp(&self) -> &BitSlice {
        &self.register[320..336]
    }

    fn bpl(&self) -> &BitSlice {
        &self.register[320..328]
    }

    fn rsi(&self) -> &BitSlice {
        &self.register[384..448]
    }

    fn esi(&self) -> &BitSlice {
        &self.register[384..416]
    }

    fn si(&self) -> &BitSlice {
        &self.register[384..400]
    }

    fn sil(&self) -> &BitSlice {
        &self.register[384..392]
    }

    fn rdi(&self) -> &BitSlice {
        &self.register[448..512]
    }

    fn edi(&self) -> &BitSlice {
        &self.register[448..480]
    }

    fn di(&self) -> &BitSlice {
        &self.register[448..464]
    }

    fn dil(&self) -> &BitSlice {
        &self.register[448..456]
    }

    fn r8(&self) -> &BitSlice {
        &self.register[512..576]
    }

    fn r8d(&self) -> &BitSlice {
        &self.register[512..544]
    }

    fn r8w(&self) -> &BitSlice {
        &self.register[512..528]
    }

    fn r8b(&self) -> &BitSlice {
        &self.register[512..520]
    }

    fn r9(&self) -> &BitSlice {
        &self.register[576..640]
    }

    fn r9d(&self) -> &BitSlice {
        &self.register[576..608]
    }

    fn r9w(&self) -> &BitSlice {
        &self.register[576..592]
    }

    fn r9b(&self) -> &BitSlice {
        &self.register[576..584]
    }

    fn r10(&self) -> &BitSlice {
        &self.register[640..704]
    }

    fn r10d(&self) -> &BitSlice {
        &self.register[640..672]
    }

    fn r10w(&self) -> &BitSlice {
        &self.register[640..656]
    }

    fn r10b(&self) -> &BitSlice {
        &self.register[640..648]
    }

    fn r11(&self) -> &BitSlice {
        &self.register[704..768]
    }

    fn r11d(&self) -> &BitSlice {
        &self.register[704..736]
    }

    fn r11w(&self) -> &BitSlice {
        &self.register[704..720]
    }

    fn r11b(&self) -> &BitSlice {
        &self.register[704..712]
    }

    fn r12(&self) -> &BitSlice {
        &self.register[768..832]
    }

    fn r12d(&self) -> &BitSlice {
        &self.register[768..800]
    }

    fn r12w(&self) -> &BitSlice {
        &self.register[768..784]
    }

    fn r12b(&self) -> &BitSlice {
        &self.register[768..776]
    }

    fn r13(&self) -> &BitSlice {
        &self.register[832..896]
    }

    fn r13d(&self) -> &BitSlice {
        &self.register[832..864]
    }

    fn r13w(&self) -> &BitSlice {
        &self.register[832..848]
    }

    fn r13b(&self) -> &BitSlice {
        &self.register[832..840]
    }

    fn r14(&self) -> &BitSlice {
        &self.register[896..960]
    }

    fn r14d(&self) -> &BitSlice {
        &self.register[896..928]
    }

    fn r14w(&self) -> &BitSlice {
        &self.register[896..912]
    }

    fn r14b(&self) -> &BitSlice {
        &self.register[896..904]
    }

    fn r15(&self) -> &BitSlice {
        &self.register[960..1024]
    }

    fn r15d(&self) -> &BitSlice {
        &self.register[960..992]
    }

    fn r15w(&self) -> &BitSlice {
        &self.register[960..976]
    }

    fn r15b(&self) -> &BitSlice {
        &self.register[960..968]
    }

    fn cs(&self) -> &BitSlice {
        &self.register[1024..1040]
    }

    fn ds(&self) -> &BitSlice {
        &self.register[1040..1056]
    }

    fn es(&self) -> &BitSlice {
        &self.register[1056..1072]
    }

    fn fs(&self) -> &BitSlice {
        &self.register[1072..1088]
    }

    fn gs(&self) -> &BitSlice {
        &self.register[1088..1104]
    }

    fn ss(&self) -> &BitSlice {
        &self.register[1104..1120]
    }

    // 최적화를 위해 1252까지 스킵

    fn rip(&self) -> &BitSlice {
        &self.register[1152..1216]
    }

    fn eip(&self) -> &BitSlice {
        &self.register[1152..1184]
    }

    fn ip(&self) -> &BitSlice {
        &self.register[1152..1168]
    }

    fn rflags(&self) -> &BitSlice {
        &self.register[1216..1280]
    }

    fn eflags(&self) -> &BitSlice {
        &self.register[1216..1248]
    }

    fn flags(&self) -> &BitSlice {
        &self.register[1216..1232]
    }

    fn cf(&self) -> &BitSlice {
        &self.register[1216..1217]
    }

    fn pf(&self) -> &BitSlice {
        &self.register[1218..1219]
    }

    fn af(&self) -> &BitSlice {
        &self.register[1220..1221]
    }

    fn zf(&self) -> &BitSlice {
        &self.register[1222..1223]
    }

    fn sf(&self) -> &BitSlice {
        &self.register[1223..1224]
    }

    fn tf(&self) -> &BitSlice {
        &self.register[1224..1225]
    }

    fn r#if(&self) -> &BitSlice {
        &self.register[1225..1226]
    }

    fn df(&self) -> &BitSlice {
        &self.register[1226..1227]
    }

    fn of(&self) -> &BitSlice {
        &self.register[1227..1228]
    }

    fn iopl(&self) -> &BitSlice {
        &self.register[1228..1230]
    }

    fn nt(&self) -> &BitSlice {
        &self.register[1230..1231]
    }

    fn rf(&self) -> &BitSlice {
        &self.register[1232..1233]
    }

    fn vm(&self) -> &BitSlice {
        &self.register[1233..1234]
    }

    fn ac(&self) -> &BitSlice {
        &self.register[1234..1235]
    }

    fn vif(&self) -> &BitSlice {
        &self.register[1235..1236]
    }

    fn vip(&self) -> &BitSlice {
        &self.register[1236..1237]
    }

    fn id(&self) -> &BitSlice {
        &self.register[1237..1238]
    }

    fn less(&self) -> &BitSlice {
        &self.register[1280..1281]
    }

    fn less_or_equal(&self) -> &BitSlice {
        &self.register[1281..1282]
    }

    fn below_or_equal(&self) -> &BitSlice {
        &self.register[1282..1283]
    }

    // 최적화를 위해 1344까지 스킵

    fn fpu_status_word(&self) -> &BitSlice {
        &self.register[1344..1360]
    }

    fn fpu_ie(&self) -> &BitSlice {
        &self.register[1344..1345]
    }

    fn fpu_de(&self) -> &BitSlice {
        &self.register[1345..1346]
    }

    fn fpu_ze(&self) -> &BitSlice {
        &self.register[1346..1347]
    }

    fn fpu_oe(&self) -> &BitSlice {
        &self.register[1347..1348]
    }

    fn fpu_ue(&self) -> &BitSlice {
        &self.register[1348..1349]
    }

    fn fpu_pe(&self) -> &BitSlice {
        &self.register[1349..1350]
    }

    fn fpu_sf(&self) -> &BitSlice {
        &self.register[1350..1351]
    }

    fn fpu_es(&self) -> &BitSlice {
        &self.register[1351..1352]
    }

    fn fpu_c0(&self) -> &BitSlice {
        &self.register[1352..1353]
    }

    fn fpu_c1(&self) -> &BitSlice {
        &self.register[1353..1354]
    }

    fn fpu_c2(&self) -> &BitSlice {
        &self.register[1354..1355]
    }

    fn fpu_top(&self) -> &BitSlice {
        &self.register[1355..1358]
    }

    fn fpu_c3(&self) -> &BitSlice {
        &self.register[1358..1359]
    }

    fn fpu_b(&self) -> &BitSlice {
        &self.register[1359..1360]
    }

    // 최적화를 위해 1408까지 스킵

    fn st0(&self) -> &BitSlice {
        &self.register[1408..1488]
    }

    fn st1(&self) -> &BitSlice {
        &self.register[1488..1568]
    }

    fn st2(&self) -> &BitSlice {
        &self.register[1568..1648]
    }

    fn st3(&self) -> &BitSlice {
        &self.register[1648..1728]
    }

    fn st4(&self) -> &BitSlice {
        &self.register[1728..1808]
    }

    fn st5(&self) -> &BitSlice {
        &self.register[1808..1888]
    }

    fn st6(&self) -> &BitSlice {
        &self.register[1888..1968]
    }

    fn st7(&self) -> &BitSlice {
        &self.register[1968..2048]
    }

    fn mm0(&self) -> &BitSlice {
        &self.register[2048..2112]
    }

    fn mm1(&self) -> &BitSlice {
        &self.register[2112..2176]
    }

    fn mm2(&self) -> &BitSlice {
        &self.register[2176..2240]
    }

    fn mm3(&self) -> &BitSlice {
        &self.register[2240..2304]
    }

    fn mm4(&self) -> &BitSlice {
        &self.register[2304..2368]
    }

    fn mm5(&self) -> &BitSlice {
        &self.register[2368..2432]
    }

    fn mm6(&self) -> &BitSlice {
        &self.register[2432..2496]
    }

    fn mm7(&self) -> &BitSlice {
        &self.register[2496..2560]
    }

    fn xmm0(&self) -> &BitSlice {
        &self.register[2560..2688]
    }

    fn xmm1(&self) -> &BitSlice {
        &self.register[2688..2816]
    }

    fn xmm2(&self) -> &BitSlice {
        &self.register[2816..2944]
    }

    fn xmm3(&self) -> &BitSlice {
        &self.register[2944..3072]
    }

    fn xmm4(&self) -> &BitSlice {
        &self.register[3072..3200]
    }

    fn xmm5(&self) -> &BitSlice {
        &self.register[3200..3328]
    }

    fn xmm6(&self) -> &BitSlice {
        &self.register[3328..3456]
    }

    fn xmm7(&self) -> &BitSlice {
        &self.register[3456..3584]
    }

    fn xmm8(&self) -> &BitSlice {
        &self.register[3584..3712]
    }

    fn xmm9(&self) -> &BitSlice {
        &self.register[3712..3840]
    }

    fn xmm10(&self) -> &BitSlice {
        &self.register[3840..3968]
    }

    fn xmm11(&self) -> &BitSlice {
        &self.register[3968..4096]
    }

    fn xmm12(&self) -> &BitSlice {
        &self.register[4096..4224]
    }

    fn xmm13(&self) -> &BitSlice {
        &self.register[4224..4352]
    }

    fn xmm14(&self) -> &BitSlice {
        &self.register[4352..4480]
    }

    fn xmm15(&self) -> &BitSlice {
        &self.register[4480..4608]
    }

    fn cr0(&self) -> &BitSlice {
        &self.register[4608..4640]
    }

    fn cr1(&self) -> &BitSlice {
        &self.register[4640..4672]
    }

    fn cr2(&self) -> &BitSlice {
        &self.register[4672..4704]
    }

    fn cr3(&self) -> &BitSlice {
        &self.register[4704..4736]
    }

    fn cr4(&self) -> &BitSlice {
        &self.register[4736..4768]
    }

    fn cr5(&self) -> &BitSlice {
        &self.register[4768..4800]
    }

    fn cr6(&self) -> &BitSlice {
        &self.register[4800..4832]
    }

    fn cr7(&self) -> &BitSlice {
        &self.register[4832..4864]
    }

    fn cr8(&self) -> &BitSlice {
        &self.register[4864..4896]
    }

    fn cr9(&self) -> &BitSlice {
        &self.register[4896..4928]
    }

    fn cr10(&self) -> &BitSlice {
        &self.register[4928..4960]
    }

    fn cr11(&self) -> &BitSlice {
        &self.register[4960..4992]
    }

    fn cr12(&self) -> &BitSlice {
        &self.register[4992..5024]
    }

    fn cr13(&self) -> &BitSlice {
        &self.register[5024..5056]
    }

    fn cr14(&self) -> &BitSlice {
        &self.register[5056..5088]
    }

    fn cr15(&self) -> &BitSlice {
        &self.register[5088..5120]
    }

    fn dr0(&self) -> &BitSlice {
        &self.register[5120..5152]
    }

    fn dr1(&self) -> &BitSlice {
        &self.register[5152..5184]
    }

    fn dr2(&self) -> &BitSlice {
        &self.register[5184..5216]
    }

    fn dr3(&self) -> &BitSlice {
        &self.register[5216..5248]
    }

    fn dr4(&self) -> &BitSlice {
        &self.register[5248..5280]
    }

    fn dr5(&self) -> &BitSlice {
        &self.register[5280..5312]
    }

    fn dr6(&self) -> &BitSlice {
        &self.register[5312..5344]
    }

    fn dr7(&self) -> &BitSlice {
        &self.register[5344..5376]
    }

    fn dr8(&self) -> &BitSlice {
        &self.register[5376..5408]
    }

    fn dr9(&self) -> &BitSlice {
        &self.register[5408..5440]
    }

    fn dr10(&self) -> &BitSlice {
        &self.register[5440..5472]
    }

    fn dr11(&self) -> &BitSlice {
        &self.register[5472..5504]
    }

    fn dr12(&self) -> &BitSlice {
        &self.register[5504..5536]
    }

    fn dr13(&self) -> &BitSlice {
        &self.register[5536..5568]
    }

    fn dr14(&self) -> &BitSlice {
        &self.register[5568..5600]
    }

    fn dr15(&self) -> &BitSlice {
        &self.register[5600..5632]
    }

    fn tmp8(&self) -> &BitSlice {
        &self.register[5632..5640]
    }

    fn tmp16(&self) -> &BitSlice {
        &self.register[5632..5648]
    }

    fn tmp32(&self) -> &BitSlice {
        &self.register[5632..5664]
    }

    fn tmp64(&self) -> &BitSlice {
        &self.register[5632..5696]
    }
}
