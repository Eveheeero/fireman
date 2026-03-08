# Analyzation strategies

- [x] Binary format loading — Parse PE/ELF/Mach-O, map segments/sections, apply relocations, resolve imports.
- [~] Entrypoint & init discovery — Identify entrypoint, CRT startup, constructors, and init arrays to find “real” code.
- [~] Code–data separation heuristics — Classify bytes as code vs data using xrefs, permissions, and decoding confidence.
- [ ] Linear sweep disassembly — Decode sequentially; useful for dense code but prone to decoding data as instructions.
  > 디스어셈블러 코어 재설계 필요 — 현재 재귀 순회 방식과 병행 불가
- [x] Recursive traversal disassembly — Follow control-flow targets to decode only reachable code; misses hidden/obfuscated code.
- [ ] Hybrid disassembly — Combine sweep + traversal with conflict resolution and probability scoring.
  > 디스어셈블러 코어 재설계 필요 — 현재 재귀 순회 방식과 병행 불가
- [~] Function boundary detection — Infer function starts/ends from call targets, prologues, epilogues, and fallthrough patterns.
  > ir_function.rs에서 entry block의 prologue 패턴(push rbp/mov rbp,rsp, FPO sub rsp, CET endbr64) 감지 및 로그 출력. call target 기반 함수 경계 추론은 block_grouper에서 수행. 고급 fallthrough 패턴은 미구현
- [x] Prologue/epilogue pattern matching — Detect common stack frame setup/teardown idioms per compiler/ABI.
- [x] Tail-call detection — Recognize jump-as-call patterns and preserve call semantics in output.
- [x] Thunk/wrapper collapsing — Identify tiny forwarding functions and collapse or annotate them.
- [~] Import/IAT/PLT resolution — Bind external call sites to known API symbols for better names and types.
- [~] Signature-based library identification — Match standard runtime/library routines (e.g., memcpy/memset) by byte/IR patterns.
  > 호출 이름 기반 memcpy/memset/strcpy 등 표준 라이브러리 함수 감지 및 주석 추가 (바이트 패턴 매칭 미구현)
- [~] Compiler/optimization fingerprinting — Detect compiler family/flags to tune heuristics (prologues, idioms, EH layout).
  > ir_function.rs에서 entry block 명령어 패턴으로 GCC/Clang(endbr64), MSVC(shadow space mov), leaf function 감지 및 로그 출력. EH 레이아웃/최적화 플래그 감지는 미구현
- [~] Instruction decoding normalization — Canonicalize instruction variants (aliases, addressing modes) before lifting.
- [~] Architecture/ABI modeling — Encode calling conventions, callee/caller-saved regs, stack alignment, red zones.
- [x] CFG construction — Build basic blocks and directed edges from branches/calls/returns.
- [x] Basic block normalization — Split/merge blocks at targets/fallthroughs to stabilize later structuring.
- [x] Dominator tree computation — Compute dominators to support loop finding, structuring, and SSA placement.
- [x] Postdominator analysis — Support if/else recovery, region formation, and structured exits.
- [~] Control-dependence analysis — Determine which predicates guard which statements for clean high-level control.
  > IR CFG 기준 control-dependence 계산 구현 완료 (dominator.rs::ControlDependence). AST statement origin이 block topology를 보존하지 않아 cleanup_control_flow/loop_analyzation 연결은 미구현
- [~] Indirect branch target recovery — Resolve computed jumps via dataflow, value sets, and table recognition.
- [~] Jump table detection — Recognize switch tables (bounds checks + indexed loads + indirect jump).
  > switch_reconstruction.rs에서 if-else chain을 switch로 재구성. IR 레벨 bounds check + indexed load + indirect jump 직접 감지는 미구현
- [x] Switch reconstruction — Emit switch/case from jump tables or compare chains with case clustering.
- [~] Exception-handling recovery — Parse unwind metadata and reconstruct try/catch/finally regions and edges.
  > auto_comment.rs에서 __cxa_throw/__cxa_begin_catch 등 C++ EH 런타임 호출 감지 및 주석 추가 (메타데이터 파서 미구현)
- [~] Setjmp/longjmp modeling — Special-case non-local control flow to avoid misleading structured output.
  > setjmp/longjmp/sigsetjmp 호출 감지 및 주석 추가 (제어흐름 모델링 미구현)
- [~] SSA conversion — Translate IR to SSA form to simplify analysis and reconstruction.
  > ssa.rs: Phase 1(phi 배치) + additive Phase 2(리네이밍/소유권) 구현 완료. compute_phi_sites()와 SsaFunction::from_ir_blocks()/construct_ssa()로 Cytron 알고리즘 기반 phi 위치 계산, SSA 버전 부여, 블록별 def/use 및 phi input/output 요약을 생성하며, ir_function.rs가 이를 IrFunction::get_ssa()로 노출한다. 다만 IR 자체를 versioned SSA statement로 재작성하는 정식 스위치오버는 아직 미구현
- [~] Phi-node placement — Insert merges at CFG joins to represent value merging cleanly.
  > ssa.rs: compute_phi_sites()로 DominanceFrontier 기반 phi 위치 계산, build_ssa_rename_summary()로 PhiNode output/input 요약 생성 구현. 실제 IrStatement에 phi를 삽입하는 정식 SSA 재작성 단계는 미구현
- [x] Def-use / use-def chains — Track where values come from and where they flow for variable/type recovery.
- [~] Reaching definitions — Determine which assignments may reach each use (critical for decompilation accuracy).
- [~] Liveness analysis — Compute live ranges to create source-like variables and reduce temporary noise.
- [~] Value numbering — Identify equivalent expressions to eliminate redundancy and stabilize output.
  > common_subexpression_elimination.rs에서 해시 기반 CSE로 동치 표현식 제거 구현. GVN(Global Value Numbering)은 SSA 형태 선행 필요
  > SSA 형태 선행 시 효과적 — 현재는 CSE로 대체
- [x] Constant propagation — Push constants through IR to simplify conditions, addresses, and expressions.
- [x] Constant folding — Evaluate constant expressions (including bit tricks) into simpler literals.
- [x] Copy propagation — Remove move chains to reduce temporaries and reveal original expressions.
- [x] Dead code elimination — Remove computations without observable effect (after side-effect modeling).
- [x] Side-effect modeling — Track memory writes, volatile accesses, syscalls, and I/O to avoid unsafe eliminations.
- [x] Common subexpression elimination — Reuse repeated expressions to reduce clutter and improve readability.
- [x] Algebraic simplification — Canonicalize arithmetic/bitwise forms (e.g., x+0, x^0, reassociation rules).
- [x] Bit-trick idiom recognition — Detect rotates, bswap, popcount, clz/ctz, min/max idioms, etc.
- [x] Magic-constant division recovery — Recognize multiply/shift sequences used for optimized div/mod.
- [~] Flag/condition recovery — Model CPU flags to reconstruct high-level comparisons and boolean logic.
  > cmovcc/setcc 개별 조건 코드 IR 핸들러 구현 완료; 전체 플래그 전파/소거는 미구현
- [x] Short-circuit boolean reconstruction — Convert branch patterns into &&/|| when semantics match.
- [x] Ternary operator recovery — Recognize select/phi patterns to emit cond ? a : b.
- [~] Region/structural analysis — Convert CFG into structured if/else, while, for, do-while regions.
  > simplified Phoenix 기반 structuring.rs가 현재 CFG/loop API에 맞게 정리되어 run_cfg_shape_analysis() 및 IrFunction::structured에 연결됨. 또한 ir_analyzation 직후 guarded structured_region_lowering.rs가 pure If head block과 branch/body polarity를 증명할 수 있는 단순 While/DoWhile control block을 StructuredRegion 기반으로 AST에 반영하며, explicit `StructuredRegion::Switch`도 fail-closed lowering 경로를 갖춤. 다만 CFG structuring 단계가 아직 `Switch` region을 실제로 생성하지 않아서 switch 소비는 scaffold 수준이며, 더 복잡한 loop 형태, 정밀 region 복원, full AST emission 전환, irreducible CFG 변환은 아직 미구현
- [~] Reducibility transformation — Apply node splitting/edge rewriting to structure irreducible CFGs when possible.
  > dominator.rs: CFG::is_reducible() 환원성 검사 구현 완료. 비환원 CFG 처리는 StructuredRegion::Goto/Label 폴백으로 설계 (docs/plans/ssa-cfg-structuring-design.md). 노드 분할 변환은 Phase 4에서 구현 예정
- [x] Loop detection via back-edges — Identify natural loops using dominators and back-edge discovery.
- [x] Loop reconstruction heuristics — Choose while vs do-while vs for based on header/test placement.
- [x] Induction variable analysis — Detect counters/strides/bounds to emit for (i=…; …; i+=…).
- [~] Loop-invariant code motion (reverse) — Recognize hoisted expressions and place them naturally in source output.
  > 루프 본문 내 불변 표현식 감지 및 주석 추가 구현 (auto_comment.rs). 실제 코드 이동은 포인터/앨리어싱 분석 선행 필요
- [x] Control-flow simplification — Remove redundant gotos, invert conditions, merge equivalent tails.
- [x] Goto containment heuristics — Use labeled blocks sparingly; prefer structured constructs when safe.
- [~] Stack pointer tracking — Track SP deltas across blocks to recover frame layout even without frame pointers.
- [~] Frame-pointer omission handling — Infer locals/spills when compiler omits FP (FPO) and uses SP-relative addressing.
  > ir_function.rs: FPO prologue 감지 + count_sp_relative_accesses()로 SP-relative 메모리 접근 수 집계 구현. 실제 로컬 변수 추론은 스택 델타 추적 프레임워크 필요
- [~] Stack realignment recovery — Detect alignment prologues/epilogues and suppress them in high-level output.
  > ir_function.rs: detect_stack_realignment()로 `and rsp, -N` 패턴 감지 및 정렬 값 추출 구현. 출력 억제(AST 변환)는 미구현
- [ ] Stack slot coalescing (reverse) — Split merged stack slots back into distinct variables using lifetimes/types.
  > 스택 슬롯 생명주기 분석 필요. 현재 variable_coalescing.rs에서 레지스터 기반 병합은 수행하나 스택 슬롯 분할은 미구현
- [x] Register-to-variable recovery — Turn register lifetimes into named locals and parameters.
- [x] Temporary elimination — Merge short-lived SSA temps into expressions to look like C code.
- [x] Parameter recovery — Infer which incoming values are true parameters (registers/stack slots) vs incidental.
- [~] Return value recovery — Infer returned expressions, including hidden returns (sret) and multi-register returns.
- [~] Varargs detection — Identify vararg call sites and apply format-string/type heuristics.
  > auto_comment.rs: call_name_matches_vararg()로 printf/scanf/exec 계열 vararg 호출 탐지 및 주석 부착 구현. 포맷 문자열 파싱/타입 추론은 미구현.
- [~] Calling convention inference — Infer ABI per function (cdecl/stdcall/thiscall/sysv/ms) from usage patterns.
- [~] “this” pointer inference — Detect implicit object pointer in C++ methods from member access and vtable usage.
  > auto_comment.rs: annotate_this_or_sret_pointer()로 첫 번째 파라미터의 base+offset 멤버 접근 패턴 탐지 및 주석 부착 구현. vtable 분석 기반 클래스 복원은 미구현.
- [~] SRet/hidden parameter inference — Detect structure return via hidden pointer parameters.
  > auto_comment.rs: annotate_this_or_sret_pointer()로 첫 번째 파라미터가 주로 deref 대상으로 사용되는 sret 패턴 탐지 및 주석 부착 구현. ABI 기반 호출 규약 확장은 미구현.
- [~] Interprocedural analysis — Propagate types/constants across call boundaries for better signatures.
- [~] Summary-based interprocedural analysis — Build per-function summaries (effects, returns, param usage) to scale.
  > `ir/analyze/function_summary.rs`의 register read/write, returns, side effects, direct callee, escaped_registers 요약이 `IrFunction` 생성 경로에 연결되어 보관/로그된다. `ir_to_ast.rs`가 direct internal callee에 한해 한 번의 caller→callee escape projection을 수행하므로 첫 summary 소비자가 생겼지만, param usage 정밀화, type/resource propagation, call-graph 고정점 전파는 아직 미구현
- [ ] Context-sensitive analysis — Distinguish behaviors per call site for more precise type/target recovery.
  > 호출 컨텍스트 프레임워크 필요. call_graph.rs에서 함수간 호출 관계는 추적하나 call-site별 동작 구분은 미구현
- [~] Points-to analysis — Approximate what pointers can reference to improve indirect load/store understanding.
  > `ir/analyze/points_to.rs`의 Steensgaard 스타일 분석이 `IrFunction` 생성 경로에 연결되어 결과를 보관하고 location/edge 요약을 로그로 남긴다. 이제 `JumpByCall`마다 synthetic `Heap(call_site_id)` allocation site도 보수적으로 시드하지만, AST/최적화 패스 소비, stack slot 정밀화, allocator 식별 기반 heap site 정교화는 미구현
- [~] Alias analysis — Determine when two memory references may overlap to drive simplification safely.
  > points-to 결과가 파이프라인에 올라왔지만 아직 dead_store_elimination/copy_propagation 등에서 소비하지 않음. 따라서 alias 정보는 여전히 보수적이며 파생 활용만 부분 진척
