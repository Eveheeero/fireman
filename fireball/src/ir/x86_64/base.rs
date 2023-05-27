use crate::{
    ir::{x86_64::X64, Ir},
    prelude::BitSlice,
};

impl X64 for Ir {
    fn new() -> Self {
        let mut register = bitvec::prelude::BitVec::new();
        // TODO X64컴퓨터에 맞는 모든 레지스터 사이즈를 구해 넣어야 한다.
        register.resize(100, false);
        Self {
            register: register.into_boxed_bitslice(),
        }
    }

    fn eax(&self) -> &BitSlice {
        // TODO X64컴퓨터의 eax레지스터는 ~부터 ~까지의 공간을 차지한다 로 구현해야 한다.
        &self.register[0..16]
    }

    fn rax(&self) -> &BitSlice {
        todo!()
    }

    fn ax(&self) -> &BitSlice {
        todo!()
    }

    fn al(&self) -> &BitSlice {
        todo!()
    }

    fn ah(&self) -> &BitSlice {
        todo!()
    }

    fn rbx(&self) -> &BitSlice {
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
    }

    fn ds(&self) -> &BitSlice {
        todo!()
    }

    fn es(&self) -> &BitSlice {
        todo!()
    }

    fn fs(&self) -> &BitSlice {
        todo!()
    }

    fn gs(&self) -> &BitSlice {
        todo!()
    }

    fn ss(&self) -> &BitSlice {
        todo!()
    }

    fn rip(&self) -> &BitSlice {
        todo!()
    }

    fn eip(&self) -> &BitSlice {
        todo!()
    }

    fn ip(&self) -> &BitSlice {
        todo!()
    }

    fn rflags(&self) -> &BitSlice {
        todo!()
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
        todo!()
    }

    fn less_or_equal(&self) -> &BitSlice {
        todo!()
    }

    fn below_or_equal(&self) -> &BitSlice {
        todo!()
    }

    fn fpu_status_word(&self) -> &BitSlice {
        todo!()
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

    fn st0(&self) -> &BitSlice {
        todo!()
    }

    fn st1(&self) -> &BitSlice {
        todo!()
    }

    fn st2(&self) -> &BitSlice {
        todo!()
    }

    fn st3(&self) -> &BitSlice {
        todo!()
    }

    fn st4(&self) -> &BitSlice {
        todo!()
    }

    fn st5(&self) -> &BitSlice {
        todo!()
    }

    fn st6(&self) -> &BitSlice {
        todo!()
    }

    fn st7(&self) -> &BitSlice {
        todo!()
    }

    fn mm0(&self) -> &BitSlice {
        todo!()
    }

    fn mm1(&self) -> &BitSlice {
        todo!()
    }

    fn mm2(&self) -> &BitSlice {
        todo!()
    }

    fn mm3(&self) -> &BitSlice {
        todo!()
    }

    fn mm4(&self) -> &BitSlice {
        todo!()
    }

    fn mm5(&self) -> &BitSlice {
        todo!()
    }

    fn mm6(&self) -> &BitSlice {
        todo!()
    }

    fn mm7(&self) -> &BitSlice {
        todo!()
    }

    fn xmm0(&self) -> &BitSlice {
        todo!()
    }

    fn xmm1(&self) -> &BitSlice {
        todo!()
    }

    fn xmm2(&self) -> &BitSlice {
        todo!()
    }

    fn xmm3(&self) -> &BitSlice {
        todo!()
    }

    fn xmm4(&self) -> &BitSlice {
        todo!()
    }

    fn xmm5(&self) -> &BitSlice {
        todo!()
    }

    fn xmm6(&self) -> &BitSlice {
        todo!()
    }

    fn xmm7(&self) -> &BitSlice {
        todo!()
    }

    fn xmm8(&self) -> &BitSlice {
        todo!()
    }

    fn xmm9(&self) -> &BitSlice {
        todo!()
    }

    fn xmm10(&self) -> &BitSlice {
        todo!()
    }

    fn xmm11(&self) -> &BitSlice {
        todo!()
    }

    fn xmm12(&self) -> &BitSlice {
        todo!()
    }

    fn xmm13(&self) -> &BitSlice {
        todo!()
    }

    fn xmm14(&self) -> &BitSlice {
        todo!()
    }

    fn xmm15(&self) -> &BitSlice {
        todo!()
    }

    fn cr0(&self) -> &BitSlice {
        todo!()
    }

    fn cr1(&self) -> &BitSlice {
        todo!()
    }

    fn cr2(&self) -> &BitSlice {
        todo!()
    }

    fn cr3(&self) -> &BitSlice {
        todo!()
    }

    fn cr4(&self) -> &BitSlice {
        todo!()
    }

    fn cr5(&self) -> &BitSlice {
        todo!()
    }

    fn cr6(&self) -> &BitSlice {
        todo!()
    }

    fn cr7(&self) -> &BitSlice {
        todo!()
    }

    fn cr8(&self) -> &BitSlice {
        todo!()
    }

    fn cr9(&self) -> &BitSlice {
        todo!()
    }

    fn cr10(&self) -> &BitSlice {
        todo!()
    }

    fn cr11(&self) -> &BitSlice {
        todo!()
    }

    fn cr12(&self) -> &BitSlice {
        todo!()
    }

    fn cr13(&self) -> &BitSlice {
        todo!()
    }

    fn cr14(&self) -> &BitSlice {
        todo!()
    }

    fn cr15(&self) -> &BitSlice {
        todo!()
    }

    fn dr0(&self) -> &BitSlice {
        todo!()
    }

    fn dr1(&self) -> &BitSlice {
        todo!()
    }

    fn dr2(&self) -> &BitSlice {
        todo!()
    }

    fn dr3(&self) -> &BitSlice {
        todo!()
    }

    fn dr4(&self) -> &BitSlice {
        todo!()
    }

    fn dr5(&self) -> &BitSlice {
        todo!()
    }

    fn dr6(&self) -> &BitSlice {
        todo!()
    }

    fn dr7(&self) -> &BitSlice {
        todo!()
    }

    fn dr8(&self) -> &BitSlice {
        todo!()
    }

    fn dr9(&self) -> &BitSlice {
        todo!()
    }

    fn dr10(&self) -> &BitSlice {
        todo!()
    }

    fn dr11(&self) -> &BitSlice {
        todo!()
    }

    fn dr12(&self) -> &BitSlice {
        todo!()
    }

    fn dr13(&self) -> &BitSlice {
        todo!()
    }

    fn dr14(&self) -> &BitSlice {
        todo!()
    }

    fn dr15(&self) -> &BitSlice {
        todo!()
    }

    fn tmp8(&self) -> &BitSlice {
        todo!()
    }

    fn tmp16(&self) -> &BitSlice {
        todo!()
    }

    fn tmp32(&self) -> &BitSlice {
        todo!()
    }

    fn tmp64(&self) -> &BitSlice {
        todo!()
    }
}
