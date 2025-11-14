package p1103

func distributeCandies(candies int, num_people int) []int {
	result := make([]int, num_people)
	for i := 0; candies > 0; i++ {
		result[(i)%num_people] += min(i+1, candies)
		candies -= (i + 1)
	}
	return result
}
