// 26 删除有序数组中的重复项

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut fast = 1;
    let mut slow = 1;
    while fast < len {
        if nums[fast] != nums[fast - 1] {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}
