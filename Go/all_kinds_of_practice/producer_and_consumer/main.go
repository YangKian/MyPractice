package main

import "fmt"

var messages = []string{
	"The world itself's",
	"just one big hoax.",
	"Spamming each other with our",
	"running commentary of bullshit,",
	"masquerading as insight, our social media",
	"faking as intimacy.",
	"Or is it that we voted for this?",
	"Not with our rigged elections,",
	"but with our things, our property, our money.",
	"I'm not saying anything new.",
	"We all know why we do this,",
	"not because Hunger Games",
	"books make us happy,",
	"but because we wanna be sedated.",
	"Because it's painful not to pretend,",
	"because we're cowards.",
	"- Elliot Alderson",
	"Mr. Robot",
}

const producerCount int = 3
const consumerCount int = 3

var workers []*producers

type producers struct {
	myQ chan string
	quit chan bool
	id int
}

func execute(jobQ chan<- string, workerPool chan *producers, allDone chan<- bool) {
	for _, j := range messages {
		jobQ <- j
	}
	close(jobQ)

	for _, w := range workers {
		w.quit <- true
	}
	close(workerPool)

	allDone <- true
}

func produce(jobQ <-chan string, p *producers, workerPool chan *producers) {
	for {
		select {
		case msg := <- jobQ:
			workerPool <- p
			if len(msg) > 0 {
				fmt.Printf("Job \"%v\" produced by worker %v\n", msg, p.id)
			}
			p.myQ <- msg
		case <- p.quit:
			return
		}
	}
}

func consume(cIdx int, workerPool <- chan *producers) {
	for {
		worker := <- workerPool
		if msg, ok := <- worker.myQ; ok {
			if len(msg) > 0 {
				fmt.Printf("Job \"%v\" is consumed by consumer %v from worker %v\n", msg, cIdx, worker.id)
			}
		}
	}
}

func main() {
	jobQ := make(chan string)
	allDone := make(chan bool)
	workerPool := make(chan *producers)

	for i := 0; i < producerCount; i++ {
		workers = append(workers, &producers{
			myQ:  make(chan string),
			quit: make(chan bool),
			id:   i,
		})
		go produce(jobQ, workers[i], workerPool)
	}

	go execute(jobQ, workerPool, allDone)

	for i := 0; i < consumerCount; i++ {
		go consume(i, workerPool)
	}
	<- allDone
}