- [ ] Memory SSA — Bring loads/stores into SSA-like form to reason about memory dependencies.
  > SSA 변환(L37) + alias 분석(L95) 선행 필요. dominance frontier 구현 완료로 MemorySSA phi 삽입 위치는 결정 가능
- [~] Value-set analysis — Track possible integer/address ranges to resolve indirect jumps and bounds checks.
  > `ir/analyze/value_set.rs`의 interval 기반 값 범위 분석이 현재 IR enum에 맞게 정리되어 `IrFunction` 생성 경로에 연결되었고 결과를 보관/로그한다. 아직 indirect jump 해석, AST/최적화 소비, CFG edge 정밀화는 미구현
- [~] Range analysis — Infer variable ranges to simplify conditions and reconstruct comparisons.
  > 기본 interval 추적과 분기 조건 협착은 구현되어 파이프라인에 올라왔지만, 비교식 재구성 전반과 unsigned/signed/loop-carried 정밀화는 아직 부분 구현
- [x] Signedness inference — Infer signed vs unsigned from compares, extensions, shifts, and API usage.
- [x] Extension modeling — Track sign/zero-extend operations to recover correct C casts and types.
  > movsx/movsxd/movzx IR 핸들러 + 할당 크기 기반 (int32_t)/(uint8_t) 등 명시적 C 캐스트 출력 구현 완료
- [~] Pointer arithmetic lifting — Convert address math into ptr + i / &arr[i] forms where safe.
- [~] Array vs struct discrimination — Decide whether offset patterns represent arrays, structs, or unions.
  > `ir/analyze/struct_recovery.rs`의 offset-pattern 스캐너가 `IrFunction` 생성 경로에 연결되어 aggregate 후보와 array/struct 판정을 보관하고 요약 로그를 남김. 아직 union 구분, AST 타입 반영, 포인터 기반 base 추적 정밀화는 미구현
- [~] Field offset clustering — Group recurring offsets into candidate struct fields with consistent access sizes.
  > base register별 offset 묶음과 read/write 빈도 집계는 구현되어 파이프라인에 보관되지만, access size 반영과 실제 struct field/type 승격은 아직 미구현
- [~] Bitfield recovery — Detect mask/shift patterns and emit packed fields (or clearer helper expressions).
  > (x>>N)&M, (x&M)>>N, x&M 패턴 감지하여 bits[lo..hi] comment 주석 (bit_trick_recognition.rs). 구조체 필드 재구성은 미구현
- [~] Enum inference — Identify constant sets used in compares/switches and label them as enums.
  > auto_comment.rs에서 switch 케이스 3개 이상 시 상수 집합 주석 추가 (이름 부여는 미구현)
- [x] String literal propagation — Track string references to improve variable/function naming and format inference.
- [~] Format-string driven typing — Infer argument types from printf/scanf-like format strings.
  > auto_comment.rs: annotate_format_string_types()로 printf/scanf 계열 포맷 문자열에서 %d/%s/%p 등 지정자 파싱하여 예상 인자 타입 주석 생성. 타입 시스템 반영은 미구현
- [~] API prototype seeding — Use known library prototypes to seed parameter/return types at call sites.
  > `pe/api_prototypes.rs`의 외부 프로토타입 DB는 구현되어 있으며, `call_argument_analyzation.rs`에서 `ext_*` 외부 호출 이름을 조회해 알려진 API의 인자 개수로 추론 인자를 보수적으로 제한함. 반환 타입/파라미터 타입을 AST 타입 시스템에 반영하는 작업은 아직 미구현
- [~] Global variable recovery — Identify globals/TLS, their references, and assign stable names/types.
  > `ir/analyze/global_recovery.rs`가 data section 내 `Dereference(Constant(addr))` 패턴을 스캔해 후보를 복구하며, `pe/fire.rs`에서 decompile 직전 `PreDefinedOffsets`에 `global_<ADDR>` 이름으로 시드함. AST 타입 반영, 전역 식 자체를 named global로 치환하는 표현 복구, TLS와의 구분은 아직 미구현
- [~] TLS recovery — Recognize thread-local storage access sequences (GS/FS, TLV) and model them as TLS vars.
  > auto_comment.rs에서 TlsAlloc/TlsGetValue/pthread_getspecific 등 TLS API 호출 감지 및 주석 추가
- [~] Relocation-aware pointer typing — Use relocations/symbol refs to distinguish pointers from integers.
  > PE 기반 재배치 테이블 파싱 구현 완료 (_pe.rs) — HIGHLOW/DIR64 엔트리에서 포인터 주소 수집. 타입 추론 파이프라인에 relocation_addresses 전달 및 포인터/정수 구분 로직은 미구현
- [ ] PIC/GOT/PLT modeling — Correctly lift position-independent addressing and external linkage scaffolding.
  > 위치 독립 코드 모델링 필요
- [~] Vtable detection — Identify vtables via RTTI patterns and indirect call sites.
  > `fireball/src/pe/rtti.rs`에서 Win64 MSVC RTTI-backed vtable 탐지를 구현. `.rdata`/`.data`의 Type Descriptor 문자열과 Complete Object Locator(`pSelf` self-check 포함)를 보수적으로 검증한 뒤, metadata slot이 COL을 가리키고 연속 함수 포인터가 실행 섹션으로 향하는 경우만 vtable 후보로 인정한다. RTTI-less vtable 탐지와 indirect call site 사용처 연결은 아직 미구현
- [~] RTTI parsing — Use RTTI metadata (where present) to recover class names and inheritance links.
  > `rtti.rs`가 MSVC x64 Type Descriptor / Complete Object Locator / Class Hierarchy Descriptor 참조를 스캔해 class name, type descriptor RVA, COL RVA, CHD RVA, method count를 `Pe::rtti_entries()`로 노출하고 누락된 주소에 `vtable_for_*`, `typeinfo_for_*` 이름을 시드한다. 상속 그래프 해석, Win32 RTTI, 비-MSVC 포맷은 아직 미구현
- [ ] Devirtualization — Resolve virtual calls to concrete targets using type/points-to constraints.
  > vtable 분석 및 points-to 분석 선행 필요 — RTTI-backed vtable/RTTI 메타데이터 노출은 추가되었지만 virtual call site와의 연결 및 points-to 기반 타깃 축소는 아직 미구현
- [~] Constructor/destructor identification — Detect ctor/dtor patterns (vptr writes, base calls) for better class output.
  > auto_comment.rs: annotate_this_or_sret_pointer()로 첫 번째 파라미터의 deref store(ctor) / free/delete 호출(dtor) 패턴 주석 생성. 완전한 C++ 객체 모델링은 vtable 분석 선행 필요
- [ ] Inlining detection/undo — Identify inlined library/user functions and optionally “outline” them as calls.
  > 인라인 탐지 프레임워크 필요
- [~] Idiom-to-intrinsic lifting — Map SIMD/bit ops to intrinsics or clean C equivalents.
  > bit_trick_recognition.rs: try_recognize_intrinsic_idiom()로 branchless abs ((x^(x>>31))-(x>>31)), branchless min/max (x^((x^y)&mask)), de Bruijn ctz/clz (0x077CB531 등) 패턴 감지 및 주석 추가. SIMD intrinsic 매핑은 IR 수준 SIMD 모델링 선행 필요
- [~] Floating-point semantic modeling — Properly handle x87 stack vs SSE registers and rounding modes.
  > datatype.rs에서 Float32/Float64/Float80 구분과 보수적 XMM/YMM/ZMM·stN 스칼라 추론을 추가함. 다만 SSE/x87 명령 리프팅, x87 스택 시뮬레이션, MXCSR/예외/라운딩 모드 모델링은 아직 없음
- [~] Atomic/volatile recognition — Detect atomic sequences and volatile accesses to preserve ordering/side effects.
  > __atomic_*/__sync_*/InterlockedCompareExchange 등 원자 연산 호출 감지 및 주석 추가
- [ ] Self-modifying code detection — Detect writes to code pages; fall back to dynamic techniques when needed.
  > 동적 분석 프레임워크 필요
- [ ] Emulation-assisted lifting — Use instruction emulation for precise effects on flags/memory for tricky sequences.
  > 에뮬레이터 통합 필요
- [ ] Symbolic execution assistance — Use symbolic reasoning to simplify conditions and resolve computed targets.
  > 심볼릭 실행 엔진 필요
- [ ] Concolic execution — Combine concrete traces with symbolic constraints to cover more paths/targets.
  > 심볼릭/동적 실행 엔진 필요
- [ ] Dynamic tracing instrumentation — Record runtime targets for indirect calls/jumps to refine static results.
  > 동적 계측 프레임워크 필요
- [~] Obfuscation pattern detection — Identify opaque predicates, junk blocks, flattening dispatchers, and CFG noise.
  > auto_comment.rs: annotate_obfuscation_indicators()로 goto 밀도(>30%) 및 중첩 깊이(>10) 기반 난독화 휴리스틱 탐지 및 주석 부착 구현. opaque predicate/SMT 기반 분석은 미구현.
- [ ] Opaque predicate pruning — Prove predicates constant (or near-constant) via abstract interpretation/value sets.
  > 추상 해석/SMT 프레임워크 필요
- [ ] Control-flow flattening undo — Recover dispatcher-based state machines into structured control flow when possible.
  > loop_analyzation.rs에 while(true)+switch dispatch loop 탐지, switch 판별 변수/상수 case 나열, branch별 dispatcher 변수 재할당 탐지까지는 구현. state→block 매핑 복원, 상태 전이 그래프, CFG 재구성은 미구현.
- [ ] String/constant decryption patterning — Detect decode loops and represent results as recovered literals/arrays.
  > 에뮬레이션/심볼릭 실행 필요
- [ ] VM/protector detection — Identify virtualization/protection stubs and isolate them from normal decompilation flow.
  > VM 탐지 프레임워크 필요
- [x] Casts insertion minimization — Insert only necessary casts to keep output readable while type-correct.
- [x] Declaration placement heuristics — Place variable declarations near first use or at block start for readability.
- [x] Name recovery heuristics — Derive names from API usage, field offsets, strings, and role patterns (len, idx, buf).
- [x] Pretty-printer structuring — Emit stable formatting, block scopes, and expression parentheses to match C semantics.
- [ ] Semantic equivalence checking — Validate lifted output via re-execution/emulation on test inputs when feasible.
  > 에뮬레이션 기반 검증 필요
- [ ] Differential/variant analysis — Compare multiple builds/versions to improve function matching and naming.
  > 바이너리 비교 프레임워크 필요
- [ ] User-guided annotations — Accept manual types/structs/symbols to steer inference and re-run analyses.
  > UI/인터페이스 레이어 필요
- [~] DWARF debug-info parsing — Embedded DWARF subprogram names are merged into predefined offsets when `.debug_*` sections are present.
  > 타입/변수/스코프/라인 매핑은 아직 미구현
- [~] PDB/CodeView parsing — Use Windows PDB/CodeView records to recover symbols, types, and function boundaries.
  > pdb_parser.rs: pdb crate로 글로벌 심볼(Public) + 모듈별 심볼(Procedure/Data) 파싱 및 PreDefinedOffsets 통합 구현. C++/Rust 디맹글링 적용. 타입 스트림/섹션 기여 매핑은 미구현
- [~] Symbol table ingestion — Import ELF symbols/export tables to seed names, sizes, and addresses.
- [~] Name demangling — Convert mangled C++/Swift/Rust symbol names into human-readable identifiers.
  > C++ Itanium ABI 디맹글링 (cpp_demangle) + Rust 심볼 디맹글링 (rustc-demangle) 구현 완료. Swift 디맹글링 미구현
- [~] Linker map parsing — Read linker map files to seed symbol ranges and section-to-symbol ownership.
  > linker_map.rs: MSVC (.map) + GNU ld 맵 파일 파싱 구현. 심볼을 PreDefinedOffsets에 직접 통합. section-to-symbol 소유권 매핑은 미구현
- [ ] Build-ID / UUID correlation — Match binaries to symbol servers/artifacts using build identifiers.
  > 심볼 서버 통합 필요
- [~] .eh_frame / CFI exploitation — Use unwind CFI to infer stack layout, saved regs, and call frame structure.
  > fireball/src/pe/cfi_parser.rs에서 Win64 `.pdata` / `.xdata` unwind metadata 파싱 구현. RUNTIME_FUNCTION 범위, prolog 크기, frame register/offset, 저장된 GPR/XMM 레지스터, 고정 stack allocation, handler/chained 여부를 요약하고 `Pe::unwind_functions()`로 노출한다. Win32 SEH scope/try-catch 재구성, ELF/Mach-O/ARM unwind 포맷은 미구현
- [x] Stack cookie / canary suppression — Detect compiler-inserted canary checks and omit them from high-level output.
- [x] Stack probing suppression — Identify_chkstk/stack-touch loops and render as allocation semantics.
- [x] CET/CFG pattern recognition — Detect indirect-branch hardening (CET IBT/SHSTK, CFG) and de-noise it.
- [x] NOP/padding classification — Treat alignment padding and multi-byte NOPs as non-semantic fillers.
- [~] Hot–cold function chunk stitching — Reconnect split function fragments produced by PGO or linker optimizations.
  > ir_function.rs에 detect_address_gap_chunks() 구현 — 4KB 이상 주소 갭으로 hot-cold 분리 감지 및 로깅. structuring.rs는 이 청크 정보를 사용해 CFG 순회 시 같은 청크 successor를 우선 방문하는 보수적 재정렬을 수행하지만, 실제 청크 재조합(블록 재배치/함수 병합)은 미구현
