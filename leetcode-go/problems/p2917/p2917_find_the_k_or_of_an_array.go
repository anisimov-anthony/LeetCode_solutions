package p2917

func findKOr(nums []int, k int) int {
	digitsFreqs := [32]int{}
	for _, nm := range nums {
		for i := range 32 {
			shifted := nm >> i
			bit := shifted & 1
			digitsFreqs[i] += bit
		}
	}

	result := 0
	for dg, fr := range digitsFreqs {
		if fr >= k {
			result += (1 << dg)
		}
	}
	return result
}
