package p2578

import (
	"strconv"
	"testing"
)

func TestSplitNum(t *testing.T) {
	tests := []struct {
		num  int
		want int
	}{
		{4325, 59},
		{687, 75},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := splitNum(tt.num)
			if got != tt.want {
				t.Errorf("splitNum(%v) = %v;\nwant %v", tt.num, got, tt.want)
			}
		})
	}
}
