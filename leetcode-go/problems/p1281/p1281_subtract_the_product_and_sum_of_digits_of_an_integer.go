package p1281

func subtractProductAndSum(n int) int {
	digits := make([]int, 0)
	for n > 0 {
		dg := n % 10
		digits = append(digits, dg)
		n /= 10
	}

	product := 1
	sum := 0
	for _, dg := range digits {
		product *= dg
		sum += dg
	}

	return product - sum
}
