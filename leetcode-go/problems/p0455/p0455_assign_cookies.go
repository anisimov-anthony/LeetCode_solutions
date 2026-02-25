package p0455

import "slices"

func findContentChildren(g []int, s []int) int {
	if len(g) == 0 || len(s) == 0 {
		return 0
	}

	slices.Sort(s)
	slices.Sort(g)

	counter := 0
	gptr := 0
	sptr := 0

	for gptr != len(g) && sptr != len(s) {
		if g[gptr] > s[sptr] {
			sptr++
		} else {
			sptr++
			gptr++
			counter++
		}

	}
	return counter
}
