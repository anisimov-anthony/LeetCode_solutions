package p3210

func getEncryptedString(s string, k int) string {
	result := ""
	for i := range s {
		result += string(s[(i+k)%(len(s))])
	}
	return result
}
