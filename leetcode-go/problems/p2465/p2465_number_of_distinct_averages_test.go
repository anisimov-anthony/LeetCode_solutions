package p2465

import (
	"strconv"
	"testing"
)

func TestDistinctAverages(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{4, 1, 4, 0, 3, 5}, 2},
		{[]int{1, 100}, 1},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := distinctAverages(tt.nums)
			if got != tt.want {
				t.Errorf("distinctAverages(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
