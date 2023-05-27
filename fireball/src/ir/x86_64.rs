//! x86_64 CPU 컴퓨터를 IR구조로 변환하는데 사용되는 서브모듈입니다.

use crate::prelude::BitSlice;

/// X64(32비트 포함) 레지스터를 가져오는 인터페이스입니다.
pub trait X64 {
    fn new() -> Self;

    fn rax(&self) -> &BitSlice;
    fn eax(&self) -> &BitSlice;
    fn ax(&self) -> &BitSlice;
    fn al(&self) -> &BitSlice;
    fn ah(&self) -> &BitSlice;

    fn rbx(&self) -> &BitSlice;
    fn ebx(&self) -> &BitSlice;
    fn bx(&self) -> &BitSlice;
    fn bl(&self) -> &BitSlice;
    fn bh(&self) -> &BitSlice;

    fn rcx(&self) -> &BitSlice;
    fn ecx(&self) -> &BitSlice;
    fn cx(&self) -> &BitSlice;
    fn cl(&self) -> &BitSlice;
    fn ch(&self) -> &BitSlice;

    fn rdx(&self) -> &BitSlice;
    fn edx(&self) -> &BitSlice;
    fn dx(&self) -> &BitSlice;
    fn dl(&self) -> &BitSlice;
    fn dh(&self) -> &BitSlice;

    fn rsp(&self) -> &BitSlice;
    fn esp(&self) -> &BitSlice;
    fn sp(&self) -> &BitSlice;
    fn spl(&self) -> &BitSlice;

    fn rbp(&self) -> &BitSlice;
    fn ebp(&self) -> &BitSlice;
    fn bp(&self) -> &BitSlice;
    fn bpl(&self) -> &BitSlice;

    fn rsi(&self) -> &BitSlice;
    fn esi(&self) -> &BitSlice;
    fn si(&self) -> &BitSlice;
    fn sil(&self) -> &BitSlice;

    fn rdi(&self) -> &BitSlice;
    fn edi(&self) -> &BitSlice;
    fn di(&self) -> &BitSlice;
    fn dil(&self) -> &BitSlice;

    fn r8(&self) -> &BitSlice;
    fn r8d(&self) -> &BitSlice;
    fn r8w(&self) -> &BitSlice;
    fn r8b(&self) -> &BitSlice;

    fn r9(&self) -> &BitSlice;
    fn r9d(&self) -> &BitSlice;
    fn r9w(&self) -> &BitSlice;
    fn r9b(&self) -> &BitSlice;

    fn r10(&self) -> &BitSlice;
    fn r10d(&self) -> &BitSlice;
    fn r10w(&self) -> &BitSlice;
    fn r10b(&self) -> &BitSlice;

    fn r11(&self) -> &BitSlice;
    fn r11d(&self) -> &BitSlice;
    fn r11w(&self) -> &BitSlice;
    fn r11b(&self) -> &BitSlice;

    fn r12(&self) -> &BitSlice;
    fn r12d(&self) -> &BitSlice;
    fn r12w(&self) -> &BitSlice;
    fn r12b(&self) -> &BitSlice;

    fn r13(&self) -> &BitSlice;
    fn r13d(&self) -> &BitSlice;
    fn r13w(&self) -> &BitSlice;
    fn r13b(&self) -> &BitSlice;

    fn r14(&self) -> &BitSlice;
    fn r14d(&self) -> &BitSlice;
    fn r14w(&self) -> &BitSlice;
    fn r14b(&self) -> &BitSlice;

    fn r15(&self) -> &BitSlice;
    fn r15d(&self) -> &BitSlice;
    fn r15w(&self) -> &BitSlice;
    fn r15b(&self) -> &BitSlice;

    fn cs(&self) -> &BitSlice;
    fn ds(&self) -> &BitSlice;
    fn es(&self) -> &BitSlice;
    fn fs(&self) -> &BitSlice;
    fn gs(&self) -> &BitSlice;
    fn ss(&self) -> &BitSlice;

    fn rip(&self) -> &BitSlice;
    fn eip(&self) -> &BitSlice;
    fn ip(&self) -> &BitSlice;

    fn rflags(&self) -> &BitSlice;
    fn eflags(&self) -> &BitSlice;
    fn flags(&self) -> &BitSlice;
    fn cf(&self) -> &BitSlice;
    fn pf(&self) -> &BitSlice;
    fn af(&self) -> &BitSlice;
    fn zf(&self) -> &BitSlice;
    fn sf(&self) -> &BitSlice;
    fn tf(&self) -> &BitSlice;
    fn r#if(&self) -> &BitSlice;
    fn df(&self) -> &BitSlice;
    fn of(&self) -> &BitSlice;
    fn iopl(&self) -> &BitSlice;
    fn nt(&self) -> &BitSlice;
    fn rf(&self) -> &BitSlice;
    fn vm(&self) -> &BitSlice;
    fn ac(&self) -> &BitSlice;
    fn vif(&self) -> &BitSlice;
    fn vip(&self) -> &BitSlice;
    fn id(&self) -> &BitSlice;

