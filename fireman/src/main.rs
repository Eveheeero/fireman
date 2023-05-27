use clap::Parser;
use fireball::Fireball;

/// 디컴파일러 CLI 도구입니다.
/// 현재 개발중입니다.
#[derive(Parser)]
struct Args {
    /// 파일 경로
    #[arg(short = 'i', long = "input", value_name = "PATH")]
    input_path: String,

    /// 설정값 경로
    #[arg(short = 'j', long = "json", value_name = "PATH")]
    json_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let fire = Fireball::from_path(&args.input_path).unwrap();
    dbg!(fire);
}
