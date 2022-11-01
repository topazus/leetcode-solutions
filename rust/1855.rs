// brutal for loop
pub fn max_distance1(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums1.len() {
        for j in i..nums2.len() {
            if nums1[i] <= nums2[j] {
                // println!("{},{}", i, j);
                res = res.max(j - i);
            } else {
                break;
            }
        }
    }
    res as i32
}
pub fn max_distance1_(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums1.len() {
        let mut j = i;
        while j < nums2.len() {
            if nums1[i] <= nums2[j] {
                // println!("{},{}", i, j);
                res = res.max(j - i);
                j += 1;
            } else {
                break;
            }
        }
    }
    res as i32
}
// binary search
pub fn max_distance2(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums1.len() {
        let mut left = i;
        let mut right = nums2.len() - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            if nums1[i] <= nums2[mid] {
                left = mid;
            } else {
                right = mid - 1;
            }
            res = res.max(left - i);
        }
    }
    res as i32
}

pub fn max_distance3(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    for (i, v) in nums1.iter().enumerate() {
        let mut left = i;
        let mut right = nums2.len() - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            if *v <= nums2[mid] {
                left = mid;
            } else {
                right = mid - 1;
            }
            res = res.max(left - i);
        }
    }
    res as i32
}
pub fn max_distance4(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            res = res.max(j - i);
            j += 1;
        } else {
            i += 1;
            j = j.max(i);
        }
    }
    res as i32
}
fn main() {
    let nums1 = vec![
        9820, 8937, 7936, 4855, 4830, 4122, 2327, 1342, 1167, 815, 414,
    ];
    let nums2 = vec![
        9889, 9817, 9800, 9777, 9670, 9646, 9304, 8977, 8974, 8802, 8626, 8622, 8456,
    ];
    assert_eq!(max_distance1(nums1, nums2), 10);
    assert_eq!(max_distance2(nums1, nums2), 10);
    assert_eq!(max_distance3(nums1, nums2), 10);
}
