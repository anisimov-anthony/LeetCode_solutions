package p2367

import (
	"strconv"
	"testing"
)

func TestArithmeticTriplets(t *testing.T) {
	tests := []struct {
		nums []int
		diff int
		want int
	}{
		{[]int{0, 1, 4, 6, 7, 10}, 3, 2},
		{[]int{4, 5, 6, 7, 8, 9}, 2, 2},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := arithmeticTriplets(tt.nums, tt.diff)
			if got != tt.want {
				t.Errorf("arithmeticTriplets(%v, %v) = %v;\nwant %v", tt.nums, tt.diff, got, tt.want)
			}
		})
	}
}
