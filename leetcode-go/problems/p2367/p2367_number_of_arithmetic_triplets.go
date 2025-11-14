package p2367

func arithmeticTriplets(nums []int, diff int) int {
	result := 0
	for i := range len(nums) {
		for j := i + 1; j < len(nums); j++ {
			for k := j + 1; k < len(nums); k++ {
				if nums[j]-nums[i] == diff && nums[k]-nums[j] == diff {
					result += 1
				} else if nums[j]-nums[i] > diff || nums[k]-nums[j] > diff {
					break
				}
			}
		}
	}

	return result
}
