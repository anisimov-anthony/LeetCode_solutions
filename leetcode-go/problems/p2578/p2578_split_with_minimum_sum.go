package p2578

import (
	"sort"
	"strconv"
)

func splitNum(num int) int {
	numStr := strconv.Itoa(num)
	digits := make([]byte, 0, len(numStr))
	for _, ch := range numStr {
		digits = append(digits, byte(ch))
	}

	sort.Slice(digits, func(i, j int) bool {
		return digits[i] <= digits[j]
	})

	var num1, num2 int
	for i, digit := range digits {
		d := int(digit - '0')
		if i%2 == 0 {
			num1 = num1*10 + d
		} else {
			num2 = num2*10 + d
		}
	}

	return num1 + num2
}
