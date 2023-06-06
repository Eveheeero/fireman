//! IR의 각 명령이 담겨져 있는 모듈

/// IR의 각 명령에 대한 Enum
#[derive(Debug, Clone, Copy)]
pub enum IRStatement {
    /// 해석할 수 없는 명령, 인라인 어셈블리로 처리됩니다.
    Unknown,
    /// 변수 할당
    Assignment,
    /// 명령 라인 변경
    Jump,
    /// 함수 호출
    Call,
    /// 함수 호출 후 반환
    Halt,
    /// 메모리 접근
    Touch,
    // Callback,
    // RememberReachingDefinitions
    // User
}
