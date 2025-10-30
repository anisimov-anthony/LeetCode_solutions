package p2404

import (
	"slices"
)

func mostFrequentEven(nums []int) int {
	freqs := make(map[int]int)

	for _, nm := range nums {
		if nm%2 == 0 {
			freqs[nm]++
		}
	}

	maxFreq := 0
	for _, fr := range freqs {
		maxFreq = max(maxFreq, fr)
	}

	if len(freqs) == 0 {
		return -1
	}

	candidates := make([]int, 0)
	for nm, fr := range freqs {
		if fr == maxFreq {
			candidates = append(candidates, nm)
		}
	}

	return slices.Min(candidates)
}
