package p1122

import "sort"

func relativeSortArray(arr1 []int, arr2 []int) []int {
	freqs := make(map[int]int)
	for _, el := range arr1 {
		freqs[el]++
	}

	ptr := 0
	for _, el := range arr2 {
		for freqs[el] > 0 {
			arr1[ptr] = el
			freqs[el]--
			ptr++
		}
	}

	unmatched := make([]int, 0, len(arr1)-ptr)
	for el, count := range freqs {
		for count > 0 {
			unmatched = append(unmatched, el)
			count--
		}
	}

	sort.Slice(unmatched, func(i, j int) bool {
		return unmatched[i] < unmatched[j]
	})

	for _, el := range unmatched {
		arr1[ptr] = el
		ptr++
	}

	return arr1
}
