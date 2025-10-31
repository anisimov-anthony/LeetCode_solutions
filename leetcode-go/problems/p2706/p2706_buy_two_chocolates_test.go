package p2706

import (
	"strconv"
	"testing"
)

func TestBuyChoco(t *testing.T) {
	tests := []struct {
		prices []int
		money  int
		want   int
	}{
		{[]int{1, 2, 2}, 3, 0},
		{[]int{3, 2, 3}, 3, 3},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := buyChoco(tt.prices, tt.money)
			if got != tt.want {
				t.Errorf("buyChoco(%v, %v) = %v;\nwant %v", tt.prices, tt.money, got, tt.want)
			}
		})
	}
}
