package p2460

import (
	"reflect"
	"strconv"
	"testing"
)

func TestApplyOperations(t *testing.T) {
	tests := []struct {
		nums []int
		want []int
	}{}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := applyOperations(tt.nums)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("applyOperations(%v) = %v;\nwant %v", tt.nums, got, tt.want)
			}
		})
	}
}
