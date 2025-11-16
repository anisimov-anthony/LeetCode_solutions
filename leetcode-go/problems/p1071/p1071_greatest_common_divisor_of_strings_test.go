package p1071

import (
	"reflect"
	"strconv"
	"testing"
)

func TestGcdOfStrings(t *testing.T) {
	tests := []struct {
		str1 string
		str2 string
		want string
	}{
		{"ABCABC", "ABC", "ABC"},
		{"ABABAB", "ABAB", "AB"},
		{"LEET", "CODE", ""},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := gcdOfStrings(tt.str1, tt.str2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("gcdOfStrings(%v, %v) = %v;\nwant %v", tt.str1, tt.str2, got, tt.want)
			}
		})
	}
}
