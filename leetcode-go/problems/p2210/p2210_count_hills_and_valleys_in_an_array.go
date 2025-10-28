package problems

import "slices"

func countHillValley(nums []int) int {
	if len(nums) < 3 {
		return 0
	}

	nums = slices.Compact(nums)

	res := 0

	for i := 1; i < len(nums)-1; i++ {
		l := nums[i-1]
		r := nums[i+1]

		if (l < nums[i] && nums[i] > r) || (l > nums[i] && nums[i] < r) {
			res += 1
		}

	}

	return res
}
