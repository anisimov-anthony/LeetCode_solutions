package p1281

import (
	"strconv"
	"testing"
)

func TestSubtractProductAndSum(t *testing.T) {
	tests := []struct {
		n    int
		want int
	}{
		{234, 15},
		{4421, 21},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := subtractProductAndSum(tt.n)
			if got != tt.want {
				t.Errorf("subtractProductAndSum(%v) = %v;\nwant %v", tt.n, got, tt.want)
			}
		})
	}
}
