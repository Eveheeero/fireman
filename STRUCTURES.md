# Fireman Decompiler - Architecture & Structure Diagrams

## High-Level Architecture

```mermaid
graph TB
    subgraph "Input Layer"
        BINARY[Binary File<br/>PE/ELF/Mach-O]
        DEBUG[Debug Info<br/>PDB/DWARF]
    end

    subgraph "Parsing Layer"
        LOADER[Binary Loader<br/>goblin]
        DISASM[Disassembler<br/>Capstone/XED]
    end

    subgraph "Analysis Layer"
        CFG[Control Flow<br/>Graph Builder]
        DFA[Data Flow<br/>Analyzer]
        TYPE[Type<br/>Recovery]
        PATTERN[Pattern<br/>Matcher]
    end

    subgraph "IR Layer"
        LIFT[IR Lifter<br/>x86→IR]
        OPT[IR Optimizer]
        SSA[SSA Transform]
    end

    subgraph "Output Layer"
        CGEN[C Generator]
        GUI[GUI Display]
        PLUGIN[Plugin API]
    end

    BINARY --> LOADER
    DEBUG --> LOADER
    LOADER --> DISASM
    DISASM --> LIFT
    LIFT --> CFG
    LIFT --> DFA
    CFG --> TYPE
    DFA --> TYPE
    TYPE --> OPT
    OPT --> SSA
    SSA --> CGEN
    SSA --> GUI
    PATTERN --> OPT
    PLUGIN -.-> LIFT
    PLUGIN -.-> OPT
    PLUGIN -.-> CGEN
```

## Decompilation Pipeline

```mermaid
flowchart LR
    subgraph "Stage 1: Decoding"
        A1[Raw Bytes] --> A2[Instruction<br/>Decoder]
        A2 --> A3[Basic<br/>Blocks]
    end

    subgraph "Stage 2: Lifting"
        A3 --> B1[IR<br/>Generation]
        B1 --> B2[Flag<br/>Elimination]
        B2 --> B3[Stack<br/>Analysis]
    end

    subgraph "Stage 3: Analysis"
        B3 --> C1[Data Flow<br/>Analysis]
        C1 --> C2[Type<br/>Inference]
        C2 --> C3[Variable<br/>Recovery]
    end

    subgraph "Stage 4: Optimization"
        C3 --> D1[Dead Code<br/>Elimination]
        D1 --> D2[Expression<br/>Simplification]
        D2 --> D3[Pattern<br/>Recognition]
    end

    subgraph "Stage 5: Generation"
        D3 --> E1[Control Flow<br/>Structuring]
        E1 --> E2[C Code<br/>Generation]
        E2 --> E3[Pretty<br/>Printing]
    end
```

## IR Structure

```mermaid
classDiagram
    class Statement {
        +id: StatementId
        +location: Address
        +metadata: Metadata
    }

    class Expression {
        +type: DataType
        +size: Size
    }

    Statement <|-- Assignment
    Statement <|-- Jump
    Statement <|-- Call
    Statement <|-- Return

    class Assignment {
        +dst: LValue
        +src: Expression
    }

    class Jump {
        +condition: Expression
        +target: Label
    }

    Expression <|-- BinaryOp
    Expression <|-- UnaryOp
    Expression <|-- Memory
    Expression <|-- Register
    Expression <|-- Constant

    class BinaryOp {
        +op: Operator
        +left: Expression
        +right: Expression
    }

    class Memory {
        +base: Expression
        +offset: Expression
        +size: Size
    }
```

## Control Flow Graph Structure

```mermaid
graph TD
    subgraph "Function CFG"
        ENTRY[Entry Block]
        BB1[Basic Block 1<br/>mov eax, [ebp+8]<br/>test eax, eax]
        BB2[Basic Block 2<br/>je .error]
        BB3[Basic Block 3<br/>call validate<br/>test eax, eax]
        BB4[Basic Block 4<br/>mov eax, 1<br/>ret]
        ERROR[Error Block<br/>xor eax, eax<br/>ret]
        EXIT[Exit Block]
    end

    ENTRY --> BB1
    BB1 --> BB2
    BB2 -->|condition: ZF=1| ERROR
    BB2 -->|condition: ZF=0| BB3
    BB3 --> BB4
    BB4 --> EXIT
    ERROR --> EXIT

    style ENTRY fill:#90EE90
    style EXIT fill:#FFB6C1
    style ERROR fill:#FFA07A
```

## Data Flow Analysis

```mermaid
flowchart TB
    subgraph "Reaching Definitions"
        DEF1[x = 5]
        DEF2[y = x + 3]
        DEF3[x = y * 2]
        USE1[return x + y]
    end

    DEF1 -.->|x defined| DEF2
    DEF2 -.->|y defined| DEF3
    DEF1 -.->|x killed| DEF3
    DEF3 -.->|x reaches| USE1
    DEF2 -.->|y reaches| USE1

    subgraph "Live Variables"
        LIVE1["{x, y} live"]
        LIVE2["{y} live"]
        LIVE3["{} live"]
    end
```

## Type Recovery Process

```mermaid
stateDiagram-v2
    [*] --> Unknown
    Unknown --> Integer: Size constraint
    Unknown --> Pointer: Dereference
    Integer --> SignedInt: Comparison < 0
    Integer --> UnsignedInt: High bit usage
    Pointer --> ArrayPtr: Indexing pattern
    Pointer --> StructPtr: Field access
    ArrayPtr --> TypedArray: Element type inferred
    StructPtr --> TypedStruct: Field types inferred
    TypedArray --> [*]
    TypedStruct --> [*]
```

## Pattern Matching System

