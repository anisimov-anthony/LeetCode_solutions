package problems

import "slices"

func largestLocal(grid [][]int) [][]int {
	n := len(grid)
	result := make([][]int, n-2)

	for i := 0; i < n-2; i++ {
		result[i] = make([]int, n-2)
		for j := 0; j < n-2; j++ {
			first := slices.Max(grid[i][j : j+3])
			second := slices.Max(grid[i+1][j : j+3])
			third := slices.Max(grid[i+2][j : j+3])
			result[i][j] = max(first, max(second, third))

		}
	}

	return result
}
