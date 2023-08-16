#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut visit = vec![false; n as usize];

        let mut i = 1 as usize;
        let mut last = 0;
        visit[last] = true;
        loop {
            let step = k as usize * i;
            let next = (last + step) % n as usize;
            if visit[next] {
                break;
            }
            last = next;
            visit[last] = true;
            i += 1;
        }

        for i in 0..n {
            if !visit[i as usize] {
                res.push(i + 1);
            }
        }

        res
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
    assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
}
