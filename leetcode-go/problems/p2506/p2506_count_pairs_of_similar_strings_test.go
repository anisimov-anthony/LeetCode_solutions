package p2506

import (
	"strconv"
	"testing"
)

func TestSimilarPairs(t *testing.T) {
	tests := []struct {
		words []string
		want  int
	}{
		{[]string{"aba", "aabb", "abcd", "bac", "aabc"}, 2},
		{[]string{"aabb", "ab", "ba"}, 3},
		{[]string{"nba", "cba", "dba"}, 0},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := similarPairs(tt.words)
			if got != tt.want {
				t.Errorf("similarPairs(%v) = %v;\nwant %v", tt.words, got, tt.want)
			}
		})
	}
}
