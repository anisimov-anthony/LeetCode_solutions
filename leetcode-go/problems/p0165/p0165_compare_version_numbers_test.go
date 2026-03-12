package p0165

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCompareVersion(t *testing.T) {
	tests := []struct {
		version1 string
		version2 string
		result   int
	}{
		{
			"1.2",
			"1.10",
			-1,
		},
		{
			"1.01",
			"1.001",
			0,
		},
		{
			"1.0",
			"1.0.0.0",
			0,
		},
	}

	for i, tt := range tests {
		t.Run(strconv.Itoa(i), func(t *testing.T) {
			got := compareVersion(tt.version1, tt.version2)
			assert.Equal(t, tt.result, got)
		})
	}
}
