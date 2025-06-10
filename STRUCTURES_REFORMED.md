# Fireman Decompiler - Reformed Architecture & Structure Diagrams

## Current Architecture Overview

```mermaid
graph TB
    subgraph "Input Layer"
        BINARY[Binary Files<br/>PE/ELF/Mach-O]
        BYTES[Raw Bytes]
    end

subgraph "Binary Format Layer"
PE[PE Parser<br/>binary/pe/]
ELF[ELF Parser<br/>binary/elf/]
MACHO[Mach-O Parser<br/>binary/macho/]
AUTODETECT[Format Detection<br/>Magic Bytes]
end

subgraph "Architecture Layer"
X86_64[x86_64<br/>✅ Implemented]
X86[x86<br/>🚧 Planned]
ARM32[ARM32<br/>🚧 Planned]
ARM64[ARM64<br/>🚧 Planned]
ARCHDETECT[Arch Detection]
end

subgraph "Core Components"
FIRE[Fire Trait<br/>Main Interface]
FIRERAW[FireRaw<br/>Low-level Ops]
BLOCKS[Blocks<br/>Basic Blocks]
INSTR[Instructions]
ADDR[Addressing<br/>VMA/File]
end

subgraph "Multi-Level IR"
LOWIR[Low IR<br/>Direct Translation]
MEDIUMIR[Medium IR<br/>Pattern Recognition]
HIGHIR[High IR<br/>Structured Code]
end

subgraph "Analysis Pipeline"
CFG[CFG Construction]
DFA[Data Flow Analysis]
TYPE[Type Recovery]
VAR[Variable Analysis]
STRUCT[Struct Reconstruction]
LOOP[Loop Analysis]
end

subgraph "Output Layer"
ENHANCEDC[Enhanced C<br/>Modern Features]
GUI[GUI Display<br/>Tauri/React]
CLI[CLI Output]
end

BINARY --> AUTODETECT
BYTES --> AUTODETECT
AUTODETECT --> PE & ELF & MACHO
PE & ELF & MACHO --> ARCHDETECT
ARCHDETECT --> X86_64 & X86 & ARM32 & ARM64
X86_64 --> FIRE
FIRE --> FIRERAW
FIRERAW --> BLOCKS & INSTR & ADDR
BLOCKS --> LOWIR
LOWIR --> MEDIUMIR
MEDIUMIR --> HIGHIR
LOWIR & MEDIUMIR & HIGHIR --> CFG & DFA & TYPE & VAR & STRUCT & LOOP
HIGHIR --> ENHANCEDC
ENHANCEDC --> GUI & CLI

style X86_64 fill: #90EE90
style X86 fill: #FFE4B5
style ARM32 fill:#FFE4B5
style ARM64 fill: #FFE4B5
```

## Multi-Level IR Architecture

```mermaid
graph LR
    subgraph "Low IR - Direct Translation"
        L1[Machine Instructions]
        L2[Register Operations]
        L3[Memory Access]
        L4[Flags Preservation]
        L5[SSA Transform]
    end

    subgraph "Medium IR - Pattern Recognition"
        M1[Function Patterns]
        M2[Loop Patterns]
        M3[Switch Patterns]
        M4[Library Patterns]
        M5[Confidence Tracking]
    end

    subgraph "High IR - Structured Code"
        H1[Control Flow<br/>if/while/for]
        H2[Type Definitions<br/>struct/union/enum]
        H3[Variable Names]
        H4[Function Signatures]
        H5[Enhanced C AST]
    end

    L1 --> M1
    L2 --> M2
    L3 --> M3
    L4 --> M4
    L5 --> M5
    M1 --> H1
    M2 --> H2
    M3 --> H3
    M4 --> H4
    M5 --> H5
```

## IR Type System

```mermaid
classDiagram
    class IRType {
        <<enumeration>>
        I8
        I16
        I32
        I64
        F32
        F64
        Pointer(Box~IRType~)
        Array(Box~IRType~, usize)
        Struct(Vec~IRType~)
        Void
    }

    class Statement {
        +id: StatementId
        +address: Address
        +metadata: Metadata
    }

    class Expression {
        +ty: IRType
        +value: Value
    }

    Statement <|-- Assignment
    Statement <|-- Branch
    Statement <|-- Call
    Statement <|-- Return
    Statement <|-- Store
    Statement <|-- Load
    Expression <|-- BinOp
    Expression <|-- UnaryOp
    Expression <|-- Constant
    Expression <|-- Variable
    Expression <|-- MemoryRef

    class BinOp {
        +op: Operator
        +lhs: Expression
        +rhs: Expression
    }

    class Assignment {
        +dest: Variable
        +src: Expression
    }

    class Branch {
        +condition: Option~Expression~
        +target: BlockId
        +fallthrough: Option~BlockId~
    }
```

