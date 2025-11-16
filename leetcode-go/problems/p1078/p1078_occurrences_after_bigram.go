package p1078

import (
	"strings"
)

func findOcurrences(text string, first string, second string) []string {
	words := strings.Split(text, " ")
	result := make([]string, 0, len(text)/2)
	for i := 0; i < len(words)-2; i += 1 {
		if words[i] == first && words[i+1] == second {
			result = append(result, words[i+2])
		}
	}

	return result
}
