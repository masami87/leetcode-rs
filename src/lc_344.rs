#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[test]
fn it_works() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];

    Solution::reverse_string(&mut s);

    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}
