package p1961

import "strings"

func isPrefixString(s string, words []string) bool {
	var sb strings.Builder
	for _, w := range words {
		if sb.Len()+len(w) > len(s) {
			break
		}
		sb.WriteString(w)
		if sb.Len() == len(s) {
			return sb.String() == s
		}
	}
	return false
}
