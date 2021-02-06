struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut r: Vec<Vec<String>> = Vec::new();
        r.push(vec!["".to_owned()]);
        r.push(vec!["()".to_owned()]);
        for i in 2..=n {
            let mut q: Vec<String> = Vec::new();
            for j in 0..i {
                for s in &r[j] {
                    for ss in &r[i - j - 1] {
                        q.push(format!("({}){}", s, ss));
                    }
                }
            }
            r.push(q);
        }
        r[n].clone()
    }
}

fn main() {
    for i in 1..=7 {
        println!("{:#?}", Solution::generate_parenthesis(i));
    }
}
