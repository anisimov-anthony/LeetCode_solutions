package p1078

import (
	"reflect"
	"strconv"
	"testing"
)

func TestFindOcurrences(t *testing.T) {
	tests := []struct {
		text   string
		first  string
		second string
		want   []string
	}{
		{"alice is a good girl she is a good student", "a", "good", []string{"girl", "student"}},
		{"we will we will rock you", "we", "will", []string{"we", "rock"}},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := findOcurrences(tt.text, tt.first, tt.second)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("findOcurrences(%v, %v, %v) = %v;\nwant %v", tt.text, tt.first, tt.second, got, tt.want)
			}
		})
	}
}
