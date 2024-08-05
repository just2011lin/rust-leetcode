// 86 合并两个有序数组

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    let len = (m + n) as usize;
    let mut nums = vec![0; len];
    let mut x = 0usize;
    let mut y = 0;
    loop {
        if x + y >= len {
            break;
        }
        // 1中的值取完了
        if x >= m {
            nums[x + y] = nums2[y];
            y += 1;
            continue;
        }
        // 2中的值取完了
        if y >= n {
            nums[x + y] = nums1[x];
            x += 1;
            continue;
        }
        if nums1[x] <= nums2[y] {
            nums[x + y] = nums1[x];
            x += 1;
            continue;
        } else {
            nums[x + y] = nums2[y];
            y += 1;
            continue;
        }
    }
    for i in 0..len {
        nums1[i] = nums[i];
    }
}
