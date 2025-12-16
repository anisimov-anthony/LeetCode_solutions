package p1475

import (
	"reflect"
	"strconv"
	"testing"
)

func TestFinalPrices(t *testing.T) {
	tests := []struct {
		prices []int
		want   []int
	}{
		{[]int{8, 4, 6, 2, 3}, []int{4, 2, 4, 2, 3}},
		{[]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5}},
		{[]int{10, 1, 1, 6}, []int{9, 0, 1, 6}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := finalPrices(tt.prices)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("finalPrices(%v) = %v;\nwant %v", tt.prices, got, tt.want)
			}
		})
	}
}
