package p0015

import "slices"

func threeSum(nums []int) [][]int {
	slices.Sort(nums)
	result := make([][]int, 0, 1)

	for first := 0; first < len(nums)-2; first++ {
		// pass duplicates for first
		if first > 0 && nums[first] == nums[first-1] {
			continue
		}

		sum := -nums[first]

		second := first + 1
		third := len(nums) - 1

		for second < third {
			currentSum := nums[second] + nums[third]

			if currentSum < sum {
				second++
			} else if currentSum > sum {
				third--
			} else {
				result = append(result, []int{nums[first], nums[second], nums[third]})

				// pass duplicates for second
				for second < third && nums[second] == nums[second+1] {
					second++
				}

				// pass duplicates for third
				for second < third && nums[third] == nums[third-1] {
					third--
				}

				second++
				third--
			}
		}
	}

	return result

}
