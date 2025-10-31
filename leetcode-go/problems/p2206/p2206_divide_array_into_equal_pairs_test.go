package p2206

import (
	"strconv"
	"testing"
)

func TestDivideArray(t *testing.T) {
	tests := []struct {
		nums []int
		want bool
	}{
		{[]int{3, 2, 3, 2, 2, 2}, true},
		{[]int{1, 2, 3, 4}, false},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := divideArray(tt.nums)
			if got != tt.want {
				t.Errorf("divideArray(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
