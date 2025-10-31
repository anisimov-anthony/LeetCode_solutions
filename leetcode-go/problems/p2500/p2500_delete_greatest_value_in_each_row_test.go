package p2500

import (
	"strconv"
	"testing"
)

func TestDeleteGreatestValue(t *testing.T) {
	tests := []struct {
		nums [][]int
		want int
	}{
		{[][]int{{1, 2, 4}, {3, 3, 1}}, 8},
		{[][]int{{10}}, 10},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := deleteGreatestValue(tt.nums)
			if got != tt.want {
				t.Errorf("deleteGreatestValue(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
