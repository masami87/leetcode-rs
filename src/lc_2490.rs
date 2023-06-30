#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split(" ").collect::<Vec<_>>();
        if words[0].chars().next().unwrap() != words[words.len() - 1].chars().last().unwrap() {
            return false;
        }

        for i in 1..words.len() {
            if words[i].chars().next().unwrap() != words[i - 1].chars().last().unwrap() {
                return false;
            }
        }
        true
    }
}

#[test]
fn it_works() {
    let sentence = "leetcode exercises sound delightful";
    let res = Solution::is_circular_sentence(sentence.to_string());

    assert_eq!(res, true);
}
