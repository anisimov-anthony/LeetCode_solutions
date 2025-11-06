package p3210

import (
	"reflect"
	"strconv"
	"testing"
)

func TestLeftRightDifference(t *testing.T) {
	tests := []struct {
		s    string
		k    int
		want string
	}{
		{"dart", 3, "tdar"},
		{"aaa", 1, "aaa"},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := getEncryptedString(tt.s, tt.k)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("getEncryptedString(%v, %v) = %v;\nwant %v", tt.s, tt.k, got, tt.want)
			}
		})
	}
}
