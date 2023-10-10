//1.两数之和
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    
    nums.into_iter()
        .enumerate()
        .find_map(|(idx, e)| match map.get(&(target - e)) {
            Some(i) => Some(vec![*i, idx as i32]),
            None => {
                map.insert(e, idx as i32);
                None
            }
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solution_0001::two_sum;

    #[test]
    fn example01() {
        let mut nums = Vec::new();
        nums.push(2);
        nums.push(7);
        nums.push(11);
        nums.push(15);
        let target = 9;

        assert_eq!(two_sum(nums, target), [0, 1]);
        println!("example01: pass");
    }

    #[test]
    fn example02() {
        let mut nums = Vec::new();
        nums.push(3);
        nums.push(2);
        nums.push(4);
        let target = 6;

        assert_eq!(two_sum(nums, target), [1, 2]);
        println!("example02: pass");
    }

    #[test]
    fn example03() {
        let mut nums = Vec::new();
        nums.push(3);
        nums.push(3);
        let target = 6;

        assert_eq!(two_sum(nums, target), [0, 1]);
        println!("example03: pass");
    }

}