- [~] Multi-entry “function” handling — Represent shared code tails/entries safely without inventing invalid C.
  > dominator.rs에 multi_entry_blocks() 구현 — 외부 진입점 감지 및 로깅. 공유 꼬리 블록의 안전한 C 표현(goto/label 또는 함수 분리)은 미구현
- [ ] Trampoline/hook stub detection — Identify detours/patch stubs and recover the intended target flow.
  > 바이너리 패턴 분석 필요
- [ ] Overlapping decode detection — Flag overlapping instruction streams (common in obfuscation) and branch accordingly.
  > 디스어셈블러 변경 필요
- [~] Relocation-driven code pointer scan — Use relocations to identify embedded function pointers and jump targets.
  > analysis.rs: find_code_relocations()로 실행 가능 섹션 내 재배치 엔트리 식별 구현. 함수 포인터/점프 대상 자동 디스어셈블리 연결은 미구현
- [ ] Read-only pointer pool discovery — Detect const pointer tables (vtables, dispatch tables) via xref density and alignment.
  > 포인터 풀 분석기 필요
- [~] Wide-string identification — Detect UTF-16/UTF-32 string blobs and render as wide literals or arrays.
  > analysis.rs: scan_wide_strings()로 UTF-16LE 문자열 스캔 구현 (최소 4자). UTF-32/AST 리터럴 생성은 미구현
- [ ] String encoding inference — Infer ASCII/UTF-8/UTF-16/locale encodings based on usage and API context.
  > 인코딩 추론 프레임워크 필요
- [ ] Constant pool labeling — Classify literal pools (ARM literal loads, MIPS gp-relative) and attach symbolic names.
  > 아키텍처별 리터럴 풀 분석 필요
- [ ] Architecture mode tracking — Track ARM/Thumb state (and similar mode switches) to disassemble correctly.
  > 디스어셈블러 레이어 변경 필요
- [ ] ARM IT-block recovery — Reconstruct predicated instruction blocks into structured conditional logic.
  > ARM 디스어셈블러 변경 필요
- [ ] MIPS/SPARC delay-slot modeling — Account for delay slots to avoid incorrect control-flow and side-effect order.
  > MIPS 디스어셈블러 변경 필요
- [ ] Branch-likely semantics — Handle “likely” branches (MIPS) that nullify delay slots on non-taken paths.
  > MIPS 디스어셈블러 변경 필요
- [ ] PAC stripping/inference (ARM) — Recognize pointer authentication sequences and recover the underlying pointer flow.
  > ARM PAC 모델링 필요 — IR 변경
- [ ] Syscall number resolution — Map syscall numbers to names/ABIs to improve semantics and signatures.
  > syscall 명령어 IR 핸들러 미구현 — arch 레이어에서 syscall/sysenter 디코딩 선행 필요
- [ ] Kernel-vs-user ABI detection — Adjust calling convention and privilege assumptions based on context and imports.
  > ABI 컨텍스트 감지 필요
- [ ] PLT lazy-binding de-noising — Recognize PLT/GOT resolver scaffolding and collapse to direct external calls.
  > PLT 구조 분석 필요
- [ ] Veneer/stub resolution (ARM) — Collapse linker-generated veneers to their ultimate targets.
  > ARM 링커 스텁 분석 필요
- [~] Thunk chain flattening — Resolve multi-hop jumps/calls through stubs into a single callsite target.
- [x] Call graph construction — Build an interprocedural call graph for navigation, prioritization, and propagation.
  > AST 함수 바디에서 AstCall::Function 타겟 수집하여 caller/callee 그래프 구축 (call_graph.rs)
- [x] Call graph SCC analysis — Detect recursion and mutually-recursive clusters to guide structuring and type propagation.
  > Tarjan SCC 알고리즘 + find_recursive_functions 구현 (call_graph.rs)
- [x] Function “importance” ranking — Use graph centrality/xref counts to prioritize analysis and naming.
  > rank_by_importance: caller 수(in-degree) 기반 함수 중요도 순위 (call_graph.rs)
- [ ] Path-sensitive condition refinement — Track predicate constraints per path to simplify branches more aggressively.
  > 경로 민감 분석 프레임워크 필요
- [ ] Abstract interpretation (intervals) — Compute intervals/ranges via fixpoint iteration to simplify checks and bounds.
  > 추상 해석 프레임워크 필요
- [ ] Widening/narrowing strategy — Ensure abstract interpretation converges quickly while retaining precision.
  > 추상 해석 프레임워크 선행 필요
- [ ] Nullness analysis — Infer where pointers can/can’t be null to simplify guards and improve types.
  > 포인터 분석 프레임워크 필요
- [~] “ptr+len” pairing detection — Detect common buffer pointer/length parameter pairs to label and type them.
  > auto_comment.rs: annotate_ptr_len_pairs()로 연속 파라미터의 deref + comparison/bound 사용 패턴 탐지하여 ptr+len 쌍 주석 생성. 타입 시스템 반영은 시그니처 분석 확장 필요
- [ ] Bounds/size inference — Infer buffer sizes from compares, loop trip counts, and allocation sizes.
  > 범위 분석 프레임워크 필요
- [ ] Heap allocation site typing — Infer heap object “types” from allocation size + subsequent field accesses.
  > 힙 할당 추적 프레임워크 필요
- [~] Ownership/escape heuristics — Infer whether pointers escape scope to decide stack vs heap representation.
  > `ir/analyze/escape.rs`가 points-to 결과를 바탕으로 포인터형 레지스터가 공통 호출 인자 레지스터로 살아남거나 non-stack memory store로 저장될 때 intraprocedural escape로 표기한다. heap/stack ownership 결정, stack-slot escape, interprocedural propagation은 미구현
- [x] Lifetime-based scoping — Emit tighter C scopes based on live ranges to reduce variable clutter.
- [x] Variable coalescing by interference — Merge non-overlapping temporaries into a single C local when safe/readable.
- [~] Register spill pattern recovery — Identify spill/reload sequences and treat them as variable preservation, not logic.
  > control_flow_cleanup.rs: annotate_register_spill_patterns()로 temp=var; call(); var=temp 패턴 탐지 및 주석 부착 구현. 레지스터 할당 역추적 기반 제거는 미구현.
- [ ] Shadow-space modeling (Win64) — Recognize Win64 home space usage and suppress it in output.
  > Win64 ABI 스택 모델링 필요
- [ ] Red-zone modeling (SysV) — Recognize red-zone stack usage and prevent mis-classifying it as locals.
  > SysV ABI 레드존 모델링 필요
- [~] Alloca/VLA recovery — Detect dynamic stack allocations and render as alloca/VLA-like constructs.
  > auto_comment.rs에서 alloca/__chkstk/__alloca_probe 호출 감지 및 주석 추가
- [~] Alignment inference — Infer alignment requirements from masking, loads, and SIMD usage to improve types.
  > x & ~(N-1) 정렬 마스크 패턴 감지 및 주석 추가 (align down/up to N)
- [ ] Packed-struct inference — Detect unaligned field accesses suggesting packed layouts.
  > 비정렬 접근 분석 필요
- [ ] Union vs struct inference — Infer unions when conflicting field types/sizes share the same base offset.
  > 타입 통합 프레임워크 필요
- [ ] Field type unification — Solve constraints from loads/stores/casts to converge on consistent field types.
  > 타입 제약 풀이기 필요
- [ ] Type unification solver — Use constraint solving (unification) over IR to reconcile conflicting type evidence.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Probabilistic type scoring — Rank candidate types using likelihood models (API usage, sizes, ops, casts).
  > 확률적 타입 추론 프레임워크 필요
- [ ] Bitcast/punning recognition — Detect intentional type punning (e.g., float-int) and emit explicit unions/memcpy casts.
  > 타입 펀닝 분석 필요
- [ ] Strict-aliasing-safe emission — Prefer safe idioms (memcpy) when the recovered logic implies aliasing-UB in C.
  > 앨리어싱 분석 프레임워크 필요
- [ ] Signed-overflow modeling — Preserve semantics by avoiding C signed-overflow UB in emitted expressions.
  > 오버플로우 분석 프레임워크 필요
- [ ] Pointer provenance heuristics — Keep integer-vs-pointer distinctions stable to avoid nonsense pointer arithmetic.
  > 포인터 출처 추적 프레임워크 필요
- [~] Checked arithmetic recovery — Recognize overflow-checked add/sub/mul patterns and render as guarded ops.
  > (a+b)<a, a>(a+b), (a-b)>a 패턴 + (result/a)!=b mul 오버플로우 + (result>>32)!=0 high-bits 검사 패턴 감지 (bit_trick_recognition.rs). guarded op 변환은 미구현
- [x] Saturating arithmetic detection — Detect clamp/min/max patterns used for saturation.
  > x > C ? C : x 및 x < C ? C : x 패턴을 ternary에서 감지하여 comment 주석 추가 (bit_trick_recognition.rs)
- [~] Crypto primitive recognition — Identify AES/SHA/CRC-like instruction patterns and annotate recovered routines.
  > AES S-box, SHA/MD5/CRC 초기화 상수 기반 함수 핑거프린팅 (auto_comment.rs). 명령어 패턴 매칭은 미구현
- [~] Hash function fingerprinting — Detect common hash families via constants and mixing patterns to label functions.
  > MD5/SHA-1/SHA-256/CRC-32 초기화 상수 3개 이상 포함 시 함수 주석 추가 (auto_comment.rs)
- [ ] Parser/state-machine inference — Recover table-driven parsers and emit explicit state enums/dispatch.
  > 상태 머신 복구 프레임워크 필요
- [x] Error-handling “cleanup” patterns — Recognize goto cleanup shapes and render as single-exit cleanup blocks.
- [~] Lock/unlock pairing recognition — Identify synchronization API pairs and reconstruct critical sections cleanly.
  > auto_comment.rs에서 pthread_mutex_lock/unlock, EnterCriticalSection 등 호출 감지 및 주석 추가
- [~] Reference-count pattern recognition — Detect AddRef/Release-style idioms and annotate ownership semantics.
  > auto_comment.rs에서 AddRef/Release/InterlockedIncrement/Decrement 호출 감지 및 주석 추가
- [ ] RAII shape recovery — Reconstruct destructor-driven cleanup patterns from C++ unwinding/cleanup code.
  > C++ 소멸자 패턴 분석 필요
- [~] Static-local guard detection — Detect thread-safe static initialization guards and de-noise them.
  > auto_comment.rs에서 __cxa_guard_acquire/release, _Init_thread_header/footer 호출 감지 및 주석 추가
- [ ] Coroutine state machine recovery — Identify compiler-generated coroutine frames and render as state-based logic.
  > 코루틴 상태 머신 분석 필요
- [ ] Async/await frame recovery — Detect async state machines (where applicable) and present as structured states.
  > 비동기 상태 머신 분석 필요
- [ ] Objective‑C metadata parsing — Use ObjC runtime sections to recover classes, selectors, and method names.
  > ObjC 런타임 메타데이터 파서 필요
- [ ] Swift metadata exploitation — Use Swift reflection/runtime metadata to recover type and symbol context (when present).
  > Swift 런타임 메타데이터 파서 필요
- [ ] Go pclntab parsing — Use Go runtime tables to recover function names, line info, and stack maps.
  > Go 런타임 테이블 파서 필요
- [~] Rust panic/runtime patterning — Identify Rust runtime scaffolding to focus on user logic.
  > rust_begin_unwind/core::panicking/panic_fmt 등 Rust 패닉 런타임 호출 감지 및 주석 추가
- [ ] .NET / managed boundary detection — Detect CLR headers/IL stubs vs native code and route analysis accordingly.
  > CLR/IL 분석기 필요
- [ ] IL2CPP metadata usage — Use Unity IL2CPP metadata to recover method/type names when available.
  > IL2CPP 메타데이터 파서 필요
- [ ] JNI boundary recognition — Detect JNI entrypoints/signatures to recover parameter types and naming.
  > JNI 시그니처 파서 필요
- [ ] Inline vectorization reversal — Recognize auto-vectorized loops and emit scalar loops (or intrinsics) for clarity.
  > SIMD 역벡터화 프레임워크 필요
- [ ] SIMD lane-type inference — Infer whether vectors hold ints/floats/bytes from ops and shuffles.
  > SIMD 타입 추론 프레임워크 필요
- [ ] Endianness-aware reconstruction — Normalize byte swaps and bitfield extraction based on target endianness.
  > 엔디안 인식 프레임워크 필요
- [ ] E-graph expression simplification — Use e-graphs to find globally minimal equivalent expressions.
  > e-graph 라이브러리 통합 필요
- [ ] Cost-model pretty-printing — Choose among equivalent expressions/structures using a readability cost function.
  > 가독성 비용 모델 필요
- [x] Parenthesis minimization — Emit minimal parentheses while preserving exact precedence and associativity.
- [x] Operator canonicalization — Normalize commutative/re-associated expressions for stable, comparable output.
- [ ] Semantic hashing (per basic block) — Compute semantics-aware hashes for matching and regression tests.
  > 의미론적 해싱 프레임워크 필요
