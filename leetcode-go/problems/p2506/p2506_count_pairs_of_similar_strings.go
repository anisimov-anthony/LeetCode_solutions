package p2506

import "sort"

func similarPairs(words []string) int {
	sims := make(map[string]int)

	for _, wd := range words {
		set := make(map[rune]bool)
		for _, rn := range wd {
			set[rn] = true
		}

		runes := make([]rune, 0, len(set))
		for r := range set {
			runes = append(runes, r)
		}

		sort.Slice(runes, func(i, j int) bool { return runes[i] < runes[j] })
		key := string(runes)

		sims[key]++
	}

	result := 0
	for _, v := range sims {
		if v > 1 {
			result += v * (v - 1) / 2
		}
	}
	return result
}
