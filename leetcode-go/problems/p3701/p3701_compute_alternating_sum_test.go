package p3701

import (
	"strconv"
	"testing"
)

func TestAlternatingSum(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := alternatingSum(tt.nums)
			if got != tt.want {
				t.Errorf("alternatingSum(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
