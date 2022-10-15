use fireball::Fireball;

fn main() {
    let target = std::env::args().nth(1).unwrap();
    let fire = Fireball::from_path(&target).unwrap();
    dbg!(fire);
}
