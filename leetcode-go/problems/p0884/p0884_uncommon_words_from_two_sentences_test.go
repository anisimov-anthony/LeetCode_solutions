package problems

import (
	"reflect"
	"sort"
	"strconv"
	"testing"
)

func TestUncommonFromSentences(t *testing.T) {
	tests := []struct {
		s1   string
		s2   string
		want []string
	}{
		{
			s1:   "this apple is sweet",
			s2:   "this apple is sour",
			want: []string{"sweet", "sour"},
		},
		{
			s1:   "apple apple",
			s2:   "banana",
			want: []string{"banana"},
		},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := uncommonFromSentences(tt.s1, tt.s2)

			sort.Strings(got)
			sort.Strings(tt.want)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("uncommonFromSentences(%q, %q) = %v; \nwant %v", tt.s1, tt.s2, got, tt.want)
			}
		})
	}
}
