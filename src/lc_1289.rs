#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut first_num = grid[0].iter().min().unwrap().to_owned();
        let mut first_idx = grid[0].iter().position(|&x| x == first_num).unwrap();

        let mut second_num = grid[0]
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx != &first_idx)
            .min()
            .unwrap_or((n, &i32::MAX))
            .1
            .to_owned();

        for i in 1..n {
            let mut cur_first_num = i32::MAX;
            let mut cur_first_idx = n;
            let mut cur_second_num = i32::MAX;
            for j in 0..n {
                let cur = match j == first_idx {
                    true => second_num + grid[i][j],
                    false => first_num + grid[i][j],
                };

                if cur < cur_first_num {
                    cur_second_num = cur_first_num;
                    cur_first_num = cur;
                    cur_first_idx = j;
                } else if cur < cur_second_num {
                    cur_second_num = cur;
                }
            }

            first_num = cur_first_num;
            first_idx = cur_first_idx;
            second_num = cur_second_num;
        }

        first_num
    }
}

#[test]
fn it_works() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let res = Solution::min_falling_path_sum(grid);

    assert_eq!(res, 13);
}
