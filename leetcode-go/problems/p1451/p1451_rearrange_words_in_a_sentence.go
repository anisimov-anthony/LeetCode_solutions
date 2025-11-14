package p1451

import (
	"sort"
	"strings"
)

func arrangeWords(text string) string {
	type wdEntry struct {
		word      string
		length    int
		origIndex int
	}
	words := strings.Fields(text)
	wordEntries := make([]wdEntry, 0, len(words))
	for i, word := range words {
		wordEntries = append(wordEntries, wdEntry{word: strings.ToLower(word), length: len(word), origIndex: i})
	}

	sort.Slice(wordEntries, func(i, j int) bool {
		if wordEntries[i].length != wordEntries[j].length {
			return wordEntries[i].length < wordEntries[j].length
		}
		return wordEntries[i].origIndex < wordEntries[j].origIndex
	})

	if len(wordEntries) > 0 {
		firstWord := wordEntries[0].word
		if len(firstWord) > 0 {
			wordEntries[0].word = strings.ToUpper(firstWord[:1]) + firstWord[1:]
		}
	}

	result := ""
	for i, entry := range wordEntries {
		result += entry.word

		if i != len(wordEntries)-1 {
			result += " "
		}
	}

	return result
}