- [ ] Cross-binary symbol transfer — Transfer names/types from a symbolized build to a stripped build via matching.
  > 바이너리 매칭 프레임워크 필요
- [ ] CFG graph matching — Match functions across versions using graph isomorphism-ish heuristics and features.
  > 그래프 동형 매칭 필요
- [ ] Differential analysis across builds — Use multi-version diffs to isolate user code vs toolchain noise.
  > 멀티 버전 비교 프레임워크 필요
- [ ] Incremental reanalysis — Recompute only affected functions after user type hints or code/data reclassification.
  > 증분 분석 프레임워크 필요
- [ ] Analysis artifact caching — Cache IR/SSA/types per function to speed repeated runs and UI interaction.
  > 캐시 프레임워크 필요
- [ ] Parallel per-function pipeline — Run independent analyses concurrently with deterministic merge rules.
  > 병렬 분석 프레임워크 필요
- [x] Deterministic tie-breaking — Ensure stable output by deterministic ordering in heuristics and naming.
  > Reinforced with deterministic successor batching in `pe/fire/analyze_all.rs` and deterministic trace ordering in `ir/analyze/variables.rs`.
- [ ] Confidence scoring per recovery — Attach confidence to inferred types/structuring to guide UI and fallbacks.
  > 신뢰도 추적 프레임워크 필요
- [x] Provenance tracking — Track “why” a type/name/structure was inferred to enable explainable decompilation.
- [ ] Fallback tiering strategy — Gradually degrade from structured C → labeled blocks → mixed C/asm for hard cases.
  > 다단계 출력 전략 프레임워크 필요
- [~] Mixed-mode emission — Emit C with inline asm for instructions/regions that can’t be safely lifted.
- [x] Address-to-source annotation — Emit comments with original addresses/blocks to aid auditing and patching.
- [x] Auto-comment synthesis — Generate brief comments for recognized scaffolding (cookies, probes, guards, stubs).
- [x] User rule/rewriter engine — Allow user-defined peephole rewrites over IR to simplify domain-specific idioms.
- [x] Scripting pass hooks — Provide APIs to inject custom analyses, naming, and type rules into the pipeline.
- [~] Entropy-based packing detection — Flag packed/encrypted sections via entropy and anomalous permissions.
  > analysis.rs: shannon_entropy() + section_entropies()로 섹션별 Shannon 엔트로피 계산 및 >7.0 패킹 탐지 구현. 자동 언패킹은 미구현
- [~] RWX anomaly heuristics — Detect suspicious memory permission layouts (RWX) to inform unpacking/dynamic steps.
  > analysis.rs: detect_rwx_sections()로 동시 Read+Write+Execute 섹션 탐지 구현. 동적 언패킹 트리거는 미구현
- [ ] Dynamic unpack + reimport — Dump unpacked memory image and restart static analysis on the clean snapshot.
  > 동적 언패킹 프레임워크 필요
- [ ] Record/replay trace integration — Use deterministic traces to resolve indirect targets and validate semantics.
  > 트레이스 통합 필요
- [ ] Coverage-guided path exploration — Drive dynamic runs to maximize basic-block coverage for target resolution.
  > 커버리지 기반 탐색 필요
- [ ] Heap snapshot correlation — Use runtime heap snapshots to infer object layouts and field interpretations.
  > 런타임 힙 분석 필요
- [ ] API-hook based target capture — Capture indirect call/jump targets at runtime to refine static CFG/callgraph.
  > 런타임 후킹 필요
- [~] Anti-debug/anti-VM spotting — Detect common checks and annotate them as environment/analysis defenses.
  > auto_comment.rs에서 IsDebuggerPresent, ptrace, sysctl 등 안티디버그 API 호출 감지 및 주석 추가
- [~] Timing-check classification — Identify high-resolution timer checks and treat them as anti-analysis scaffolding.
  > auto_comment.rs에서 QueryPerformanceCounter/GetTickCount/rdtsc 등 호출 감지 및 주석 추가
- [~] Decompression routine detection — Detect inflate/LZ-like loops and label them to reduce noise in reverse engineering.
  > auto_comment.rs에서 zlib/DEFLATE/LZ 상수 핑거프린팅으로 압축 해제 루틴 감지 및 주석 추가
- [~] Config/string xref mining — Extract likely config keys/paths/URLs by xref patterns and usage context.
  > auto_comment.rs: annotate_config_string_xrefs()로 함수 내 문자열 리터럴에서 URL/경로/config 키 패턴 추출 + annotate_domain_vocabulary()로 도메인 어휘 주석 생성. xref 기반 전체 바이너리 분석은 미구현
- [x] Function purity/effect inference — Infer pure/readonly-like behavior to enable stronger simplifications.
  > infer_pure_functions: 로컬 바디 분석 + 호출 그래프 전이적 추론으로 순수 함수 식별 (call_graph.rs)
- [ ] API effect modeling — Maintain a database of external function effects (alloc/free/throws/locks) for safer DCE.
  > 외부 함수 효과 DB 필요
- [ ] SMT-backed micro-equivalence — Use SMT solvers for small straight-line regions to validate simplifications.
  > SMT 풀이기 통합 필요
- [ ] Test-input synthesis — Generate inputs (where feasible) to compare lifted behavior vs emulation for validation.
  > 테스트 입력 생성 프레임워크 필요
- [~] Program slicing — Extract only statements that influence a chosen value (e.g., return or parameter) to reduce noise.
  > `ir/analyze/slicer.rs`의 기본 backward slice가 `IrFunction` 생성 경로에 연결되어 return-value slice를 계산, 보관, 로그한다. 아직 parameter/API sink 기준, interprocedural slice, AST/최적화 소비는 미구현
- [~] Backward slicing (from sinks) — Trace dependencies backward from API calls/returns to recover intent and types.
  > 현재는 return-value 기준 register dependency 추적만 구현되어 있으며 statement-level coverage 로깅까지 연결됨. API call sink, memory dependency, SSA def-use 기반 정밀화는 아직 미구현
- [~] Forward slicing (from sources) — Track how inputs (args/globals) propagate to outputs to identify roles.
  > `ir/analyze/slicer.rs`의 보수적 parameter-seeded forward slice가 `IrFunction` 생성 경로에 연결되어 보관/로그된다. 현재는 x86-64 공통 인자 레지스터 seed, register-level 전파, 조건식/일부 call edge만 다루며, global source, memory/alias propagation, API sink 기준 정밀화, interprocedural slice, AST/최적화 소비는 아직 미구현
- [~] Taint analysis — Mark data from sources and follow it through transforms to classify inputs/outputs and checks.
  > `ir/analyze/taint.rs`의 기본 register-level taint가 `IrFunction` 생성 경로에 연결되어 보관/로그되며, x86-64 공통 인자 레지스터와 call return register에 대한 보수적 seed까지 추가됨. 아직 memory/global source, sink classification, interprocedural propagation, AST/최적화 소비는 미구현
- [~] Data-dependence graph construction — Build def-use dependencies as a graph to drive refactoring and naming.
  > `ir/analyze/data_dependence.rs`의 기본 statement-level register def-use graph가 `IrFunction` 생성 경로에 연결되어 보관/로그된다. 현재는 평탄화된 statement 순서와 call return register(`rax/eax`)만 다루는 보수적 구현이며, memory dependency, SSA-level precision, control-sensitive merge, AST/이름짓기 소비는 미구현
- [~] Control-equivalence detection — Find predicates that are logically the same to simplify repeated conditions.
  > 구조적 동등성(expr_structurally_equal) + 조건 정규화(operator_canonicalization) + 정규화 후 연속 조건 병합으로 부분 구현. SMT 기반 논리적 동치는 미구현
- [~] Predicate abstraction — Replace complex expressions with boolean symbols during structuring; refine later.
  > structured_region_lowering.rs에서 `if`-only 보수적 predicate abstraction 추가. 순수한 control-prefix `Declaration/Assignment`를 허용하고, 조건식이 복잡하면 지역 `bool` temp로 추상화한다. loop header/latch, switch selector, SMT 기반 정제는 미구현.
- [~] SESE region discovery — Identify single-entry/single-exit regions to map cleanly into structured C blocks.
  > `structuring.rs`의 기존 StructuredRegion 빌더가 보수적인 goto-free SESE region metric을 노출/로그한다. 아직 irreducible region 정리, edge contract enforcement, full AST switchover는 미구현
- [~] Interval-based structuring — Use interval analysis to structure CFGs into loops/conditionals deterministically.
  > `structuring.rs`가 현재 CFG에 대한 보수적인 first-pass interval partition(`header/block_ids/exit_blocks`)을 계산하고 `IrFunction`/structuring log에 interval count, multi-block count, max interval size를 노출한다. 아직 interval collapse 반복, quotient graph 재작성, 실제 interval-driven region builder 전환은 미구현
- [~] Relooper-style structuring — Convert irreducible CFGs into structured forms using labeled regions and dispatch.
  > `structuring.rs`가 이제 fallback `Goto` target에 대해 보수적으로 `Label`을 자동 삽입하고, `labels` / `unresolved goto targets` metric을 노출한다. shared-tail/irreducible fallback이 최소한 일관된 goto+label 형태로 AST lowering 되지만, dispatcher region, state variable, block reordering, dispatch loop synthesis는 아직 미구현
- [~] If-conversion reversal — Detect predicated/select-based code and recover explicit if/else.
  > AST 최적화 파이프라인에 보수적인 `if_conversion_reversal` 패스가 추가되어, `lhs = cond ? (inner ? a : b) : c` 같은 중첩 ternary assignment를 중첩 `if/else` assignment 트리로 역변환한다. 기존 `ternary_recovery`가 다시 접어버리지 않도록 직접적인 중첩 ternary branch가 있는 경우에만 동작한다. 일반적인 단일 ternary, declaration initializer, non-variable LHS, cmov/select 기반 IR-level 판별은 아직 미구현
- [~] Duff’s device detection — Recognize unrolled switch/loop hybrids and emit canonical loop + switch forms.
  > `loop_analyzation.rs`가 이제 `while(true)`뿐 아니라 `DoWhile(true)`까지 재귀적으로 살피며, top-level `Switch` 뒤에 추가 loop-body work가 이어지는 무한 loop를 `likely Duff's device switch/loop hybrid` 주석으로 보수적으로 표기한다. 또한 기존 state-machine dispatch 주석도 `DoWhile(true)`를 인식한다. 아직 실제 case fallthrough 분석, canonical loop+switch 재구성, unrolled body 회수, dispatch/state 변수 복원은 미구현
  > 루프 구조/시맨틱 분석 프레임워크 구현 필요
- [~] Loop unrolling reversal — Detect unrolled bodies and recover compact loops with correct bounds/strides.
  > control_flow_cleanup.rs에서 blake3 해시로 루프 본문 반복 패턴 감지 및 주석 추가 (실제 압축은 미구현)
- [~] Strength-reduction reversal — Recognize induction updates turned into adds/shifts and restore multiplications/indexing.
  > (x<<N)+x → x*(2^N+1), (x<<N)-x → x*(2^N-1), (x<<N)+(x<<M) → x*(2^N+2^M) 변환 (bit_trick_recognition.rs). 루프 인덕션 변수 역변환은 미구현
- [x] Common tail factoring — Merge duplicated tails into shared blocks or structured break/return paths.
  > if/else 분기의 동일한 후미 문장을 blake3 해싱으로 감지 후 분기 밖으로 추출 (control_flow_cleanup.rs)
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [x] Early-return normalization — Transform nested conditionals into guard clauses to resemble typical source style.
- [x] Guarded-call recovery — Detect if (ptr) call(ptr) patterns from compare+branch around indirect calls.
- [ ] Call/ret pairing validation — Sanity-check stack/ABI effects around call sites to catch bad disassembly.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Fallthrough intent inference — Decide whether adjacent blocks represent switch fallthrough vs accidental layout.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [x] Case clustering — Group cases with identical bodies into case A: case B: patterns.
  > Structural Blake3 hashing merges adjacent identical bodies, including comment/empty-only placeholder labels (`switch_reconstruction.rs`).
- [x] If-ladder to switch promotion — Upgrade compare/jump ladders into switch even without explicit tables.
  > if-else 체인에서 x==c 패턴 감지 → switch문 변환 구현 완료 (switch_reconstruction.rs). 최근 DoWhile 재귀도 추가되어 loop 정규화 이후 숨겨진 switch 후보도 계속 탐색함.
- [~] Loop exit classification — Distinguish break, continue, return, and goto-like exits from edge shapes.
  > loop_analyzation.rs: annotate_loop_exit_patterns()로 루프 내 goto-as-break 및 return 탈출 패턴 탐지 구현, 이어서 convert_loop_gotos_to_break_continue()로 루프 바로 다음 레이블로 향하는 goto를 `Break`로, 주석이 달린 continue-like back-edge를 `Continue`로 안전하게 치환. 중첩 루프를 관통하는 탈출과 break-flag 기반 재작성은 아직 미구현.
- [~] Multi-exit loop rewriting — Rewrite nested gotos into structured loops with break flags where safe.
  > loop_analyzation.rs: annotate_loop_exit_patterns()로 다중 탈출 루프(2+ exits) 탐지 및 주석 부착 구현. break 플래그 기반 구조 변환은 미구현.
