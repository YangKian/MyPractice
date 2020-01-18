package main

import "fmt"

func main() {
	chan1 := make(chan int, 100000)
	chan2 := make(chan int, 100000)
	chan3 := make(chan int, 100000)

	for j := 0; j < 10000; j++ {
		for i := 0; i < 100; i++ {
			select {
			case chan1 <- i:
			case chan2 <- i:
			default:
				chan3 <- i
			}
		}
		if len(chan3) != 0 {
			fmt.Println("len chan1: ", len(chan1))
			fmt.Println("len chan2: ", len(chan2))
			fmt.Println("len chan3: ", len(chan3))
		}
		chan1 = make(chan int, 100000)
		chan2 = make(chan int, 100000)
		chan3 = make(chan int, 100000)
	}
	fmt.Println("Done")
}
