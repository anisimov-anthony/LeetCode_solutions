package p0438

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindAnagrams(t *testing.T) {
	tests := []struct {
		s      string
		p      string
		result []int
	}{
		{
			"cbaebabacd",
			"abc",
			[]int{0, 6},
		},
		{
			"abab",
			"ab",
			[]int{0, 1, 2},
		},
	}

	for i, tc := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := findAnagrams(tc.s, tc.p)
			assert.Equal(t, tc.result, got)
		})
	}
}
