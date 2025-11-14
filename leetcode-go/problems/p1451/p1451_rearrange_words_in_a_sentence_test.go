package p1451

import (
	"strconv"
	"testing"
)

func TestArrangeWords(t *testing.T) {
	tests := []struct {
		text string
		want string
	}{
		{"Leetcode is cool", "Is cool leetcode"},
		{"Keep calm and code on", "On and keep calm code"},
		{"To be or not to be", "To be or to be not"},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := arrangeWords(tt.text)
			if got != tt.want {
				t.Errorf("arrangeWords(%v) = %v;\nwant %v", tt.text, got, tt.want)
			}
		})
	}
}
