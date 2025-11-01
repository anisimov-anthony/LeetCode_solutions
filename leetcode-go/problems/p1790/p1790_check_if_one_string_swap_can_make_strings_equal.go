package p1790

func areAlmostEqual(s1 string, s2 string) bool {
	counter := 0
	s1freqs := make(map[byte]int)
	s2freqs := make(map[byte]int)

	for i := range len(s1) {
		s1freqs[s1[i]]++
		s2freqs[s2[i]]++

		if s1[i] != s2[i] {
			counter++
			if counter > 2 {
				return false
			}
		}
	}

	for ch1, fr1 := range s1freqs {
		if s2freqs[ch1] != fr1 {
			return false
		}
	}

	return (counter == 0) || (counter == 2)
}
