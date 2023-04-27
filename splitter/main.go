package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var rg = regexp.MustCompile(`^# (.+) \((\d+)\-(\d+)\-(\d+)\)$`)

func main() {
	file, err := os.Open("archive.md")
	if err != nil {
		panic(err)
	}

	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	sc := bufio.NewScanner(file)

	var f *os.File

	for sc.Scan() {
		line := sc.Text()

		// New file.
		if rg.MatchString(line) {
			name := fileName(line)

			path := filepath.Join(cwd, "split", name)
			f, err = os.Create(path)
			if err != nil {
				panic(err)
			}
		}

		_, err = f.WriteString(line + "\n")
		if err != nil {
			panic(err)
		}
	}
}

func fileName(line string) string {
	matches := rg.FindStringSubmatch(line)
	if matches == nil {
		panic(line)
	}

	replacer := strings.NewReplacer(
		":", "",
		".", "",
		",", "",
		"/", "",
		"(", "",
		")", "",
		"ß", "ss",
		"ä", "ae",
		"ü", "ue",
		"ö", "oe",
		" ", "_",
		"&", "_and_",
	)

	title := replacer.Replace(strings.ToLower(matches[1]))
	date := fmt.Sprintf("%v-%v-%v", matches[2], matches[3], matches[4])

	return date + "_" + title + ".md"
}
