package p1560

import (
	"reflect"
	"strconv"
	"testing"
)

func TestMostVisited(t *testing.T) {
	tests := []struct {
		n      int
		rounds []int
		want   []int
	}{
		{4, []int{1, 3, 1, 2}, []int{1, 2}},
		{2, []int{2, 1, 2, 1, 2, 1, 2, 1, 2}, []int{2}},
		{7, []int{1, 3, 5, 7}, []int{1, 2, 3, 4, 5, 6, 7}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := mostVisited(tt.n, tt.rounds)
			if !reflect.DeepEqual(tt.want, got) {
				t.Errorf("mostVisited(%v, %v) = %v;\nwant %v", tt.n, tt.rounds, got, tt.want)
			}
		})
	}
}
