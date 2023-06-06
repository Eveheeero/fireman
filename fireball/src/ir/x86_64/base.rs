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
        todo!()
    }

    fn bx(&self) -> &BitSlice {
        todo!()
    }

    fn bl(&self) -> &BitSlice {
        todo!()
    }

    fn bh(&self) -> &BitSlice {
        todo!()
    }

    fn rcx(&self) -> &BitSlice {
        &self.register[128..192]
    }

    fn ecx(&self) -> &BitSlice {
        todo!()
    }

    fn cx(&self) -> &BitSlice {
        todo!()
    }

    fn cl(&self) -> &BitSlice {
        todo!()
    }

    fn ch(&self) -> &BitSlice {
        todo!()
    }

    fn rdx(&self) -> &BitSlice {
        &self.register[192..256]
    }

    fn edx(&self) -> &BitSlice {
        todo!()
    }

    fn dx(&self) -> &BitSlice {
        todo!()
    }

    fn dl(&self) -> &BitSlice {
        todo!()
    }

    fn dh(&self) -> &BitSlice {
        todo!()
    }

    fn rsp(&self) -> &BitSlice {
        &self.register[256..320]
    }

    fn esp(&self) -> &BitSlice {
        todo!()
    }

    fn sp(&self) -> &BitSlice {
        todo!()
    }

    fn spl(&self) -> &BitSlice {
        todo!()
    }

    fn rbp(&self) -> &BitSlice {
        &self.register[320..384]
    }

    fn ebp(&self) -> &BitSlice {
        todo!()
    }

    fn bp(&self) -> &BitSlice {
        todo!()
    }

    fn bpl(&self) -> &BitSlice {
        todo!()
    }

    fn rsi(&self) -> &BitSlice {
        &self.register[384..448]
    }

    fn esi(&self) -> &BitSlice {
        todo!()
    }

    fn si(&self) -> &BitSlice {
        todo!()
    }

    fn sil(&self) -> &BitSlice {
        todo!()
    }

    fn rdi(&self) -> &BitSlice {
        &self.register[448..512]
    }

    fn edi(&self) -> &BitSlice {
        todo!()
    }

    fn di(&self) -> &BitSlice {
        todo!()
    }

    fn dil(&self) -> &BitSlice {
        todo!()
    }

    fn r8(&self) -> &BitSlice {
        &self.register[512..576]
    }

    fn r8d(&self) -> &BitSlice {
        todo!()
    }

    fn r8w(&self) -> &BitSlice {
        todo!()
    }

    fn r8b(&self) -> &BitSlice {
        todo!()
    }

    fn r9(&self) -> &BitSlice {
        &self.register[576..640]
    }

    fn r9d(&self) -> &BitSlice {
        todo!()
    }

    fn r9w(&self) -> &BitSlice {
        todo!()
    }

    fn r9b(&self) -> &BitSlice {
        todo!()
    }

    fn r10(&self) -> &BitSlice {
        &self.register[640..704]
    }

    fn r10d(&self) -> &BitSlice {
        todo!()
    }

    fn r10w(&self) -> &BitSlice {
        todo!()
    }

    fn r10b(&self) -> &BitSlice {
        todo!()
    }

    fn r11(&self) -> &BitSlice {
        &self.register[704..768]
    }

    fn r11d(&self) -> &BitSlice {
        todo!()
    }

    fn r11w(&self) -> &BitSlice {
        todo!()
    }

    fn r11b(&self) -> &BitSlice {
        todo!()
    }

    fn r12(&self) -> &BitSlice {
        &self.register[768..832]
    }

    fn r12d(&self) -> &BitSlice {
        todo!()
    }

    fn r12w(&self) -> &BitSlice {
        todo!()
    }

    fn r12b(&self) -> &BitSlice {
        todo!()
    }

    fn r13(&self) -> &BitSlice {
        &self.register[832..896]
    }

    fn r13d(&self) -> &BitSlice {
        todo!()
    }

    fn r13w(&self) -> &BitSlice {
        todo!()
    }

    fn r13b(&self) -> &BitSlice {
        todo!()
    }

    fn r14(&self) -> &BitSlice {
        &self.register[896..960]
    }

    fn r14d(&self) -> &BitSlice {
        todo!()
    }

    fn r14w(&self) -> &BitSlice {
        todo!()
    }

    fn r14b(&self) -> &BitSlice {
        todo!()
    }

    fn r15(&self) -> &BitSlice {
        &self.register[960..1024]
    }

    fn r15d(&self) -> &BitSlice {
        todo!()
    }

    fn r15w(&self) -> &BitSlice {
        todo!()
    }

    fn r15b(&self) -> &BitSlice {
        todo!()
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
        todo!()
    }

    fn ip(&self) -> &BitSlice {
        todo!()
    }

    fn rflags(&self) -> &BitSlice {
        &self.register[1216..1280]
    }

    fn eflags(&self) -> &BitSlice {
        todo!()
    }

    fn flags(&self) -> &BitSlice {
        todo!()
    }

    fn cf(&self) -> &BitSlice {
        todo!()
    }

    fn pf(&self) -> &BitSlice {
        todo!()
    }

    fn af(&self) -> &BitSlice {
        todo!()
    }

    fn zf(&self) -> &BitSlice {
        todo!()
    }

    fn sf(&self) -> &BitSlice {
        todo!()
    }

    fn tf(&self) -> &BitSlice {
        todo!()
    }

    fn r#if(&self) -> &BitSlice {
        todo!()
    }

    fn df(&self) -> &BitSlice {
        todo!()
    }

    fn of(&self) -> &BitSlice {
        todo!()
    }

    fn iopl(&self) -> &BitSlice {
        todo!()
    }

    fn nt(&self) -> &BitSlice {
        todo!()
    }

    fn rf(&self) -> &BitSlice {
        todo!()
    }

    fn vm(&self) -> &BitSlice {
        todo!()
    }

    fn ac(&self) -> &BitSlice {
        todo!()
    }

    fn vif(&self) -> &BitSlice {
        todo!()
    }

    fn vip(&self) -> &BitSlice {
        todo!()
    }

    fn id(&self) -> &BitSlice {
        todo!()
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
        todo!()
    }

    fn fpu_de(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_ze(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_oe(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_ue(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_pe(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_sf(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_es(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_c0(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_c1(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_c2(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_top(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_c3(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_b(&self) -> &BitSlice {
        todo!()
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
