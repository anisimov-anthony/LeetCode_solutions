#[allow(dead_code)]
// https://keithschwarz.com/interesting/code/?dir=find-duplicate
// 24 hours for Knuth))))))
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0] as usize;
    let mut fast = nums[0] as usize;

    // Phase 1: Find intersection point of slow and fast pointers
    loop {
        slow = nums[slow] as usize; // Move one step
        fast = nums[nums[fast] as usize] as usize; // Move two steps
        if slow == fast {
            break;
        }
    }

    // Phase 2: Find entrance to the cycle
    let mut finder = nums[0] as usize;
    while slow != finder {
        slow = nums[slow] as usize; // Move one step
        finder = nums[finder] as usize; // Move one step
        if slow == finder {
            return slow as i32;
        }
    }

    slow as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_basic() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