- [x] Infinite-loop recognition — Detect for(;;) loops (watchdog, event loop) and suppress misleading conditions.
  > while(1)/while(nonzero) → while(true) 정규화 구현 완료 (loop_analyzation.rs)
- [~] Finite-state variable detection — Identify the “state” variable driving dispatch to reconstruct state machines.
  > loop_analyzation.rs: annotate_state_machine_loops()로 while(true)+switch 패턴 탐지, switch 판별 변수 이름 및 case 상수 나열 주석 부착 구현. enum 복원 및 실제 상태 머신 AST 리프팅은 미구현.
- [~] Dispatcher-variable recovery — Reconstruct flattened CFG dispatch variables used by obfuscators or coroutines.
  > loop_analyzation.rs: annotate_state_machine_loops()가 switch 판별 변수와 branch별 동일 변수 재할당 여부를 주석으로 복구. 값 추적 기반 dispatcher-state 매핑, coroutine frame/state 복원은 미구현.
- [~] Region reordering heuristics — Choose source-like block order based on dominator/postdominator relationships.
  > `structuring.rs`가 이제 SESE이면서 `goto/label` 폴백이 없는 `Sequence` 형제 region에 한해 dominator/postdominator 관계가 역순임을 명확히 가리킬 때 보수적으로 재배열한다. 전역 block reordering, hot/cold split 복원, flattening dispatcher 재조립은 미구현.
- [~] Code layout de-biasing — Ignore physical layout heuristics when PGO/hot-cold splitting skews adjacency.
  > `structuring.rs`가 address-gap chunk를 감지한 뒤 reverse-postorder DFS에서 같은 청크 successor를 우선 방문해 block-id/주소 정렬 편향을 줄인다. 전역 CFG 재배치, hot/cold chunk 재조합, chunk 간 논리 순서 복원은 미구현.
- [~] Pointer escape analysis — Infer whether a pointer escapes a scope to decide stack vs heap semantics.
  > `ir/analyze/escape.rs`가 points-to 기반 register-level escape 요약을 생성하고 `IrFunction`에 보관한다. 현재는 call-argument escape와 non-stack memory-store escape만 보수적으로 추적하며, stack-slot/base-pointer 정밀 추적과 heap vs stack semantics 결정은 미구현
- [~] Interprocedural escape propagation — Propagate escape facts through calls to refine lifetimes and aliasing.
  > `ir_to_ast.rs`가 이제 생성된 `IrFunction`들을 한 번 모은 뒤 `entry address -> FunctionSummary` 맵을 만들고, direct internal callee의 `escaped_registers`를 caller의 동일 argument register escape로 한 번 투영한다. 따라서 caller-side escape fact 갱신이 시작됐지만, 재귀/순환 call-graph 고정점, indirect call target 해석, context-sensitive propagation은 아직 미구현
- [~] Object sensitivity (points-to) — Distinguish heap objects by allocation site to improve field/type recovery.
  > `points_to.rs`가 이제 `JumpByCall`마다 synthetic `Heap(call_site_id)` 객체를 만들고 `rax/eax` 반환 경로에 연결해 call-site 기준 heap object 구분을 시작했다. 하지만 allocator 식별, 실제 pointer-return 판별, field-sensitive 후속 소비, heap type recovery는 아직 미구현
- [~] Field-sensitive alias analysis — Track aliasing per field/offset rather than per base pointer.
  > `ir/analyze/field_alias.rs`가 aggregate recovery의 base+offset field 후보를 object-sensitive points-to 타깃과 결합해 `(abstract_base, offset)` 단위 field projection을 생성하고 `IrFunction`에 보관한다. 따라서 동일 base라도 offset별로 분리된 alias surface가 생겼지만, load/store-level must-alias 판단, nested field chain, memory SSA 소비자는 아직 미구현
- [~] Array shape inference — Infer 1D/2D array dimensions from nested index math and stride constants.
- Only conservative 1D constant-stride shape hints are exposed today via `ir/analyze/array_shape.rs`, built from existing aggregate recovery (`base`, `stride`, observed offset range, contiguous coverage). Nested index math, 2D dimensions, and dynamic bound inference remain unimplemented.
  > 타입 제약 풀이 프레임워크 필요
- [~] Stride detection — Detect constant strides in address arithmetic to recover arr[i] and struct arrays.
  > bit_trick_recognition.rs에서 var + index*STRIDE 패턴 감지 (알려진 타입 크기만 주석 추가)
- [~] Container recognition — Detect vector/list/map-like access idioms to label data structures.
  > `auto_comment.rs`가 preserved symbol name이 남은 `AstCall::Unknown` 호출에서 `vector/deque`, `list/forward_list`, `map/unordered_map/hash_map` 계열 메서드명(`push_back`, `insert`, `find`, `rehash` 등)을 보수적으로 감지해 container-like operation 주석을 추가한다. 그러나 이는 이름 기반 힌트일 뿐이며, 실제 구조체 레이아웃 복구, iterator/state tracking, field/type propagation은 아직 미구현
- [~] Linked-list shape recognition — Identify next/prev pointers and traversal loops to annotate list structures.
  > `loop_analyzation.rs`가 `while (p) { ... p = p->next; }` 같은 패턴을 보수적으로 감지해 `likely linked-list/iterator traversal` 주석을 추가한다. 다만 이는 self-deref 진행형 순회 힌트에 한정되며, 실제 `next`/`prev` 필드 식별, 노드 레이아웃 복구, 다중 포인터 상관관계, 삽입/삭제 연산 복구는 아직 미구현
- [~] Tree shape recognition — Detect left/right traversal patterns and recursive calls to label tree nodes.
  > `auto_comment.rs`가 현재 함수로 재귀 호출되는 `AstCall::Function` 인자에서 `.left` / `.right` / `left_child` / `right_child` / `lchild` / `rchild` member access를 보수적으로 감지해 `likely recursive tree traversal (...)` 주석을 추가한다. 그러나 이는 이름 기반 재귀 child-call 힌트일 뿐이며, 실제 node layout 복구, 부모 포인터/균형 정보 식별, 반복형 tree walk, 검색트리/힙 구분은 아직 미구현
- [~] Hash table recognition — Detect modulo/mask bucket indexing and chaining/probing loops.
  > `auto_comment.rs`가 `arr[hash % n]`, `arr[hash & mask]` 같은 bucket index 표현을 보수적으로 감지해 `likely hash bucket indexing` 주석을 추가하고, 같은 loop body 안에서 bucket index 사용과 `idx = (idx + k) % n` / `idx = (idx + k) & mask` 형태 갱신이 함께 보이면 `likely hash table probing loop` 주석을 추가한다. 그러나 이는 AST 기반 힌트일 뿐이며, 실제 chaining node 식별, load-factor/rehash 추론, probe 종류(linear/quadratic/double hashing) 판별, hash node layout 복구는 아직 미구현
- [~] Ring-buffer recognition — Detect wrap-around arithmetic and head/tail usage patterns for queue semantics.
  > `auto_comment.rs`가 `head`/`tail`/`read_pos`/`write_idx` 계열 이름을 가진 위치 변수가 `idx = (idx + k) % cap` 또는 `idx = (idx + k) & mask` 형태로 갱신되는 패턴을 보수적으로 감지해 `likely ring-buffer head/tail advance` 주석을 추가하고, 같은 loop body 안에서 그런 위치 변수가 배열 인덱스로도 사용되면 `likely ring-buffer queue loop` 주석을 추가한다. 그러나 이는 이름+AST 기반 힌트일 뿐이며, 실제 버퍼/용량 필드 복구, enqueue/dequeue 방향 판별, producer/consumer 구분, 다중 인덱스 동기화, lock-free queue semantics 복구는 아직 미구현
- [~] Ref-count field identification — Recognize increment/decrement patterns on a stable offset as refcount fields.
  > `auto_comment.rs`가 `obj.refcount = obj.refcount +/- 1` 계열의 assignment-based member-field self-update를 보수적으로 감지해 `likely refcount field increment/decrement` 주석을 추가한다. 다만 이는 refcount-like field name(`refcount`, `refs`, `use_count`, `retain_count` 등)에 의존하는 AST 기반 힌트일 뿐이며, 실제 stable offset 추적, alias-aware pointer field identification, atomic builtin(`Interlocked*`, `fetch_add`) 투영, 이름이 없는 필드 복구는 아직 미구현
- [~] Length-field pairing — Detect {ptr,len} or {buf,cap} field pairs in structs by correlated usage.
  > `auto_comment.rs`가 같은 AST body 안에서 sibling member-field path를 수집한 뒤, `.buf` / `.data` / `.ptr` / `.str` 계열 필드가 deref/array base로 실제 사용되고 `.len` / `.size` / `.cap` 계열 필드가 비교식/loop bound 문맥에서 실제 사용되는 경우에만 `likely (buf, len/cap) field pair: ...` 주석을 추가한다. 다만 이는 이름+국소 사용 패턴 기반 힌트일 뿐이며, 실제 struct layout recovery, alias-aware field correlation, 이름 없는 offset pair 복구, type propagation은 아직 미구현
- [~] Ownership transfer inference — Infer “takes ownership” vs “borrows” based on frees/releases after calls.
  > `auto_comment.rs`가 같은 block 안의 `call(arg); cleanup(arg);` / `call(obj.field); free(obj.field);` 계열의 직접적인 post-call release 패턴을 보수적으로 감지해 `likely borrow-only call: caller releases ... after call` 주석을 추가한다. 다만 이는 인접한 AST statement와 직접적인 variable/member-path 일치에만 의존하는 borrow-side 힌트일 뿐이며, 실제 ownership transfer(`takes ownership`) 판정, 비인접 경로 추적, alias-aware matching, return-value ownership, interprocedural propagation은 아직 미구현
- [~] Allocator/free pairing inference — Match allocation APIs with corresponding frees to label lifetimes and types.
  > auto_comment.rs에서 malloc/free/HeapAlloc/VirtualAlloc 등 호출 감지 및 주석 추가
- [~] Heap metadata avoidance — Recognize allocator bookkeeping patterns to avoid mis-typing metadata as user fields.
  > auto_comment.rs에서 allocator 호출이 있는 동일 AST body 안의 fd/bk/prev_size/header/footer류 필드나 *(ptr + 0x10)류 접근에 보수적 "heap metadata access" 주석 추가. 실제 힙 메타데이터 분리/타입 억제는 미구현
- [~] Memcpy/memset loop lifting — Replace byte/word copy/set loops with memcpy/memset equivalents.
  > loop_analyzation.rs: annotate_loop_semantics()로 memcpy/memset 루프 패턴 탐지 + replace_loop_with_call()로 memset 루프를 AstCall("memset") 치환 구현. memcpy 치환은 미구현
- [~] Memcmp/strcmp loop lifting — Replace compare loops with memcmp/strcmp when semantics match.
  > loop_analyzation.rs: annotate_loop_semantics()로 memcmp/strcmp 루프 패턴 탐지 및 주석 부착 구현. 실제 함수 호출 치환은 미구현.
- [~] Strlen/scan loop lifting — Detect null-terminated scans and emit strlen/strchr-style calls.
  > loop_analyzation.rs: annotate_loop_semantics()로 strlen/scan 루프 패턴 탐지 및 주석 부착 구현. 실제 함수 호출 치환은 미구현.
- [~] Bounds-check synthesis — Emit explicit bounds checks from compare+branch patterns around loads/stores.
  > `auto_comment.rs`가 보수적으로 `if (idx < bound) { ... arr[idx] ... }` / `if (bound > idx) { ... arr[idx] ... }` 형태를 감지해 `bounds-checked indexed access` 주석을 붙인다. 실제 bounds-check AST 재구성, lower-bound 결합(`idx >= 0 && idx < len`), deref/store 전반 합성, 타입 제약 풀이 기반 범위 증명은 아직 미구현
- [x] Null-check canonicalization — Normalize pointer guards into if (p == NULL) / if (!p) forms.
  > x != 0 → x, x == 0 → !x 조건 정규화 구현 완료 (operator_canonicalization.rs)
- [x] Sign/zero-extend cast recovery — Turn extension sequences into explicit (int8_t), (uint32_t) casts.
  > IR→AST 변환 시 할당 크기(IrAccessSize)로 CastSigned/CastUnsigned를 Cast(Int32)/(UInt8) 등 명시적 타입 캐스트로 변환
- [~] Bitfield pack/unpack reconstruction — Convert repeated mask/shift sequences into named fields or helper macros.
  > auto_comment.rs: annotate_bitfield_patterns()로 동일 변수에 대한 mask/shift 추출 패턴 3회 이상 감지 시 bitfield 접근 주석 생성. 구조체 필드 복원은 타입 제약 풀이 필요
- [~] Byte-order field recovery — Recognize htons/ntohl-like patterns and annotate endianness conversions.
  > bit_trick_recognition.rs에서 bswap16/bswap32 시프트+OR 패턴 감지 및 주석 추가
- [~] Floating compare special-case handling — Preserve NaN-sensitive compare semantics when mapping to C operators.
  > auto_comment.rs에서 float/double 비교 if에 NaN-sensitive floating comparison 주석 추가. unordered/NaN 의미 보존용 실제 AST 재구성은 아직 없음
