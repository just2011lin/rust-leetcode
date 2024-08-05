// 86 合并两个有序数组

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let len = (m + n) as usize;

    for i in 0..len {
        let i = len - 1 - i;
        if m == 0 {
            nums1[i] = nums2[n - 1];
            n -= 1;
            continue;
        }
        if n == 0 {
            break;
        }
        if nums2[n - 1] >= nums1[m - 1] {
            nums1[i] = nums2[n - 1];
            n -= 1;
        } else {
            nums1[i] = nums1[m - 1];
            m -= 1;
        }
    }
}
