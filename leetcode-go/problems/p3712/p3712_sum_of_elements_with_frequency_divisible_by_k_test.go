package p3712

import (
	"strconv"
	"testing"
)

func TestSumDivisibleByK(t *testing.T) {
	tests := []struct {
		nums []int
		k    int
		want int
	}{
		{[]int{1, 2, 2, 3, 3, 3, 3, 4}, 2, 16},
		{[]int{1, 2, 3, 4, 5}, 2, 0},
		{[]int{4, 4, 4, 1, 2, 3}, 3, 12},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := sumDivisibleByK(tt.nums, tt.k)
			if got != tt.want {
				t.Errorf("sumDivisibleByK(%v, %v) = %v;\nwant %v", tt.nums, tt.k, got, tt.want)
			}
		})
	}
}
