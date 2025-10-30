package problems

import (
	"reflect"
	"strconv"
	"testing"
)

func TestMergeArrays(t *testing.T) {
	tests := []struct {
		nums1 [][]int
		nums2 [][]int
		want  [][]int
	}{
		{[][]int{{1, 2}, {2, 3}, {4, 5}}, [][]int{{1, 4}, {3, 2}, {4, 1}}, [][]int{{1, 6}, {2, 3}, {3, 2}, {4, 6}}},
		{[][]int{{2, 4}, {3, 6}, {5, 5}}, [][]int{{1, 3}, {4, 3}}, [][]int{{1, 3}, {2, 4}, {3, 6}, {4, 3}, {5, 5}}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := mergeArrays(tt.nums1, tt.nums2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("mergeArrays(%v, %v) = %v;\nwant %v", tt.nums1, tt.nums2, got, tt.want)
			}
		})
	}
}
