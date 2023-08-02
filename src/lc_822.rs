#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let n = fronts.len();

        let mut disabled = HashSet::new();

        let mut nums = HashSet::new();

        for i in 0..n {
            if fronts[i] == backs[i] {
                disabled.insert(fronts[i]);
                continue;
            }
            nums.insert(fronts[i]);
            nums.insert(backs[i]);
        }

        let res = nums
            .iter()
            .filter(|&x| !disabled.contains(x))
            .min()
            .map(|&x| x)
            .unwrap_or(0);
        res
    }
}

#[test]
fn it_works() {
    let fronts = [1, 2, 4, 4, 7];
    let backs = [1, 3, 4, 1, 3];

    let res = Solution::flipgame(fronts.to_vec(), backs.to_vec());
    assert_eq!(res, 2)
}
