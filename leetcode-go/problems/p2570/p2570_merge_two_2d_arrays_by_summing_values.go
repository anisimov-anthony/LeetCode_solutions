package problems

import "sort"

func mergeArrays(nums1 [][]int, nums2 [][]int) [][]int {
	result := make([][]int, 0, len(nums1)+len(nums2))

	ids := make(map[int]int)

	for _, nm := range nums1 {
		ids[nm[0]] += nm[1]
	}

	for _, nm := range nums2 {
		ids[nm[0]] += nm[1]
	}

	for k, v := range ids {
		result = append(result, []int{k, v})
	}

	sort.Slice(result, func(i, j int) bool {
		return result[i][0] < result[j][0]
	})

	return result
}
