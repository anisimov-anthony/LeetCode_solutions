package p1347

import (
	"strconv"
	"testing"
)

func TestMinSteps(t *testing.T) {
	tests := []struct {
		s    string
		t    string
		want int
	}{
		{"bab", "aba", 1},
		{"leetcode", "practice", 5},
		{"anagram", "mangaar", 0},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := minSteps(tt.s, tt.t)
			if got != tt.want {
				t.Errorf("minSteps(%v, %v) = %v;\nwant %v", tt.s, tt.t, got, tt.want)
			}
		})
	}
}
