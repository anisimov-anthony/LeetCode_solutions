package p1252

import (
	"strconv"
	"testing"
)

func TestOddCells(t *testing.T) {
	tests := []struct {
		m       int
		n       int
		indices [][]int
		want    int
	}{
		{2, 3, [][]int{{0, 1}, {1, 1}}, 6},
		{2, 2, [][]int{{1, 1}, {0, 0}}, 0},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := oddCells(tt.m, tt.n, tt.indices)
			if got != tt.want {
				t.Errorf("oddCells(%v,%v,%v) = %v;\nwant %v", tt.m, tt.n, tt.indices, got, tt.want)
			}
		})
	}
}
