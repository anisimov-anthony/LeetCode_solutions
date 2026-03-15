package p0438

func findAnagrams(s string, p string) []int {
	sRunes := []rune(s)
	pRunes := []rune(p)

	pFreqs := make(map[rune]int)
	for _, rn := range pRunes {
		pFreqs[rn]++
	}

	result := make([]int, 0, 1)

	for i := 0; i <= len(sRunes)-len(pRunes); i++ {

		tmpFreqs := make(map[rune]int)
		for _, rn := range sRunes[i : i+len(pRunes)] {
			tmpFreqs[rn]++
		}

		match := true
		for pRn, pFr := range pFreqs {
			if tmpFreqs[pRn] != pFr {
				match = false
				break
			}
		}

		if match {
			result = append(result, i)
		}
	}

	return result
}
