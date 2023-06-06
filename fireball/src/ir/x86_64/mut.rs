use crate::{
    ir::{
        x86_64::{X64Mut, X64},
        Ir,
    },
    prelude::BitSlice,
};

/// x64_to_mut!(rax);가 들어올 경우 다음을 반환함
/// ```ignore
/// fn rax(&mut self) -> &mut BitSlice {
///     Self::const_bitslice_to_mut(X64::rax(self))
/// }
/// ```
macro_rules! x64_to_mut {
    ($reg:ident) => {
        #[inline(always)]
        fn $reg(&mut self) -> &mut BitSlice {
            Self::const_bitslice_to_mut(X64::$reg(self))
        }
    };
}

impl X64Mut for Ir {
    #[inline(always)]
    fn const_bitslice_to_mut(data: &BitSlice) -> &mut BitSlice {
        unsafe { &mut *(data as *const BitSlice as *mut BitSlice) }
    }

    x64_to_mut!(rax);
    x64_to_mut!(eax);
    x64_to_mut!(ax);
    x64_to_mut!(al);
    x64_to_mut!(ah);

    x64_to_mut!(rbx);
    x64_to_mut!(ebx);
    x64_to_mut!(bx);
    x64_to_mut!(bl);
    x64_to_mut!(bh);

    x64_to_mut!(rcx);
    x64_to_mut!(ecx);
    x64_to_mut!(cx);
    x64_to_mut!(cl);
    x64_to_mut!(ch);

    x64_to_mut!(rdx);
    x64_to_mut!(edx);
    x64_to_mut!(dx);
    x64_to_mut!(dl);
    x64_to_mut!(dh);

    x64_to_mut!(rsp);
    x64_to_mut!(esp);
    x64_to_mut!(sp);
    x64_to_mut!(spl);

    x64_to_mut!(rbp);
    x64_to_mut!(ebp);
    x64_to_mut!(bp);
    x64_to_mut!(bpl);

    x64_to_mut!(rsi);
    x64_to_mut!(esi);
    x64_to_mut!(si);
    x64_to_mut!(sil);

    x64_to_mut!(rdi);
    x64_to_mut!(edi);
    x64_to_mut!(di);
    x64_to_mut!(dil);

    x64_to_mut!(r8);
    x64_to_mut!(r8d);
    x64_to_mut!(r8w);
    x64_to_mut!(r8b);

    x64_to_mut!(r9);
    x64_to_mut!(r9d);
    x64_to_mut!(r9w);
    x64_to_mut!(r9b);

    x64_to_mut!(r10);
    x64_to_mut!(r10d);
    x64_to_mut!(r10w);
    x64_to_mut!(r10b);

    x64_to_mut!(r11);
    x64_to_mut!(r11d);
    x64_to_mut!(r11w);
    x64_to_mut!(r11b);

    x64_to_mut!(r12);
    x64_to_mut!(r12d);
    x64_to_mut!(r12w);
    x64_to_mut!(r12b);

    x64_to_mut!(r13);
    x64_to_mut!(r13d);
    x64_to_mut!(r13w);
    x64_to_mut!(r13b);

    x64_to_mut!(r14);
    x64_to_mut!(r14d);
    x64_to_mut!(r14w);
    x64_to_mut!(r14b);

    x64_to_mut!(r15);
    x64_to_mut!(r15d);
    x64_to_mut!(r15w);
    x64_to_mut!(r15b);

    x64_to_mut!(cs);
    x64_to_mut!(ds);
    x64_to_mut!(es);
    x64_to_mut!(fs);
    x64_to_mut!(gs);
    x64_to_mut!(ss);

    x64_to_mut!(rip);
    x64_to_mut!(eip);
    x64_to_mut!(ip);

