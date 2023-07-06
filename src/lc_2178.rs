#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum <= 0 || final_sum % 2 == 1 {
            return vec![];
        }
        let mut res = vec![];
        let mut final_sum = final_sum;

        for i in (2..final_sum).step_by(2) {
            if final_sum - i <= i {
                break;
            }
            res.push(i);
            final_sum -= i;
        }
        if final_sum > 0 {
            res.push(final_sum);
        }
        res
    }
}

#[test]
fn it_works() {
    let res = Solution::maximum_even_split(28);
    assert_eq!(res, vec![2, 4, 6, 16])
}