## Analysis Pipeline Detail

```mermaid
flowchart TB
    subgraph "Stage 1: CFG Construction"
        A1[Basic Blocks] --> A2[Block Relations]
        A2 --> A3[Dominance Tree]
        A3 --> A4[Loop Detection]
    end

    subgraph "Stage 2: Data Flow"
        A4 --> B1[Reaching Definitions]
        B1 --> B2[Live Variables]
        B2 --> B3[Use-Def Chains]
        B3 --> B4[SSA Construction]
    end

    subgraph "Stage 3: Type Recovery"
        B4 --> C1[Constraint Collection]
        C1 --> C2[Type Inference]
        C2 --> C3[Struct Recognition]
        C3 --> C4[Pointer Analysis]
    end

    subgraph "Stage 4: High-Level Recovery"
        C4 --> D1[Variable Naming]
        D1 --> D2[Function Detection]
        D2 --> D3[Loop Structuring]
        D3 --> D4[Pattern Matching]
    end

    subgraph "Stage 5: Code Generation"
        D4 --> E1[AST Construction]
        E1 --> E2[Enhanced C Features]
        E2 --> E3[Pretty Printing]
    end
```

## Deterministic Processing Architecture

```mermaid
graph TD
    subgraph "Deterministic Guarantees"
        D1[Sorted Collections<br/>BTreeMap/BTreeSet]
        D2[Fixed Address Format<br/>16-digit hex]
        D3[Deterministic Naming<br/>purpose.address.counter]
        D4[Ordered Processing<br/>By Address]
    end

    subgraph "Naming System"
        N1[Block Names<br/>block_0000000140001000]
        N2[Variable Names<br/>var_rsp_plus_8]
        N3[Temp Names<br/>temp_140001234_0]
        N4[Function Names<br/>sub_140001000]
    end

    D1 --> N1 & N2 & N3 & N4
    D2 --> N1 & N3 & N4
    D3 --> N2 & N3
    D4 --> Processing[Deterministic Output]
```

## Enhanced C Generation System

```mermaid
stateDiagram-v2
    [*] --> AnalyzeIR: High IR Input
    AnalyzeIR --> CheckConfidence: Analyze Patterns
    CheckConfidence --> ModernFeatures: High Confidence
    CheckConfidence --> ConservativeC: Low Confidence
    ModernFeatures --> UseAuto: Complex Types
    ModernFeatures --> UseNullptr: Pointer Init
    ModernFeatures --> UseRangeFor: Array Iteration
    ModernFeatures --> UseFixedInt: Integer Types
    ConservativeC --> StandardC: Traditional C
    UseAuto --> GenerateAST
    UseNullptr --> GenerateAST
    UseRangeFor --> GenerateAST
    UseFixedInt --> GenerateAST
    StandardC --> GenerateAST
    GenerateAST --> PrettyPrint: Format Code
    PrettyPrint --> [*]: Enhanced C Output
```

## Current Implementation Status

```mermaid
gantt
    title Fireman Implementation Progress
    dateFormat X
    axisFormat %s

    section Core
        Fire/FireRaw Interface: done, 0, 10
        Block/Instruction Types: done, 0, 10
        Deterministic Framework: done, 0, 10

    section Binary Formats
        PE Parser: done, 0, 10
        ELF Parser: active, 5, 10
        Mach-O Parser: active, 3, 10

    section Architectures
        x86_64 Support: done, 0, 8
        x86_64 Full Coverage: active, 8, 10
        x86 Support: 2, 10
        ARM32 Support: 1, 10
        ARM64 Support: 1, 10

    section IR System
        Low IR: done, 0, 10
        Medium IR: active, 0, 7
        High IR: done, 0, 10

    section Analysis
        CFG Construction: done, 0, 10
        Type Recovery: done, 0, 10
        Variable Analysis: done, 0, 10
        Struct Reconstruction: done, 0, 10
        Loop Analysis: active, 0, 8

    section Output
        Enhanced C Generation: done, 0, 10
        GUI Visualization: done, 0, 10
        Plugin System: 0, 10
```

