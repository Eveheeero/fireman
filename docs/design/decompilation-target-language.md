# Decompilation Target Language Design

## Overview

The decompilation target is not standard C/C++, but a carefully designed C-like language optimized for:
- **Human readability**: Clear, intuitive code structure
- **Decompilation accuracy**: Preserves low-level semantics when needed
- **Analysis-friendly**: Explicit about uncertainties and assumptions

## Language Features

### 1. Type System

#### Explicit Size Types
```c
// Standard sized integers
int8_t, int16_t, int32_t, int64_t
uint8_t, uint16_t, uint32_t, uint64_t

// Pointer types with explicit attributes
int32_t* __ptr32 p32;  // 32-bit pointer
int64_t* __ptr64 p64;  // 64-bit pointer
void* __unknown ptr;   // Unknown pointer type
```

#### Decompiler-Specific Types
```c
// Unknown or partially known types
__unknown_t var1;              // Completely unknown
__unknown_size(4) var2;        // Known size, unknown type
__probable_int32 var3;         // Likely int32_t
__probable_struct(size=16) s1; // Likely a struct

// Union types for ambiguous cases
__variant {
    int32_t as_int;
    float as_float;
    void* as_ptr;
} ambiguous_var;
```

### 2. Function Declarations

#### Calling Convention Annotations
```c
// Explicit calling conventions
int32_t __cdecl func1(int32_t a, int32_t b);
int32_t __stdcall func2(int32_t a);
int32_t __fastcall func3(int32_t a, int32_t b);
int32_t __regparm(3) func4(int32_t a, int32_t b, int32_t c);

// Custom/unknown conventions
int32_t __custom_cc("rdi,rsi,rdx") func5(int32_t a, int32_t b, int32_t c);
```

#### Uncertainty Markers
```c
// Confidence levels
__confident int32_t definitely_returns_int(void);
__probable void* probably_allocates(size_t size);
__uncertain __unknown_t mystery_function(__unknown_t arg);

// Parameter uncertainties
int32_t process(
    int32_t arg1,
    __maybe_unused int32_t arg2,
    __probably_out int32_t* result
);
```

### 3. Control Flow

#### Reconstructed Loops
```c
// Loop with confidence annotation
__loop_confidence(high) 
for (int32_t i = 0; i < count; i++) {
    array[i] = process(array[i]);
}

// Partially reconstructed loop
__loop_confidence(medium) {
    int32_t i = 0;
    __loop_label: {
        if (i >= count) goto __loop_exit;
        array[i] = process(array[i]);
        i++;
        goto __loop_label;
    }
    __loop_exit: ;
}

// Unstructured loop (low confidence)
__loop_confidence(low) {
    // Original assembly-like structure preserved
    r0 = 0;
loop_0x1000:
    if (r0 >= r1) goto exit_0x1050;
    *(base + r0 * 4) = process(*(base + r0 * 4));
    r0 = r0 + 1;
    goto loop_0x1000;
exit_0x1050:
}
```

#### Switch Statements
```c
// High-confidence switch
switch (command) {
    case 1: return handle_read();
    case 2: return handle_write();
    case 3: return handle_delete();
    default: return ERROR_UNKNOWN_COMMAND;
}

// Jump table with annotations
__switch_jumptable(base=0x1000, count=4) {
    switch (index) {
        __case_addr(0x1100): return process_a();
        __case_addr(0x1200): return process_b();
        __case_addr(0x1300): return process_c();
        __case_addr(0x1400): return process_d();
    }
}
```

### 4. Memory Access

#### Explicit Memory Operations
```c
// Clear memory access patterns
int32_t value = __read_memory_32(address);
__write_memory_32(address, value);

// Atomic operations
int32_t old = __atomic_exchange_32(&variable, new_value);
__atomic_compare_exchange_64(&variable, &expected, desired);

// Memory barriers
__memory_barrier_full();
__memory_barrier_acquire();
__memory_barrier_release();
```

#### Structure Recovery
```c
// Partially recovered structure
struct __recovered_struct_1 {
    int32_t field_0x00;        // Confident field
    __padding(4);              // Explicit padding
    __probable_ptr field_0x08; // Probable pointer
    __unknown_size(8) field_0x10; // Unknown 8-byte field
    
    // Inline assembly for unclear access patterns
    __asm_access {
        "mov eax, [rcx+0x18]"
        "test eax, eax"
    }
};

// Virtual table annotation
struct __class_with_vtable {
    __vtable {
        void (*destructor)(void* this);
        int32_t (*method1)(void* this, int32_t arg);
        void* (*method2)(void* this);
    } *vptr;
    
    // Member data follows
    int32_t member1;
    void* member2;
};
```

### 5. Inline Assembly Integration

