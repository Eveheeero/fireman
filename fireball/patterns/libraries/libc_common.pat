# Common C library function patterns

FUNCTION: libc::malloc
  RETURNS: void*
  PARAM: size u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::free
  RETURNS: void
  PARAM: ptr void*
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::calloc
  RETURNS: void*
  PARAM: nmemb u64
  PARAM: size u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::realloc
  RETURNS: void*
  PARAM: ptr void*
  PARAM: size u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::strlen
  RETURNS: u64
  PARAM: s char*
  CONVENTION: systemv
  BEHAVIOR: pure

FUNCTION: libc::strcpy
  RETURNS: char*
  PARAM: dest char*
  PARAM: src char*
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::strncpy
  RETURNS: char*
  PARAM: dest char*
  PARAM: src char*
  PARAM: n u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::strcmp
  RETURNS: int
  PARAM: s1 char*
  PARAM: s2 char*
  CONVENTION: systemv
  BEHAVIOR: pure

FUNCTION: libc::strcat
  RETURNS: char*
  PARAM: dest char*
  PARAM: src char*
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::memcpy
  RETURNS: void*
  PARAM: dest void*
  PARAM: src void*
  PARAM: n u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::memset
  RETURNS: void*
  PARAM: s void*
  PARAM: c int
  PARAM: n u64
  CONVENTION: systemv
  BEHAVIOR: modifies_memory

FUNCTION: libc::printf
  RETURNS: int
  PARAM: format char*
  CONVENTION: systemv
  BEHAVIOR: io_write

FUNCTION: libc::scanf
  RETURNS: int
  PARAM: format char*
  CONVENTION: systemv
  BEHAVIOR: io_read

FUNCTION: libc::fopen
  RETURNS: void*
  PARAM: pathname char*
  PARAM: mode char*
  CONVENTION: systemv
  BEHAVIOR: io_read

FUNCTION: libc::fclose
  RETURNS: int
  PARAM: stream void*
  CONVENTION: systemv
  BEHAVIOR: io_write

FUNCTION: libc::fread
  RETURNS: u64
  PARAM: ptr void*
  PARAM: size u64
  PARAM: nmemb u64
  PARAM: stream void*
  CONVENTION: systemv
  BEHAVIOR: io_read

FUNCTION: libc::fwrite
  RETURNS: u64
  PARAM: ptr void*
  PARAM: size u64
  PARAM: nmemb u64
  PARAM: stream void*
  CONVENTION: systemv
  BEHAVIOR: io_write
