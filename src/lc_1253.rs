#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; colsum.len()]; 2];

        let n = colsum.len();

        let mut upper = upper;
        let mut lower = lower;

        let cnt_2 = colsum.iter().filter(|&&x| x == 2).count() as i32;

        upper -= cnt_2;
        lower -= cnt_2;

        if upper < 0 || lower < 0 {
            return vec![];
        }

        if upper + lower != colsum.iter().filter(|&&x| x == 1).count() as i32 {
            return vec![];
        }

        for i in 0..n {
            if colsum[i] == 2 {
                res[0][i] = 1;
                res[1][i] = 1;
            } else if colsum[i] == 1 {
                if upper > 0 {
                    res[0][i] = 1;
                    upper -= 1;
                } else {
                    res[1][i] = 1;
                    lower -= 1;
                }
            }
        }

        res
    }
}

#[test]
fn it_works() {
    let res = Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]);
    assert_eq!(
        res,
        vec![
            vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
        ]
    );
}
