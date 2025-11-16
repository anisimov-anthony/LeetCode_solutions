package p1560

import "sort"

func mostVisited(n int, rounds []int) []int {
	var result []int
	start := rounds[0]
	end := rounds[len(rounds)-1]

	for start != end {
		result = append(result, start)
		start++
		if start > n {
			start -= n
		}
	}
	result = append(result, start)

	sort.Ints(result)
	return result
}
