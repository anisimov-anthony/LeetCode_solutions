package p1071

import (
	"strings"
)

func gcdOfStrings(str1 string, str2 string) string {
	result := ""

	for slen := len(str2) - 1; slen >= 0; slen-- {
		candidate := str2[:slen+1]

		if divides(str1, candidate) && divides(str2, candidate) {
			if len(candidate) > len(result) {
				result = candidate
			}
		}
	}

	return result
}

func divides(s, t string) bool {
	if len(t) == 0 || len(s)%len(t) != 0 {
		return false
	}
	factor := len(s) / len(t)
	return strings.Repeat(t, factor) == s
}
