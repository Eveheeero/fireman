/// From intel manual, chapter 5. Instruction Set Summary
///
/// ### register <-> hex transform documentation
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=43>
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=44>
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=45>
///
/// - [Opcode definition](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=112)
/// - [Instruction definition](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=115)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X64Statement {
    #[doc = include_str!("../../doc/intel/AAA.md")]
    Aaa,
    #[doc = include_str!("../../doc/intel/AAD.md")]
    Aad,
    #[doc = include_str!("../../doc/intel/AAM.md")]
    Aam,
    #[doc = include_str!("../../doc/intel/AAS.md")]
    Aas,
    #[doc = include_str!("../../doc/intel/ADC.md")]
    Adc,
    #[doc = include_str!("../../doc/intel/ADCX.md")]
    Adcx,
    #[doc = include_str!("../../doc/intel/ADD.md")]
    Add,
    #[doc = include_str!("../../doc/intel/ADDPD.md")]
    Addpd,
    #[doc = include_str!("../../doc/intel/ADDPS.md")]
    Addps,
    #[doc = include_str!("../../doc/intel/ADDSD.md")]
    Addsd,
    #[doc = include_str!("../../doc/intel/ADDSS.md")]
    Addss,
    #[doc = include_str!("../../doc/intel/ADDSUBPD.md")]
    Addsubpd,
    #[doc = include_str!("../../doc/intel/ADDSUBPS.md")]
    Addsubps,
    #[doc = include_str!("../../doc/intel/ADOX.md")]
    Adox,
    #[doc = include_str!("../../doc/intel/AESDEC.md")]
    Aesdec,
    #[doc = include_str!("../../doc/intel/AESDEC128KL.md")]
    Aesdec128kl,
    #[doc = include_str!("../../doc/intel/AESDEC256KL.md")]
    Aesdec256kl,
    #[doc = include_str!("../../doc/intel/AESDECLAST.md")]
    Aesdeclast,
    #[doc = include_str!("../../doc/intel/AESDECWIDE128KL.md")]
    Aesdecwide128kl,
    #[doc = include_str!("../../doc/intel/AESDECWIDE256KL.md")]
    Aesdecwide256kl,
    #[doc = include_str!("../../doc/intel/AESENC.md")]
    Aesenc,
    #[doc = include_str!("../../doc/intel/AESENC128KL.md")]
    Aesenc128kl,
    #[doc = include_str!("../../doc/intel/AESENC256KL.md")]
    Aesenc256kl,
    #[doc = include_str!("../../doc/intel/AESENCLAST.md")]
    Aesenclast,
    #[doc = include_str!("../../doc/intel/AESENCWIDE128KL.md")]
    Aesencwide128kl,
    #[doc = include_str!("../../doc/intel/AESENCWIDE256KL.md")]
    Aesencwide256kl,
    #[doc = include_str!("../../doc/intel/AESIMC.md")]
    Aesimc,
    #[doc = include_str!("../../doc/intel/AESKEYGENASSIST.md")]
    Aeskeygenassist,
    #[doc = include_str!("../../doc/intel/AND.md")]
    And,
    #[doc = include_str!("../../doc/intel/ANDN.md")]
    Andn,
    #[doc = include_str!("../../doc/intel/ANDNPD.md")]
    Andnpd,
    #[doc = include_str!("../../doc/intel/ANDNPS.md")]
    Andnps,
    #[doc = include_str!("../../doc/intel/ANDPD.md")]
    Andpd,
    #[doc = include_str!("../../doc/intel/ANDPS.md")]
    Andps,
    #[doc = include_str!("../../doc/intel/ARPL.md")]
    Arpl,
    #[doc = include_str!("../../doc/intel/BEXTR.md")]
    Bextr,
    #[doc = include_str!("../../doc/intel/BLENDPD.md")]
    Blendpd,
    #[doc = include_str!("../../doc/intel/BLENDPS.md")]
    Blendps,
    #[doc = include_str!("../../doc/intel/BLENDVPD.md")]
    Blendvpd,
    #[doc = include_str!("../../doc/intel/BLENDVPS.md")]
    Blendvps,
    #[doc = include_str!("../../doc/intel/BLSI.md")]
    Blsi,
    #[doc = include_str!("../../doc/intel/BLSMSK.md")]
    Blsmsk,
    #[doc = include_str!("../../doc/intel/BLSR.md")]
    Blsr,
    #[doc = include_str!("../../doc/intel/BNDCL.md")]
    Bndcl,
    #[doc = include_str!("../../doc/intel/BNDCN.md")]
    Bndcn,
    #[doc = include_str!("../../doc/intel/BNDCU.md")]
    Bndcu,
    #[doc = include_str!("../../doc/intel/BNDLDX.md")]
    Bndldx,
    #[doc = include_str!("../../doc/intel/BNDMK.md")]
    Bndmk,
    #[doc = include_str!("../../doc/intel/BNDMOV.md")]
    Bndmov,
    #[doc = include_str!("../../doc/intel/BNDSTX.md")]
    Bndstx,
    #[doc = include_str!("../../doc/intel/BOUND.md")]
    Bound,
    #[doc = include_str!("../../doc/intel/BSF.md")]
    Bsf,
    #[doc = include_str!("../../doc/intel/BSR.md")]
    Bsr,
    #[doc = include_str!("../../doc/intel/BSWAP.md")]
    Bswap,
    #[doc = include_str!("../../doc/intel/BT.md")]
    Bt,
    #[doc = include_str!("../../doc/intel/BTC.md")]
    Btc,
    #[doc = include_str!("../../doc/intel/BTR.md")]
    Btr,
    #[doc = include_str!("../../doc/intel/BTS.md")]
    Bts,
    #[doc = include_str!("../../doc/intel/BZHI.md")]
    Bzhi,
    #[doc = include_str!("../../doc/intel/CALL.md")]
    Call,
    #[doc = include_str!("../../doc/intel/CBW.md")]
    Cbw,
    #[doc = include_str!("../../doc/intel/CDQ.md")]
    Cdq,
    #[doc = include_str!("../../doc/intel/CDQE.md")]
    Cdqe,
    #[doc = include_str!("../../doc/intel/CLAC.md")]
    Clac,
    #[doc = include_str!("../../doc/intel/CLC.md")]
    Clc,
    #[doc = include_str!("../../doc/intel/CLD.md")]
    Cld,
    #[doc = include_str!("../../doc/intel/CLDEMOTE.md")]
    Cldemote,
    #[doc = include_str!("../../doc/intel/CLFLUSH.md")]
    Clflush,
    #[doc = include_str!("../../doc/intel/CLFLUSHOPT.md")]
    Clflushopt,
    #[doc = include_str!("../../doc/intel/CLI.md")]
    Cli,
    #[doc = include_str!("../../doc/intel/CLRSSBSY.md")]
    Clrssbsy,
    #[doc = include_str!("../../doc/intel/CLTS.md")]
    Clts,
    #[doc = include_str!("../../doc/intel/CLUI.md")]
    Clui,
    #[doc = include_str!("../../doc/intel/CLWB.md")]
    Clwb,
    #[doc = include_str!("../../doc/intel/CMC.md")]
    Cmc,
    #[doc = include_str!("../../doc/intel/CMOVcc.md")]
    Cmovcc,
    #[doc = include_str!("../../doc/intel/CMP.md")]
    Cmp,
    #[doc = include_str!("../../doc/intel/CMPPD.md")]
    Cmppd,
    #[doc = include_str!("../../doc/intel/CMPPS.md")]
    Cmpps,
    #[doc = include_str!("../../doc/intel/CMPS.md")]
    Cmps,
    #[doc = include_str!("../../doc/intel/CMPSB.md")]
    Cmpsb,
    #[doc = include_str!("../../doc/intel/CMPSD.md")]
    Cmpsd,
    #[doc = include_str!("../../doc/intel/CMPSQ.md")]
    Cmpsq,
    #[doc = include_str!("../../doc/intel/CMPSS.md")]
    Cmpss,
    #[doc = include_str!("../../doc/intel/CMPSW.md")]
    Cmpsw,
    #[doc = include_str!("../../doc/intel/CMPXCHG.md")]
    Cmpxchg,
    #[doc = include_str!("../../doc/intel/CMPXCHG16B.md")]
    Cmpxchg16b,
    #[doc = include_str!("../../doc/intel/CMPXCHG8B.md")]
    Cmpxchg8b,
    #[doc = include_str!("../../doc/intel/COMISD.md")]
    Comisd,
    #[doc = include_str!("../../doc/intel/COMISS.md")]
    Comiss,
    #[doc = include_str!("../../doc/intel/CPUID.md")]
    Cpuid,
    #[doc = include_str!("../../doc/intel/CQO.md")]
    Cqo,
    #[doc = include_str!("../../doc/intel/CRC32.md")]
    Crc32,
    #[doc = include_str!("../../doc/intel/CVTDQ2PD.md")]
    Cvtdq2pd,
    #[doc = include_str!("../../doc/intel/CVTDQ2PS.md")]
    Cvtdq2ps,
    #[doc = include_str!("../../doc/intel/CVTPD2DQ.md")]
    Cvtpd2dq,
    #[doc = include_str!("../../doc/intel/CVTPD2PI.md")]
    Cvtpd2pi,
    #[doc = include_str!("../../doc/intel/CVTPD2PS.md")]
    Cvtpd2ps,
    #[doc = include_str!("../../doc/intel/CVTPI2PD.md")]
    Cvtpi2pd,
    #[doc = include_str!("../../doc/intel/CVTPI2PS.md")]
    Cvtpi2ps,
    #[doc = include_str!("../../doc/intel/CVTPS2DQ.md")]
    Cvtps2dq,
    #[doc = include_str!("../../doc/intel/CVTPS2PD.md")]
    Cvtps2pd,
    #[doc = include_str!("../../doc/intel/CVTPS2PI.md")]
    Cvtps2pi,
    #[doc = include_str!("../../doc/intel/CVTSD2SI.md")]
    Cvtsd2si,
    #[doc = include_str!("../../doc/intel/CVTSD2SS.md")]
    Cvtsd2ss,
    #[doc = include_str!("../../doc/intel/CVTSI2SD.md")]
    Cvtsi2sd,
    #[doc = include_str!("../../doc/intel/CVTSI2SS.md")]
    Cvtsi2ss,
    #[doc = include_str!("../../doc/intel/CVTSS2SD.md")]
    Cvtss2sd,
    #[doc = include_str!("../../doc/intel/CVTSS2SI.md")]
    Cvtss2si,
    #[doc = include_str!("../../doc/intel/CVTTPD2DQ.md")]
    Cvttpd2dq,
    #[doc = include_str!("../../doc/intel/CVTTPD2PI.md")]
    Cvttpd2pi,
    #[doc = include_str!("../../doc/intel/CVTTPS2DQ.md")]
    Cvttps2dq,
    #[doc = include_str!("../../doc/intel/CVTTPS2PI.md")]
    Cvttps2pi,
    #[doc = include_str!("../../doc/intel/CVTTSD2SI.md")]
    Cvttsd2si,
    #[doc = include_str!("../../doc/intel/CVTTSS2SI.md")]
    Cvttss2si,
    #[doc = include_str!("../../doc/intel/CWD.md")]
    Cwd,
    #[doc = include_str!("../../doc/intel/CWDE.md")]
    Cwde,
    #[doc = include_str!("../../doc/intel/DAA.md")]
    Daa,
    #[doc = include_str!("../../doc/intel/DAS.md")]
    Das,
    #[doc = include_str!("../../doc/intel/DEC.md")]
    Dec,
    #[doc = include_str!("../../doc/intel/DIV.md")]
    Div,
    #[doc = include_str!("../../doc/intel/DIVPD.md")]
    Divpd,
    #[doc = include_str!("../../doc/intel/DIVPS.md")]
    Divps,
    #[doc = include_str!("../../doc/intel/DIVSD.md")]
    Divsd,
    #[doc = include_str!("../../doc/intel/DIVSS.md")]
    Divss,
    #[doc = include_str!("../../doc/intel/DPPD.md")]
    Dppd,
    #[doc = include_str!("../../doc/intel/DPPS.md")]
    Dpps,
    #[doc = include_str!("../../doc/intel/EMMS.md")]
    Emms,
    #[doc = include_str!("../../doc/intel/ENCODEKEY128.md")]
    Encodekey128,
    #[doc = include_str!("../../doc/intel/ENCODEKEY256.md")]
    Encodekey256,
    #[doc = include_str!("../../doc/intel/ENDBR32.md")]
    Endbr32,
    #[doc = include_str!("../../doc/intel/ENDBR64.md")]
    Endbr64,
    #[doc = include_str!("../../doc/intel/ENQCMD.md")]
    Enqcmd,
    #[doc = include_str!("../../doc/intel/ENQCMDS.md")]
    Enqcmds,
    #[doc = include_str!("../../doc/intel/ENTER.md")]
    Enter,
    #[doc = include_str!("../../doc/intel/EXTRACTPS.md")]
    Extractps,
    #[doc = include_str!("../../doc/intel/F2XM1.md")]
    F2xm1,
    #[doc = include_str!("../../doc/intel/FABS.md")]
    Fabs,
    #[doc = include_str!("../../doc/intel/FADD.md")]
    Fadd,
    #[doc = include_str!("../../doc/intel/FADDP.md")]
    Faddp,
    #[doc = include_str!("../../doc/intel/FBLD.md")]
    Fbld,
    #[doc = include_str!("../../doc/intel/FBSTP.md")]
    Fbstp,
    #[doc = include_str!("../../doc/intel/FCHS.md")]
    Fchs,
    #[doc = include_str!("../../doc/intel/FCLEX.md")]
    Fclex,
    #[doc = include_str!("../../doc/intel/FCMOVcc.md")]
    Fcmovcc,
    #[doc = include_str!("../../doc/intel/FCOM.md")]
    Fcom,
    #[doc = include_str!("../../doc/intel/FCOMI.md")]
    Fcomi,
    #[doc = include_str!("../../doc/intel/FCOMIP.md")]
    Fcomip,
    #[doc = include_str!("../../doc/intel/FCOMP.md")]
    Fcomp,
    #[doc = include_str!("../../doc/intel/FCOMPP.md")]
    Fcompp,
    #[doc = include_str!("../../doc/intel/FCOS.md")]
    Fcos,
    #[doc = include_str!("../../doc/intel/FDECSTP.md")]
    Fdecstp,
    #[doc = include_str!("../../doc/intel/FDIV.md")]
    Fdiv,
    #[doc = include_str!("../../doc/intel/FDIVP.md")]
    Fdivp,
    #[doc = include_str!("../../doc/intel/FDIVR.md")]
    Fdivr,
    #[doc = include_str!("../../doc/intel/FDIVRP.md")]
    Fdivrp,
    #[doc = include_str!("../../doc/intel/FFREE.md")]
    Ffree,
    #[doc = include_str!("../../doc/intel/FIADD.md")]
    Fiadd,
    #[doc = include_str!("../../doc/intel/FICOM.md")]
    Ficom,
    #[doc = include_str!("../../doc/intel/FICOMP.md")]
    Ficomp,
    #[doc = include_str!("../../doc/intel/FIDIV.md")]
    Fidiv,
    #[doc = include_str!("../../doc/intel/FIDIVR.md")]
    Fidivr,
    #[doc = include_str!("../../doc/intel/FILD.md")]
    Fild,
    #[doc = include_str!("../../doc/intel/FIMUL.md")]
    Fimul,
    #[doc = include_str!("../../doc/intel/FINCSTP.md")]
    Fincstp,
    #[doc = include_str!("../../doc/intel/FINIT.md")]
    Finit,
    #[doc = include_str!("../../doc/intel/FIST.md")]
    Fist,
    #[doc = include_str!("../../doc/intel/FISTP.md")]
    Fistp,
    #[doc = include_str!("../../doc/intel/FISTTP.md")]
    Fisttp,
    #[doc = include_str!("../../doc/intel/FISUB.md")]
    Fisub,
    #[doc = include_str!("../../doc/intel/FISUBR.md")]
    Fisubr,
    #[doc = include_str!("../../doc/intel/FLD.md")]
    Fld,
    #[doc = include_str!("../../doc/intel/FLD1.md")]
    Fld1,
    #[doc = include_str!("../../doc/intel/FLDCW.md")]
    Fldcw,
    #[doc = include_str!("../../doc/intel/FLDENV.md")]
    Fldenv,
    #[doc = include_str!("../../doc/intel/FLDL2E.md")]
    Fldl2e,
    #[doc = include_str!("../../doc/intel/FLDL2T.md")]
    Fldl2t,
    #[doc = include_str!("../../doc/intel/FLDLG2.md")]
    Fldlg2,
    #[doc = include_str!("../../doc/intel/FLDLN2.md")]
    Fldln2,
    #[doc = include_str!("../../doc/intel/FLDPI.md")]
    Fldpi,
    #[doc = include_str!("../../doc/intel/FLDZ.md")]
    Fldz,
    #[doc = include_str!("../../doc/intel/FMUL.md")]
    Fmul,
    #[doc = include_str!("../../doc/intel/FMULP.md")]
    Fmulp,
    #[doc = include_str!("../../doc/intel/FNCLEX.md")]
    Fnclex,
    #[doc = include_str!("../../doc/intel/FNINIT.md")]
    Fninit,
    #[doc = include_str!("../../doc/intel/FNOP.md")]
    Fnop,
    #[doc = include_str!("../../doc/intel/FNSAVE.md")]
    Fnsave,
    #[doc = include_str!("../../doc/intel/FNSTCW.md")]
    Fnstcw,
    #[doc = include_str!("../../doc/intel/FNSTENV.md")]
    Fnstenv,
    #[doc = include_str!("../../doc/intel/FNSTSW.md")]
    Fnstsw,
    #[doc = include_str!("../../doc/intel/FPATAN.md")]
    Fpatan,
    #[doc = include_str!("../../doc/intel/FPREM.md")]
    Fprem,
    #[doc = include_str!("../../doc/intel/FPREM1.md")]
    Fprem1,
    #[doc = include_str!("../../doc/intel/FPTAN.md")]
    Fptan,
    #[doc = include_str!("../../doc/intel/FRNDINT.md")]
    Frndint,
    #[doc = include_str!("../../doc/intel/FRSTOR.md")]
    Frstor,
    #[doc = include_str!("../../doc/intel/FSAVE.md")]
    Fsave,
    #[doc = include_str!("../../doc/intel/FSCALE.md")]
    Fscale,
    #[doc = include_str!("../../doc/intel/FSIN.md")]
    Fsin,
    #[doc = include_str!("../../doc/intel/FSINCOS.md")]
    Fsincos,
    #[doc = include_str!("../../doc/intel/FSQRT.md")]
    Fsqrt,
    #[doc = include_str!("../../doc/intel/FST.md")]
    Fst,
    #[doc = include_str!("../../doc/intel/FSTCW.md")]
    Fstcw,
    #[doc = include_str!("../../doc/intel/FSTENV.md")]
    Fstenv,
    #[doc = include_str!("../../doc/intel/FSTP.md")]
    Fstp,
    #[doc = include_str!("../../doc/intel/FSTSW.md")]
    Fstsw,
    #[doc = include_str!("../../doc/intel/FSUB.md")]
    Fsub,
    #[doc = include_str!("../../doc/intel/FSUBP.md")]
    Fsubp,
    #[doc = include_str!("../../doc/intel/FSUBR.md")]
    Fsubr,
    #[doc = include_str!("../../doc/intel/FSUBRP.md")]
    Fsubrp,
    #[doc = include_str!("../../doc/intel/FTST.md")]
    Ftst,
    #[doc = include_str!("../../doc/intel/FUCOM.md")]
    Fucom,
    #[doc = include_str!("../../doc/intel/FUCOMI.md")]
    Fucomi,
    #[doc = include_str!("../../doc/intel/FUCOMIP.md")]
    Fucomip,
    #[doc = include_str!("../../doc/intel/FUCOMP.md")]
    Fucomp,
    #[doc = include_str!("../../doc/intel/FUCOMPP.md")]
    Fucompp,
    #[doc = include_str!("../../doc/intel/FWAIT.md")]
    Fwait,
    #[doc = include_str!("../../doc/intel/FXAM.md")]
    Fxam,
    #[doc = include_str!("../../doc/intel/FXCH.md")]
    Fxch,
    #[doc = include_str!("../../doc/intel/FXRSTOR.md")]
    Fxrstor,
    #[doc = include_str!("../../doc/intel/FXSAVE.md")]
    Fxsave,
    #[doc = include_str!("../../doc/intel/FXTRACT.md")]
    Fxtract,
    #[doc = include_str!("../../doc/intel/FYL2X.md")]
    Fyl2x,
    #[doc = include_str!("../../doc/intel/FYL2XP1.md")]
    Fyl2xp1,
    #[doc = include_str!("../../doc/intel/GF2P8AFFINEINVQB.md")]
    Gf2p8affineinvqb,
    #[doc = include_str!("../../doc/intel/GF2P8AFFINEQB.md")]
    Gf2p8affineqb,
    #[doc = include_str!("../../doc/intel/GF2P8MULB.md")]
    Gf2p8mulb,
    #[doc = include_str!("../../doc/intel/HADDPD.md")]
    Haddpd,
    #[doc = include_str!("../../doc/intel/HADDPS.md")]
    Haddps,
    #[doc = include_str!("../../doc/intel/HLT.md")]
    Hlt,
    #[doc = include_str!("../../doc/intel/HRESET.md")]
    Hreset,
    #[doc = include_str!("../../doc/intel/HSUBPD.md")]
    Hsubpd,
    #[doc = include_str!("../../doc/intel/HSUBPS.md")]
    Hsubps,
    #[doc = include_str!("../../doc/intel/IDIV.md")]
    Idiv,
    #[doc = include_str!("../../doc/intel/IMUL.md")]
    Imul,
    #[doc = include_str!("../../doc/intel/IN.md")]
    In,
    #[doc = include_str!("../../doc/intel/INC.md")]
    Inc,
    #[doc = include_str!("../../doc/intel/INCSSPD.md")]
    Incsspd,
    #[doc = include_str!("../../doc/intel/INCSSPQ.md")]
    Incsspq,
    #[doc = include_str!("../../doc/intel/INS.md")]
    Ins,
    #[doc = include_str!("../../doc/intel/INSB.md")]
    Insb,
    #[doc = include_str!("../../doc/intel/INSD.md")]
    Insd,
    #[doc = include_str!("../../doc/intel/INSERTPS.md")]
    Insertps,
    #[doc = include_str!("../../doc/intel/INSW.md")]
    Insw,
    #[doc = include_str!("../../doc/intel/INT.md")]
    Int,
    #[doc = include_str!("../../doc/intel/INT1.md")]
    Int1,
    #[doc = include_str!("../../doc/intel/INT3.md")]
    Int3,
    #[doc = include_str!("../../doc/intel/INTO.md")]
    Into,
    #[doc = include_str!("../../doc/intel/INVD.md")]
    Invd,
    #[doc = include_str!("../../doc/intel/INVLPG.md")]
    Invlpg,
    #[doc = include_str!("../../doc/intel/INVPCID.md")]
    Invpcid,
    #[doc = include_str!("../../doc/intel/IRET.md")]
    Iret,
    #[doc = include_str!("../../doc/intel/IRETD.md")]
    Iretd,
    #[doc = include_str!("../../doc/intel/IRETQ.md")]
    Iretq,
    #[doc = include_str!("../../doc/intel/JMP.md")]
    Jmp,
    #[doc = include_str!("../../doc/intel/Jcc.md")]
    Jcc,
    #[doc = include_str!("../../doc/intel/KADDB.md")]
    Kaddb,
    #[doc = include_str!("../../doc/intel/KADDD.md")]
    Kaddd,
    #[doc = include_str!("../../doc/intel/KADDQ.md")]
    Kaddq,
    #[doc = include_str!("../../doc/intel/KADDW.md")]
    Kaddw,
    #[doc = include_str!("../../doc/intel/KANDB.md")]
    Kandb,
    #[doc = include_str!("../../doc/intel/KANDD.md")]
    Kandd,
    #[doc = include_str!("../../doc/intel/KANDNB.md")]
    Kandnb,
    #[doc = include_str!("../../doc/intel/KANDND.md")]
    Kandnd,
    #[doc = include_str!("../../doc/intel/KANDNQ.md")]
    Kandnq,
    #[doc = include_str!("../../doc/intel/KANDNW.md")]
    Kandnw,
    #[doc = include_str!("../../doc/intel/KANDQ.md")]
    Kandq,
    #[doc = include_str!("../../doc/intel/KANDW.md")]
    Kandw,
    #[doc = include_str!("../../doc/intel/KMOVB.md")]
    Kmovb,
    #[doc = include_str!("../../doc/intel/KMOVD.md")]
    Kmovd,
    #[doc = include_str!("../../doc/intel/KMOVQ.md")]
    Kmovq,
    #[doc = include_str!("../../doc/intel/KMOVW.md")]
    Kmovw,
    #[doc = include_str!("../../doc/intel/KNOTB.md")]
    Knotb,
    #[doc = include_str!("../../doc/intel/KNOTD.md")]
    Knotd,
    #[doc = include_str!("../../doc/intel/KNOTQ.md")]
    Knotq,
    #[doc = include_str!("../../doc/intel/KNOTW.md")]
    Knotw,
    #[doc = include_str!("../../doc/intel/KORB.md")]
    Korb,
    #[doc = include_str!("../../doc/intel/KORD.md")]
    Kord,
    #[doc = include_str!("../../doc/intel/KORQ.md")]
    Korq,
    #[doc = include_str!("../../doc/intel/KORTESTB.md")]
    Kortestb,
    #[doc = include_str!("../../doc/intel/KORTESTD.md")]
    Kortestd,
    #[doc = include_str!("../../doc/intel/KORTESTQ.md")]
    Kortestq,
    #[doc = include_str!("../../doc/intel/KORTESTW.md")]
    Kortestw,
    #[doc = include_str!("../../doc/intel/KORW.md")]
    Korw,
    #[doc = include_str!("../../doc/intel/KSHIFTLB.md")]
    Kshiftlb,
    #[doc = include_str!("../../doc/intel/KSHIFTLD.md")]
    Kshiftld,
    #[doc = include_str!("../../doc/intel/KSHIFTLQ.md")]
    Kshiftlq,
    #[doc = include_str!("../../doc/intel/KSHIFTLW.md")]
    Kshiftlw,
    #[doc = include_str!("../../doc/intel/KSHIFTRB.md")]
    Kshiftrb,
    #[doc = include_str!("../../doc/intel/KSHIFTRD.md")]
    Kshiftrd,
    #[doc = include_str!("../../doc/intel/KSHIFTRQ.md")]
    Kshiftrq,
    #[doc = include_str!("../../doc/intel/KSHIFTRW.md")]
    Kshiftrw,
    #[doc = include_str!("../../doc/intel/KTESTB.md")]
    Ktestb,
    #[doc = include_str!("../../doc/intel/KTESTD.md")]
    Ktestd,
    #[doc = include_str!("../../doc/intel/KTESTQ.md")]
    Ktestq,
    #[doc = include_str!("../../doc/intel/KTESTW.md")]
    Ktestw,
    #[doc = include_str!("../../doc/intel/KUNPCKBW.md")]
    Kunpckbw,
    #[doc = include_str!("../../doc/intel/KUNPCKDQ.md")]
    Kunpckdq,
    #[doc = include_str!("../../doc/intel/KUNPCKWD.md")]
    Kunpckwd,
    #[doc = include_str!("../../doc/intel/KXNORB.md")]
    Kxnorb,
    #[doc = include_str!("../../doc/intel/KXNORD.md")]
    Kxnord,
    #[doc = include_str!("../../doc/intel/KXNORQ.md")]
    Kxnorq,
    #[doc = include_str!("../../doc/intel/KXNORW.md")]
    Kxnorw,
    #[doc = include_str!("../../doc/intel/KXORB.md")]
    Kxorb,
    #[doc = include_str!("../../doc/intel/KXORD.md")]
    Kxord,
    #[doc = include_str!("../../doc/intel/KXORQ.md")]
    Kxorq,
    #[doc = include_str!("../../doc/intel/KXORW.md")]
    Kxorw,
    #[doc = include_str!("../../doc/intel/LAHF.md")]
    Lahf,
    #[doc = include_str!("../../doc/intel/LAR.md")]
    Lar,
    #[doc = include_str!("../../doc/intel/LDDQU.md")]
    Lddqu,
    #[doc = include_str!("../../doc/intel/LDMXCSR.md")]
    Ldmxcsr,
    #[doc = include_str!("../../doc/intel/LDS.md")]
    Lds,
    #[doc = include_str!("../../doc/intel/LDTILECFG.md")]
    Ldtilecfg,
    #[doc = include_str!("../../doc/intel/LEA.md")]
    Lea,
    #[doc = include_str!("../../doc/intel/LEAVE.md")]
    Leave,
    #[doc = include_str!("../../doc/intel/LES.md")]
    Les,
    #[doc = include_str!("../../doc/intel/LFENCE.md")]
    Lfence,
    #[doc = include_str!("../../doc/intel/LFS.md")]
    Lfs,
    #[doc = include_str!("../../doc/intel/LGDT.md")]
    Lgdt,
    #[doc = include_str!("../../doc/intel/LGS.md")]
    Lgs,
    #[doc = include_str!("../../doc/intel/LIDT.md")]
    Lidt,
    #[doc = include_str!("../../doc/intel/LLDT.md")]
    Lldt,
    #[doc = include_str!("../../doc/intel/LMSW.md")]
    Lmsw,
    #[doc = include_str!("../../doc/intel/LOADIWKEY.md")]
    Loadiwkey,
    #[doc = include_str!("../../doc/intel/LOCK.md")]
    Lock,
    #[doc = include_str!("../../doc/intel/LODS.md")]
    Lods,
    #[doc = include_str!("../../doc/intel/LODSB.md")]
    Lodsb,
    #[doc = include_str!("../../doc/intel/LODSD.md")]
    Lodsd,
    #[doc = include_str!("../../doc/intel/LODSQ.md")]
    Lodsq,
    #[doc = include_str!("../../doc/intel/LODSW.md")]
    Lodsw,
    #[doc = include_str!("../../doc/intel/LOOP.md")]
    Loop,
    #[doc = include_str!("../../doc/intel/LOOPcc.md")]
    Loopcc,
    #[doc = include_str!("../../doc/intel/LSL.md")]
    Lsl,
    #[doc = include_str!("../../doc/intel/LSS.md")]
    Lss,
    #[doc = include_str!("../../doc/intel/LTR.md")]
    Ltr,
    #[doc = include_str!("../../doc/intel/MASKMOVDQU.md")]
    Maskmovdqu,
    #[doc = include_str!("../../doc/intel/MASKMOVQ.md")]
    Maskmovq,
    #[doc = include_str!("../../doc/intel/MAXPD.md")]
    Maxpd,
    #[doc = include_str!("../../doc/intel/MAXPS.md")]
    Maxps,
    #[doc = include_str!("../../doc/intel/MAXSD.md")]
    Maxsd,
    #[doc = include_str!("../../doc/intel/MAXSS.md")]
    Maxss,
    #[doc = include_str!("../../doc/intel/MFENCE.md")]
    Mfence,
    #[doc = include_str!("../../doc/intel/MINPD.md")]
    Minpd,
    #[doc = include_str!("../../doc/intel/MINPS.md")]
    Minps,
    #[doc = include_str!("../../doc/intel/MINSD.md")]
    Minsd,
    #[doc = include_str!("../../doc/intel/MINSS.md")]
    Minss,
    #[doc = include_str!("../../doc/intel/MONITOR.md")]
    Monitor,
    #[doc = include_str!("../../doc/intel/MOV.md")]
    Mov,
    #[doc = include_str!("../../doc/intel/MOVAPD.md")]
    Movapd,
    #[doc = include_str!("../../doc/intel/MOVAPS.md")]
    Movaps,
    #[doc = include_str!("../../doc/intel/MOVBE.md")]
    Movbe,
    #[doc = include_str!("../../doc/intel/MOVD.md")]
    Movd,
    #[doc = include_str!("../../doc/intel/MOVDDUP.md")]
    Movddup,
    #[doc = include_str!("../../doc/intel/MOVDIR64B.md")]
    Movdir64b,
    #[doc = include_str!("../../doc/intel/MOVDIRI.md")]
    Movdiri,
    #[doc = include_str!("../../doc/intel/MOVDQ16.md")]
    Movdq16,
    #[doc = include_str!("../../doc/intel/MOVDQ2Q.md")]
    Movdq2q,
    #[doc = include_str!("../../doc/intel/MOVDQ32.md")]
    Movdq32,
    #[doc = include_str!("../../doc/intel/MOVDQ64.md")]
    Movdq64,
    #[doc = include_str!("../../doc/intel/MOVDQA.md")]
    Movdqa,
    #[doc = include_str!("../../doc/intel/MOVDQU.md")]
    Movdqu,
    #[doc = include_str!("../../doc/intel/MOVHLPS.md")]
    Movhlps,
    #[doc = include_str!("../../doc/intel/MOVHPD.md")]
    Movhpd,
    #[doc = include_str!("../../doc/intel/MOVHPS.md")]
    Movhps,
    #[doc = include_str!("../../doc/intel/MOVLHPS.md")]
    Movlhps,
    #[doc = include_str!("../../doc/intel/MOVLPD.md")]
    Movlpd,
    #[doc = include_str!("../../doc/intel/MOVLPS.md")]
    Movlps,
    #[doc = include_str!("../../doc/intel/MOVMSKPD.md")]
    Movmskpd,
    #[doc = include_str!("../../doc/intel/MOVMSKPS.md")]
    Movmskps,
    #[doc = include_str!("../../doc/intel/MOVNTDQ.md")]
    Movntdq,
    #[doc = include_str!("../../doc/intel/MOVNTDQA.md")]
    Movntdqa,
    #[doc = include_str!("../../doc/intel/MOVNTI.md")]
    Movnti,
    #[doc = include_str!("../../doc/intel/MOVNTPD.md")]
    Movntpd,
    #[doc = include_str!("../../doc/intel/MOVNTPS.md")]
    Movntps,
    #[doc = include_str!("../../doc/intel/MOVNTQ.md")]
    Movntq,
    #[doc = include_str!("../../doc/intel/MOVQ.md")]
    Movq,
    #[doc = include_str!("../../doc/intel/MOVQ2DQ.md")]
    Movq2dq,
    #[doc = include_str!("../../doc/intel/MOVS.md")]
    Movs,
    #[doc = include_str!("../../doc/intel/MOVSB.md")]
    Movsb,
    #[doc = include_str!("../../doc/intel/MOVSD.md")]
    Movsd,
    #[doc = include_str!("../../doc/intel/MOVSHDUP.md")]
    Movshdup,
    #[doc = include_str!("../../doc/intel/MOVSLDUP.md")]
    Movsldup,
    #[doc = include_str!("../../doc/intel/MOVSQ.md")]
    Movsq,
    #[doc = include_str!("../../doc/intel/MOVSS.md")]
    Movss,
    #[doc = include_str!("../../doc/intel/MOVSW.md")]
    Movsw,
    #[doc = include_str!("../../doc/intel/MOVSX.md")]
    Movsx,
    #[doc = include_str!("../../doc/intel/MOVSXD.md")]
    Movsxd,
    #[doc = include_str!("../../doc/intel/MOVUPD.md")]
    Movupd,
    #[doc = include_str!("../../doc/intel/MOVUPS.md")]
    Movups,
    #[doc = include_str!("../../doc/intel/MOVZX.md")]
    Movzx,
    #[doc = include_str!("../../doc/intel/MPSADBW.md")]
    Mpsadbw,
    #[doc = include_str!("../../doc/intel/MUL.md")]
    Mul,
    #[doc = include_str!("../../doc/intel/MULPD.md")]
    Mulpd,
    #[doc = include_str!("../../doc/intel/MULPS.md")]
    Mulps,
    #[doc = include_str!("../../doc/intel/MULSD.md")]
    Mulsd,
    #[doc = include_str!("../../doc/intel/MULSS.md")]
    Mulss,
    #[doc = include_str!("../../doc/intel/MULX.md")]
    Mulx,
    #[doc = include_str!("../../doc/intel/MWAIT.md")]
    Mwait,
    #[doc = include_str!("../../doc/intel/NEG.md")]
    Neg,
    #[doc = include_str!("../../doc/intel/NOP.md")]
    Nop,
    #[doc = include_str!("../../doc/intel/NOT.md")]
    Not,
    #[doc = include_str!("../../doc/intel/OR.md")]
    Or,
    #[doc = include_str!("../../doc/intel/ORPD.md")]
    Orpd,
    #[doc = include_str!("../../doc/intel/ORPS.md")]
    Orps,
    #[doc = include_str!("../../doc/intel/OUT.md")]
    Out,
    #[doc = include_str!("../../doc/intel/OUTS.md")]
    Outs,
    #[doc = include_str!("../../doc/intel/OUTSB.md")]
    Outsb,
    #[doc = include_str!("../../doc/intel/OUTSD.md")]
    Outsd,
    #[doc = include_str!("../../doc/intel/OUTSW.md")]
    Outsw,
    #[doc = include_str!("../../doc/intel/PABSB.md")]
    Pabsb,
    #[doc = include_str!("../../doc/intel/PABSD.md")]
    Pabsd,
    #[doc = include_str!("../../doc/intel/PABSQ.md")]
    Pabsq,
    #[doc = include_str!("../../doc/intel/PABSW.md")]
    Pabsw,
    #[doc = include_str!("../../doc/intel/PACKSSDW.md")]
    Packssdw,
    #[doc = include_str!("../../doc/intel/PACKSSWB.md")]
    Packsswb,
    #[doc = include_str!("../../doc/intel/PACKUSDW.md")]
    Packusdw,
    #[doc = include_str!("../../doc/intel/PACKUSWB.md")]
    Packuswb,
    #[doc = include_str!("../../doc/intel/PADDB.md")]
    Paddb,
    #[doc = include_str!("../../doc/intel/PADDD.md")]
    Paddd,
    #[doc = include_str!("../../doc/intel/PADDQ.md")]
    Paddq,
    #[doc = include_str!("../../doc/intel/PADDSB.md")]
    Paddsb,
    #[doc = include_str!("../../doc/intel/PADDSW.md")]
    Paddsw,
    #[doc = include_str!("../../doc/intel/PADDUSB.md")]
    Paddusb,
    #[doc = include_str!("../../doc/intel/PADDUSW.md")]
    Paddusw,
    #[doc = include_str!("../../doc/intel/PADDW.md")]
    Paddw,
    #[doc = include_str!("../../doc/intel/PALIGNR.md")]
    Palignr,
    #[doc = include_str!("../../doc/intel/PAND.md")]
    Pand,
    #[doc = include_str!("../../doc/intel/PANDN.md")]
    Pandn,
    #[doc = include_str!("../../doc/intel/PAUSE.md")]
    Pause,
    #[doc = include_str!("../../doc/intel/PAVGB.md")]
    Pavgb,
    #[doc = include_str!("../../doc/intel/PAVGW.md")]
    Pavgw,
    #[doc = include_str!("../../doc/intel/PBLENDVB.md")]
    Pblendvb,
    #[doc = include_str!("../../doc/intel/PBLENDW.md")]
    Pblendw,
    #[doc = include_str!("../../doc/intel/PCLMULQDQ.md")]
    Pclmulqdq,
    #[doc = include_str!("../../doc/intel/PCMPEQB.md")]
    Pcmpeqb,
    #[doc = include_str!("../../doc/intel/PCMPEQD.md")]
    Pcmpeqd,
    #[doc = include_str!("../../doc/intel/PCMPEQQ.md")]
    Pcmpeqq,
    #[doc = include_str!("../../doc/intel/PCMPEQW.md")]
    Pcmpeqw,
    #[doc = include_str!("../../doc/intel/PCMPESTRI.md")]
    Pcmpestri,
    #[doc = include_str!("../../doc/intel/PCMPESTRM.md")]
    Pcmpestrm,
    #[doc = include_str!("../../doc/intel/PCMPGTB.md")]
    Pcmpgtb,
    #[doc = include_str!("../../doc/intel/PCMPGTD.md")]
    Pcmpgtd,
    #[doc = include_str!("../../doc/intel/PCMPGTQ.md")]
    Pcmpgtq,
    #[doc = include_str!("../../doc/intel/PCMPGTW.md")]
    Pcmpgtw,
    #[doc = include_str!("../../doc/intel/PCMPISTRI.md")]
    Pcmpistri,
    #[doc = include_str!("../../doc/intel/PCMPISTRM.md")]
    Pcmpistrm,
    #[doc = include_str!("../../doc/intel/PCONFIG.md")]
    Pconfig,
    #[doc = include_str!("../../doc/intel/PDEP.md")]
    Pdep,
    #[doc = include_str!("../../doc/intel/PEXT.md")]
    Pext,
    #[doc = include_str!("../../doc/intel/PEXTRB.md")]
    Pextrb,
    #[doc = include_str!("../../doc/intel/PEXTRD.md")]
    Pextrd,
    #[doc = include_str!("../../doc/intel/PEXTRQ.md")]
    Pextrq,
    #[doc = include_str!("../../doc/intel/PEXTRW.md")]
    Pextrw,
    #[doc = include_str!("../../doc/intel/PHADDD.md")]
    Phaddd,
    #[doc = include_str!("../../doc/intel/PHADDSW.md")]
    Phaddsw,
    #[doc = include_str!("../../doc/intel/PHADDW.md")]
    Phaddw,
    #[doc = include_str!("../../doc/intel/PHMINPOSUW.md")]
    Phminposuw,
    #[doc = include_str!("../../doc/intel/PHSUBD.md")]
    Phsubd,
    #[doc = include_str!("../../doc/intel/PHSUBSW.md")]
    Phsubsw,
    #[doc = include_str!("../../doc/intel/PHSUBW.md")]
    Phsubw,
    #[doc = include_str!("../../doc/intel/PINSRB.md")]
    Pinsrb,
    #[doc = include_str!("../../doc/intel/PINSRD.md")]
    Pinsrd,
    #[doc = include_str!("../../doc/intel/PINSRQ.md")]
    Pinsrq,
    #[doc = include_str!("../../doc/intel/PINSRW.md")]
    Pinsrw,
    #[doc = include_str!("../../doc/intel/PMADDUBSW.md")]
    Pmaddubsw,
    #[doc = include_str!("../../doc/intel/PMADDWD.md")]
    Pmaddwd,
    #[doc = include_str!("../../doc/intel/PMAXSB.md")]
    Pmaxsb,
    #[doc = include_str!("../../doc/intel/PMAXSD.md")]
    Pmaxsd,
    #[doc = include_str!("../../doc/intel/PMAXSQ.md")]
    Pmaxsq,
    #[doc = include_str!("../../doc/intel/PMAXSW.md")]
    Pmaxsw,
    #[doc = include_str!("../../doc/intel/PMAXUB.md")]
    Pmaxub,
    #[doc = include_str!("../../doc/intel/PMAXUD.md")]
    Pmaxud,
    #[doc = include_str!("../../doc/intel/PMAXUQ.md")]
    Pmaxuq,
    #[doc = include_str!("../../doc/intel/PMAXUW.md")]
    Pmaxuw,
    #[doc = include_str!("../../doc/intel/PMINSB.md")]
    Pminsb,
    #[doc = include_str!("../../doc/intel/PMINSD.md")]
    Pminsd,
    #[doc = include_str!("../../doc/intel/PMINSQ.md")]
    Pminsq,
    #[doc = include_str!("../../doc/intel/PMINSW.md")]
    Pminsw,
    #[doc = include_str!("../../doc/intel/PMINUB.md")]
    Pminub,
    #[doc = include_str!("../../doc/intel/PMINUD.md")]
    Pminud,
    #[doc = include_str!("../../doc/intel/PMINUQ.md")]
    Pminuq,
    #[doc = include_str!("../../doc/intel/PMINUW.md")]
    Pminuw,
    #[doc = include_str!("../../doc/intel/PMOVMSKB.md")]
    Pmovmskb,
    #[doc = include_str!("../../doc/intel/PMOVSX.md")]
    Pmovsx,
    #[doc = include_str!("../../doc/intel/PMOVZX.md")]
    Pmovzx,
    #[doc = include_str!("../../doc/intel/PMULDQ.md")]
    Pmuldq,
    #[doc = include_str!("../../doc/intel/PMULHRSW.md")]
    Pmulhrsw,
    #[doc = include_str!("../../doc/intel/PMULHUW.md")]
    Pmulhuw,
    #[doc = include_str!("../../doc/intel/PMULHW.md")]
    Pmulhw,
    #[doc = include_str!("../../doc/intel/PMULLD.md")]
    Pmulld,
    #[doc = include_str!("../../doc/intel/PMULLQ.md")]
    Pmullq,
    #[doc = include_str!("../../doc/intel/PMULLW.md")]
    Pmullw,
    #[doc = include_str!("../../doc/intel/PMULUDQ.md")]
    Pmuludq,
    #[doc = include_str!("../../doc/intel/POP.md")]
    Pop,
    #[doc = include_str!("../../doc/intel/POPA.md")]
    Popa,
    #[doc = include_str!("../../doc/intel/POPAD.md")]
    Popad,
    #[doc = include_str!("../../doc/intel/POPCNT.md")]
    Popcnt,
    #[doc = include_str!("../../doc/intel/POPF.md")]
    Popf,
    #[doc = include_str!("../../doc/intel/POPFD.md")]
    Popfd,
    #[doc = include_str!("../../doc/intel/POPFQ.md")]
    Popfq,
    #[doc = include_str!("../../doc/intel/POR.md")]
    Por,
    #[doc = include_str!("../../doc/intel/PREFETCHW.md")]
    Prefetchw,
    #[doc = include_str!("../../doc/intel/PREFETCHh.md")]
    Prefetchh,
    #[doc = include_str!("../../doc/intel/PSADBW.md")]
    Psadbw,
    #[doc = include_str!("../../doc/intel/PSHUFB.md")]
    Pshufb,
    #[doc = include_str!("../../doc/intel/PSHUFD.md")]
    Pshufd,
    #[doc = include_str!("../../doc/intel/PSHUFHW.md")]
    Pshufhw,
    #[doc = include_str!("../../doc/intel/PSHUFLW.md")]
    Pshuflw,
    #[doc = include_str!("../../doc/intel/PSHUFW.md")]
    Pshufw,
    #[doc = include_str!("../../doc/intel/PSIGNB.md")]
    Psignb,
    #[doc = include_str!("../../doc/intel/PSIGND.md")]
    Psignd,
    #[doc = include_str!("../../doc/intel/PSIGNW.md")]
    Psignw,
    #[doc = include_str!("../../doc/intel/PSLLD.md")]
    Pslld,
    #[doc = include_str!("../../doc/intel/PSLLDQ.md")]
    Pslldq,
    #[doc = include_str!("../../doc/intel/PSLLQ.md")]
    Psllq,
    #[doc = include_str!("../../doc/intel/PSLLW.md")]
    Psllw,
    #[doc = include_str!("../../doc/intel/PSRAD.md")]
    Psrad,
    #[doc = include_str!("../../doc/intel/PSRAQ.md")]
    Psraq,
    #[doc = include_str!("../../doc/intel/PSRAW.md")]
    Psraw,
    #[doc = include_str!("../../doc/intel/PSRLD.md")]
    Psrld,
    #[doc = include_str!("../../doc/intel/PSRLDQ.md")]
    Psrldq,
    #[doc = include_str!("../../doc/intel/PSRLQ.md")]
    Psrlq,
    #[doc = include_str!("../../doc/intel/PSRLW.md")]
    Psrlw,
    #[doc = include_str!("../../doc/intel/PSUBB.md")]
    Psubb,
    #[doc = include_str!("../../doc/intel/PSUBD.md")]
    Psubd,
    #[doc = include_str!("../../doc/intel/PSUBQ.md")]
    Psubq,
    #[doc = include_str!("../../doc/intel/PSUBSB.md")]
    Psubsb,
    #[doc = include_str!("../../doc/intel/PSUBSW.md")]
    Psubsw,
    #[doc = include_str!("../../doc/intel/PSUBUSB.md")]
    Psubusb,
    #[doc = include_str!("../../doc/intel/PSUBUSW.md")]
    Psubusw,
    #[doc = include_str!("../../doc/intel/PSUBW.md")]
    Psubw,
    #[doc = include_str!("../../doc/intel/PTEST.md")]
    Ptest,
    #[doc = include_str!("../../doc/intel/PTWRITE.md")]
    Ptwrite,
    #[doc = include_str!("../../doc/intel/PUNPCKHBW.md")]
    Punpckhbw,
    #[doc = include_str!("../../doc/intel/PUNPCKHDQ.md")]
    Punpckhdq,
    #[doc = include_str!("../../doc/intel/PUNPCKHQDQ.md")]
    Punpckhqdq,
    #[doc = include_str!("../../doc/intel/PUNPCKHWD.md")]
    Punpckhwd,
    #[doc = include_str!("../../doc/intel/PUNPCKLBW.md")]
    Punpcklbw,
    #[doc = include_str!("../../doc/intel/PUNPCKLDQ.md")]
    Punpckldq,
    #[doc = include_str!("../../doc/intel/PUNPCKLQDQ.md")]
    Punpcklqdq,
    #[doc = include_str!("../../doc/intel/PUNPCKLWD.md")]
    Punpcklwd,
    #[doc = include_str!("../../doc/intel/PUSH.md")]
    Push,
    #[doc = include_str!("../../doc/intel/PUSHA.md")]
    Pusha,
    #[doc = include_str!("../../doc/intel/PUSHAD.md")]
    Pushad,
    #[doc = include_str!("../../doc/intel/PUSHF.md")]
    Pushf,
    #[doc = include_str!("../../doc/intel/PUSHFD.md")]
    Pushfd,
    #[doc = include_str!("../../doc/intel/PUSHFQ.md")]
    Pushfq,
    #[doc = include_str!("../../doc/intel/PXOR.md")]
    Pxor,
    #[doc = include_str!("../../doc/intel/RCL.md")]
    Rcl,
    #[doc = include_str!("../../doc/intel/RCPPS.md")]
    Rcpps,
    #[doc = include_str!("../../doc/intel/RCPSS.md")]
    Rcpss,
    #[doc = include_str!("../../doc/intel/RCR.md")]
    Rcr,
    #[doc = include_str!("../../doc/intel/RDFSBASE.md")]
    Rdfsbase,
    #[doc = include_str!("../../doc/intel/RDGSBASE.md")]
    Rdgsbase,
    #[doc = include_str!("../../doc/intel/RDMSR.md")]
    Rdmsr,
    #[doc = include_str!("../../doc/intel/RDPID.md")]
    Rdpid,
    #[doc = include_str!("../../doc/intel/RDPKRU.md")]
    Rdpkru,
    #[doc = include_str!("../../doc/intel/RDPMC.md")]
    Rdpmc,
    #[doc = include_str!("../../doc/intel/RDRAND.md")]
    Rdrand,
    #[doc = include_str!("../../doc/intel/RDSEED.md")]
    Rdseed,
    #[doc = include_str!("../../doc/intel/RDSSPD.md")]
    Rdsspd,
    #[doc = include_str!("../../doc/intel/RDSSPQ.md")]
    Rdsspq,
    #[doc = include_str!("../../doc/intel/RDTSC.md")]
    Rdtsc,
    #[doc = include_str!("../../doc/intel/RDTSCP.md")]
    Rdtscp,
    #[doc = include_str!("../../doc/intel/REP.md")]
    Rep,
    #[doc = include_str!("../../doc/intel/REPE.md")]
    Repe,
    #[doc = include_str!("../../doc/intel/REPNE.md")]
    Repne,
    #[doc = include_str!("../../doc/intel/REPNZ.md")]
    Repnz,
    #[doc = include_str!("../../doc/intel/REPZ.md")]
    Repz,
    #[doc = include_str!("../../doc/intel/RET.md")]
    Ret,
    #[doc = include_str!("../../doc/intel/ROL.md")]
    Rol,
    #[doc = include_str!("../../doc/intel/ROR.md")]
    Ror,
    #[doc = include_str!("../../doc/intel/RORX.md")]
    Rorx,
    #[doc = include_str!("../../doc/intel/ROUNDPD.md")]
    Roundpd,
    #[doc = include_str!("../../doc/intel/ROUNDPS.md")]
    Roundps,
    #[doc = include_str!("../../doc/intel/ROUNDSD.md")]
    Roundsd,
    #[doc = include_str!("../../doc/intel/ROUNDSS.md")]
    Roundss,
    #[doc = include_str!("../../doc/intel/RSM.md")]
    Rsm,
    #[doc = include_str!("../../doc/intel/RSQRTPS.md")]
    Rsqrtps,
    #[doc = include_str!("../../doc/intel/RSQRTSS.md")]
    Rsqrtss,
    #[doc = include_str!("../../doc/intel/RSTORSSP.md")]
    Rstorssp,
    #[doc = include_str!("../../doc/intel/SAHF.md")]
    Sahf,
    #[doc = include_str!("../../doc/intel/SAL.md")]
    Sal,
    #[doc = include_str!("../../doc/intel/SAR.md")]
    Sar,
    #[doc = include_str!("../../doc/intel/SARX.md")]
    Sarx,
    #[doc = include_str!("../../doc/intel/SAVEPREVSSP.md")]
    Saveprevssp,
    #[doc = include_str!("../../doc/intel/SBB.md")]
    Sbb,
    #[doc = include_str!("../../doc/intel/SCAS.md")]
    Scas,
    #[doc = include_str!("../../doc/intel/SCASB.md")]
    Scasb,
    #[doc = include_str!("../../doc/intel/SCASD.md")]
    Scasd,
    #[doc = include_str!("../../doc/intel/SCASW.md")]
    Scasw,
    #[doc = include_str!("../../doc/intel/SENDUIPI.md")]
    Senduipi,
    #[doc = include_str!("../../doc/intel/SERIALIZE.md")]
    Serialize,
    #[doc = include_str!("../../doc/intel/SETSSBSY.md")]
    Setssbsy,
    #[doc = include_str!("../../doc/intel/SETcc.md")]
    Setcc,
    #[doc = include_str!("../../doc/intel/SFENCE.md")]
    Sfence,
    #[doc = include_str!("../../doc/intel/SGDT.md")]
    Sgdt,
    #[doc = include_str!("../../doc/intel/SHA1MSG1.md")]
    Sha1msg1,
    #[doc = include_str!("../../doc/intel/SHA1MSG2.md")]
    Sha1msg2,
    #[doc = include_str!("../../doc/intel/SHA1NEXTE.md")]
    Sha1nexte,
    #[doc = include_str!("../../doc/intel/SHA1RNDS4.md")]
    Sha1rnds4,
    #[doc = include_str!("../../doc/intel/SHA256MSG1.md")]
    Sha256msg1,
    #[doc = include_str!("../../doc/intel/SHA256MSG2.md")]
    Sha256msg2,
    #[doc = include_str!("../../doc/intel/SHA256RNDS2.md")]
    Sha256rnds2,
    #[doc = include_str!("../../doc/intel/SHL.md")]
    Shl,
    #[doc = include_str!("../../doc/intel/SHLD.md")]
    Shld,
    #[doc = include_str!("../../doc/intel/SHLX.md")]
    Shlx,
    #[doc = include_str!("../../doc/intel/SHR.md")]
    Shr,
    #[doc = include_str!("../../doc/intel/SHRD.md")]
    Shrd,
    #[doc = include_str!("../../doc/intel/SHRX.md")]
    Shrx,
    #[doc = include_str!("../../doc/intel/SHUFPD.md")]
    Shufpd,
    #[doc = include_str!("../../doc/intel/SHUFPS.md")]
    Shufps,
    #[doc = include_str!("../../doc/intel/SIDT.md")]
    Sidt,
    #[doc = include_str!("../../doc/intel/SLDT.md")]
    Sldt,
    #[doc = include_str!("../../doc/intel/SMSW.md")]
    Smsw,
    #[doc = include_str!("../../doc/intel/SQRTPD.md")]
    Sqrtpd,
    #[doc = include_str!("../../doc/intel/SQRTPS.md")]
    Sqrtps,
    #[doc = include_str!("../../doc/intel/SQRTSD.md")]
    Sqrtsd,
    #[doc = include_str!("../../doc/intel/SQRTSS.md")]
    Sqrtss,
    #[doc = include_str!("../../doc/intel/STAC.md")]
    Stac,
    #[doc = include_str!("../../doc/intel/STC.md")]
    Stc,
    #[doc = include_str!("../../doc/intel/STD.md")]
    Std,
    #[doc = include_str!("../../doc/intel/STI.md")]
    Sti,
    #[doc = include_str!("../../doc/intel/STMXCSR.md")]
    Stmxcsr,
    #[doc = include_str!("../../doc/intel/STOS.md")]
    Stos,
    #[doc = include_str!("../../doc/intel/STOSB.md")]
    Stosb,
    #[doc = include_str!("../../doc/intel/STOSD.md")]
    Stosd,
    #[doc = include_str!("../../doc/intel/STOSQ.md")]
    Stosq,
    #[doc = include_str!("../../doc/intel/STOSW.md")]
    Stosw,
    #[doc = include_str!("../../doc/intel/STR.md")]
    Str,
    #[doc = include_str!("../../doc/intel/STTILECFG.md")]
    Sttilecfg,
    #[doc = include_str!("../../doc/intel/STUI.md")]
    Stui,
    #[doc = include_str!("../../doc/intel/SUB.md")]
    Sub,
    #[doc = include_str!("../../doc/intel/SUBPD.md")]
    Subpd,
    #[doc = include_str!("../../doc/intel/SUBPS.md")]
    Subps,
    #[doc = include_str!("../../doc/intel/SUBSD.md")]
    Subsd,
    #[doc = include_str!("../../doc/intel/SUBSS.md")]
    Subss,
    #[doc = include_str!("../../doc/intel/SWAPGS.md")]
    Swapgs,
    #[doc = include_str!("../../doc/intel/SYSCALL.md")]
    Syscall,
    #[doc = include_str!("../../doc/intel/SYSENTER.md")]
    Sysenter,
    #[doc = include_str!("../../doc/intel/SYSEXIT.md")]
    Sysexit,
    #[doc = include_str!("../../doc/intel/SYSRET.md")]
    Sysret,
    #[doc = include_str!("../../doc/intel/TDPBF16PS.md")]
    Tdpbf16ps,
    #[doc = include_str!("../../doc/intel/TDPBSSD.md")]
    Tdpbssd,
    #[doc = include_str!("../../doc/intel/TDPBSUD.md")]
    Tdpbsud,
    #[doc = include_str!("../../doc/intel/TDPBUSD.md")]
    Tdpbusd,
    #[doc = include_str!("../../doc/intel/TDPBUUD.md")]
    Tdpbuud,
    #[doc = include_str!("../../doc/intel/TEST.md")]
    Test,
    #[doc = include_str!("../../doc/intel/TESTUI.md")]
    Testui,
    #[doc = include_str!("../../doc/intel/TILELOADD.md")]
    Tileloadd,
    #[doc = include_str!("../../doc/intel/TILELOADDT1.md")]
    Tileloaddt1,
    #[doc = include_str!("../../doc/intel/TILERELEASE.md")]
    Tilerelease,
    #[doc = include_str!("../../doc/intel/TILESTORED.md")]
    Tilestored,
    #[doc = include_str!("../../doc/intel/TILEZERO.md")]
    Tilezero,
    #[doc = include_str!("../../doc/intel/TPAUSE.md")]
    Tpause,
    #[doc = include_str!("../../doc/intel/TZCNT.md")]
    Tzcnt,
    #[doc = include_str!("../../doc/intel/UCOMISD.md")]
    Ucomisd,
    #[doc = include_str!("../../doc/intel/UCOMISS.md")]
    Ucomiss,
    #[doc = include_str!("../../doc/intel/UD.md")]
    Ud,
    #[doc = include_str!("../../doc/intel/UIRET.md")]
    Uiret,
    #[doc = include_str!("../../doc/intel/UMONITOR.md")]
    Umonitor,
    #[doc = include_str!("../../doc/intel/UMWAIT.md")]
    Umwait,
    #[doc = include_str!("../../doc/intel/UNPCKHPD.md")]
    Unpckhpd,
    #[doc = include_str!("../../doc/intel/UNPCKHPS.md")]
    Unpckhps,
    #[doc = include_str!("../../doc/intel/UNPCKLPD.md")]
    Unpcklpd,
    #[doc = include_str!("../../doc/intel/VADDPH.md")]
    Vaddph,
    #[doc = include_str!("../../doc/intel/VADDSH.md")]
    Vaddsh,
    #[doc = include_str!("../../doc/intel/VALIGND.md")]
    Valignd,
    #[doc = include_str!("../../doc/intel/VALIGNQ.md")]
    Valignq,
    #[doc = include_str!("../../doc/intel/VBLENDMPD.md")]
    Vblendmpd,
    #[doc = include_str!("../../doc/intel/VBLENDMPS.md")]
    Vblendmps,
    #[doc = include_str!("../../doc/intel/VBROADCAST.md")]
    Vbroadcast,
    #[doc = include_str!("../../doc/intel/VCMPPH.md")]
    Vcmpph,
    #[doc = include_str!("../../doc/intel/VCMPSH.md")]
    Vcmpsh,
    #[doc = include_str!("../../doc/intel/VCOMISH.md")]
    Vcomish,
    #[doc = include_str!("../../doc/intel/VCOMPRESSPD.md")]
    Vcompresspd,
    #[doc = include_str!("../../doc/intel/VCOMPRESSPS.md")]
    Vcompressps,
    #[doc = include_str!("../../doc/intel/VCOMPRESSW.md")]
    Vcompressw,
    #[doc = include_str!("../../doc/intel/VCVTDQ2PH.md")]
    Vcvtdq2ph,
    #[doc = include_str!("../../doc/intel/VCVTNE2PS2BF16.md")]
    Vcvtne2ps2bf16,
    #[doc = include_str!("../../doc/intel/VCVTNEPS2BF16.md")]
    Vcvtneps2bf16,
    #[doc = include_str!("../../doc/intel/VCVTPD2PH.md")]
    Vcvtpd2ph,
    #[doc = include_str!("../../doc/intel/VCVTPD2QQ.md")]
    Vcvtpd2qq,
    #[doc = include_str!("../../doc/intel/VCVTPD2UDQ.md")]
    Vcvtpd2udq,
    #[doc = include_str!("../../doc/intel/VCVTPD2UQQ.md")]
    Vcvtpd2uqq,
    #[doc = include_str!("../../doc/intel/VCVTPH2DQ.md")]
    Vcvtph2dq,
    #[doc = include_str!("../../doc/intel/VCVTPH2PD.md")]
    Vcvtph2pd,
    #[doc = include_str!("../../doc/intel/VCVTPH2PS.md")]
    Vcvtph2ps,
    #[doc = include_str!("../../doc/intel/VCVTPH2PSX.md")]
    Vcvtph2psx,
    #[doc = include_str!("../../doc/intel/VCVTPH2QQ.md")]
    Vcvtph2qq,
    #[doc = include_str!("../../doc/intel/VCVTPH2UDQ.md")]
    Vcvtph2udq,
    #[doc = include_str!("../../doc/intel/VCVTPH2UQQ.md")]
    Vcvtph2uqq,
    #[doc = include_str!("../../doc/intel/VCVTPH2UW.md")]
    Vcvtph2uw,
    #[doc = include_str!("../../doc/intel/VCVTPH2W.md")]
    Vcvtph2w,
    #[doc = include_str!("../../doc/intel/VCVTPS2PH.md")]
    Vcvtps2ph,
    #[doc = include_str!("../../doc/intel/VCVTPS2PHX.md")]
    Vcvtps2phx,
    #[doc = include_str!("../../doc/intel/VCVTPS2QQ.md")]
    Vcvtps2qq,
    #[doc = include_str!("../../doc/intel/VCVTPS2UDQ.md")]
    Vcvtps2udq,
    #[doc = include_str!("../../doc/intel/VCVTPS2UQQ.md")]
    Vcvtps2uqq,
    #[doc = include_str!("../../doc/intel/VCVTQQ2PD.md")]
    Vcvtqq2pd,
    #[doc = include_str!("../../doc/intel/VCVTQQ2PH.md")]
    Vcvtqq2ph,
    #[doc = include_str!("../../doc/intel/VCVTQQ2PS.md")]
    Vcvtqq2ps,
    #[doc = include_str!("../../doc/intel/VCVTSD2SH.md")]
    Vcvtsd2sh,
    #[doc = include_str!("../../doc/intel/VCVTSD2USI.md")]
    Vcvtsd2usi,
    #[doc = include_str!("../../doc/intel/VCVTSH2SD.md")]
    Vcvtsh2sd,
    #[doc = include_str!("../../doc/intel/VCVTSH2SI.md")]
    Vcvtsh2si,
    #[doc = include_str!("../../doc/intel/VCVTSH2SS.md")]
    Vcvtsh2ss,
    #[doc = include_str!("../../doc/intel/VCVTSH2USI.md")]
    Vcvtsh2usi,
    #[doc = include_str!("../../doc/intel/VCVTSI2SH.md")]
    Vcvtsi2sh,
    #[doc = include_str!("../../doc/intel/VCVTSS2SH.md")]
    Vcvtss2sh,
    #[doc = include_str!("../../doc/intel/VCVTSS2USI.md")]
    Vcvtss2usi,
    #[doc = include_str!("../../doc/intel/VCVTTPD2QQ.md")]
    Vcvttpd2qq,
    #[doc = include_str!("../../doc/intel/VCVTTPD2UDQ.md")]
    Vcvttpd2udq,
    #[doc = include_str!("../../doc/intel/VCVTTPD2UQQ.md")]
    Vcvttpd2uqq,
    #[doc = include_str!("../../doc/intel/VCVTTPH2DQ.md")]
    Vcvttph2dq,
    #[doc = include_str!("../../doc/intel/VCVTTPH2QQ.md")]
    Vcvttph2qq,
    #[doc = include_str!("../../doc/intel/VCVTTPH2UDQ.md")]
    Vcvttph2udq,
    #[doc = include_str!("../../doc/intel/VCVTTPH2UQQ.md")]
    Vcvttph2uqq,
    #[doc = include_str!("../../doc/intel/VCVTTPH2UW.md")]
    Vcvttph2uw,
    #[doc = include_str!("../../doc/intel/VCVTTPH2W.md")]
    Vcvttph2w,
    #[doc = include_str!("../../doc/intel/VCVTTPS2QQ.md")]
    Vcvttps2qq,
    #[doc = include_str!("../../doc/intel/VCVTTPS2UDQ.md")]
    Vcvttps2udq,
    #[doc = include_str!("../../doc/intel/VCVTTPS2UQQ.md")]
    Vcvttps2uqq,
    #[doc = include_str!("../../doc/intel/VCVTTSD2USI.md")]
    Vcvttsd2usi,
    #[doc = include_str!("../../doc/intel/VCVTTSH2SI.md")]
    Vcvttsh2si,
    #[doc = include_str!("../../doc/intel/VCVTTSH2USI.md")]
    Vcvttsh2usi,
    #[doc = include_str!("../../doc/intel/VCVTTSS2USI.md")]
    Vcvttss2usi,
    #[doc = include_str!("../../doc/intel/VCVTUDQ2PD.md")]
    Vcvtudq2pd,
    #[doc = include_str!("../../doc/intel/VCVTUDQ2PH.md")]
    Vcvtudq2ph,
    #[doc = include_str!("../../doc/intel/VCVTUDQ2PS.md")]
    Vcvtudq2ps,
    #[doc = include_str!("../../doc/intel/VCVTUQQ2PD.md")]
    Vcvtuqq2pd,
    #[doc = include_str!("../../doc/intel/VCVTUQQ2PH.md")]
    Vcvtuqq2ph,
    #[doc = include_str!("../../doc/intel/VCVTUQQ2PS.md")]
    Vcvtuqq2ps,
    #[doc = include_str!("../../doc/intel/VCVTUSI2SD.md")]
    Vcvtusi2sd,
    #[doc = include_str!("../../doc/intel/VCVTUSI2SH.md")]
    Vcvtusi2sh,
    #[doc = include_str!("../../doc/intel/VCVTUSI2SS.md")]
    Vcvtusi2ss,
    #[doc = include_str!("../../doc/intel/VCVTUW2PH.md")]
    Vcvtuw2ph,
    #[doc = include_str!("../../doc/intel/VCVTW2PH.md")]
    Vcvtw2ph,
    #[doc = include_str!("../../doc/intel/VDBPSADBW.md")]
    Vdbpsadbw,
    #[doc = include_str!("../../doc/intel/VDIVPH.md")]
    Vdivph,
    #[doc = include_str!("../../doc/intel/VDIVSH.md")]
    Vdivsh,
    #[doc = include_str!("../../doc/intel/VDPBF16PS.md")]
    Vdpbf16ps,
    #[doc = include_str!("../../doc/intel/VERR.md")]
    Verr,
    #[doc = include_str!("../../doc/intel/VERW.md")]
    Verw,
    #[doc = include_str!("../../doc/intel/VEXPANDPD.md")]
    Vexpandpd,
    #[doc = include_str!("../../doc/intel/VEXPANDPS.md")]
    Vexpandps,
    #[doc = include_str!("../../doc/intel/VEXTRACTF64x4.md")]
    Vextractf64x4,
    #[doc = include_str!("../../doc/intel/VEXTRACTI128.md")]
    Vextracti128,
    #[doc = include_str!("../../doc/intel/VEXTRACTI32x4.md")]
    Vextracti32x4,
    #[doc = include_str!("../../doc/intel/VEXTRACTI32x8.md")]
    Vextracti32x8,
    #[doc = include_str!("../../doc/intel/VEXTRACTI64x2.md")]
    Vextracti64x2,
    #[doc = include_str!("../../doc/intel/VEXTRACTI64x4.md")]
    Vextracti64x4,
    #[doc = include_str!("../../doc/intel/VFCMADDCPH.md")]
    Vfcmaddcph,
    #[doc = include_str!("../../doc/intel/VFCMADDCSH.md")]
    Vfcmaddcsh,
    #[doc = include_str!("../../doc/intel/VFCMULCPH.md")]
    Vfcmulcph,
    #[doc = include_str!("../../doc/intel/VFCMULCSH.md")]
    Vfcmulcsh,
    #[doc = include_str!("../../doc/intel/VFIXUPIMMPD.md")]
    Vfixupimmpd,
    #[doc = include_str!("../../doc/intel/VFIXUPIMMPS.md")]
    Vfixupimmps,
    #[doc = include_str!("../../doc/intel/VFIXUPIMMSD.md")]
    Vfixupimmsd,
    #[doc = include_str!("../../doc/intel/VFIXUPIMMSS.md")]
    Vfixupimmss,
    #[doc = include_str!("../../doc/intel/VFMADD.md")]
    Vfmadd,
    #[doc = include_str!("../../doc/intel/VFMADD132PD.md")]
    Vfmadd132pd,
    #[doc = include_str!("../../doc/intel/VFMADD132PS.md")]
    Vfmadd132ps,
    #[doc = include_str!("../../doc/intel/VFMADD132SD.md")]
    Vfmadd132sd,
    #[doc = include_str!("../../doc/intel/VFMADD132SS.md")]
    Vfmadd132ss,
    #[doc = include_str!("../../doc/intel/VFMADD213PD.md")]
    Vfmadd213pd,
    #[doc = include_str!("../../doc/intel/VFMADD213PS.md")]
    Vfmadd213ps,
    #[doc = include_str!("../../doc/intel/VFMADD213SD.md")]
    Vfmadd213sd,
    #[doc = include_str!("../../doc/intel/VFMADD213SS.md")]
    Vfmadd213ss,
    #[doc = include_str!("../../doc/intel/VFMADD231PD.md")]
    Vfmadd231pd,
    #[doc = include_str!("../../doc/intel/VFMADD231PS.md")]
    Vfmadd231ps,
    #[doc = include_str!("../../doc/intel/VFMADD231SD.md")]
    Vfmadd231sd,
    #[doc = include_str!("../../doc/intel/VFMADD231SS.md")]
    Vfmadd231ss,
    #[doc = include_str!("../../doc/intel/VFMADDCPH.md")]
    Vfmaddcph,
    #[doc = include_str!("../../doc/intel/VFMADDCSH.md")]
    Vfmaddcsh,
    #[doc = include_str!("../../doc/intel/VFMADDSUB132PD.md")]
    Vfmaddsub132pd,
    #[doc = include_str!("../../doc/intel/VFMADDSUB132PH.md")]
    Vfmaddsub132ph,
    #[doc = include_str!("../../doc/intel/VFMADDSUB132PS.md")]
    Vfmaddsub132ps,
    #[doc = include_str!("../../doc/intel/VFMADDSUB213PD.md")]
    Vfmaddsub213pd,
    #[doc = include_str!("../../doc/intel/VFMADDSUB213PH.md")]
    Vfmaddsub213ph,
    #[doc = include_str!("../../doc/intel/VFMADDSUB213PS.md")]
    Vfmaddsub213ps,
    #[doc = include_str!("../../doc/intel/VFMADDSUB231PD.md")]
    Vfmaddsub231pd,
    #[doc = include_str!("../../doc/intel/VFMADDSUB231PH.md")]
    Vfmaddsub231ph,
    #[doc = include_str!("../../doc/intel/VFMADDSUB231PS.md")]
    Vfmaddsub231ps,
    #[doc = include_str!("../../doc/intel/VFMSUB.md")]
    Vfmsub,
    #[doc = include_str!("../../doc/intel/VFMSUB132PD.md")]
    Vfmsub132pd,
    #[doc = include_str!("../../doc/intel/VFMSUB132PS.md")]
    Vfmsub132ps,
    #[doc = include_str!("../../doc/intel/VFMSUB132SD.md")]
    Vfmsub132sd,
    #[doc = include_str!("../../doc/intel/VFMSUB132SS.md")]
    Vfmsub132ss,
    #[doc = include_str!("../../doc/intel/VFMSUB213PD.md")]
    Vfmsub213pd,
    #[doc = include_str!("../../doc/intel/VFMSUB213PS.md")]
    Vfmsub213ps,
    #[doc = include_str!("../../doc/intel/VFMSUB213SD.md")]
    Vfmsub213sd,
    #[doc = include_str!("../../doc/intel/VFMSUB213SS.md")]
    Vfmsub213ss,
    #[doc = include_str!("../../doc/intel/VFMSUB231PD.md")]
    Vfmsub231pd,
    #[doc = include_str!("../../doc/intel/VFMSUB231PS.md")]
    Vfmsub231ps,
    #[doc = include_str!("../../doc/intel/VFMSUB231SD.md")]
    Vfmsub231sd,
    #[doc = include_str!("../../doc/intel/VFMSUB231SS.md")]
    Vfmsub231ss,
    #[doc = include_str!("../../doc/intel/VFMSUBADD132PD.md")]
    Vfmsubadd132pd,
    #[doc = include_str!("../../doc/intel/VFMSUBADD132PH.md")]
    Vfmsubadd132ph,
    #[doc = include_str!("../../doc/intel/VFMSUBADD132PS.md")]
    Vfmsubadd132ps,
    #[doc = include_str!("../../doc/intel/VFMSUBADD213PD.md")]
    Vfmsubadd213pd,
    #[doc = include_str!("../../doc/intel/VFMSUBADD213PH.md")]
    Vfmsubadd213ph,
    #[doc = include_str!("../../doc/intel/VFMSUBADD213PS.md")]
    Vfmsubadd213ps,
    #[doc = include_str!("../../doc/intel/VFMSUBADD231PD.md")]
    Vfmsubadd231pd,
    #[doc = include_str!("../../doc/intel/VFMSUBADD231PH.md")]
    Vfmsubadd231ph,
    #[doc = include_str!("../../doc/intel/VFMSUBADD231PS.md")]
    Vfmsubadd231ps,
    #[doc = include_str!("../../doc/intel/VFMULCPH.md")]
    Vfmulcph,
    #[doc = include_str!("../../doc/intel/VFMULCSH.md")]
    Vfmulcsh,
    #[doc = include_str!("../../doc/intel/VFNMADD.md")]
    Vfnmadd,
    #[doc = include_str!("../../doc/intel/VFNMADD132PD.md")]
    Vfnmadd132pd,
    #[doc = include_str!("../../doc/intel/VFNMADD132PS.md")]
    Vfnmadd132ps,
    #[doc = include_str!("../../doc/intel/VFNMADD132SD.md")]
    Vfnmadd132sd,
    #[doc = include_str!("../../doc/intel/VFNMADD132SS.md")]
    Vfnmadd132ss,
    #[doc = include_str!("../../doc/intel/VFNMADD213PD.md")]
    Vfnmadd213pd,
    #[doc = include_str!("../../doc/intel/VFNMADD213PS.md")]
    Vfnmadd213ps,
    #[doc = include_str!("../../doc/intel/VFNMADD213SD.md")]
    Vfnmadd213sd,
    #[doc = include_str!("../../doc/intel/VFNMADD213SS.md")]
    Vfnmadd213ss,
    #[doc = include_str!("../../doc/intel/VFNMADD231PD.md")]
    Vfnmadd231pd,
    #[doc = include_str!("../../doc/intel/VFNMADD231PS.md")]
    Vfnmadd231ps,
    #[doc = include_str!("../../doc/intel/VFNMADD231SD.md")]
    Vfnmadd231sd,
    #[doc = include_str!("../../doc/intel/VFNMADD231SS.md")]
    Vfnmadd231ss,
    #[doc = include_str!("../../doc/intel/VFNMSUB.md")]
    Vfnmsub,
    #[doc = include_str!("../../doc/intel/VFNMSUB132PD.md")]
    Vfnmsub132pd,
    #[doc = include_str!("../../doc/intel/VFNMSUB132PS.md")]
    Vfnmsub132ps,
    #[doc = include_str!("../../doc/intel/VFNMSUB132SD.md")]
    Vfnmsub132sd,
    #[doc = include_str!("../../doc/intel/VFNMSUB132SS.md")]
    Vfnmsub132ss,
    #[doc = include_str!("../../doc/intel/VFNMSUB213PD.md")]
    Vfnmsub213pd,
    #[doc = include_str!("../../doc/intel/VFNMSUB213PS.md")]
    Vfnmsub213ps,
    #[doc = include_str!("../../doc/intel/VFNMSUB213SD.md")]
    Vfnmsub213sd,
    #[doc = include_str!("../../doc/intel/VFNMSUB213SS.md")]
    Vfnmsub213ss,
    #[doc = include_str!("../../doc/intel/VFNMSUB231PD.md")]
    Vfnmsub231pd,
    #[doc = include_str!("../../doc/intel/VFNMSUB231PS.md")]
    Vfnmsub231ps,
    #[doc = include_str!("../../doc/intel/VFNMSUB231SD.md")]
    Vfnmsub231sd,
    #[doc = include_str!("../../doc/intel/VFNMSUB231SS.md")]
    Vfnmsub231ss,
    #[doc = include_str!("../../doc/intel/VFPCLASSPD.md")]
    Vfpclasspd,
    #[doc = include_str!("../../doc/intel/VFPCLASSPH.md")]
    Vfpclassph,
    #[doc = include_str!("../../doc/intel/VFPCLASSPS.md")]
    Vfpclassps,
    #[doc = include_str!("../../doc/intel/VFPCLASSSD.md")]
    Vfpclasssd,
    #[doc = include_str!("../../doc/intel/VFPCLASSSH.md")]
    Vfpclasssh,
    #[doc = include_str!("../../doc/intel/VFPCLASSSS.md")]
    Vfpclassss,
    #[doc = include_str!("../../doc/intel/VGATHERDPD.md")]
    Vgatherdpd,
    #[doc = include_str!("../../doc/intel/VGATHERDPS.md")]
    Vgatherdps,
    #[doc = include_str!("../../doc/intel/VGATHERQPD.md")]
    Vgatherqpd,
    #[doc = include_str!("../../doc/intel/VGATHERQPS.md")]
    Vgatherqps,
    #[doc = include_str!("../../doc/intel/VGETEXPPD.md")]
    Vgetexppd,
    #[doc = include_str!("../../doc/intel/VGETEXPPH.md")]
    Vgetexpph,
    #[doc = include_str!("../../doc/intel/VGETEXPPS.md")]
    Vgetexpps,
    #[doc = include_str!("../../doc/intel/VGETEXPSD.md")]
    Vgetexpsd,
    #[doc = include_str!("../../doc/intel/VGETEXPSH.md")]
    Vgetexpsh,
    #[doc = include_str!("../../doc/intel/VGETEXPSS.md")]
    Vgetexpss,
    #[doc = include_str!("../../doc/intel/VGETMANTPD.md")]
    Vgetmantpd,
    #[doc = include_str!("../../doc/intel/VGETMANTPH.md")]
    Vgetmantph,
    #[doc = include_str!("../../doc/intel/VGETMANTPS.md")]
    Vgetmantps,
    #[doc = include_str!("../../doc/intel/VGETMANTSD.md")]
    Vgetmantsd,
    #[doc = include_str!("../../doc/intel/VGETMANTSH.md")]
    Vgetmantsh,
    #[doc = include_str!("../../doc/intel/VGETMANTSS.md")]
    Vgetmantss,
    #[doc = include_str!("../../doc/intel/VINSERTF128.md")]
    Vinsertf128,
    #[doc = include_str!("../../doc/intel/VINSERTF32x4.md")]
    Vinsertf32x4,
    #[doc = include_str!("../../doc/intel/VINSERTF32x8.md")]
    Vinsertf32x8,
    #[doc = include_str!("../../doc/intel/VINSERTF64x2.md")]
    Vinsertf64x2,
    #[doc = include_str!("../../doc/intel/VINSERTF64x4.md")]
    Vinsertf64x4,
    #[doc = include_str!("../../doc/intel/VINSERTI128.md")]
    Vinserti128,
    #[doc = include_str!("../../doc/intel/VINSERTI32x4.md")]
    Vinserti32x4,
    #[doc = include_str!("../../doc/intel/VINSERTI32x8.md")]
    Vinserti32x8,
    #[doc = include_str!("../../doc/intel/VINSERTI64x2.md")]
    Vinserti64x2,
    #[doc = include_str!("../../doc/intel/VINSERTI64x4.md")]
    Vinserti64x4,
    #[doc = include_str!("../../doc/intel/VMASKMOV.md")]
    Vmaskmov,
    #[doc = include_str!("../../doc/intel/VMAXPH.md")]
    Vmaxph,
    #[doc = include_str!("../../doc/intel/VMAXSH.md")]
    Vmaxsh,
    #[doc = include_str!("../../doc/intel/VMINPH.md")]
    Vminph,
    #[doc = include_str!("../../doc/intel/VMINSH.md")]
    Vminsh,
    #[doc = include_str!("../../doc/intel/VMOVDQA32.md")]
    Vmovdqa32,
    #[doc = include_str!("../../doc/intel/VMOVDQU8.md")]
    Vmovdqu8,
    #[doc = include_str!("../../doc/intel/VMOVSH.md")]
    Vmovsh,
    #[doc = include_str!("../../doc/intel/VMOVW.md")]
    Vmovw,
    #[doc = include_str!("../../doc/intel/VMULPH.md")]
    Vmulph,
    #[doc = include_str!("../../doc/intel/VMULSH.md")]
    Vmulsh,
    #[doc = include_str!("../../doc/intel/VP2INTERSECTD.md")]
    Vp2intersectd,
    #[doc = include_str!("../../doc/intel/VP2INTERSECTQ.md")]
    Vp2intersectq,
    #[doc = include_str!("../../doc/intel/VPBLENDD.md")]
    Vpblendd,
    #[doc = include_str!("../../doc/intel/VPBLENDMB.md")]
    Vpblendmb,
    #[doc = include_str!("../../doc/intel/VPBLENDMD.md")]
    Vpblendmd,
    #[doc = include_str!("../../doc/intel/VPBLENDMQ.md")]
    Vpblendmq,
    #[doc = include_str!("../../doc/intel/VPBLENDMW.md")]
    Vpblendmw,
    #[doc = include_str!("../../doc/intel/VPBROADCAST.md")]
    Vpbroadcast,
    #[doc = include_str!("../../doc/intel/VPBROADCASTB.md")]
    Vpbroadcastb,
    #[doc = include_str!("../../doc/intel/VPBROADCASTD.md")]
    Vpbroadcastd,
    #[doc = include_str!("../../doc/intel/VPBROADCASTM.md")]
    Vpbroadcastm,
    #[doc = include_str!("../../doc/intel/VPBROADCASTQ.md")]
    Vpbroadcastq,
    #[doc = include_str!("../../doc/intel/VPBROADCASTW.md")]
    Vpbroadcastw,
    #[doc = include_str!("../../doc/intel/VPCMPB.md")]
    Vpcmpb,
    #[doc = include_str!("../../doc/intel/VPCMPD.md")]
    Vpcmpd,
    #[doc = include_str!("../../doc/intel/VPCMPQ.md")]
    Vpcmpq,
    #[doc = include_str!("../../doc/intel/VPCMPUB.md")]
    Vpcmpub,
    #[doc = include_str!("../../doc/intel/VPCMPUD.md")]
    Vpcmpud,
    #[doc = include_str!("../../doc/intel/VPCMPUQ.md")]
    Vpcmpuq,
    #[doc = include_str!("../../doc/intel/VPCMPUW.md")]
    Vpcmpuw,
    #[doc = include_str!("../../doc/intel/VPCMPW.md")]
    Vpcmpw,
    #[doc = include_str!("../../doc/intel/VPCOMPRESSB.md")]
    Vpcompressb,
    #[doc = include_str!("../../doc/intel/VPCOMPRESSD.md")]
    Vpcompressd,
    #[doc = include_str!("../../doc/intel/VPCOMPRESSQ.md")]
    Vpcompressq,
    #[doc = include_str!("../../doc/intel/VPCONFLICTD.md")]
    Vpconflictd,
    #[doc = include_str!("../../doc/intel/VPCONFLICTQ.md")]
    Vpconflictq,
    #[doc = include_str!("../../doc/intel/VPDPBUSD.md")]
    Vpdpbusd,
    #[doc = include_str!("../../doc/intel/VPDPBUSDS.md")]
    Vpdpbusds,
    #[doc = include_str!("../../doc/intel/VPDPWSSD.md")]
    Vpdpwssd,
    #[doc = include_str!("../../doc/intel/VPDPWSSDS.md")]
    Vpdpwssds,
    #[doc = include_str!("../../doc/intel/VPERM2F128.md")]
    Vperm2f128,
    #[doc = include_str!("../../doc/intel/VPERM2I128.md")]
    Vperm2i128,
    #[doc = include_str!("../../doc/intel/VPERMB.md")]
    Vpermb,
    #[doc = include_str!("../../doc/intel/VPERMD.md")]
    Vpermd,
    #[doc = include_str!("../../doc/intel/VPERMI2B.md")]
    Vpermi2b,
    #[doc = include_str!("../../doc/intel/VPERMI2D.md")]
    Vpermi2d,
    #[doc = include_str!("../../doc/intel/VPERMI2PD.md")]
    Vpermi2pd,
    #[doc = include_str!("../../doc/intel/VPERMI2PS.md")]
    Vpermi2ps,
    #[doc = include_str!("../../doc/intel/VPERMI2Q.md")]
    Vpermi2q,
    #[doc = include_str!("../../doc/intel/VPERMI2W.md")]
    Vpermi2w,
    #[doc = include_str!("../../doc/intel/VPERMILPD.md")]
    Vpermilpd,
    #[doc = include_str!("../../doc/intel/VPERMILPS.md")]
    Vpermilps,
    #[doc = include_str!("../../doc/intel/VPERMPD.md")]
    Vpermpd,
    #[doc = include_str!("../../doc/intel/VPERMPS.md")]
    Vpermps,
    #[doc = include_str!("../../doc/intel/VPERMQ.md")]
    Vpermq,
    #[doc = include_str!("../../doc/intel/VPERMT2B.md")]
    Vpermt2b,
    #[doc = include_str!("../../doc/intel/VPERMT2D.md")]
    Vpermt2d,
    #[doc = include_str!("../../doc/intel/VPERMT2PD.md")]
    Vpermt2pd,
    #[doc = include_str!("../../doc/intel/VPERMT2PS.md")]
    Vpermt2ps,
    #[doc = include_str!("../../doc/intel/VPERMT2Q.md")]
    Vpermt2q,
    #[doc = include_str!("../../doc/intel/VPERMT2W.md")]
    Vpermt2w,
    #[doc = include_str!("../../doc/intel/VPERMW.md")]
    Vpermw,
    #[doc = include_str!("../../doc/intel/VPEXPANDB.md")]
    Vpexpandb,
    #[doc = include_str!("../../doc/intel/VPEXPANDD.md")]
    Vpexpandd,
    #[doc = include_str!("../../doc/intel/VPEXPANDQ.md")]
    Vpexpandq,
    #[doc = include_str!("../../doc/intel/VPEXPANDW.md")]
    Vpexpandw,
    #[doc = include_str!("../../doc/intel/VPGATHERDD.md")]
    Vpgatherdd,
    #[doc = include_str!("../../doc/intel/VPGATHERDQ.md")]
    Vpgatherdq,
    #[doc = include_str!("../../doc/intel/VPGATHERQD.md")]
    Vpgatherqd,
    #[doc = include_str!("../../doc/intel/VPGATHERQQ.md")]
    Vpgatherqq,
    #[doc = include_str!("../../doc/intel/VPLZCNTD.md")]
    Vplzcntd,
    #[doc = include_str!("../../doc/intel/VPLZCNTQ.md")]
    Vplzcntq,
    #[doc = include_str!("../../doc/intel/VPMADD52HUQ.md")]
    Vpmadd52huq,
    #[doc = include_str!("../../doc/intel/VPMADD52LUQ.md")]
    Vpmadd52luq,
    #[doc = include_str!("../../doc/intel/VPMASKMOV.md")]
    Vpmaskmov,
    #[doc = include_str!("../../doc/intel/VPMOVB2M.md")]
    Vpmovb2m,
    #[doc = include_str!("../../doc/intel/VPMOVD2M.md")]
    Vpmovd2m,
    #[doc = include_str!("../../doc/intel/VPMOVDB.md")]
    Vpmovdb,
    #[doc = include_str!("../../doc/intel/VPMOVDW.md")]
    Vpmovdw,
    #[doc = include_str!("../../doc/intel/VPMOVM2B.md")]
    Vpmovm2b,
    #[doc = include_str!("../../doc/intel/VPMOVM2D.md")]
    Vpmovm2d,
    #[doc = include_str!("../../doc/intel/VPMOVM2Q.md")]
    Vpmovm2q,
    #[doc = include_str!("../../doc/intel/VPMOVM2W.md")]
    Vpmovm2w,
    #[doc = include_str!("../../doc/intel/VPMOVQ2M.md")]
    Vpmovq2m,
    #[doc = include_str!("../../doc/intel/VPMOVQB.md")]
    Vpmovqb,
    #[doc = include_str!("../../doc/intel/VPMOVQD.md")]
    Vpmovqd,
    #[doc = include_str!("../../doc/intel/VPMOVQW.md")]
    Vpmovqw,
    #[doc = include_str!("../../doc/intel/VPMOVSDB.md")]
    Vpmovsdb,
    #[doc = include_str!("../../doc/intel/VPMOVSDW.md")]
    Vpmovsdw,
    #[doc = include_str!("../../doc/intel/VPMOVSQB.md")]
    Vpmovsqb,
    #[doc = include_str!("../../doc/intel/VPMOVSQD.md")]
    Vpmovsqd,
    #[doc = include_str!("../../doc/intel/VPMOVSQW.md")]
    Vpmovsqw,
    #[doc = include_str!("../../doc/intel/VPMOVSWB.md")]
    Vpmovswb,
    #[doc = include_str!("../../doc/intel/VPMOVUSDB.md")]
    Vpmovusdb,
    #[doc = include_str!("../../doc/intel/VPMOVUSDW.md")]
    Vpmovusdw,
    #[doc = include_str!("../../doc/intel/VPMOVUSQB.md")]
    Vpmovusqb,
    #[doc = include_str!("../../doc/intel/VPMOVUSQD.md")]
    Vpmovusqd,
    #[doc = include_str!("../../doc/intel/VPMOVUSQW.md")]
    Vpmovusqw,
    #[doc = include_str!("../../doc/intel/VPMOVUSWB.md")]
    Vpmovuswb,
    #[doc = include_str!("../../doc/intel/VPMOVW2M.md")]
    Vpmovw2m,
    #[doc = include_str!("../../doc/intel/VPMOVWB.md")]
    Vpmovwb,
    #[doc = include_str!("../../doc/intel/VPMULTISHIFTQB.md")]
    Vpmultishiftqb,
    #[doc = include_str!("../../doc/intel/VPOPCNT.md")]
    Vpopcnt,
    #[doc = include_str!("../../doc/intel/VPROLD.md")]
    Vprold,
    #[doc = include_str!("../../doc/intel/VPROLQ.md")]
    Vprolq,
    #[doc = include_str!("../../doc/intel/VPROLVD.md")]
    Vprolvd,
    #[doc = include_str!("../../doc/intel/VPROLVQ.md")]
    Vprolvq,
    #[doc = include_str!("../../doc/intel/VPRORD.md")]
    Vprord,
    #[doc = include_str!("../../doc/intel/VPRORQ.md")]
    Vprorq,
    #[doc = include_str!("../../doc/intel/VPRORVD.md")]
    Vprorvd,
    #[doc = include_str!("../../doc/intel/VPRORVQ.md")]
    Vprorvq,
    #[doc = include_str!("../../doc/intel/VPSCATTERDD.md")]
    Vpscatterdd,
    #[doc = include_str!("../../doc/intel/VPSCATTERDQ.md")]
    Vpscatterdq,
    #[doc = include_str!("../../doc/intel/VPSCATTERQD.md")]
    Vpscatterqd,
    #[doc = include_str!("../../doc/intel/VPSCATTERQQ.md")]
    Vpscatterqq,
    #[doc = include_str!("../../doc/intel/VPSHLD.md")]
    Vpshld,
    #[doc = include_str!("../../doc/intel/VPSHLDV.md")]
    Vpshldv,
    #[doc = include_str!("../../doc/intel/VPSHRD.md")]
    Vpshrd,
    #[doc = include_str!("../../doc/intel/VPSHRDV.md")]
    Vpshrdv,
    #[doc = include_str!("../../doc/intel/VPSHUFBITQMB.md")]
    Vpshufbitqmb,
    #[doc = include_str!("../../doc/intel/VPSLLVD.md")]
    Vpsllvd,
    #[doc = include_str!("../../doc/intel/VPSLLVQ.md")]
    Vpsllvq,
    #[doc = include_str!("../../doc/intel/VPSLLVW.md")]
    Vpsllvw,
    #[doc = include_str!("../../doc/intel/VPSRAVD.md")]
    Vpsravd,
    #[doc = include_str!("../../doc/intel/VPSRAVQ.md")]
    Vpsravq,
    #[doc = include_str!("../../doc/intel/VPSRAVW.md")]
    Vpsravw,
    #[doc = include_str!("../../doc/intel/VPSRLVD.md")]
    Vpsrlvd,
    #[doc = include_str!("../../doc/intel/VPSRLVQ.md")]
    Vpsrlvq,
    #[doc = include_str!("../../doc/intel/VPSRLVW.md")]
    Vpsrlvw,
    #[doc = include_str!("../../doc/intel/VPTERNLOGD.md")]
    Vpternlogd,
    #[doc = include_str!("../../doc/intel/VPTERNLOGQ.md")]
    Vpternlogq,
    #[doc = include_str!("../../doc/intel/VPTESTMB.md")]
    Vptestmb,
    #[doc = include_str!("../../doc/intel/VPTESTMD.md")]
    Vptestmd,
    #[doc = include_str!("../../doc/intel/VPTESTMQ.md")]
    Vptestmq,
    #[doc = include_str!("../../doc/intel/VPTESTMW.md")]
    Vptestmw,
    #[doc = include_str!("../../doc/intel/VPTESTNMB.md")]
    Vptestnmb,
    #[doc = include_str!("../../doc/intel/VPTESTNMD.md")]
    Vptestnmd,
    #[doc = include_str!("../../doc/intel/VPTESTNMQ.md")]
    Vptestnmq,
    #[doc = include_str!("../../doc/intel/VPTESTNMW.md")]
    Vptestnmw,
    #[doc = include_str!("../../doc/intel/VRANGEPD.md")]
    Vrangepd,
    #[doc = include_str!("../../doc/intel/VRANGEPS.md")]
    Vrangeps,
    #[doc = include_str!("../../doc/intel/VRANGESD.md")]
    Vrangesd,
    #[doc = include_str!("../../doc/intel/VRANGESS.md")]
    Vrangess,
    #[doc = include_str!("../../doc/intel/VRCP14PD.md")]
    Vrcp14pd,
    #[doc = include_str!("../../doc/intel/VRCP14PS.md")]
    Vrcp14ps,
    #[doc = include_str!("../../doc/intel/VRCP14SD.md")]
    Vrcp14sd,
    #[doc = include_str!("../../doc/intel/VRCP14SS.md")]
    Vrcp14ss,
    #[doc = include_str!("../../doc/intel/VRCPPH.md")]
    Vrcpph,
    #[doc = include_str!("../../doc/intel/VRCPSH.md")]
    Vrcpsh,
    #[doc = include_str!("../../doc/intel/VREDUCEPD.md")]
    Vreducepd,
    #[doc = include_str!("../../doc/intel/VREDUCEPH.md")]
    Vreduceph,
    #[doc = include_str!("../../doc/intel/VREDUCEPS.md")]
    Vreduceps,
    #[doc = include_str!("../../doc/intel/VREDUCESD.md")]
    Vreducesd,
    #[doc = include_str!("../../doc/intel/VREDUCESH.md")]
    Vreducesh,
    #[doc = include_str!("../../doc/intel/VREDUCESS.md")]
    Vreducess,
    #[doc = include_str!("../../doc/intel/VRNDSCALEPD.md")]
    Vrndscalepd,
    #[doc = include_str!("../../doc/intel/VRNDSCALEPH.md")]
    Vrndscaleph,
    #[doc = include_str!("../../doc/intel/VRNDSCALEPS.md")]
    Vrndscaleps,
    #[doc = include_str!("../../doc/intel/VRNDSCALESD.md")]
    Vrndscalesd,
    #[doc = include_str!("../../doc/intel/VRNDSCALESH.md")]
    Vrndscalesh,
    #[doc = include_str!("../../doc/intel/VRNDSCALESS.md")]
    Vrndscaless,
    #[doc = include_str!("../../doc/intel/VRSQRT14PD.md")]
    Vrsqrt14pd,
    #[doc = include_str!("../../doc/intel/VRSQRT14PS.md")]
    Vrsqrt14ps,
    #[doc = include_str!("../../doc/intel/VRSQRT14SD.md")]
    Vrsqrt14sd,
    #[doc = include_str!("../../doc/intel/VRSQRT14SS.md")]
    Vrsqrt14ss,
    #[doc = include_str!("../../doc/intel/VRSQRTPH.md")]
    Vrsqrtph,
    #[doc = include_str!("../../doc/intel/VRSQRTSH.md")]
    Vrsqrtsh,
    #[doc = include_str!("../../doc/intel/VSCALEFPD.md")]
    Vscalefpd,
    #[doc = include_str!("../../doc/intel/VSCALEFPH.md")]
    Vscalefph,
    #[doc = include_str!("../../doc/intel/VSCALEFPS.md")]
    Vscalefps,
    #[doc = include_str!("../../doc/intel/VSCALEFSD.md")]
    Vscalefsd,
    #[doc = include_str!("../../doc/intel/VSCALEFSH.md")]
    Vscalefsh,
    #[doc = include_str!("../../doc/intel/VSCALEFSS.md")]
    Vscalefss,
    #[doc = include_str!("../../doc/intel/VSCATTERDPD.md")]
    Vscatterdpd,
    #[doc = include_str!("../../doc/intel/VSCATTERDPS.md")]
    Vscatterdps,
    #[doc = include_str!("../../doc/intel/VSCATTERQPD.md")]
    Vscatterqpd,
    #[doc = include_str!("../../doc/intel/VSCATTERQPS.md")]
    Vscatterqps,
    #[doc = include_str!("../../doc/intel/VSHUFF32x4.md")]
    Vshuff32x4,
    #[doc = include_str!("../../doc/intel/VSHUFF64x2.md")]
    Vshuff64x2,
    #[doc = include_str!("../../doc/intel/VSHUFI32x4.md")]
    Vshufi32x4,
    #[doc = include_str!("../../doc/intel/VSHUFI64x2.md")]
    Vshufi64x2,
    #[doc = include_str!("../../doc/intel/VSQRTPH.md")]
    Vsqrtph,
    #[doc = include_str!("../../doc/intel/VSQRTSH.md")]
    Vsqrtsh,
    #[doc = include_str!("../../doc/intel/VSUBPH.md")]
    Vsubph,
    #[doc = include_str!("../../doc/intel/VSUBSH.md")]
    Vsubsh,
    #[doc = include_str!("../../doc/intel/VTESTPD.md")]
    Vtestpd,
    #[doc = include_str!("../../doc/intel/VTESTPS.md")]
    Vtestps,
    #[doc = include_str!("../../doc/intel/VUCOMISH.md")]
    Vucomish,
    #[doc = include_str!("../../doc/intel/VZEROALL.md")]
    Vzeroall,
    #[doc = include_str!("../../doc/intel/WAIT.md")]
    Wait,
    #[doc = include_str!("../../doc/intel/WBINVD.md")]
    Wbinvd,
    #[doc = include_str!("../../doc/intel/WBNOINVD.md")]
    Wbnoinvd,
    #[doc = include_str!("../../doc/intel/WRFSBASE.md")]
    Wrfsbase,
    #[doc = include_str!("../../doc/intel/WRGSBASE.md")]
    Wrgsbase,
    #[doc = include_str!("../../doc/intel/WRMSR.md")]
    Wrmsr,
    #[doc = include_str!("../../doc/intel/WRPKRU.md")]
    Wrpkru,
    #[doc = include_str!("../../doc/intel/WRSSD.md")]
    Wrssd,
    #[doc = include_str!("../../doc/intel/WRSSQ.md")]
    Wrssq,
    #[doc = include_str!("../../doc/intel/WRUSSD.md")]
    Wrussd,
    #[doc = include_str!("../../doc/intel/WRUSSQ.md")]
    Wrussq,
    #[doc = include_str!("../../doc/intel/XABORT.md")]
    Xabort,
    #[doc = include_str!("../../doc/intel/XACQUIRE.md")]
    Xacquire,
    #[doc = include_str!("../../doc/intel/XADD.md")]
    Xadd,
    #[doc = include_str!("../../doc/intel/XBEGIN.md")]
    Xbegin,
    #[doc = include_str!("../../doc/intel/XCHG.md")]
    Xchg,
    #[doc = include_str!("../../doc/intel/XEND.md")]
    Xend,
    #[doc = include_str!("../../doc/intel/XGETBV.md")]
    Xgetbv,
    #[doc = include_str!("../../doc/intel/XLAT.md")]
    Xlat,
    #[doc = include_str!("../../doc/intel/XLATB.md")]
    Xlatb,
    #[doc = include_str!("../../doc/intel/XOR.md")]
    Xor,
    #[doc = include_str!("../../doc/intel/XORPD.md")]
    Xorpd,
    #[doc = include_str!("../../doc/intel/XORPS.md")]
    Xorps,
    #[doc = include_str!("../../doc/intel/XRELEASE.md")]
    Xrelease,
    #[doc = include_str!("../../doc/intel/XRESLDTRK.md")]
    Xresldtrk,
    #[doc = include_str!("../../doc/intel/XRSTOR.md")]
    Xrstor,
    #[doc = include_str!("../../doc/intel/XRSTORS.md")]
    Xrstors,
    #[doc = include_str!("../../doc/intel/XSAVE.md")]
    Xsave,
    #[doc = include_str!("../../doc/intel/XSAVEC.md")]
    Xsavec,
    #[doc = include_str!("../../doc/intel/XSAVEOPT.md")]
    Xsaveopt,
    #[doc = include_str!("../../doc/intel/XSAVES.md")]
    Xsaves,
    #[doc = include_str!("../../doc/intel/XSETBV.md")]
    Xsetbv,
    #[doc = include_str!("../../doc/intel/XSUSLDTRK.md")]
    Xsusldtrk,
}
