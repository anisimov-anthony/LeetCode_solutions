package problems

import (
	"reflect"
	"strconv"
	"testing"
)

func TestLargestLocal(t *testing.T) {
	tests := []struct {
		nums [][]int
		want [][]int
	}{
		{[][]int{{9, 9, 8, 1}, {5, 6, 2, 6}, {8, 2, 6, 4}, {6, 2, 2, 2}}, [][]int{{9, 9}, {8, 6}}},
		{[][]int{{1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 2, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}}, [][]int{{2, 2, 2}, {2, 2, 2}, {2, 2, 2}}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := largestLocal(tt.nums)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("largestLocal(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
