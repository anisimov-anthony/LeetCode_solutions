package p2404

import (
	"strconv"
	"testing"
)

func TestMostFrequentEven(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{0, 1, 2, 2, 4, 4, 1}, 2},
		{[]int{4, 4, 4, 9, 2, 4}, 4},
		{[]int{29, 47, 21, 41, 13, 37, 25, 7}, -1},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := mostFrequentEven(tt.nums)
			if got != tt.want {
				t.Errorf("mostFrequentEven(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
