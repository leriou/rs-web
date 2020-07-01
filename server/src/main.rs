mod leetcode;

use leetcode::solution::Solution;

fn main() {
    let c = Solution::longest_common_prefix(vec![
        String::from("hello"),
        String::from("heooo"),
        String::from("hec"),
        String::from("he"),
    ]);

    println!("{:?}", c);
}
