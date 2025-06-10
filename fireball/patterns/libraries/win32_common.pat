# Common Windows API function patterns

FUNCTION: kernel32::GetModuleHandleA
  RETURNS: void*
  PARAM: lpModuleName char*
  CONVENTION: stdcall
  BEHAVIOR: pure

FUNCTION: kernel32::LoadLibraryA
  RETURNS: void*
  PARAM: lpLibFileName char*
  CONVENTION: stdcall
  BEHAVIOR: modifies_memory

FUNCTION: kernel32::GetProcAddress
  RETURNS: void*
  PARAM: hModule void*
  PARAM: lpProcName char*
  CONVENTION: stdcall
  BEHAVIOR: pure

FUNCTION: kernel32::VirtualAlloc
  RETURNS: void*
  PARAM: lpAddress void*
  PARAM: dwSize u32
  PARAM: flAllocationType u32
  PARAM: flProtect u32
  CONVENTION: stdcall
  BEHAVIOR: modifies_memory

FUNCTION: kernel32::VirtualFree
  RETURNS: int
  PARAM: lpAddress void*
  PARAM: dwSize u32
  PARAM: dwFreeType u32
  CONVENTION: stdcall
  BEHAVIOR: modifies_memory

FUNCTION: kernel32::CreateFileA
  RETURNS: void*
  PARAM: lpFileName char*
  PARAM: dwDesiredAccess u32
  PARAM: dwShareMode u32
  PARAM: lpSecurityAttributes void*
  PARAM: dwCreationDisposition u32
  PARAM: dwFlagsAndAttributes u32
  PARAM: hTemplateFile void*
  CONVENTION: stdcall
  BEHAVIOR: io_write

FUNCTION: kernel32::ReadFile
  RETURNS: int
  PARAM: hFile void*
  PARAM: lpBuffer void*
  PARAM: nNumberOfBytesToRead u32
  PARAM: lpNumberOfBytesRead u32*
  PARAM: lpOverlapped void*
  CONVENTION: stdcall
  BEHAVIOR: io_read

FUNCTION: kernel32::WriteFile
  RETURNS: int
  PARAM: hFile void*
  PARAM: lpBuffer void*
  PARAM: nNumberOfBytesToWrite u32
  PARAM: lpNumberOfBytesWritten u32*
  PARAM: lpOverlapped void*
  CONVENTION: stdcall
  BEHAVIOR: io_write

FUNCTION: kernel32::CloseHandle
  RETURNS: int
  PARAM: hObject void*
  CONVENTION: stdcall
  BEHAVIOR: modifies_memory

FUNCTION: kernel32::GetLastError
  RETURNS: u32
  CONVENTION: stdcall
  BEHAVIOR: pure

FUNCTION: user32::MessageBoxA
  RETURNS: int
  PARAM: hWnd void*
  PARAM: lpText char*
  PARAM: lpCaption char*
  PARAM: uType u32
  CONVENTION: stdcall
  BEHAVIOR: io_write

FUNCTION: user32::CreateWindowExA
  RETURNS: void*
  PARAM: dwExStyle u32
  PARAM: lpClassName char*
  PARAM: lpWindowName char*
  PARAM: dwStyle u32
  PARAM: X int
  PARAM: Y int
  PARAM: nWidth int
  PARAM: nHeight int
  PARAM: hWndParent void*
  PARAM: hMenu void*
  PARAM: hInstance void*
  PARAM: lpParam void*
  CONVENTION: stdcall
  BEHAVIOR: modifies_memory

FUNCTION: user32::ShowWindow
  RETURNS: int
  PARAM: hWnd void*
  PARAM: nCmdShow int
  CONVENTION: stdcall
  BEHAVIOR: io_write

FUNCTION: user32::GetMessageA
  RETURNS: int
  PARAM: lpMsg void*
  PARAM: hWnd void*
  PARAM: wMsgFilterMin u32
  PARAM: wMsgFilterMax u32
  CONVENTION: stdcall
  BEHAVIOR: io_read

FUNCTION: user32::DispatchMessageA
  RETURNS: int
  PARAM: lpMsg void*
  CONVENTION: stdcall
  BEHAVIOR: io_write
