package p3731

import "math"

func findMissingElements(nums []int) []int {
	minNumber := math.MaxInt
	maxNumber := math.MinInt
	containing := make(map[int]bool)
	for _, nm := range nums {
		if nm < minNumber {
			minNumber = nm
		}
		if nm > maxNumber {
			maxNumber = nm
		}
		containing[nm] = true
	}

	result := make([]int, 0, len(nums)-len(containing))
	for i := minNumber; i <= maxNumber; i++ {
		_, ok := containing[i]
		if !ok {
			result = append(result, i)
		}
	}

	return result
}
