package p2917

import (
	"strconv"
	"testing"
)

func TestFindKOr(t *testing.T) {
	tests := []struct {
		nums []int
		k    int
		want int
	}{}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := findKOr(tt.nums, tt.k)
			if got != tt.want {
				t.Errorf("findKOr(%v, %v) = %v;\nwant %v", tt.nums, tt.k, got, tt.want)
			}
		})
	}
}