    fn less(&self) -> &BitSlice;
    fn less_or_equal(&self) -> &BitSlice;
    fn below_or_equal(&self) -> &BitSlice;

    fn fpu_status_word(&self) -> &BitSlice;
    fn fpu_ie(&self) -> &BitSlice;
    fn fpu_de(&self) -> &BitSlice;
    fn fpu_ze(&self) -> &BitSlice;
    fn fpu_oe(&self) -> &BitSlice;
    fn fpu_ue(&self) -> &BitSlice;
    fn fpu_pe(&self) -> &BitSlice;
    fn fpu_sf(&self) -> &BitSlice;
    fn fpu_es(&self) -> &BitSlice;
    fn fpu_c0(&self) -> &BitSlice;
    fn fpu_c1(&self) -> &BitSlice;
    fn fpu_c2(&self) -> &BitSlice;
    fn fpu_top(&self) -> &BitSlice;
    fn fpu_c3(&self) -> &BitSlice;
    fn fpu_b(&self) -> &BitSlice;

    fn st0(&self) -> &BitSlice;
    fn st1(&self) -> &BitSlice;
    fn st2(&self) -> &BitSlice;
    fn st3(&self) -> &BitSlice;
    fn st4(&self) -> &BitSlice;
    fn st5(&self) -> &BitSlice;
    fn st6(&self) -> &BitSlice;
    fn st7(&self) -> &BitSlice;

    fn mm0(&self) -> &BitSlice;
    fn mm1(&self) -> &BitSlice;
    fn mm2(&self) -> &BitSlice;
    fn mm3(&self) -> &BitSlice;
    fn mm4(&self) -> &BitSlice;
    fn mm5(&self) -> &BitSlice;
    fn mm6(&self) -> &BitSlice;
    fn mm7(&self) -> &BitSlice;

    fn xmm0(&self) -> &BitSlice;
    fn xmm1(&self) -> &BitSlice;
    fn xmm2(&self) -> &BitSlice;
    fn xmm3(&self) -> &BitSlice;
    fn xmm4(&self) -> &BitSlice;
    fn xmm5(&self) -> &BitSlice;
    fn xmm6(&self) -> &BitSlice;
    fn xmm7(&self) -> &BitSlice;
    fn xmm8(&self) -> &BitSlice;
    fn xmm9(&self) -> &BitSlice;
    fn xmm10(&self) -> &BitSlice;
    fn xmm11(&self) -> &BitSlice;
    fn xmm12(&self) -> &BitSlice;
    fn xmm13(&self) -> &BitSlice;
    fn xmm14(&self) -> &BitSlice;
    fn xmm15(&self) -> &BitSlice;

    fn cr0(&self) -> &BitSlice;
    fn cr1(&self) -> &BitSlice;
    fn cr2(&self) -> &BitSlice;
    fn cr3(&self) -> &BitSlice;
    fn cr4(&self) -> &BitSlice;
    fn cr5(&self) -> &BitSlice;
    fn cr6(&self) -> &BitSlice;
    fn cr7(&self) -> &BitSlice;
    fn cr8(&self) -> &BitSlice;
    fn cr9(&self) -> &BitSlice;
    fn cr10(&self) -> &BitSlice;
    fn cr11(&self) -> &BitSlice;
    fn cr12(&self) -> &BitSlice;
    fn cr13(&self) -> &BitSlice;
    fn cr14(&self) -> &BitSlice;
    fn cr15(&self) -> &BitSlice;

    fn dr0(&self) -> &BitSlice;
    fn dr1(&self) -> &BitSlice;
    fn dr2(&self) -> &BitSlice;
    fn dr3(&self) -> &BitSlice;
    fn dr4(&self) -> &BitSlice;
    fn dr5(&self) -> &BitSlice;
    fn dr6(&self) -> &BitSlice;
    fn dr7(&self) -> &BitSlice;
    fn dr8(&self) -> &BitSlice;
    fn dr9(&self) -> &BitSlice;
    fn dr10(&self) -> &BitSlice;
    fn dr11(&self) -> &BitSlice;
    fn dr12(&self) -> &BitSlice;
    fn dr13(&self) -> &BitSlice;
    fn dr14(&self) -> &BitSlice;
    fn dr15(&self) -> &BitSlice;

    fn tmp8(&self) -> &BitSlice;
    fn tmp16(&self) -> &BitSlice;
    fn tmp32(&self) -> &BitSlice;
    fn tmp64(&self) -> &BitSlice;
}

mod base;
