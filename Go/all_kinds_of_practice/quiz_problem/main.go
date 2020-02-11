package main

import (
	"encoding/csv"
	"flag"
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

func main() {
	csvFilename := flag.String("csv", "problems.csv", "a csv file in the format of 'question, answer")
	timeLimit := flag.Int("limit", 30, "the time limit for quiz in second")
	flag.Parse()

	file, err := os.Open(*csvFilename)
	if err != nil {
		log.Fatalf("open file error: %v\n", err)
	}

	reader := csv.NewReader(file)
	lines, err := reader.ReadAll()
	if err != nil {
		log.Fatalf("read from file error: %v\n", err)
	}
	problems := parseLines(lines)

	count := 0
	timer := time.NewTimer(time.Duration(*timeLimit) * time.Second)
	answerChannel := make(chan string)
	wrong := []problem{}
	Loop:
	for i, p := range problems {
		fmt.Printf("Problem #%d: %s = \n", i + 1, p.question)
		go func() {
			var answer  string
			fmt.Scan(&answer)
			answerChannel <- answer
		}()

		select {
		case <- timer.C:
			break Loop //这里使用跳出语句时为了保证不管是超时还是完成了所有遍历最后的print都能被执行
		case answer := <- answerChannel:
			if answer == p.correctAnswer {
				count++
			} else {
				p.youAnswer = answer
				wrong = append(wrong, p)
			}
		}
	}

	fmt.Printf("correct: %d in %d problems.\n", count, len(problems))
	if len(wrong) != 0 {
		for _, v := range wrong {
			fmt.Printf("Wrong Problems: %#v\n", v)
		}
	}
}

type problem struct {
	question      string
	correctAnswer string
	youAnswer       string
}

func parseLines(lines [][]string) []problem {
	question := make([]problem, 0, len(lines))
	for _, line := range lines {
		temp := problem{}
		temp.question = line[0]
		temp.correctAnswer = strings.TrimSpace(line[1]) // 增加鲁棒性
		question = append(question, temp)
	}
	return question
}