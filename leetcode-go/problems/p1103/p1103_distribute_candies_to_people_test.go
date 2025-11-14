package p1103

import (
	"reflect"
	"strconv"
	"testing"
)

func TestDistributeCandies(t *testing.T) {
	tests := []struct {
		candies    int
		num_people int
		want       []int
	}{
		{7, 4, []int{1, 2, 3, 1}},
		{10, 3, []int{5, 2, 3}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := distributeCandies(tt.candies, tt.num_people)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("distributeCandies(%v,%v) = %v;\nwant %v", tt.candies, tt.num_people, got, tt.want)
			}
		})
	}
}
