package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

func FanOutPrimeFinder(num int) {
	repeatFn := func(done chan interface{}, fn func() interface{}) <- chan interface{} {
		valueStream := make(chan interface{})
		go func() {
			defer close(valueStream)
			for {
				select {
				case <- done:
					return
				case valueStream <- fn():
				}
			}
		}()
		return valueStream
	}

	take := func(done <- chan interface{}, valueStream <- chan interface{}, num int) <- chan interface{} {
		takeStream := make(chan interface{})
		go func() {
			defer close(takeStream)
			for i := 0; i < num; i++ {
				select {
				case <- done:
					return
				case takeStream <- <-valueStream:
				}
			}
		}()
		return takeStream
	}

	fanIn := func(done <- chan interface{}, channels ...<- chan interface{}) <- chan interface {} {
		var wg sync.WaitGroup
		multiplexedStream := make(chan interface{})

		multiplex := func(c <- chan interface{}) {
			defer wg.Done()
			for i := range c {
				select {
				case <- done:
					return
				case multiplexedStream <- i:
				}
			}
		}

		wg.Add(len(channels))
		for _, c := range channels {
			go multiplex(c) //为每个输入的 channel 开一个 goroutine，并发读取
		}

		go func() { //等待所有的读操作完成
			wg.Wait()
			close(multiplexedStream) //在所有的输入channel都被关闭后关闭 multiplexedStream
		}()

		return multiplexedStream
	}

	toInt := func(done <- chan interface{}, valueStream <- chan interface{}) <- chan int {
		intStream := make(chan int)
		go func() {
			defer close(intStream)
			for v := range valueStream {
				select {
				case <- done:
					return
				case intStream <- v.(int):
				}
			}
		}()
		return intStream
	}

	primeFinder := func(done <- chan interface{}, intStream <- chan int) <- chan interface{} {
		primeStream := make(chan interface{})
		go func() {
			defer close(primeStream)
			for integer := range intStream {
				integer -= 1
				prime := true
				for divisor := integer - 1; divisor > 1; divisor-- {
					if integer % divisor == 0 {
						prime = false
						break
					}
				}

				if prime {
					select {
					case <- done:
						return
					case primeStream <- integer:
					}
				}
			}
		}()
		return primeStream
	}

	rand := func() interface{} {return rand.Intn(50000000)}

	done := make(chan interface{})
	defer close(done)

	start := time.Now()

	randIntStream := toInt(done, repeatFn(done, rand))

	numFinders := runtime.NumCPU()
	fmt.Printf("Spinning up %d prime finders.\n", numFinders)
	finders := make([]<-chan interface{}, numFinders)
	fmt.Println("Primes:")
	for i := 0; i < numFinders; i++ {
		finders[i] = primeFinder(done, randIntStream)
	}

	for prime := range take(done, fanIn(done, finders...), num) {
		fmt.Printf("\t%d\n", prime)
	}
	fmt.Printf("Search took: %v", time.Since(start))
}
