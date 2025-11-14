package p3270

import (
	"strconv"
	"testing"
)

func TestGenerateKey(t *testing.T) {
	tests := []struct {
		num1 int
		num2 int
		num3 int
		want int
	}{
		{1, 10, 1000, 0},
		{987, 879, 798, 777},
		{1, 2, 3, 1},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := generateKey(tt.num1, tt.num2, tt.num3)
			if got != tt.want {
				t.Errorf("generateKey(%v,%v,%v) = %v;\nwant %v", tt.num1, tt.num2, tt.num3, got, tt.want)
			}
		})
	}
}
