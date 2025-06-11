# Pattern File Metadata
# Version: 1.0
# Author: Fireman Team
# Date: 2025-01-11
# Platform: generic
# Architecture: any

# String manipulation functions from the C standard library

FUNCTION: libc::strlen
  RETURNS: u64
  PARAM: str char*
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string

FUNCTION: libc::strcpy
  RETURNS: char*
  PARAM: dest char*
  PARAM: src const char*
  CONVENTION: cdecl
  BEHAVIOR: modifies_memory
  ATTRIBUTES: null_terminated_string, returns_dest

FUNCTION: libc::strncpy
  RETURNS: char*
  PARAM: dest char*
  PARAM: src const char*
  PARAM: n u64
  CONVENTION: cdecl
  BEHAVIOR: modifies_memory
  ATTRIBUTES: bounded_copy, returns_dest

FUNCTION: libc::strcat
  RETURNS: char*
  PARAM: dest char*
  PARAM: src const char*
  CONVENTION: cdecl
  BEHAVIOR: modifies_memory
  ATTRIBUTES: null_terminated_string, returns_dest

FUNCTION: libc::strcmp
  RETURNS: i32
  PARAM: s1 const char*
  PARAM: s2 const char*
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string

FUNCTION: libc::strncmp
  RETURNS: i32
  PARAM: s1 const char*
  PARAM: s2 const char*
  PARAM: n u64
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: bounded_compare

FUNCTION: libc::strchr
  RETURNS: char*
  PARAM: s const char*
  PARAM: c i32
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string, returns_pointer_or_null

FUNCTION: libc::strrchr
  RETURNS: char*
  PARAM: s const char*
  PARAM: c i32
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string, returns_pointer_or_null

FUNCTION: libc::strstr
  RETURNS: char*
  PARAM: haystack const char*
  PARAM: needle const char*
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string, returns_pointer_or_null

FUNCTION: libc::strtok
  RETURNS: char*
  PARAM: str char*
  PARAM: delim const char*
  CONVENTION: cdecl
  BEHAVIOR: modifies_memory
  ATTRIBUTES: null_terminated_string, stateful, not_thread_safe

# Test cases
TEST_CASE: strlen_detection
  INPUT_ASM:
    mov rdi, rax     ; string pointer
    call strlen
    mov rbx, rax     ; save result
  EXPECTED_MATCH: libc::strlen
  EXPECTED_CONFIDENCE: 95
END_TEST

TEST_CASE: strcpy_detection
  INPUT_ASM:
    mov rdi, rbx     ; dest
    mov rsi, rcx     ; src
    call strcpy
  EXPECTED_MATCH: libc::strcpy
  EXPECTED_CONFIDENCE: 95
END_TEST