- [~] Denormal/FP-exception awareness — Avoid emitting “simplified” FP expressions that change exception behavior.
  > auto_comment.rs에서 float-like +,-,*,/ 식이 선언/대입/반환에 쓰일 때 denormal / FP-exception behavior may matter 주석 추가. 실제 예외/denormal 보존형 AST 재구성은 아직 없음
- [~] SRet aggregate layout recovery — Infer returned struct layout from stores into sret pointer within callee.
  > auto_comment.rs에서 first parameter likely sret 판정 시 IR aggregate 후보의 동일 base register write offsets를 붙여 layout hint 주석 추가. 실제 반환 struct 타입/필드명 재구성은 아직 없음
- [~] Hidden byref parameter detection — Detect ABI-specific hidden pointers for large args/returns (AArch64/Win64).
  > auto_comment.rs에서 비-first ABI 레지스터 파라미터(rcx/rdx/r8/r9, x0-x7)가 IR aggregate 후보와 매칭되고 read-dominant offsets를 보이면 hidden by-reference aggregate argument 주석 추가. first parameter의 this/sret 충돌 해소, 실제 ABI 파라미터 재작성, 완전한 AArch64/Win64 규약 복원은 아직 없음
- [~] Closure/environment recovery — Detect captured-variable environment pointers in callback patterns.
  > auto_comment.rs에서 CreateThread/_beginthread/pthread_create/EnumWindows 계열 호출의 (callback, context) 인자 슬롯을 보수적으로 감지해 callback environment 주석 추가. 일반 함수 포인터 provenance, 캡처 변수 실체 복원, 클로저 frame 구조 재구성은 아직 없음
- [~] Callback signature inference — Infer function-pointer types from how callbacks are invoked across call sites.
  > auto_comment.rs에서 CreateThread/CreateRemoteThread/_beginthread/_beginthreadex/pthread_create/EnumWindows 계열의 고정 callback 슬롯을 감지해 LPTHREAD_START_ROUTINE, pthread start routine, EnumWindowsProc 형태의 callback signature 힌트를 주석으로 추가. 알려진 API 기반 힌트만 제공하며, 일반 함수 포인터 provenance 추적, 콜사이트 기반 인터프로시저럴 시그니처 추론, 실제 타입 재작성은 아직 없음
- [~] Function-pointer provenance tracking — Track where a function pointer originates (vtable, table, arg, global).
  > auto_comment.rs에서 알려진 callback 등록 API(CreateThread/CreateRemoteThread/_beginthread/_beginthreadex/pthread_create/EnumWindows 계열)에 한해 callback 식이 parameter/global variable, member field, vtable-like field(vtable/vfptr/vptr), table slot(ArrayAccess)에서 오는 경우 provenance 주석을 추가. 일반 간접 호출 전체에 대한 provenance 추적, points-to/alias 기반 흐름 추적, interprocedural 함수 포인터 전파는 아직 없음
- [~] Jump-target set inference — Infer potential targets for computed jumps from value sets and table contents.
  > ir_analyzation/convert.rs에서 간접 call/jump 대상 식이 단일 상수 주소로 접히고 그 주소가 이미 알려진 AstFunctionId와 정확히 일치하면 AstCall::Function / AstJumpTarget::Function으로 승격. 다중 후보 target set, value_set.rs interval 해석, jump table/table contents 열거, switch/jumptable 복원은 아직 없음
- [~] Pointer-tagging detection — Detect low-bit tags on pointers and recover untagging operations in C.
  > `auto_comment.rs`에서 `*(p & ~0x3)`, `(p & 1) != 0`, `if (p & 3)` 같은 AST 패턴을 보수적으로 감지해 low-bit tagged pointer clear/test 주석을 추가한다. 그러나 이는 comment-only 힌트일 뿐이며, 실제 untagged pointer 식 재작성, tag propagation, alias/points-to 기반 추적, interprocedural provenance, typed pointer recovery는 아직 미구현
- [~] Tagged-union inference — Infer discriminated unions from tag checks followed by field access variants.
  > `auto_comment.rs`에서 멤버 필드 tag에 대한 `if`/`switch` 분기와 같은 root object의 branch-specific field access를 보수적으로 감지해 tagged-union variant dispatch comment를 추가한다. 그러나 이는 comment-only 힌트일 뿐이며, 실제 union/enum type 재구성, 타입 제약 풀이, data-flow 기반 증명, AST/type rewrite는 아직 미구현
- [~] Sentinel-value inference — Detect special constants (e.g., -1, NULL) used as sentinels and annotate types.
  > x == -1, x == 0xFFFFFFFF 비교 감지하여 sentinel comment 주석 (bit_trick_recognition.rs). 타입 변경은 미구현
- [x] Magic-number cataloging — Classify repeated constants (flags, sizes, limits) to drive naming and enums.
  > PE/ELF 시그니처, 디버그 패턴, 한계값, 해시/CRC 상수 등 매직넘버 카탈로그 comment 주석 (bit_trick_recognition.rs)
- [ ] Relocation-scan for vtables — Use relocation patterns to find vtables even without RTTI.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] RTTI-less class inference — Infer class layouts from consistent this+offset usage and vptr writes.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Stdlib type layout recognition — Detect common std::string/std::vector/std::map layouts by idioms.
  > 외부 DB/카탈로그 필요 — 현재 인프라 없음
- [ ] COM interface recovery — Identify COM vtable calls and GUID references to recover interface-like signatures.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] WinRT metadata correlation — Correlate native call patterns with WinRT/metadata artifacts when available.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] Qt metaobject parsing — Use Qt meta-data blobs to recover signal/slot names and class relationships.
  > 외부 DB/카탈로그 필요 — 현재 인프라 없음
- [ ] JNI signature recovery — Use JNINativeMethod tables to recover Java method names and signatures.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [~] Objective‑C selector resolution — Resolve objc_msgSend patterns into selectors and class/method names.
  > auto_comment.rs에서 objc_msgSend/objc_alloc/objc_retain 등 호출 감지 및 주석 추가 (셀렉터 이름 복원은 미구현)
- [ ] Swift thunk normalization — Collapse Swift thunks and runtime shims to expose user-level functions.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] Go wrapper/stub normalization — Collapse Go ABI wrappers and recover user function signatures.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] Rust monomorphization grouping — Group generic instantiations by similar bodies to reduce duplication and noise.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] GC stack map usage — Parse stack maps (when present) to infer pointer-typed locals/args reliably.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Exception personality correlation — Use personality functions and tables to improve EH region recovery.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] PLT resolver edge pruning — Remove lazy-binding resolver edges from CFG to improve structuring and naming.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Linker relaxation detection — Handle branch/call relaxation artifacts (e.g., RISC‑V/ARM) when identifying calls.
  > ISA/아키텍처별 처리 필요
- [ ] Relocatable object (.o) decompilation — Use relocations and symbols to decompile object files before linking.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Packer stub classification — Identify common unpacking stubs and isolate them from “real” program logic.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [~] Opaque arithmetic simplification — Simplify algebraic “noise” that preserves values (often from obfuscators).
  > bit_trick_recognition.rs에서 x^0, x+0, x*1, x&0xFFFFFFFF 등 항등 연산 제거
- [~] Bogus control-edge pruning — Detect always-false/true branches introduced to confuse CFG recovery.
  > if(0)/if(nonzero_int) 상수 조건 분기 제거 구현 (control_flow_cleanup.rs). 값 분석 기반 opaque predicate 제거는 미구현
- [ ] Dispatcher table discovery — Identify flattening dispatch tables and reconstruct the original region flow.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [ ] Virtualization loop spotting — Identify interpreter loops (fetch/decode/dispatch) and mark VM boundaries.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [~] Anti-tamper integrity check detection — Detect self-hash/checksum loops and label them as integrity logic.
  > auto_comment.rs에서 루프 내 acc=acc OP mem_read 누적 패턴 감지 및 주석 추가
- [~] Anti-debug API clustering — Cluster checks around timing/debugger/syscalls and annotate as anti-analysis.
  > auto_comment.rs에서 안티디버그 API 호출 감지 및 주석 추가 (Anti-debug/anti-VM spotting과 공유)
- [~] Decryption stub recognition — Recognize common XOR/RC4/AES key schedule + loop shapes for string/data decryptors.
  > auto_comment.rs에서 루프 내 *ptr ^= key 패턴 감지 및 XOR 복호화 루프 주석 추가
- [ ] Key-material lifetime tracing — Track derived keys through registers/memory to avoid mis-typing them as ints.
  > 포인터/앨리어싱 분석 프레임워크 필요
- [ ] Self-checking code marking — Mark regions that compare against embedded constants/code bytes to avoid over-simplifying.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [ ] CFG refinement from dynamic edges — Incorporate runtime-observed indirect targets back into static CFG/callgraph.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Path constraint extraction — Extract branch constraints from traces to simplify dead arms and recover guards.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Dynamic type hinting — Use runtime observed value patterns (pointer-like, small int, enum-like) to seed typing.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Heap layout sampling — Sample object field distributions at runtime to validate inferred struct layouts.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] API argument capture — Hook key APIs (alloc, file, net) to recover semantic parameter roles and naming hints.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Rare-path exploration — Drive execution to cover error-handling paths that static analysis underestimates.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Trace-based loop bound recovery — Use observed trip counts and invariants to suggest loop bounds and types.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Translation validation sampling — Compare decompiled IR vs emulation on sampled inputs to catch lifting mistakes.
  > 검증/테스팅 프레임워크 필요
- [ ] A-normal form conversion — Normalize complex expressions into let-bound temporaries to stabilize analyses.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Expression DAG recovery — Merge SSA defs into expression DAGs while preserving side-effect ordering.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Side-effect sequencing enforcement — Emit sequence points/temporaries to preserve exact evaluation order in C.
  > C 의미론 모델링 프레임워크 필요
- [ ] Volatile barrier preservation — Ensure volatile loads/stores aren’t reordered or removed in emitted code.
  > C 의미론 모델링 프레임워크 필요
- [ ] Undefined-behavior avoidance rewriting — Emit safe constructs to avoid relying on C UB (shift widths, overflow).
  > C 의미론 모델링 프레임워크 필요
- [ ] Integer wrap semantics modeling — Prefer explicit unsigned arithmetic/casts when machine semantics wrap.
  > C 의미론 모델링 프레임워크 필요
- [ ] Pointer–integer roundtrip handling — Preserve casts carefully to avoid breaking provenance assumptions in C.
  > C 의미론 모델링 프레임워크 필요
- [ ] Strict aliasing compliance mode — Emit memcpy-based punning or unions to avoid aliasing-UB.
  > C 의미론 모델링 프레임워크 필요
- [~] Deterministic IR canonicalization — Canonicalize commutative operations and block ordering for stable output diffs.
- [ ] Probabilistic recovery ensembles — Run multiple heuristics/models and choose the highest-confidence reconstruction.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Counterexample-guided refinement — When validation fails, refine assumptions (types/targets) to remove the mismatch.
  > SMT/형식 검증 프레임워크 필요
- [ ] Regression harness diffing — Track decompiler output changes per pass to detect instability and false improvements.
  > 검증/테스팅 프레임워크 필요
- [ ] Per-pass “blame” accounting — Attribute output constructs to analyses to explain and debug bad recoveries.
  > 검증/테스팅 프레임워크 필요
- [ ] Staged decompilation tiers — Start with conservative output, then progressively apply riskier rewrites when safe.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
- [ ] Speculative disassembly with rollback — Try multiple decode hypotheses in ambiguous regions and keep the globally most consistent result.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Probabilistic code discovery — Score byte ranges as code/data using model-based likelihood (xrefs, entropy, decode success).
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Constraint-based CFG repair — Solve for missing edges/blocks by enforcing consistency constraints (stack balance, reachability).
  > SMT/형식 검증 프레임워크 필요
- [ ] Stack-balance verification — Validate paths for consistent SP deltas to detect bad decoding or missing edges.
  > IR 리프팅/디코딩 레이어 확장 필요
- [x] Non-returning function inference — Detect noreturn callees (abort, exit, fatal) to prune impossible fallthrough edges.
- [~] Unreachable block pruning — Remove blocks proven unreachable after CFG + noreturn + constant-condition analysis.
  > control_flow_cleanup.rs에서 if(true/false), if(!0/!1), 정수 상수 조건 분기를 AST 단계에서 제거
- [ ] Cross-reference graph saturation — Iteratively add xrefs from decoded instructions/data to converge on full coverage.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Inter-segment pointer chasing — Follow pointers across segments (code↔rodata↔data) to find hidden tables and thunks.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Function chunk recovery via xref density — Identify function bodies by clustering dense intra-region xrefs and fallthroughs.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Fallthrough plausibility scoring — Prefer fallthrough edges only when instruction semantics and alignment support it.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] BTF (BPF Type Format) ingestion — Use Linux kernel/user BTF to recover C types/struct layouts when present.
  > 디버그 정보 파서 필요 — 현재 인프라 없음
- [ ] STABS debug parsing — Exploit older STABS symbols for types and source-level names.
  > 디버그 정보 파서 필요 — 현재 인프라 없음
- [ ] COFF auxiliary symbols use — Use COFF symbol aux records to refine sizes and section ownership.
  > 바이너리 포맷 파서 확장 필요
- [ ] ELF symbol versioning awareness — Use symbol versions to pick correct prototypes/names for glibc-style imports.
  > 바이너리 포맷 파서 확장 필요
