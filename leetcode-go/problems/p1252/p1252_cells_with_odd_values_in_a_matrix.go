package p1252

func oddCells(m int, n int, indices [][]int) int {
	result := 0

	rowsIncr := make(map[int]int)
	columnsIncr := make(map[int]int)

	for _, index := range indices {
		rowsIncr[index[0]]++
		columnsIncr[index[1]]++
	}

	for r := range m {
		if rowsIncr[r] == 0 {
			for c := range n {
				if columnsIncr[c]%2 != 0 {
					result++
				}
			}
		} else {
			if rowsIncr[r]%2 == 0 {
				for c := range n {
					if columnsIncr[c]%2 != 0 {
						result++
					}
				}
			} else {
				for c := range n {
					if columnsIncr[c]%2 == 0 {
						result++
					}
				}
			}
		}
	}

	return result
}
