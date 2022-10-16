/// 파서 모듈에 대한 트레이트
pub trait Fire {
    /// 파일 경로를 기반으로 파서 객체를 생성한다.
    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 바이너리를 기반으로 파서 객체를 생성한다.
    fn from_binary(binary: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 파일 경로를 반환한다.
    fn get_path(&self) -> Option<String>;

    /// 바이너리를 반환한다.
    fn get_binary(&self) -> &Vec<u8>;
}