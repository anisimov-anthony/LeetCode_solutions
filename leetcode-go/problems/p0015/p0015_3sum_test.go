package p0015

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestThreeSum(t *testing.T) {
	tests := []struct {
		input  []int
		result [][]int
	}{
		{
			[]int{-1, 0, 1, 2, -1, -4},
			[][]int{{-1, -1, 2}, {-1, 0, 1}},
		},
		{
			[]int{0, 1, 1},
			[][]int{},
		},
		{
			[]int{0, 0, 0},
			[][]int{{0, 0, 0}},
		},
	}

	for i, tc := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := threeSum(tc.input)
			assert.Equal(t, tc.result, got)
		})
	}
}
