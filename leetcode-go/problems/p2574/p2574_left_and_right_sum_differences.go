package problems

import "math"

func leftRightDifference(nums []int) []int {
	res := make([]int, len(nums))

	l := 0
	r := 0
	for _, nm := range nums {
		r += nm
	}

	for i := range len(nums) {
		if i == 0 {
			l = 0
			r -= nums[0]
		} else if i == len(nums)-1 {
			l += nums[i-1]
			r = 0
		} else {
			l += nums[i-1]
			r -= nums[i]

		}
		res[i] = int(math.Abs(float64(l) - float64(r)))

	}
	return res
}
