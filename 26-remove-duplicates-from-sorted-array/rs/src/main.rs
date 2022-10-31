
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut cur = 0;

        loop {
            if nums[cur] == nums[cur + 1] {
                nums.remove(cur);
            } else {
                cur += 1;
            }

            if cur + 1 == nums.len() {
                break;
            }
        }

      

        nums.len() as i32
    }
}

fn main() {}

// 1 1 2 3 4 4 4 5 6 8 8 9 10
// 10 1 2 3 9 8 5 6 8

// 1 1 2 3 4 4 4 5 6 8 8 9 10
// *       * *       *
