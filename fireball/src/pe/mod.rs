/// PE파일 파서
pub struct PE {
    /// 파일 경로
    path: Option<String>,
    /// 바이너리
    binary: Vec<u8>,
}

/// 코어 트레이트에 대한 구현이 담겨있는 모듈
mod fire;

/// 출력에 대한 구현이 담겨있는 모듈
mod fmt;
