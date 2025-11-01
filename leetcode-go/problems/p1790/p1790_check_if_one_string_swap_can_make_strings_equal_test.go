package p1790

import (
	"strconv"
	"testing"
)

func TestOddCells(t *testing.T) {
	tests := []struct {
		s1   string
		s2   string
		want bool
	}{
		{"bank", "kanb", true},
		{"attack", "defend", false},
		{"kelb", "kelb", true},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := areAlmostEqual(tt.s1, tt.s2)
			if got != tt.want {
				t.Errorf("areAlmostEqual(%v,%v) = %v;\nwant %v", tt.s1, tt.s2, got, tt.want)
			}
		})
	}
}