## Memory Model and Representations

```mermaid
graph TB
    subgraph "Binary Memory Layout"
        CODE[.text Section<br/>Executable Code]
        DATA[.data Section<br/>Initialized Data]
        BSS[.bss Section<br/>Uninitialized Data]
        RODATA[.rodata Section<br/>Read-Only Data]
    end

    subgraph "IR Memory Model"
        STACK[Stack Frame<br/>RSP-relative]
        HEAP[Heap References<br/>Tracked Pointers]
        GLOBAL[Global Data<br/>Absolute Addresses]
        REG[Register File<br/>CPU State]
    end

    subgraph "Address Representation"
        VMA[Virtual Address<br/>0x140001000]
        FILE[File Offset<br/>0x400]
        RVA[Relative VA<br/>0x1000]
    end

    CODE --> REG
    DATA --> GLOBAL
    BSS --> GLOBAL
    RODATA --> GLOBAL
    STACK --> VMA
    HEAP --> VMA
    GLOBAL --> VMA
    VMA <--> FILE
    VMA <--> RVA
```

## Testing Architecture

```mermaid
graph LR
    subgraph "Test Categories"
        UNIT[Unit Tests<br/>Individual Components]
        INTEGRATION[Integration Tests<br/>Pipeline Testing]
        DETERMINISM[Determinism Tests<br/>Reproducibility]
        E2E[End-to-End Tests<br/>Real Binaries]
    end

    subgraph "Test Binaries"
        HELLO[hello_world.exe]
        COMPLEX[Complex Programs]
        MALFORMED[Edge Cases]
    end

    subgraph "Verification"
        OUTPUT[Output Comparison]
        PERF[Performance Metrics]
        MEM[Memory Usage]
    end

    UNIT --> OUTPUT
    INTEGRATION --> OUTPUT
    DETERMINISM --> OUTPUT
    E2E --> OUTPUT
    HELLO --> E2E
    COMPLEX --> E2E
    MALFORMED --> E2E
    OUTPUT --> PERF & MEM
```

## Workspace Dependencies

```mermaid
graph BT
    subgraph "External Crates"
        GOBLIN[goblin<br/>Binary Parsing]
        CAPSTONE[capstone<br/>Disassembly]
        TAURI[tauri<br/>GUI Framework]
        SERDE[serde<br/>Serialization]
    end

    subgraph "Workspace Crates"
        FIREBALL[fireball<br/>Core Library]
        FIREMAN[fireman<br/>CLI App]
        FIREBAT[firebat<br/>GUI App]
        ICEBALL[iceball<br/>Disassembler]
        DRYICE[dryice<br/>Pattern Matching]
        MACRO[fireman_macro<br/>Proc Macros]
    end

    FIREBALL --> GOBLIN
    FIREBALL --> CAPSTONE
    FIREBALL --> SERDE
    FIREBAT --> FIREBALL
    FIREBAT --> TAURI
    FIREMAN --> FIREBALL
    ICEBALL -.-> FIREBALL
    DRYICE -.-> FIREBALL
    FIREBALL --> MACRO
    style ICEBALL fill: #FFE4B5
    style DRYICE fill: #FFE4B5
```

## Key Architectural Decisions

### 1. **Determinism First**

- Every operation must be reproducible
- Sorted data structures throughout
- No randomness or time-dependent behavior

### 2. **Multi-Level IR**

- Gradual abstraction from machine code to source
- Preserve semantics at each level
- Enable targeted optimizations

### 3. **Enhanced C Output**

- Modern C features for readability
- Confidence-based feature selection
- Maintain decompiler characteristics

### 4. **Extensible Architecture**

- Trait-based design for formats/architectures
- Clear separation of concerns
- Plugin system planned for customization

### 5. **Performance Focus**

- Zero-copy where possible
- Parallel analysis capabilities
- Efficient memory usage

## Future Roadmap

1. **Near Term**
    - Complete x86_64 instruction coverage
    - Finish ELF/Mach-O parsers
    - Improve pattern matching

2. **Medium Term**
    - ARM architecture support
    - Advanced optimization passes
    - Symbol resolution

3. **Long Term**
    - Plugin system
    - Cloud-based analysis
    - Machine learning integration

This reformed architecture represents the current state of the Fireman decompiler, focusing on practical implementation
while maintaining the vision of a high-performance, deterministic decompilation framework.
