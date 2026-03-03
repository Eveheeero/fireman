# Implementation Plan: Expanded-Scope Analyzation Strategies

## Discovery: Critical Missing Instruction Handlers

The single highest-impact improvement is implementing **missing x86-64 instruction handlers** in `instruction_analyze.rs`. Currently, blocks containing ANY unsupported instruction are **silently skipped** (return `None`), making entire functions undecompilable. The following instructions are **extremely common** in real binaries but return `None?`:

| Instruction | Frequency | What It Does |
|---|---|---|
| `movsx/movsxd` | Very High | Sign-extend (e.g., `char` → `int`) |
| `movzx` | Very High | Zero-extend (e.g., `uint8_t` → `uint32_t`) |
| `neg` | High | Arithmetic negate (`-x`) |
| `not` | High | Bitwise NOT (`~x`) |
| `sar` | High | Arithmetic right shift (`x >> n` signed) |
| `cmovcc` | High | Conditional move (branchless if/else) |
| `setcc` | High | Set byte from flags (boolean result) |
| `leave` | Medium | Stack frame cleanup (`mov rsp,rbp; pop rbp`) |
| `idiv` | Medium | Signed integer division |
| `sbb` | Medium | Subtract with borrow (multi-precision, idioms) |

---

## Phase 1: Missing Instruction Handlers (arch layer)

**File**: `fireball/src/arch/x86_64/instruction_analyze.rs` + submodules

All handlers follow the existing pattern: `#[box_to_static_reference] pub(super) fn name() -> &'static [IrStatement]` using the shortcuts (`assign`, `condition`, `calc_flags_automatically`, `u::`, `b::`, etc.)

### 1a. `movsx` / `movsxd` — Sign Extension

```
movsx dst, src  →  assign(u::sign_extend(o2()), o1(), o1_size())
```
- Exactly like `mov()` but uses `u::sign_extend` instead of `u::zero_extend`
- `movsxd` is the same (64-bit variant)
- Add to `s.rs` or `m.rs`

**TODO line 107**: Extension modeling → enables [x]

### 1b. `movzx` — Zero Extension

```
movzx dst, src  →  assign(u::zero_extend(o2()), o1(), o1_size())
```
- Identical to current `mov()` (which already does `u::zero_extend(o2())`)
- Should be a direct alias

### 1c. `neg` — Arithmetic Negation

```
neg dst  →  assign(u::neg(o1()), o1(), o1_size())
             + calc_flags_automatically(neg_result, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf])
```
- Pattern: same as `inc`/`dec` but with `u::neg`

### 1d. `not` — Bitwise NOT

```
not dst  →  assign(u::not(o1()), o1(), o1_size())
```
- No flags affected (unlike neg)
- Simplest possible handler

### 1e. `sar` — Shift Arithmetic Right

```
sar dst, count  →  (same pattern as shl/shr but using b::sar)
```
- Pattern: exactly like `shr()` in `s.rs` but with `b::sar` instead of `b::shr`
- `b::sar()` already exists in shortcuts

### 1f. `cmovcc` — Conditional Move (all variants)

```
cmovCC dst, src  →  condition(CC_flag_expr, [assign(o2(), o1(), o1_size())], [])
```
- Reuses the **same flag conditions** as `jcc` (e.g., `cmova` = `!CF && !ZF`, same as `ja`)
- Key difference: no jump, just conditional assignment
- Needs a `cmovcc(condition_data)` helper similar to `jcc(condition_data)` in `j.rs`
- Create new file `c.rs` additions or extend existing `c.rs`

**iceball mapping**: `X64Statement::Cmovcc` is a single variant — need to check how iceball distinguishes cmova/cmovb/cmovg etc. (likely via condition code in operands or sub-opcode)

> **Important**: Need to verify how iceball represents the condition code for CMOVcc. If it's a single `Cmovcc` variant, the condition must come from the instruction prefix or opcode encoding. Check iceball's `X64Statement::Cmovcc` handling.

**TODO line 407**: If-conversion reversal → enables [~] or [x]

### 1g. `setcc` — Set Byte from Condition

```
setCC dst  →  condition(CC_flag_expr,
    [assign(c(1), o1(), size_byte)],
    [assign(c(0), o1(), size_byte)]
)
```
- Same flag conditions as `jcc`/`cmovcc`
- Result is always a byte (0 or 1)
- Same iceball question as cmovcc re: condition code

