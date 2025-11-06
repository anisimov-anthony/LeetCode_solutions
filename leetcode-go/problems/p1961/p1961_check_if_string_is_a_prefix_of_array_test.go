package p1961

import (
	"strconv"
	"testing"
)

func TestIsPrefixString(t *testing.T) {
	tests := []struct {
		s1   string
		s2   []string
		want bool
	}{
		{"iloveleetcode", []string{"i", "love", "leetcode", "apples"}, true},
		{"iloveleetcode", []string{"apples", "i", "love", "leetcode"}, false},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := isPrefixString(tt.s1, tt.s2)
			if got != tt.want {
				t.Errorf("isPrefixString(%v,%v) = %v;\nwant %v", tt.s1, tt.s2, got, tt.want)
			}
		})
	}
}
