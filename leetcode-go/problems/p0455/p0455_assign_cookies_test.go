package p0455

import "testing"

func TestBasics(t *testing.T) {
	tests := []struct {
		g      []int
		s      []int
		result int
	}{
		{
			[]int{1, 2, 3},
			[]int{1, 1},
			1,
		},
		{
			[]int{1, 2},
			[]int{1, 2, 3},
			2,
		},
	}

	for i, test := range tests {
		output := findContentChildren(test.g, test.s)
		if output != test.result {
			t.Errorf("for test %d expected %v, got %v", i, test.result, output)
		}
	}
}
