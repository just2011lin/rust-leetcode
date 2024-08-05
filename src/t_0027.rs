// 27 移除元素

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();

    let mut ns = vec![0; len];

    let mut j = 0;

    for i in 0..len {
        if nums[i] != val {
            ns[j] = nums[i];
            j += 1;
        }
    }
    for i in 0..j {
        nums[i] = ns[i]
    }
    j as i32
}