```c
// Inline assembly for unrecoverable patterns
__inline_asm {
    "cpuid"
    "mov [rbx], eax"
    "mov [rbx+4], ebx"
    "mov [rbx+8], ecx"
    "mov [rbx+12], edx"
}

// Mixed high-level and assembly
int32_t optimized_function(int32_t x) {
    if (x < 0) {
        __inline_asm {
            "neg eax"
            "adc eax, 0"
        }
        return __asm_result(eax);
    }
    return x;
}
```

### 6. Annotations and Metadata

#### Source Information
```c
// Function annotations
__function_info {
    .address = 0x401000,
    .size = 0x50,
    .cfg_nodes = 5,
    .complexity = "medium",
    .compiler = "msvc_v19",
    .optimization = "O2"
}
int32_t analyzed_function(int32_t param) {
    // Function body
}
```

#### Decompilation Confidence
```c
// Block-level confidence
__confidence(high) {
    result = calculate_sum(a, b);
}

__confidence(low) {
    // Complex or obfuscated code
    r1 = r2 ^ 0xDEADBEEF;
    r1 = __rotr(r1, 13);
    r3 = lookup_table[r1 & 0xFF];
}
```

### 7. Special Constructs

#### Exception Handling
```c
__try {
    risky_operation();
} __except(__exception_filter) {
    handle_error();
} __finally {
    cleanup();
}

// SEH (Structured Exception Handling)
__seh_try {
    protected_code();
} __seh_except(GetExceptionCode() == EXCEPTION_ACCESS_VIOLATION) {
    handle_access_violation();
}
```

#### Coroutines/Generators
```c
__coroutine state_machine(int32_t initial) {
    __yield_point_1:
        process_stage_1(initial);
        __yield(result1);
    
    __yield_point_2:
        process_stage_2();
        __yield(result2);
    
    __coroutine_exit:
        return final_result;
}
```

## Output Examples

### Simple Function
```c
__function_info { .address = 0x401000, .confidence = "high" }
int32_t calculate_sum(int32_t a, int32_t b) {
    int32_t result = a + b;
    if (__unlikely(result < a)) {  // Overflow check
        return INT32_MAX;
    }
    return result;
}
```

### Complex Function with Uncertainties
```c
__function_info { .address = 0x402000, .confidence = "medium" }
__probable void* complex_function(
    void* arg1,
    __unknown_size(8) arg2,
    __maybe_unused int32_t arg3
) {
    __confidence(high) {
        if (arg1 == NULL) {
            return NULL;
        }
    }
    
    __confidence(medium) {
        __probable_struct(size=24)* obj = allocate_object(24);
        if (obj == NULL) {
            return NULL;
        }
        
        // Partially recovered initialization
        obj->field_0x00 = __read_memory_64(arg1);
        obj->field_0x08 = arg2;
        __unknown_operation {
            "mov rax, [rdi+0x10]"
            "xor rax, rdx"
            "mov [rsi+0x10], rax"
        }
    }
    
    __confidence(low) {
        // Obfuscated or complex code
        __inline_asm {
            "complex assembly sequence"
        }
    }
    
    return obj;
}
```

### Recovered Class
```c
__class_info { .name = "probable_FileHandler", .confidence = "medium" }
struct FileHandler {
    __vtable {
        void (*destructor)(struct FileHandler* this);
        int32_t (*open)(struct FileHandler* this, const char* path);
        int32_t (*read)(struct FileHandler* this, void* buffer, size_t size);
        int32_t (*close)(struct FileHandler* this);
    } *vptr;
    
    int32_t file_descriptor;
    size_t file_size;
    __probable_ptr buffer;
    __unknown_size(16) internal_state;
};

__method_of(FileHandler)
int32_t FileHandler_read(struct FileHandler* this, void* buffer, size_t size) {
    if (this->file_descriptor < 0) {
        return -1;
    }
    
    // Recovered implementation
    return system_read(this->file_descriptor, buffer, size);
}
```

## Formatting Guidelines

### Indentation and Spacing
- 4 spaces for indentation
- Clear separation between logical blocks
- Annotations on separate lines for clarity

### Naming Conventions
- Original symbol names when available
- `sub_ADDRESS` for unknown functions
- `var_OFFSET` for stack variables
- `field_OFFSET` for structure fields

### Comments
```c
// High-level explanation
/* Multi-line explanation
   for complex sections */
   
// Decompiler notes
// [DECOMPILER]: Possible switch statement
// [DECOMPILER]: Inlined function detected
// [DECOMPILER]: Loop bounds uncertain
```

## Benefits

1. **Clarity**: Explicit about what is known vs. uncertain
2. **Accuracy**: Preserves low-level details when high-level reconstruction fails
3. **Analyzability**: Machine-readable annotations for further analysis
4. **Debugging**: Easy to correlate with original assembly
5. **Educational**: Shows the decompilation process and confidence levels