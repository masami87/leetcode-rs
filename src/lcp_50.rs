#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        let mut gem = gem;

        for op in operations {
            let (x, y) = (op[0] as usize, op[1] as usize);

            let d = gem[x] / 2;

            gem[x] -= d;
            gem[y] += d;
        }
        let m = gem.iter().cloned().max().unwrap();
        let s = gem.iter().cloned().min().unwrap();
        m - s
    }
}

#[test]
fn it_works() {}