- [ ] Windows API-set resolution — Map API-set forwarders to real DLL exports to improve import naming and prototypes.
  > 외부 DB/카탈로그 필요 — 현재 인프라 없음
- [ ] Mach-O dyld info parsing — Use dyld rebase/bind info to recover pointers and imported symbols.
  > 바이너리 포맷 파서 확장 필요
- [ ] ObjC method list decoding — Parse method lists to map implementation pointers to selector names.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] Swift type descriptor decoding — Use type descriptors to annotate native code with Swift-ish type names (even if emitting C).
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [ ] Go itab/interface table parsing — Recover interface method tables and dynamic dispatch patterns from Go binaries.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [~] Sanitizer instrumentation de-noising — Detect ASan/UBSan/TSan scaffolding and collapse it to comments or omitted checks.
  > auto_comment.rs에서 __asan_*/__ubsan_*/__tsan_*/__msan_*/__sanitizer_* 호출 감지 및 주석 추가
- [~] Coverage/profiling instrumentation removal — Recognize gcov/llvm-prof counters and emit them as no-ops or annotations.
  > auto_comment.rs에서 __gcov_*/__llvm_profile_* 호출 감지 및 주석 추가
- [~] Fuzzer hook suppression — Identify AFL/libFuzzer coverage edges and strip them from recovered logic.
  > auto_comment.rs에서 __afl_*/__sancov_* 호출 감지 및 주석 추가
- [~] Retpoline mitigation recognition — Collapse retpoline call/return sequences into normal indirect calls.
  > auto_comment.rs에서 __x86_indirect_thunk_* 호출 감지 및 주석 추가
- [~] Spectre fence de-noising — Recognize lfence/barrier patterns added for speculation mitigations and annotate/remove when safe.
  > auto_comment.rs에서 lfence/mfence/sfence 어셈블리 문 감지 및 주석 추가
- [~] Shadow-call-stack modeling — Handle shadow stack reads/writes as calling-convention scaffolding, not user variables.
  > IR/ASM 레벨 인식 필요 — 현재 호출 수준 주석만 지원
- [~] SafeStack/split-stack recognition — Detect stack splitting/safestack prologues and suppress them in high-level output.
  > auto_comment.rs에서 __safestack_*/__splitstack_*/__morestack 호출 감지 및 주석 추가
- [~] Stack-clash protection modeling — Recognize guard-page probing loops distinct from generic_chkstk patterns.
  > auto_comment.rs에서 __chkstk/__alloca_probe 호출 감지 및 주석 추가 (Alloca/VLA recovery와 공유)
- [ ] Formal ISA semantics lifting — Generate lifters from authoritative ISA semantics (Sail/K/Isla-style) for correctness.
  > ISA/아키텍처별 처리 필요
- [ ] Translation validation with SMT — Prove lifted IR matches instruction semantics on bounded regions using solver checks.
  > SMT/형식 검증 프레임워크 필요
- [ ] Bit-vector canonicalization — Normalize bit-level expressions (concat/extract) before simplification/type recovery.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] BDD-based predicate simplification — Use BDDs to simplify boolean logic and merge equivalent conditions.
  > SMT/형식 검증 프레임워크 필요
- [ ] E-matching rewrite libraries — Apply curated rewrite rules (algebra/bit tricks) with e-graph saturation limits.
  > SMT/형식 검증 프레임워크 필요
- [ ] Counterexample-guided deobfuscation — Simplify then validate; if mismatch found, refine rewrite constraints automatically.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [ ] Path feasibility checking — Use SMT to prove some branches infeasible (stronger than interval/range analysis).
  > SMT/형식 검증 프레임워크 필요
- [ ] Precise NaN-sensitive FP modeling — Preserve ordered/unordered comparisons by mapping to explicit helper predicates.
  > 부동소수점/SIMD 모델링 필요 — 현재 인프라 없음
- [~] Boolean type inference — Infer bool locals/params from compare/setcc patterns, masks, and branch usage.
  > 비교/setcc IR 핸들러 구현 및 datatype 추론에서 Bool 유추 추가; 전파 프레임워크는 미구현
- [ ] Bitwidth inference — Recover uint8_t/uint16_t/... from truncations, masks, and load/store widths.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Vector-to-scalar demotion — Detect “vector used as scalar” artifacts and rewrite to scalar types/ops for readability.
  > 부동소수점/SIMD 모델링 필요 — 현재 인프라 없음
- [ ] Pointer vs handle classification — Distinguish true pointers from opaque handles using API models and arithmetic patterns.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Capability pointer modeling — Handle CHERI-like or tagged-pointer architectures by preserving provenance constraints.
  > ISA/아키텍처별 처리 필요
- [ ] Pointer-alignment inference — Infer alignment from masking/mod patterns and emit aligned types or asserts.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Offset-to-offsetof recognition — Detect base + const patterns as &obj->field with explicit field naming.
  > 타입 제약 풀이 프레임워크 필요
- [ ] container_of recovery — Recognize ptr - offset patterns that compute a containing struct pointer.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Custom allocator identification — Detect malloc-like allocators (size classes, free lists) to label allocation sites/types.
  > 메모리/힙 분석 프레임워크 필요
- [ ] Arena/region allocator modeling — Recognize bump-pointer arenas and model lifetimes/scopes accordingly.
  > 메모리/힙 분석 프레임워크 필요
- [ ] Memory pool object-typing — Infer object types from pool chunk sizes and consistent field accesses post-allocation.
  > 메모리/힙 분석 프레임워크 필요
- [ ] Copy-on-write pattern detection — Detect refcount+clone-on-write idioms and annotate ownership semantics.
  > 포인터/앨리어싱 분석 프레임워크 필요
- [ ] Small-string optimization detection — Recognize SSO layouts in string implementations to improve struct recovery.
  > 자료구조 복구 프레임워크 필요 — AST 패턴만으로 불충분
- [ ] Tagged-pointer scheme recovery — Identify tag extraction/insertion conventions and expose the underlying union-like type.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Fat-pointer modeling — Recover (ptr,len) fat pointers (Rust slices, Go slices) into struct-like C representations.
  > 언어별 런타임 분석 필요 — 현재 인프라 없음
- [~] Iterator pattern recognition — Detect “begin/end + step” idioms and rewrite loops into cleaner iteration forms.
  > loop_analyzation.rs: annotate_iterator_traversals()로 while(p){...p=p->next} 연결 리스트/반복자 순회 패턴 탐지 및 주석 부착 구현. 루프 형태 변환은 미구현.
- [ ] State machine extraction (high-level) — Lift dispatch loops and state variables into explicit enums + switch-based machines.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
- [ ] Protocol/parser feature inference — Infer token classes/states from branch patterns and table-driven transitions.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
- [~] Error-code convention inference — Detect 0/-1/errno-style conventions and label return types/paths accordingly.
  > control_flow_cleanup.rs: annotate_error_code_returns()로 return 0 → "success", return non-zero → "error", return -1/0xFFFFFFFF → "sentinel -1" 주석 구현. errno 전파 추적은 미구현
- [x] Assertion pattern recovery — Recognize if(!cond) abort() or trap patterns and emit assert(cond)-like constructs.
- [~] Logging/telemetry scaffolding de-noising — Collapse repeated logging macros/wrappers into concise calls with inferred formats.
  > auto_comment.rs에서 syslog/NSLog/OutputDebugString/ETW 등 호출 감지 및 주석 추가
- [~] Resource cleanup normalization — Detect multi-resource release patterns and synthesize a single cleanup block.
  > auto_comment.rs: annotate_resource_cleanup()로 연속 2+ free/close/release/destroy 호출 감지하여 cleanup 블록 주석 생성. 블록 합성은 미구현
- [ ] Interrupt/vector table recognition — For firmware, parse vector tables to discover handlers and true entrypoints.
  > 펌웨어/임베디드 전용 분석 필요
- [ ] Memory-mapped I/O modeling — Detect volatile MMIO address ranges and preserve volatile semantics + typed registers.
  > 펌웨어/임베디드 전용 분석 필요
- [ ] Bare-metal startup suppression — Recognize reset/init boilerplate (zero BSS, copy data) and condense it.
  > 펌웨어/임베디드 전용 분석 필요
- [ ] Relocationless pointer heuristics — In fixed-address firmware, infer pointers via address-range + alignment checks.
  > 펌웨어/임베디드 전용 분석 필요
- [ ] Thumb interworking correction — Fix ARM/Thumb function pointer LSB conventions when recovering call targets.
  > ISA/아키텍처별 처리 필요
- [ ] RISC‑V compressed instruction handling — Decode mixed 16/32-bit streams and normalize into canonical IR.
  > ISA/아키텍처별 처리 필요
- [ ] x86 AVX-512 mask semantics modeling — Preserve k-mask behavior to avoid incorrect boolean reconstruction.
  > 부동소수점/SIMD 모델링 필요 — 현재 인프라 없음
- [ ] Hardware trace integration (Intel PT/ETM) — Use branch traces to recover indirect targets and rare-path CFG edges.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Last-branch record correlation — Use LBR stacks to prioritize likely edges and block ordering in output.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Syscall trace correlation — Use runtime syscall traces to label wrapper functions and infer argument roles.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Dynamic invariant mining — Learn invariants from traces (e.g., bounds, nullness) to simplify guards safely.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Crash-dump guided recovery — Use crash register/memory snapshots to infer types/values at specific points.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] ML-based variable naming — Predict meaningful local/param names from IR context, APIs, strings, and roles.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] ML-based type prediction — Predict likely C types from instruction neighborhoods, def-use features, and callsites.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Neural function similarity — Embed functions to identify libraries, match across builds, and transfer names/types.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Learned idiom recognition — Detect compiler idioms beyond hand-written patterns (div/mod tricks, bit hacks, etc.).
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Learned struct-field clustering — Cluster offsets into fields using statistical regularities across functions/binaries.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Uncertainty-aware ensembles — Combine multiple models/heuristics and keep alternatives when confidence is low.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [x] AST-level refactoring passes — After structuring, run C-centric rewrites (merge declarations, simplify loops, hoist temps).
- [~] Loop rotation normalization — Convert “rotated” loops into canonical while/for forms for readability.
  > if(cond){while(cond){body}} → while(cond){body} 부분 구현; 순수(side-effect-free) 조건만 지원. 추가로 while(true) { ... if(!cond) break; } → do { ... } while(cond) 안전 변환을 구현했지만, 비순수 조건과 body-duplication 기반의 완전한 do-while 회전은 아직 미구현 (loop_analyzation.rs)
- [x] If/else inversion heuristics — Prefer positive conditions and reduce negations based on readability cost models.
  > if(!cond){A}else{B} → if(cond){B}else{A} 변환 구현 완료 (operator_canonicalization.rs)
- [x] Switch fallthrough annotation synthesis — Emit explicit fallthrough comments/markers when semantics require it.
  > 비터미널 switch case에 "fallthrough" 주석 자동 부착 구현 완료 (control_flow_cleanup.rs)
- [x] Macro-like pattern lifting — Recognize MIN/MAX/CLAMP, ARRAY_SIZE, ROUND_UP, etc., and emit as helpers/macros.
- [~] Canonical error-handling templates — Rewrite common goto fail shapes into consistent, compact patterns.
  > control_flow_cleanup.rs: annotate_goto_cleanup_patterns()로 goto-fail 에러 처리 패턴 탐지 및 주석 부착 구현. 구조 변환은 미구현.
- [ ] Scope recovery via dominance frontiers — Use dominance + liveness to introduce minimal scopes and reduce variable lifetime.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Relational memory modeling — Track correlations between multiple variables (e.g., i < n implies bounds) beyond intervals.
  > SMT/형식 검증 프레임워크 필요
- [ ] Stride-aware range analysis — Use modular arithmetic to refine ranges for index variables and switch discriminants.
  > 루프 구조/시맨틱 분석 프레임워크 구현 필요
- [ ] Field-sensitive MemorySSA — Combine MemorySSA with per-field aliasing for cleaner load/store reconstruction.
  > 포인터/앨리어싱 분석 프레임워크 필요
- [ ] Call-effect stubs for unknowns — Model unknown calls with conservative summaries to avoid over-simplification.
  > 인터프로시저럴 분석 프레임워크 필요
- [~] Selective inlining for readability — Inline tiny wrappers only when it reduces noise (configurable via cost model).
- [ ] Outlining repeated AST fragments — Detect repeated code regions and outline into helper functions to mimic source factoring.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
- [~] Unwind-table function discovery (Win64 .pdata/.xdata) — Use unwind metadata to locate function ranges and prolog saves.
  > `cfi_parser.rs`가 Win64 RUNTIME_FUNCTION 엔트리에서 함수 시작/끝 범위를 복구하고 누락된 심볼만 `func_<rva>` 형태로 PreDefinedOffsets에 시드한다. unwind 기반 prolog/저장 레지스터 요약은 가능하지만 EH region reconstruction과 비-Win64 포맷 지원은 미구현
- [ ] SEH scope reconstruction (Win32) — Parse structured exception handler frames to recover try/except-like regions.
  > 바이너리 포맷 파서 확장 필요
- [ ] ARM .ARM.exidx/.ARM.extab driven recovery — Use ARM unwind tables to infer function boundaries and stack layout.
  > ISA/아키텍처별 처리 필요