    x64_to_mut!(rflags);
    x64_to_mut!(eflags);
    x64_to_mut!(flags);
    x64_to_mut!(cf);
    x64_to_mut!(pf);
    x64_to_mut!(af);
    x64_to_mut!(zf);
    x64_to_mut!(sf);
    x64_to_mut!(tf);
    x64_to_mut!(r#if);
    x64_to_mut!(df);
    x64_to_mut!(of);
    x64_to_mut!(iopl);
    x64_to_mut!(nt);
    x64_to_mut!(rf);
    x64_to_mut!(vm);
    x64_to_mut!(ac);
    x64_to_mut!(vif);
    x64_to_mut!(vip);
    x64_to_mut!(id);

    x64_to_mut!(less);
    x64_to_mut!(less_or_equal);
    x64_to_mut!(below_or_equal);

    x64_to_mut!(fpu_status_word);
    x64_to_mut!(fpu_ie);
    x64_to_mut!(fpu_de);
    x64_to_mut!(fpu_ze);
    x64_to_mut!(fpu_oe);
    x64_to_mut!(fpu_ue);
    x64_to_mut!(fpu_pe);
    x64_to_mut!(fpu_sf);
    x64_to_mut!(fpu_es);
    x64_to_mut!(fpu_c0);
    x64_to_mut!(fpu_c1);
    x64_to_mut!(fpu_c2);
    x64_to_mut!(fpu_top);
    x64_to_mut!(fpu_c3);
    x64_to_mut!(fpu_b);

    x64_to_mut!(st0);
    x64_to_mut!(st1);
    x64_to_mut!(st2);
    x64_to_mut!(st3);
    x64_to_mut!(st4);
    x64_to_mut!(st5);
    x64_to_mut!(st6);
    x64_to_mut!(st7);

    x64_to_mut!(mm0);
    x64_to_mut!(mm1);
    x64_to_mut!(mm2);
    x64_to_mut!(mm3);
    x64_to_mut!(mm4);
    x64_to_mut!(mm5);
    x64_to_mut!(mm6);
    x64_to_mut!(mm7);

    x64_to_mut!(xmm0);
    x64_to_mut!(xmm1);
    x64_to_mut!(xmm2);
    x64_to_mut!(xmm3);
    x64_to_mut!(xmm4);
    x64_to_mut!(xmm5);
    x64_to_mut!(xmm6);
    x64_to_mut!(xmm7);
    x64_to_mut!(xmm8);
    x64_to_mut!(xmm9);
    x64_to_mut!(xmm10);
    x64_to_mut!(xmm11);
    x64_to_mut!(xmm12);
    x64_to_mut!(xmm13);
    x64_to_mut!(xmm14);
    x64_to_mut!(xmm15);

    x64_to_mut!(cr0);
    x64_to_mut!(cr1);
    x64_to_mut!(cr2);
    x64_to_mut!(cr3);
    x64_to_mut!(cr4);
    x64_to_mut!(cr5);
    x64_to_mut!(cr6);
    x64_to_mut!(cr7);
    x64_to_mut!(cr8);
    x64_to_mut!(cr9);
    x64_to_mut!(cr10);
    x64_to_mut!(cr11);
    x64_to_mut!(cr12);
    x64_to_mut!(cr13);
    x64_to_mut!(cr14);
    x64_to_mut!(cr15);

    x64_to_mut!(dr0);
    x64_to_mut!(dr1);
    x64_to_mut!(dr2);
    x64_to_mut!(dr3);
    x64_to_mut!(dr4);
    x64_to_mut!(dr5);
    x64_to_mut!(dr6);
    x64_to_mut!(dr7);
    x64_to_mut!(dr8);
    x64_to_mut!(dr9);
    x64_to_mut!(dr10);
    x64_to_mut!(dr11);
    x64_to_mut!(dr12);
    x64_to_mut!(dr13);
    x64_to_mut!(dr14);
    x64_to_mut!(dr15);

    x64_to_mut!(tmp8);
    x64_to_mut!(tmp16);
    x64_to_mut!(tmp32);
    x64_to_mut!(tmp64);
}
