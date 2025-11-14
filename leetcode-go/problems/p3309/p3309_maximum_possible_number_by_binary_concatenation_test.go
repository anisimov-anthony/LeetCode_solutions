package p3309

import (
	"strconv"
	"testing"
)

func TestMaxGoodNumber(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{1, 2, 3}, 30},
		{[]int{2, 8, 16}, 1296},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := maxGoodNumber(tt.nums)
			if got != tt.want {
				t.Errorf("maxGoodNumber(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
