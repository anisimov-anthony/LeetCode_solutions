package problems

import (
	"strconv"
	"testing"
)

func TestCountHillValley(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{2, 4, 1, 1, 6, 5}, 3},
		{[]int{6, 6, 5, 5, 4, 1}, 0},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := countHillValley(tt.nums)
			if got != tt.want {
				t.Errorf("countHillValley(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
