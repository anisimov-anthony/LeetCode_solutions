package p2697

import (
	"reflect"
	"strconv"
	"testing"
)

func TestMakeSmallestPalindrome(t *testing.T) {
	tests := []struct {
		s    string
		want string
	}{
		{"egcfe", "efcfe"},
		{"abcd", "abba"},
		{"seven", "neven"},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := makeSmallestPalindrome(tt.s)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("makeSmallestPalindrome(%v) = %v;\nwant %v", tt.s, got, tt.want)
			}
		})
	}
}
