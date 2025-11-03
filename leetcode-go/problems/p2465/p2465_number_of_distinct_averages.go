package p2465

import "sort"

func distinctAverages(nums []int) int {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	left := 0
	right := len(nums) - 1
	averages := make(map[float64]bool)

	for left < right {
		average := float64(nums[left]+nums[right]) / 2
		averages[average] = true

		left++
		right--
	}

	return len(averages)
}
