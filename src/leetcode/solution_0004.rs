//4.寻找两个正序数组的中位数
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::find_median_sorted_arrays;

    #[test]
    fn example01() {
        let (mut nums1, mut nums2) = (Vec::new(), Vec::new());
        nums1.push(1);
        nums1.push(3);
        nums2.push(2);

        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
        println!("example01: pass");
    }

    #[test]
    fn example02() {
        let (mut nums1, mut nums2) = (Vec::new(), Vec::new());
        nums1.push(1);
        nums1.push(2);
        nums2.push(3);
        nums2.push(4);

        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
        println!("example02: pass");
    }

}