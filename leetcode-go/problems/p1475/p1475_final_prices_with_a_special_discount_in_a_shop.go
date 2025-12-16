package p1475

func finalPrices(prices []int) []int {
	result := make([]int, 0, len(prices))
	for i := range len(prices) {
		found := false
		for j := i + 1; j < len(prices); j++ {
			if prices[j] <= prices[i] {
				result = append(result, prices[i]-prices[j])
				found = true
				break
			}
		}
		if !found {
			result = append(result, prices[i])
		}
	}
	return result
}
