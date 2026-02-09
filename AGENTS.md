# Fireman AI Agent Guide

## Default

- Avoid `cargo test`, use `cargo check`

## Structure Paths (fireball)

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
