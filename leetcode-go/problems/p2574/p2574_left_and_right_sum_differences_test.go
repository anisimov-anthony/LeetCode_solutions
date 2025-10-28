package problems

import (
	"reflect"
	"strconv"
	"testing"
)

func TestLeftRightDifference(t *testing.T) {
	tests := []struct {
		nums []int
		want []int
	}{
		{[]int{10, 4, 8, 3}, []int{15, 1, 11, 22}},
		{[]int{1}, []int{0}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := leftRightDifference(tt.nums)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("leftRightDifference(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
