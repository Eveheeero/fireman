use fireball::Fireball;

use clap::Parser;

/// 디컴파일러 CLI 도구입니다.
/// 현재 개발중입니다.
#[derive(Parser)]
struct Args {
    /// 파일 경로
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let fire = Fireball::from_path(&args.path).unwrap();
    dbg!(fire);
}
