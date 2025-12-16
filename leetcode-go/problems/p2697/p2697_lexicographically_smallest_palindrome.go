package p2697

func makeSmallestPalindrome(s string) string {
	if len(s) <= 1 {
		return s
	}
	left := 0
	right := len(s) - 1
	runes := []rune(s)
	for left < right {
		if runes[left] < runes[right] {
			runes[right] = runes[left]
		} else {
			runes[left] = runes[right]
		}
		left++
		right--
	}
	return string(runes)
}
