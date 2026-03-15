package p0167

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	tests := []struct {
		input  []int
		target int
		result []int
	}{
		{
			[]int{2, 7, 11, 15},
			9,
			[]int{1, 2},
		},
		{
			[]int{2, 3, 4},
			6,
			[]int{1, 3},
		},
		{
			[]int{-1, 0},
			-1,
			[]int{1, 2},
		},
	}

	for i, tc := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := twoSum(tc.input, tc.target)
			assert.Equal(t, tc.result, got)
		})
	}
}
