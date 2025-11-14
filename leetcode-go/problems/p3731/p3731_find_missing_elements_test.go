package p3731

import (
	"reflect"
	"strconv"
	"testing"
)

func TestFindMissingElements(t *testing.T) {
	tests := []struct {
		nums []int
		want []int
	}{
		{[]int{1, 4, 2, 5}, []int{3}},
		{[]int{7, 8, 6, 9}, []int{}},
		{[]int{5, 1}, []int{2, 3, 4}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := findMissingElements(tt.nums)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("findMissingElements(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
