#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        let mut k = k;
        let mut res = 0;
        if (num_ones >= k) {
            return k;
        }
        res += num_ones;
        k = k - num_ones;
        if (num_zeros >= k) {
            return res;
        }
        k -= num_zeros;
        res -= k;
        res
    }
}

#[test]
fn it_works() {}
