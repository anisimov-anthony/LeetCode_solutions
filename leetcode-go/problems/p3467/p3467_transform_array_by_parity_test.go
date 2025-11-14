package p3467

import (
	"reflect"
	"strconv"
	"testing"
)

func TestTransformArray(t *testing.T) {
	tests := []struct {
		nums []int
		want []int
	}{
		{[]int{4, 3, 2, 1}, []int{0, 0, 1, 1}},
		{[]int{1, 5, 1, 4, 2}, []int{0, 0, 1, 1, 1}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := transformArray(tt.nums)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("transformArray(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
