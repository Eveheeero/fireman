# Decompiler-Enhanced C Output Specification

## Philosophy: Enhanced C with Modern Cherry-Picks

The output language is primarily C with carefully selected modern features that improve readability without adding
complexity. The goal is code that looks familiar to C programmers while leveraging modern constructs only where they
significantly enhance clarity.

## Core Language: C11 Base

### Why C11?

- Wide compiler support
- Familiar to reverse engineers
- Simple mental model
- Minimal runtime overhead
- Clear mapping from assembly

### Base Features Used

```c
// Standard C types with explicit sizes
uint8_t, uint16_t, uint32_t, uint64_t
int8_t, int16_t, int32_t, int64_t

// Compound literals for clarity
struct Point p = (struct Point){.x = 10, .y = 20};

// Designated initializers
int arr[] = {[0] = 1, [5] = 2, [9] = 3};

// Static assertions for invariants
_Static_assert(sizeof(Header) == 64, "Header size mismatch");
```

## Cherry-Picked Modern Features

### 1. Type Inference (auto) - Limited Use

```c
// Use auto ONLY for complex iterator types or obvious cases
auto* ptr = get_complex_structure();  // When type is clear from context

// NOT for simple types
int32_t count = 0;  // Never: auto count = 0;
```

### 2. Nullptr Instead of NULL

```c
// Modern null pointer constant
if (ptr != nullptr) {
    process(ptr);
}
```

### 3. Static_assert for Compile-Time Checks

```c
// Validate assumptions
static_assert(sizeof(void*) == 8, "64-bit pointers required");
static_assert(offsetof(struct Data, field) == 0x10, "Layout mismatch");
```

### 4. Attributes for Compiler Hints

```c
// Branch prediction hints (C++20 style in comments)
if (error_code != 0) [[unlikely]] {
    handle_error(error_code);
}

// Function attributes
[[noreturn]] void fatal_error(const char* msg);
[[nodiscard]] int32_t calculate_checksum(const uint8_t* data, size_t len);
```

### 5. Fixed-Width Integer Types Always

```c
// NEVER use int/long/short
// ALWAYS use explicit sizes
typedef struct {
    uint32_t magic;      // Not: unsigned int
    int64_t offset;      // Not: long long
    uint8_t flags;       // Not: unsigned char
} Header;
```

## Decompiler-Specific Annotations

### 1. Confidence Markers

```c
// High confidence - no annotation needed
int32_t count = header->num_entries;

// Medium confidence 
int32_t __probable("loop_counter") i = 0;

// Low confidence
void* __uncertain ptr = *(void**)0x401000;

// Confidence on types
struct __confidence(0.85) FileHeader {
    uint32_t magic;
    uint32_t version;
};
```

### 2. Address Annotations

```c
// Original addresses preserved
int32_t value = *(int32_t*)0x401000;  /* @0x401000 */

// Function addresses
void sub_401000(void);  /* Function at 0x401000 */
```

### 3. Unknown Types

```c
// Partially known structures
struct __partial struct_12 {
    uint32_t field_0;
    uint32_t field_4;
    __unknown(4) gap_8;  // 4 unknown bytes
    uint32_t field_c;
};

// Completely unknown
__unknown_t* opaque_ptr;
__unknown(256) buffer;  // 256 unknown bytes
```

### 4. Inline Assembly for Unrecoverable Patterns

```c
// When high-level representation impossible
__asm_inline {
    // Original assembly preserved
    xor eax, eax
    cpuid
    mov dword ptr [rbx], eax
}
```

## Control Flow Representation

### 1. Structured Control Flow (Preferred)

```c
// Recovered loops
for (int32_t i = 0; i < count; i++) {
    process_item(items[i]);
}

// Recovered conditions
if (status == SUCCESS) {
    return result;
} else {
    cleanup();
    return ERROR;
}

// Recovered switches
switch (opcode) {
case 0x01: handle_add(); break;
case 0x02: handle_sub(); break;
default: handle_unknown(); break;
}
```

### 2. Goto When Necessary

```c
// Irreducible control flow
retry:
    result = try_operation();
    if (result == RETRY) {
        if (++attempts < MAX_ATTEMPTS) {
            goto retry;  /* Irreducible loop */
        }
    }
```

### 3. Computed Goto for Jump Tables

```c
// When switch conversion impossible
void* jump_table[] = {&&case_0, &&case_1, &&case_2};
goto *jump_table[index];

case_0:
    handle_case_0();
    goto end;
case_1:
    handle_case_1();
    goto end;
// ...
```

## Type System

### 1. Progressive Type Recovery

```c
// Stage 1: Raw memory access
*(uint32_t*)(base + 0x10) = value;

// Stage 2: Typed pointer
((uint32_t*)base)[4] = value;

// Stage 3: Recovered structure
struct Data* data = (struct Data*)base;
data->field_10 = value;
```

### 2. Union Types for Ambiguous Data

```c
// When multiple interpretations possible
union __multi_type {
    uint32_t as_int;
    float as_float;
    void* as_ptr;
} ambiguous_value;
```

### 3. Flexible Array Members

```c
// For variable-length structures
struct PacketHeader {
    uint32_t type;
    uint32_t length;
    uint8_t data[];  // C99 flexible array
};
```

## Function Representation

### 1. Calling Convention Annotations

