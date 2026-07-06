# Fireman CLI & TUI

## CLI

### CLI Examples

```bash
fireman -i example.exe 
```

```bash
fireman -i example.exe --script myscript.fb --script myscript2.fb
```

```bash
fireman example.exe --json preset.json
```

```bash
fireman --jsonsample
```

### CLI Features

- [ ] Help menu
- [ ] Custom script support
- [ ] Json support

### CLI TODO

- [ ] Establish json structure
- [ ] Help menu
- [ ] Json preset support
- [ ] Print decompile result with args

## TUI

```bash
fireman --tui
```

```bash
fireman --tui example.exe
```

```bash
fireman --tui --json preset.json
```

```bash
fireman --tui --json preset.json -i example.exe
```

### TUI Features

- [ ] Load preset from json
- [ ] Navigate input path
- [ ] Tabs for manipulate decompile steps

### TUI TODO

- [ ] ???
