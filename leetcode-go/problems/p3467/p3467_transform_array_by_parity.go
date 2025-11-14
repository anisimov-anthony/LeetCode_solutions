package p3467

func transformArray(nums []int) []int {
	evens := 0
	odds := 0
	for _, nm := range nums {
		if nm%2 == 0 {
			evens++
		} else {
			odds++
		}
	}

	result := make([]int, 0, evens+odds)
	for i := range evens + odds {
		if i < evens {
			result = append(result, 0)
		} else {
			result = append(result, 1)
		}
	}

	return result
}
