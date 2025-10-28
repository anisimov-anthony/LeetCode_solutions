package problems

import (
	"strings"
)

func uncommonFromSentences(s1 string, s2 string) []string {
	words1 := strings.Split(s1, " ")
	words2 := strings.Split(s2, " ")
	result := make([]string, 0, len(words1)+len(words2))

	freqs := make(map[string][2]int)

	for _, wd1 := range words1 {
		arr := freqs[wd1]
		arr[0]++
		freqs[wd1] = arr
	}

	for _, wd2 := range words2 {
		arr := freqs[wd2]
		arr[1]++
		freqs[wd2] = arr
	}

	for wd, arr := range freqs {
		if (arr[0] == 1 && arr[1] == 0) || (arr[0] == 0 && arr[1] == 1) {
			result = append(result, wd)
		}
	}

	return result
}
