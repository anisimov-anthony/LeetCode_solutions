package p2206

func divideArray(nums []int) bool {
	freqs := make(map[int]int)

	for _, nm := range nums {
		freqs[nm]++
	}

	for _, fr := range freqs {
		if fr%2 != 0 {
			return false
		}
	}
	return true
}