- [ ] Mach-O LC_FUNCTION_STARTS parsing — Seed function entrypoints from function-starts compressed lists.
  > 바이너리 포맷 파서 확장 필요
- [ ] Mach-O __unwind_info exploitation — Use compact unwind to infer prolog/epilog and call frame behavior.
  > 바이너리 포맷 파서 확장 필요
- [ ] ELF .eh_frame_hdr acceleration — Leverage FDE indices to quickly map PCs to unwind entries.
  > 바이너리 포맷 파서 확장 필요
- [ ] ELF .gnu_debugdata fallback — Extract mini debug info (when present) for symbols/types without full DWARF.
  > 디버그 정보 파서 필요 — 현재 인프라 없음
- [~] Export-forwarder resolution (PE) — Resolve forwarded exports to real targets to improve import prototypes/names.
  > analysis.rs: resolve_forwarded_exports()로 goblin PE 포워딩 엑스포트 탐지 및 DLL!Symbol 형태 해석 구현. 프로토타입 전파는 미구현
- [ ] API hashing database (malware-style) — Identify resolved APIs from hash constants + resolver loops (for labeling).
  > 외부 DB/카탈로그 필요 — 현재 인프라 없음
- [ ] Syscall stub taxonomy (Windows/Linux) — Detect syscall wrappers and recover syscall numbers + argument counts.
  > 외부 DB/카탈로그 필요 — 현재 인프라 없음
- [~] Loader stub de-noising — Collapse CRT/loader shims (TLS callbacks, init thunks) into high-level “startup”.
- [ ] Opcode n‑gram code classification — Classify code vs data by statistical opcode sequence likelihood.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Instruction entropy profiling — Use local entropy and decode stability to identify packed/data regions.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Basic-block “shape” scoring — Prefer block splits that maximize valid terminators and minimize weird fallthrough.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Branch-target alignment heuristics — Use typical alignment patterns to rank plausible jump/call targets.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Call-site sanity constraints — Reject targets that violate ABI expectations (stack alignment, arg setup patterns).
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Return-address stack simulation — Track plausible return targets to validate call/ret structure in CFG.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] CFG edge plausibility via stack delta — Down-rank edges that create impossible SP/FP deltas.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Indirect-call signature matching — Infer function-pointer types from how arguments are prepared before call [reg].
  > 인터프로시저럴 분석 프레임워크 필요
- [ ] Vtable slot indexing inference — Recover virtual method indices from this->vptr[idx] patterns for naming.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Function-pointer table role inference — Distinguish jump tables vs callback arrays via usage context and calling form.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
- [x] Binary-search switch detection — Recognize compare-and-branch trees implementing switch via ordered thresholds.
  > 중첩 if-else 트리에서 </<=/>/>=로 범위 분할 + == 리프 패턴 감지하여 switch 변환 (switch_reconstruction.rs). 한편 structuring.rs/structured_region_lowering.rs에는 explicit `StructuredRegion::Switch` scaffold가 추가되었지만, CFG structuring 단계가 아직 해당 region을 생성하지 않아서 end-to-end structured switch lowering은 미완성.
- [ ] Range-check + bias switch recovery — Detect cmp/sub; ja; jmp [table+idx] with biased indices.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Computed-goto pattern lifting — Recognize state dispatch using labels-as-values idioms and emit switch/dispatch.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [~] Loop “continue” edge normalization — Rewire back-edges to canonical continue targets to improve for/while output.
  > loop_analyzation.rs: annotate_continue_like_gotos()로 루프 본문 첫 레이블로의 goto를 continue-like 백엣지로 탐지하고, convert_loop_gotos_to_break_continue()가 그중 안전한 경우를 `Continue`로 치환. 더 복잡한 비정형 back-edge와 중첩 루프 관통 케이스는 여전히 부분 지원.
- [ ] Irreducible loop splitting with heuristics — Split nodes to create reducible regions when it reduces gotos.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Structured exception edge integration — Merge EH edges into region structuring instead of leaving as raw gotos.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Speculative region forming with cost model — Try multiple structuring trees and keep the lowest “gotos + complexity” cost.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [~] Predicate hoisting/reassociation — Combine repeated guards across blocks into a single dominating if.
  > 연속 if(같은조건) 병합 구현 (control_flow_cleanup.rs). 도미네이터 기반 비연속 블록 병합은 미구현
- [~] Edge inversion for readability — Flip branches to keep fallthrough as “likely” path for cleaner output.
  > if(!cond) { A } else { B } → if(cond) { B } else { A } 변환 + 분기 본문 크기 비교 휴리스틱 구현 (control_flow_cleanup.rs). hello_world 검증 출력이 비결정적이라 [~] 유지
- [ ] Congruence analysis (mod arithmetic) — Infer facts like x ≡ k (mod m) to tighten switch/index reasoning.
  > SMT/형식 검증 프레임워크 필요
- [ ] Wrap-around aware range analysis — Model unsigned overflow explicitly to avoid incorrect simplifications.
  > C 의미론 모델링 프레임워크 필요
- [ ] Bit-level abstract interpretation — Track known/unknown bits (bitmasks) to simplify masking/shift-heavy code.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Polyhedral loop analysis — Infer affine bounds/strides in nested loops to reconstruct multi-dimensional indexing.
  > 루프 구조/시맨틱 분석 프레임워크 구현 필요
- [ ] Relational value analysis — Track relationships (e.g., i < n ⇒ ptr+i in-bounds) beyond independent intervals.
  > SMT/형식 검증 프레임워크 필요
- [ ] Path-merging with predicate guards — Keep multiple value versions guarded by conditions rather than over-approximating.
  > SMT/형식 검증 프레임워크 필요
- [ ] Symbolic simplification under assumptions — Simplify expressions within a guarded region using region predicates.
  > SMT/형식 검증 프레임워크 필요
- [ ] Typestate inference — Infer object states (init/open/closed) from control flow + API usage sequences.
  > 인터프로시저럴 분석 프레임워크 필요
- [ ] Handle-kind inference — Classify integers as file/socket/thread handles based on call graph usage patterns.
  > 인터프로시저럴 분석 프레임워크 필요
- [~] Resource flow tracking — Track acquire/release pairs (malloc/free, open/close) interprocedurally for annotation.
  > fopen/fclose, socket/closesocket, CreateFile/CloseHandle, 레지스트리 API 등 리소스 I/O 호출 감지 및 주석 추가
- [~] Error-propagation modeling — Detect “return last error” conventions and simplify redundant error paths.
  > auto_comment.rs: annotate_error_propagation()로 `var = call(); if (var < 0) return var;` 에러 전파 패턴 감지 및 주석 생성. 경로 단순화는 인터프로시저럴 분석 필요
- [~] errno/GetLastError propagation inference — Recognize wrappers that set/read last-error and annotate semantics.
  > GetLastError/SetLastError/__errno_location 호출 감지 및 주석 추가
- [ ] Capability/permission flag inference — Cluster bitwise flag usage into named flag sets for enums/bitmasks.
  > 타입 제약 풀이 프레임워크 필요
- [~] Finite-domain enum recovery — Infer enums from repeated small constant sets used in compares and tables.
  > auto_comment에서 switch 3+ 케이스의 정수 상수 집합을 주석으로 출력. 타입 시스템에 enum 정의 반영은 타입 제약 풀이 필요
- [ ] Pointer tagging scheme mining — Learn tag masks/shifts globally and normalize tagged pointers to untagged views.
  > ML/통계 모델 필요 — 현재 인프라 없음
- [ ] Refinement-type constraints — Infer constraints like “non-null”, “range-limited”, “aligned” and annotate output.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Structure packing policy inference — Infer compiler packing/alignment policy from consistent offset patterns across objects.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Field endianness inference — Detect byte-swap around specific offsets and mark fields as network-order/LE/BE.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Array-of-struct vs struct-of-array detection — Decide layout based on stride patterns across multiple fields.
  > 자료구조 복구 프레임워크 필요 — AST 패턴만으로 불충분
- [ ] Flexible array member inference — Detect structs whose last field behaves like a variable-length trailing array.
  > 타입 제약 풀이 프레임워크 필요
- [ ] Small-buffer optimization detection (beyond strings) — Recognize in-struct inline storage with heap fallback pattern.
  > 자료구조 복구 프레임워크 필요 — AST 패턴만으로 불충분
- [ ] Copy elision recognition — Collapse redundant temporary buffers introduced by optimization into direct uses.
  > IR 리프팅/디코딩 레이어 확장 필요
- [ ] Custom string class recognition — Identify length/capacity/pointer triples and normalize to string-like struct.
  > 자료구조 복구 프레임워크 필요 — AST 패턴만으로 불충분
- [ ] Decompilation-by-recompilation feedback — Compile emitted C, compare behavior/traces, iterate to fix mismatches.
  > 검증/테스팅 프레임워크 필요
- [ ] Round-trip CFG validation — Lift → structure → re-lower to CFG and verify equivalence (within model limits).
  > 검증/테스팅 프레임워크 필요
- [ ] Metamorphic lifter testing — Randomize instruction encodings with same semantics to stress decoding/lifting.
  > 검증/테스팅 프레임워크 필요
- [ ] Corpus-guided differential validation — Run multiple lifters/decompilers and flag semantic divergences for review.
  > 검증/테스팅 프레임워크 필요
- [ ] Invariant-based regression tests — Enforce invariants (stack balance, no undefined shifts) across large corpora.
  > 검증/테스팅 프레임워크 필요
- [ ] Crash-minimizing reducer integration — When analysis fails, auto-reduce the problematic function region for debugging.
  > 검증/테스팅 프레임워크 필요
- [ ] DBI-assisted target harvesting — Use dynamic binary instrumentation to log indirect call/jump targets precisely.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Selective trace sampling — Collect only edges for ambiguous regions to minimize runtime overhead.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Snapshot-at-OEP reanalysis — For packed binaries, snapshot memory at original entrypoint and re-run static passes.
  > 런타임/동적 분석 필요 — 정적 분석만 가능
- [ ] Input-guided deobfuscation — Use chosen inputs to drive through opaque predicates and confirm dead arms.
  > 난독화 해제 프레임워크 필요 — 현재 인프라 없음
- [~] Security-mitigation scaffold collapsing — Detect and condense mitigation glue (IBT landing pads, fence sequences) into annotations.
  > lfence/mfence/sfence, __chkstk, __safestack_*, retpoline 등 보안 완화 패턴 감지 및 주석 추가
- [~] Instrumentation signature catalogs — Recognize compiler-inserted probes/counters and fold into “instrumentation” nodes.
  > auto_comment에서 sanitizer/coverage 계측 호출 감지 및 주석 생성. IR 레벨 노드 폴딩은 미구현
- [~] Sanitizer shadow-memory modeling — Treat ASan/TSan shadow checks as side-effect-free guards to simplify output.
  > auto_comment에서 shadow address 계산 패턴 (addr >> 3 + large_offset) 감지하여 ASan 계측 주석 생성. TSan 패턴 감지 및 guard 제거는 미구현
- [ ] Safe UB-free emission mode — Emit helper functions/macros to preserve machine semantics without C undefined behavior.
  > C 의미론 모델링 프레임워크 필요
- [ ] Cross-function constant pool intern — Deduplicate literal pools across functions and give stable symbolic names.
  > IR 리프팅/디코딩 레이어 확장 필요
- [~] String role classification — Label strings as format/path/url/registry key/etc. by nearby API usage patterns.
  > 문자열 조작 함수(strcpy/strlen/memcpy 등) 호출 감지 및 주석 추가, 수학 라이브러리 호출 감지 추가
- [~] Domain vocabulary seeding — Use extracted strings to seed likely names/types (e.g., “cookie”, “token”, “hdr”).
  > auto_comment에서 함수 내 문자열 리터럴의 도메인 키워드(URL/crypto/auth/SQL/registry/filesystem) 감지하여 vocabulary 힌트 주석 생성. 변수 이름 반영은 미구현
- [~] Behavioral clustering for naming — Cluster functions by side effects/API sets (crypto, IO, parsing) to guide labels.
  > auto_comment에서 함수 내 API 호출 카테고리(crypto/IO/string/math/memory/network/thread/UI) 분류 후 dominant 카테고리 주석 생성. ML 기반 정밀 클러스터링은 미구현
- [ ] Graph grammar structuring — Apply grammar rules over CFG motifs to recover higher-level constructs reliably.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] SPQR decomposition for CFGs — Use graph decomposition to guide structured region extraction in complex graphs.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Region “repair” via duplication — Duplicate small blocks to eliminate irreducible joins when it reduces gotos.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [ ] Edge contract enforcement — Enforce “single-exit” contracts by introducing local flags to keep structured output.
  > CFG 구조화 알고리즘(phoenix/dream 등) 구현 필요 — 도미네이터 트리/포스트도미네이터/제어 의존성은 구현 완료 (dominator.rs)
- [~] Plugin-driven semantic intrinsics — Let users define IR intrinsics for domain ops (CRC, checksum, endian loads) for cleaner C.
- [~] Pass provenance tagging — Attach “origin” metadata to AST nodes (pattern, proof, trace) for explainability/debugging.
- [ ] Confidence-based fallback per construct — Emit structured C only above confidence threshold; otherwise keep labeled blocks/asm.
  > 고급 휴리스틱 프레임워크 필요 — 현재 인프라 부족
