# Fireman AI Agent Guide

## Default

- Using `cargo b` and `cargo t` helps to void lag when didn't touch `firebat` directory.
- Formatting uses `cargo +nightly fmt`
- To avoid context overflow from voluminous logs, use regular expressions to filter and retrieve only the necessary segments for analysis.

## Pattern Policy

- Centralize predefined `.fb` pattern and script registration in `fireball/src/abstract_syntax_tree/optimize/pattern_matching.rs`.
- Do not add optimizer-specific pattern matcher names or `do:` action names.
- Do not add one-off script helpers as a substitute for generic `.fb` features.
- If a rewrite cannot be expressed with the existing general pattern engine, keep it in Rust until a general-purpose pattern feature exists.

## Structure Paths (fireball)

### Major Structs (quick reference)

- `Address` - `fireball/src/core/address.rs`
- `Instruction` - `fireball/src/core/instruction.rs`
- `Section` / `Sections` - `fireball/src/core/section.rs`, `fireball/src/core/sections.rs`
- `Block` / `Blocks` - `fireball/src/core/block.rs`, `fireball/src/core/blocks.rs`
- `Relation` / `Relations` - `fireball/src/core/relation.rs`, `fireball/src/core/relations.rs`
- `PreDefinedOffset` / `PreDefinedOffsets` - `fireball/src/core/pre_defined_offset.rs`, `fireball/src/core/pre_defined_offsets.rs`
- `Pe` - `fireball/src/pe/mod.rs`
- `VirtualMachine`, `IrBlock`, `Ir` - `fireball/src/ir/mod.rs`
- `Register` - `fireball/src/ir/register.rs`
- `IrDataAccess` - `fireball/src/ir/data.rs`
- `IrStatementDescriptor` / `IrStatementDescriptorMap<T>` - `fireball/src/ir/utils.rs`
- `IrVariable`, `BlockGrouper`, `BlockGroup`, `KnownDataType`, `IrFunction`, `IrFunctionVariable` - `fireball/src/ir/analyze/...`
- `Ast`, `AstFunction`, `AstVariable`, `AstParameter` - `fireball/src/abstract_syntax_tree/objects/...`
- `AstPrintConfig`, `AstOptimizationConfig` - `fireball/src/abstract_syntax_tree/objects/...`
- `WrappedAstStatement`, `Wrapped<T>`, `AstDescriptor` - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `AstPattern`, `AstPatternRule`, `AstPatternIrReplacement`, `AstPatternAsmData`, `AstPatternAstData`, `AstPatternIrData`, `AstPatternScript`, `AstPatternRange` - `fireball/src/abstract_syntax_tree/optimize/pattern_matching.rs`
- `VersionMap<..>` - `fireball/src/utils/version_map.rs`

### Entry points

- `Fireball` (enum) - `fireball/src/lib.rs`
- Common imports - `fireball/src/prelude.rs`

### Core (`fireball/src/core`)

- Module exports - `fireball/src/core/mod.rs`
- `Fire` (trait) - `fireball/src/core/fire.rs`
- `FireRaw` (trait) - `fireball/src/core/fire_raw.rs`
- `Address` - `fireball/src/core/address.rs`
- `Instruction` - `fireball/src/core/instruction.rs`
- `Section` - `fireball/src/core/section.rs`
- `Sections` - `fireball/src/core/sections.rs`
- `Block` - `fireball/src/core/block.rs`
- `Blocks` - `fireball/src/core/blocks.rs`
- `Relation` - `fireball/src/core/relation.rs`
- `Relations` - `fireball/src/core/relations.rs`
- `DestinationType` (enum) - `fireball/src/core/relation.rs`
- `RelationType` (enum) - `fireball/src/core/relation.rs`
- `PreDefinedOffset` - `fireball/src/core/pre_defined_offset.rs`
- `PreDefinedOffsets` - `fireball/src/core/pre_defined_offsets.rs`

### PE (`fireball/src/pe`)

- `Pe` - `fireball/src/pe/mod.rs`

### Arch -> IR (`fireball/src/arch`)

- asm to `IrStatement` conversion - `fireball/src/arch/x86_64/instruction_analyze.rs`
- statement/data shortcuts - `fireball/src/arch/x86_64/instruction_analyze_shortcuts/...`

### IR (`fireball/src/ir`)

- Module exports - `fireball/src/ir/mod.rs`
- `VirtualMachine` - `fireball/src/ir/mod.rs`
- `IrBlock` - `fireball/src/ir/mod.rs`
- `Ir` - `fireball/src/ir/mod.rs`
- `Architecture` (enum) - `fireball/src/ir/mod.rs`
- `Register` - `fireball/src/ir/register.rs`
- `IrStatement` (enum) - `fireball/src/ir/statements.rs`
- `IrStatementSpecial` (enum) - `fireball/src/ir/statements.rs`
- `IrUnaryOperator` (enum) - `fireball/src/ir/operator.rs`
- `IrBinaryOperator` (enum) - `fireball/src/ir/operator.rs`
- `IrStatementDescriptor` - `fireball/src/ir/utils.rs`
- `IrStatementDescriptorMap<T>` - `fireball/src/ir/utils.rs`
- `IrData` (enum) - `fireball/src/ir/data.rs`
- `IrDataAccess` - `fireball/src/ir/data.rs`
- `IrIntrinsic` (enum) - `fireball/src/ir/data.rs`
- `NumCondition` (enum) - `fireball/src/ir/data.rs`
- `IrDataAccessType` (enum) - `fireball/src/ir/data.rs`
- `IrDataOperation` (enum) - `fireball/src/ir/data.rs`
- `IrAccessSize` (enum) - `fireball/src/ir/data.rs`
- `IrDataContainable` (trait) - `fireball/src/ir/data.rs`
- `ARM` (trait) - `fireball/src/ir/arm/mod.rs`
- `X64` (trait) - `fireball/src/ir/x86_64/mod.rs`
- `X64Mut` (trait) - `fireball/src/ir/x86_64/mod.rs`