```c
// Explicit calling conventions
void __stdcall api_function(int32_t param);
int32_t __fastcall optimized_func(int32_t a, int32_t b);
void __cdecl variadic_func(const char* fmt, ...);
```

### 2. Function Signatures with Confidence

```c
// High confidence signature
int32_t calculate_crc32(const uint8_t* data, size_t length);

// Uncertain parameters
void __uncertain_params sub_401200(
    __probable("buffer") void* arg1,
    __probable("size") int32_t arg2,
    __unknown_t* arg3
);
```

### 3. Variadic Functions

```c
// When detected
int32_t my_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);
    // Implementation
    va_end(args);
}
```

## Memory Access Patterns

### 1. Pointer Arithmetic (C-style)

```c
// Clear and simple
uint32_t* ptr = base;
ptr += offset / sizeof(uint32_t);
*ptr = value;
```

### 2. Array Access

```c
// Prefer array notation when possible
array[index] = value;

// Unless pointer arithmetic is clearer
*(ptr + complex_offset) = value;
```

### 3. Structure Access

```c
// Direct member access
config->flags |= FLAG_ENABLED;

// Offset-based when structure partially known
*(uint32_t*)((uint8_t*)config + 0x20) = value;
```

## Special Constructs

### 1. Compiler Intrinsics

```c
// Recognize and use intrinsics
uint32_t bit_count = __builtin_popcount(value);
uint32_t leading_zeros = __builtin_clz(value);
value = __builtin_bswap32(value);  // Byte swap
```

### 2. SIMD Operations

```c
// When SIMD patterns detected
typedef uint8_t v16qi __attribute__((vector_size(16)));

v16qi a = *(v16qi*)src;
v16qi b = *(v16qi*)dst;
v16qi result = a + b;  // Vector addition
*(v16qi*)out = result;
```

### 3. Atomic Operations

```c
// C11 atomics when detected
_Atomic uint32_t counter;
atomic_fetch_add(&counter, 1);

// Or GCC built-ins
__sync_fetch_and_add(&shared_var, 1);
```

## Output Formatting

### 1. Consistent Style

```c
// K&R brace style for familiarity
if (condition) {
    do_something();
} else {
    do_other();
}

// 4-space indentation
// 80-column limit where reasonable
```

### 2. Comments for Context

```c
// Address references
value = memory[0x1000];  /* Read from .data:0x401000 */

// Confidence indicators
result = complex_calc();  /* @0x401234 confidence: 0.7 */

// Pattern recognition
memcpy(dst, src, len);  /* Recognized pattern: rep movsb */
```

### 3. Metadata Headers

```c
/*
 * Decompiled from: example.exe
 * Function: sub_401000
 * Address range: 0x401000 - 0x401234
 * Compiler: MSVC 14.29 (confidence: 0.9)
 * Architecture: x86_64
 * Timestamp: 2024-01-01 00:00:00 UTC
 */
```

## Examples

### Simple Function

```c
/* Function at 0x401000 */
int32_t calculate_sum(int32_t* array, int32_t count) {
    int32_t sum = 0;
    
    for (int32_t i = 0; i < count; i++) {
        sum += array[i];
    }
    
    return sum;
}
```

### Complex Function with Uncertainty

```c
/* Function at 0x402000 - confidence: 0.6 */
void* __uncertain process_data(
    __probable("input_buffer") uint8_t* data,
    __probable("size") uint32_t size,
    __unknown_t* context
) {
    /* Check magic number @0x402003 */
    if (*(uint32_t*)data != 0x12345678) [[unlikely]] {
        return nullptr;
    }
    
    /* Partially recovered structure @0x402010 */
    struct __partial Header {
        uint32_t magic;
        uint32_t version;
        __unknown(8) reserved;
        uint32_t data_offset;
    } *header = (struct Header*)data;
    
    /* Jump table @0x402030 - irreducible control flow */
    void* handlers[] = {&&handle_v1, &&handle_v2, &&handle_v3};
    if (header->version < 3) {
        goto *handlers[header->version];
    }
    
handle_v1:
    /* Version 1 processing */
    return process_v1(data + header->data_offset);
    
handle_v2:
    /* Version 2 processing */
    return process_v2(data + header->data_offset);
    
handle_v3:
    /* Version 3 processing */
    return process_v3(data + header->data_offset);
}
```

### Recognized Cryptographic Function

```c
/* Recognized: CRC32 implementation @0x403000 */
uint32_t crc32_calculate(const uint8_t* data, size_t length) {
    static const uint32_t crc_table[256] = {
        0x00000000, 0x77073096, /* ... standard CRC32 table ... */
    };
    
    uint32_t crc = 0xFFFFFFFF;
    
    for (size_t i = 0; i < length; i++) {
        uint8_t index = (crc ^ data[i]) & 0xFF;
        crc = (crc >> 8) ^ crc_table[index];
    }
    
    return ~crc;
}
```

## Design Rationale

1. **C as Base**: Familiar to all reverse engineers
2. **Modern Features**: Only where they significantly improve readability
3. **Explicit Uncertainty**: Never hide what we don't know
4. **Address Preservation**: Critical for debugging
5. **Minimal Runtime**: No hidden allocations or overhead
6. **Gradual Refinement**: Output improves as analysis deepens

This approach produces output that is immediately useful while leaving room for progressive enhancement as the
decompiler's analysis improves.