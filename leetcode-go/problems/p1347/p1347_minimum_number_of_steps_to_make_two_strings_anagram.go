package p1347

func minSteps(s string, t string) int {
	sfreqs := make(map[rune]int)
	tfreqs := make(map[rune]int)

	for _, ch := range s {
		sfreqs[ch]++
	}
	for _, ch := range t {
		tfreqs[ch]++
	}

	result := 0

	for ch, fr := range sfreqs {
		diff := fr - tfreqs[ch]
		if diff > 0 {
			result += diff
		}
	}

	return result
}
