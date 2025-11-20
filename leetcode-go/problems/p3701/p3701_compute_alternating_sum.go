package p3701

func alternatingSum(nums []int) int {
	result := 0
	for i, nm := range nums {
		if i%2 == 0 {
			result += nm
		} else {
			result -= nm
		}
	}
	return result
}
