package p2460

func applyOperations(nums []int) []int {
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == nums[i+1] {
			nums[i] *= 2
			nums[i+1] = 0
		}
	}

	result := make([]int, len(nums))
	localPtr := 0
	for _, nm := range nums {
		if nm != 0 {
			result[localPtr] = nm
			localPtr += 1
		}
	}

	return result
}
