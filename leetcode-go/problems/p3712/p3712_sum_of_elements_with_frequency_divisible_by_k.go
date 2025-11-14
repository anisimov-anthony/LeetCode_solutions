package p3712

func sumDivisibleByK(nums []int, k int) int {
	freqs := make(map[int]int)
	for _, nm := range nums {
		freqs[nm]++
	}

	result := 0
	for elem, fr := range freqs {
		if fr%k == 0 {
			result += fr * elem
		}
	}

	return result
}
