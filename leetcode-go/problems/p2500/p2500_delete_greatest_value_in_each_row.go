package p2500

import "sort"

func deleteGreatestValue(grid [][]int) int {
	result := 0

	for _, row := range grid {
		sort.Slice(row, func(i, j int) bool {
			return row[i] > row[j]
		})
	}

	for i := range len(grid[0]) {
		localMax := 0
		for j := range len(grid) {
			localMax = max(localMax, grid[j][i])
		}
		result += localMax
	}
	return result
}
