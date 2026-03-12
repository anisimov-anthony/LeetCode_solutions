package p0165

import (
	"strconv"
	"strings"
)

func compareVersion(version1 string, version2 string) int {
	ver1 := strings.Split(version1, ".")
	ver2 := strings.Split(version2, ".")

	first := 0
	second := 0

	for first < len(ver1) && second < len(ver2) {
		firstElem := ver1[first]
		secondElem := ver2[second]

		firstNorm, _ := strconv.ParseInt(firstElem, 10, 64)
		secondNorm, _ := strconv.ParseInt(secondElem, 10, 64)

		if firstNorm > secondNorm {
			return 1
		} else if secondNorm > firstNorm {
			return -1
		}

		first++
		second++
	}

	for first < len(ver1) {
		firstElem := ver1[first]
		firstNorm, _ := strconv.ParseInt(firstElem, 10, 64)

		if firstNorm > 0 {
			return 1
		}

		first++
	}

	for second < len(ver2) {
		secondElem := ver2[second]
		secondNorm, _ := strconv.ParseInt(secondElem, 10, 64)

		if secondNorm > 0 {
			return -1
		}

		second++
	}

	return 0

}
