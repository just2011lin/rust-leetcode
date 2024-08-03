// 两数之和

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let i = i.try_into().unwrap();
        let diff = target - num;
        if let Some(n) = h.get(&diff) {
            return vec![i, *n];
        } else {
            h.insert(num, i);
        }
    }
    vec![]
}