### IR analysis (`fireball/src/ir/analyze`)

- `IrVariable` - `fireball/src/ir/analyze/variables.rs`
- `BlockGrouper` - `fireball/src/ir/analyze/block_grouper.rs`
- `BlockGroup` - `fireball/src/ir/analyze/block_grouper.rs`
- `KnownDataType` - `fireball/src/ir/analyze/datatype.rs`
- `DataType` (enum) - `fireball/src/ir/analyze/datatype.rs`
- `IrFunction` - `fireball/src/ir/analyze/ir_function.rs`
- `IrFunctionVariable` - `fireball/src/ir/analyze/ir_function.rs`

### AST (`fireball/src/abstract_syntax_tree`)

- Module exports - `fireball/src/abstract_syntax_tree/mod.rs`
- `Ast` - `fireball/src/abstract_syntax_tree/objects/ast.rs`
- `AstFunction` - `fireball/src/abstract_syntax_tree/objects/function.rs`
- `AstStatement` (enum) - `fireball/src/abstract_syntax_tree/objects/statement.rs`
- `AstExpression` (enum) - `fireball/src/abstract_syntax_tree/objects/expression.rs`
- `AstValueType` (enum) - `fireball/src/abstract_syntax_tree/objects/value.rs`
- `AstValue` (enum) - `fireball/src/abstract_syntax_tree/objects/value.rs`
- `AstLiteral` (enum) - `fireball/src/abstract_syntax_tree/objects/value.rs`
- `AstVariable` - `fireball/src/abstract_syntax_tree/objects/variable.rs`
- `AstVariableId` - `fireball/src/abstract_syntax_tree/objects/tags.rs`
- `AstFunctionId` - `fireball/src/abstract_syntax_tree/objects/tags.rs`
- `AstFunctionVersion` - `fireball/src/abstract_syntax_tree/objects/tags.rs`
- `AstParameter` - `fireball/src/abstract_syntax_tree/objects/parameter.rs`
- `AstParameterLocation` (enum) - `fireball/src/abstract_syntax_tree/objects/parameter.rs`
- `AstUnaryOperator` (enum) - `fireball/src/abstract_syntax_tree/objects/operator.rs`
- `AstBinaryOperator` (enum) - `fireball/src/abstract_syntax_tree/objects/operator.rs`
- `AstJumpTarget` (enum) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `AstCall` (enum) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `AstBuiltinFunctionArgument` (enum) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `AstBuiltinFunction` (enum) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `ArcAstFunctionMap` (type) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `ArcAstVariableMap` (type) - `fireball/src/abstract_syntax_tree/objects/etc.rs`
- `AstPrintConfig` - `fireball/src/abstract_syntax_tree/objects/print_config.rs`
- `AstOptimizationConfig` - `fireball/src/abstract_syntax_tree/objects/optimization.rs`
- `ProcessedOptimization` (enum) - `fireball/src/abstract_syntax_tree/objects/optimization.rs`
- `WrappedAstStatement` - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `Wrapped<T>` - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `AstDescriptor` - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `AstStatementOrigin` (enum) - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `AstValueOrigin` (enum) - `fireball/src/abstract_syntax_tree/objects/wrapper.rs`
- `PrintWithConfig` (trait) - `fireball/src/abstract_syntax_tree/traits.rs`
- `GetRelatedVariables` (trait) - `fireball/src/abstract_syntax_tree/traits.rs`
- `AstVariableAccessType` (enum) - `fireball/src/abstract_syntax_tree/traits.rs`

### AST optimize (`fireball/src/abstract_syntax_tree/optimize`)

- `AstPattern` - `fireball/src/abstract_syntax_tree/optimize/pattern_matching.rs`
- `AstPatternOrigin` (enum) - `fireball/src/abstract_syntax_tree/optimize/pattern_matching.rs`
- `AstPatternArgType` (enum) - `fireball/src/abstract_syntax_tree/optimize/pattern_matching.rs`

### Utils / errors (`fireball/src/utils`)

- `VersionMap<..>` - `fireball/src/utils/version_map.rs`
- `Aos<T>` (enum) - `fireball/src/utils/arc_or_static.rs`
- `FireballError` (enum) - `fireball/src/utils/error.rs`
- `DecompileError` (enum) - `fireball/src/utils/error/decompile_error.rs`
- `DisassembleError` (enum) - `fireball/src/utils/error/disassemble_error.rs`
- `IoError` (enum) - `fireball/src/utils/error/io_error.rs`
- `IrAnalyzeAssertionFailure` (enum) - `fireball/src/utils/error/ir_analyze_assertion_error.rs`

If you want to find another struct/enum/trait/type, search by regex (e.g. `rg "^\\s*pub\\s+(struct|enum|trait|type)\\s+Name"`).

## Testing

Due to the difficulty of setting up AST and IR environments, agents must refrain from writing internal tests for them and instead conduct testing using reversing results from tests/resources/hello_world.exe.
To verify and compare disassembly and decompiled outputs during testing, use the radare (`r2`) CLI.
After tests succeed, analyze log files to confirm the decompilation results are normal and match the expected output.
