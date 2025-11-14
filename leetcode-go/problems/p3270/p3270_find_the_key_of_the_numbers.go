package p3270

import (
	"math"
	"slices"
	"strconv"
)

func generateKey(num1 int, num2 int, num3 int) int {
	num1Str := strconv.Itoa(num1)
	num2Str := strconv.Itoa(num2)
	num3Str := strconv.Itoa(num3)

	digits := make(map[int][]int)

	for i := 0; i < 4; i++ {
		if i < len(num1Str) {
			digits[i] = append(digits[i], int(num1Str[len(num1Str)-i-1]-'0'))
		} else {
			digits[i] = append(digits[i], 0)
		}
	}

	for i := 0; i < 4; i++ {
		if i < len(num2Str) {
			digits[i] = append(digits[i], int(num2Str[len(num2Str)-i-1]-'0'))
		} else {
			digits[i] = append(digits[i], 0)
		}
	}

	for i := 0; i < 4; i++ {
		if i < len(num3Str) {
			digits[i] = append(digits[i], int(num3Str[len(num3Str)-i-1]-'0'))
		} else {
			digits[i] = append(digits[i], 0)
		}
	}

	result := 0
	for i, digits := range digits {
		minimum := slices.Min(digits)

		result += int(math.Pow(float64(10), float64(i))) * minimum
	}

	return result
}
