struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut best_i = 0;
        let mut best_l = 0;
        let chs: Vec<char> = s.chars().collect();
        let n = chs.len();
        for i in 0..n {
            'l: for l in (best_l + 1)..=(n - i) {
                for j in 0..(l / 2) {
                    if chs[i + j] != chs[i + l - 1 - j] {
                        continue 'l;
                    }
                }
                println!("{:#?}", &chs[i..(i + l)]);
                best_l = l;
                best_i = i;
            }
        }
        chs.iter().skip(best_i).take(best_l).collect()
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome(std::env::args().nth(1).unwrap())
    )
}
