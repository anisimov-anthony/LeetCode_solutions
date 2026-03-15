package p0167

func twoSum(numbers []int, target int) []int {
	if len(numbers) == 2 {
		return []int{1, 2}
	}

	left := 0
	right := len(numbers) - 1

	for left < right {
		if numbers[left]+numbers[right] > target {
			right--
		} else if numbers[left]+numbers[right] < target {
			left++
		} else {
			return []int{left + 1, right + 1}
		}
	}

	return []int{left + 1, right + 1}
}
