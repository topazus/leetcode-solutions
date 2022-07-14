fn main() {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = 1;
        let mut t = 1;
        for i in 0..(nums.len() - 1) {
            if nums[i] != nums[i + 1] {
                nums[n as usize] = nums[i + 1];
                n += 1;
            } else {
                t += 1;
                if t == 2 {
                    nums[n as usize] = nums[i + 1];
                }
            }
        }
        return n;
    }
}
