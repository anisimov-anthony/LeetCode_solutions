package p3309

import (
	"math"
	"strconv"
)

func maxGoodNumber(nums []int) int {
	a := strconv.FormatInt(int64(nums[0]), 2)
	b := strconv.FormatInt(int64(nums[1]), 2)
	c := strconv.FormatInt(int64(nums[2]), 2)

	abc_str := a + b + c
	acb_str := a + c + b
	bac_str := b + a + c
	bca_str := b + c + a
	cab_str := c + a + b
	cba_str := c + b + a

	abc, _ := strconv.ParseInt(abc_str, 2, 64)
	acb, _ := strconv.ParseInt(acb_str, 2, 64)
	bac, _ := strconv.ParseInt(bac_str, 2, 64)
	bca, _ := strconv.ParseInt(bca_str, 2, 64)
	cab, _ := strconv.ParseInt(cab_str, 2, 64)
	cba, _ := strconv.ParseInt(cba_str, 2, 64)

	a_max := int(math.Max(float64(abc), float64(acb)))
	b_max := int(math.Max(float64(bac), float64(bca)))
	c_max := int(math.Max(float64(cab), float64(cba)))

	ab_max := int(math.Max(float64(a_max), float64(b_max)))

	return int(math.Max(float64(ab_max), float64(c_max)))
}