```mermaid
graph LR
    subgraph "Pattern Database"
        P1[strlen Pattern]
        P2[memcpy Pattern]
        P3[malloc Pattern]
        P4[switch Pattern]
    end

    subgraph "Matching Engine"
        M1[Fuzzy Matcher]
        M2[Exact Matcher]
        M3[Statistical Matcher]
    end

    subgraph "IR Stream"
        IR1[IR Block 1]
        IR2[IR Block 2]
        IR3[IR Block 3]
    end

    IR1 --> M1
    IR2 --> M2
    IR3 --> M3
    P1 --> M1
    P2 --> M2
    P3 --> M3
    P4 --> M1
    M1 --> MATCH[Pattern Match]
    M2 --> MATCH
    M3 --> MATCH
```

## Memory Model

```mermaid
graph TD
    subgraph "Process Memory"
        STACK[Stack<br/>Local Variables<br/>Return Addresses]
        HEAP[Heap<br/>Dynamic Allocations]
        DATA[Data Section<br/>Global Variables]
        CODE[Code Section<br/>Instructions]
    end

    subgraph "IR Memory Representation"
        STACKIR[Stack Slots<br/>sp+offset]
        HEAPIR[Heap Objects<br/>tracked pointers]
        GLOBIR[Global Refs<br/>absolute/relative]
        CONSTIR[Constants<br/>immediate values]
    end

    STACK --> STACKIR
    HEAP --> HEAPIR
    DATA --> GLOBIR
    CODE --> CONSTIR
```

## Plugin Architecture

```mermaid
sequenceDiagram
    participant User
    participant GUI
    participant Core
    participant Plugin
    participant IR

    User->>GUI: Load binary
    GUI->>Core: Initialize analysis
    Core->>Plugin: on_binary_loaded()
    Core->>IR: Generate initial IR
    IR->>Plugin: on_ir_generated()
    Plugin->>IR: Transform IR
    Plugin->>Core: Register patterns
    Core->>Plugin: on_analysis_complete()
    Plugin->>GUI: Add custom views
    GUI->>User: Display results
```

## Optimization Pipeline

```mermaid
graph LR
    subgraph "IR Optimization Passes"
        RAW[Raw IR] --> DCE[Dead Code<br/>Elimination]
        DCE --> CSE[Common Subexpr<br/>Elimination]
        CSE --> CP[Constant<br/>Propagation]
        CP --> CF[Constant<br/>Folding]
        CF --> SIMP[Algebraic<br/>Simplification]
        SIMP --> INLINE[Function<br/>Inlining]
        INLINE --> FINAL[Optimized IR]
    end
```

## Error Recovery Strategy

```mermaid
stateDiagram-v2
    [*] --> Parsing
    Parsing --> Success: Valid instruction
    Parsing --> PartialFail: Unknown opcode
    Parsing --> TotalFail: Invalid bytes
    
    PartialFail --> Recovery: Mark as data
    TotalFail --> Recovery: Skip bytes
    Recovery --> Parsing: Continue
    
    Success --> Analysis
    Analysis --> TypeInference
    TypeInference --> Timeout: Too complex
    Timeout --> Fallback: Use generic type
    Fallback --> CodeGen
    TypeInference --> CodeGen: Success
    CodeGen --> [*]
```

## Component Dependencies

```mermaid
graph BT
    subgraph "External Dependencies"
        GOBLIN[goblin<br/>Binary Parser]
        CAPSTONE[capstone<br/>Disassembler]
        TAURI[tauri<br/>GUI Framework]
    end

    subgraph "Core Components"
        FIREBALL[fireball<br/>Core Library]
        FIREBAT[firebat<br/>GUI App]
        FIREMAN[fireman<br/>CLI]
    end

    subgraph "Utility Crates"
        ICEBALL[iceball<br/>Disassembly]
        DRYICE[dryice<br/>Pattern Matching]
        MACRO[fireman_macro<br/>Proc Macros]
    end

    FIREBALL --> GOBLIN
    FIREBALL --> CAPSTONE
    FIREBAT --> TAURI
    FIREBAT --> FIREBALL
    FIREMAN --> FIREBALL
    ICEBALL --> FIREBALL
    DRYICE --> FIREBALL
    FIREBALL --> MACRO
```

## Performance Architecture

```mermaid
graph TB
    subgraph "Parallel Processing"
        QUEUE[Work Queue]
        W1[Worker 1]
        W2[Worker 2]
        W3[Worker 3]
        W4[Worker 4]
        MERGE[Result Merger]
    end

    subgraph "Caching Layer"
        L1[L1: Instruction Cache]
        L2[L2: IR Cache]
        L3[L3: Analysis Cache]
    end

    subgraph "Incremental Updates"
        DIFF[Change Detector]
        DIRTY[Dirty Tracker]
        REBUILD[Selective Rebuild]
    end

    QUEUE --> W1 & W2 & W3 & W4
    W1 & W2 & W3 & W4 --> MERGE
    MERGE --> L3
    L1 --> L2 --> L3
    DIFF --> DIRTY --> REBUILD
    REBUILD --> QUEUE
```

## Summary

These diagrams illustrate Fireman's architecture from multiple perspectives:

1. **Data Flow**: How binary data transforms into C code
2. **Component Structure**: How different parts interact
3. **Analysis Algorithms**: How we recover high-level information
4. **Performance Design**: How we achieve speed goals
5. **Error Handling**: How we deal with real-world binaries

The key insight is that decompilation is not just "disassembly in reverse" - it's a complex process of information recovery, pattern recognition, and intelligent transformation. Our architecture is designed to make this process fast, accurate, and extensible.