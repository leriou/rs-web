use utils::tools;

mod corner;
use corner::inner;

fn main() {
    inner::test::test();
    println!("tools -> {}", tools::test());
}