**TODO line 57**: Flag/condition recovery → partial progress

### 1h. `leave` — Stack Frame Teardown

```
leave  →  assign(rbp, rsp, size_architecture())   // mov rsp, rbp
           assign(d(rsp), rbp, size_architecture())  // pop rbp (load [rsp] into rbp)
           assign(b::add(rsp, architecture_byte_size()), rsp, size_architecture())  // rsp += 8
```
- Equivalent to `mov rsp, rbp; pop rbp`
- Follow the `pop()` pattern in `p.rs` for the second part

### 1i. `idiv` — Signed Integer Division

```
idiv src  →  (similar to div() pattern)
    8-bit:  al = signed_div(ax, sign_extend(o1))
            ah = signed_rem(ax, sign_extend(o1))
    else:   rax = signed_div(rdx:rax_combined, sign_extend(o1))
            rdx = signed_rem(rdx:rax_combined, sign_extend(o1))
```
- Pattern: like `div()` in `d.rs` but with `b::signed_div`/`b::signed_rem`

### 1j. `sbb` — Subtract with Borrow

```
sbb dst, src  →  assign(b::sub(b::sub(o1(), o2()), u::zero_extend(cf)), o1(), o1_size())
                  + calc_flags_automatically(...)
```
- Pattern: `dst = dst - src - CF`
- Similar to `adc` (which does `dst + src + CF`)

---

## Phase 2: IR Datatype Inference Enhancement

**File**: `fireball/src/ir/analyze/datatype.rs`

The TODO at line 52-53 says: "인스트럭션을 통한 데이터 타입 추가 유추 필요"

Currently `analyze_datatype_raw()` marks Assignment operands as `DataType::Unknown`. Enhance:

1. **Walk `from` field of Assignment**: If it contains `IrDataOperation::Binary` with:
   - `Add/Sub/Mul/SignedDiv/UnsignedDiv/Shl/Shr/Sar/And/Or/Xor` → `DataType::Int` for both operands and result
   - `Equal/SignedLess/UnsignedLess` etc. → `DataType::Bool` for result

2. **Walk `from` field**: If it contains `IrDataOperation::Unary` with:
   - `SignExtend/ZeroExtend` → `DataType::Int`
   - `Not` → `DataType::Int`
   - `Negation` → `DataType::Int`

3. **Walk `to` field**: If target is a register known to be a flag register → `DataType::Bool`

---

## Phase 3: Update strategies.md

Mark items enabled by Phase 1-2:

| Line | Item | Status Change | Reason |
|------|------|---|---|
| 107 | Extension modeling | [ ] → [~] | movsx/movzx implemented in IR, AST-level cast recovery still needed |
| 57 | Flag/condition recovery | [ ] → [~] | setcc/cmovcc use flags; full flag modeling still needed |
| 407 (late) | If-conversion reversal | [ ] → [~] | cmovcc generates conditional assignment |

Update infeasibility reasons:
- Remove "AST optimize 레이어 범위 밖" from items now addressed
- Keep genuine infrastructure blockers (dominator tree, SSA, points-to analysis, etc.)

---

## Execution Order

1. **Phase 1a-1e first** (movsx, movzx, neg, not, sar) — These are straightforward, self-contained, and follow existing patterns exactly
2. **Phase 1h** (leave) — Simple stack operation
3. **Phase 1i-1j** (idiv, sbb) — Follow div/adc patterns
4. **Phase 1f-1g** (cmovcc, setcc) — Requires iceball condition code investigation first
5. **Phase 2** (datatype inference) — Independent, can be done in parallel
6. **Phase 3** (strategies.md) — After all code changes pass build + tests

---

## Build Verification

- `cargo b` — zero errors
- `cargo t` — all tests pass
- `cargo +nightly fmt` — formatting clean
- Test decompilation of `tests/resources/hello_world.exe` — verify new instructions are now decompiled

---

### Critical Files for Implementation

- `fireball/src/arch/x86_64/instruction_analyze.rs` — Main dispatch: change `None?` to actual handler calls
- `fireball/src/arch/x86_64/instruction_analyze/s.rs` — Add sar(), add movsx helpers
- `fireball/src/arch/x86_64/instruction_analyze/m.rs` — Add movsx(), movsxd(), movzx()
- `fireball/src/arch/x86_64/instruction_analyze/c.rs` — Add cmovcc() handler
- `fireball/src/ir/analyze/datatype.rs` — Enhanced type inference from operations
