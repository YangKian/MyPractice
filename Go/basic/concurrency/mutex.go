package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var m sync.Mutex
	c := sync.NewCond(&m)
	ready := make(chan struct{})
	isReady := false
	fmt.Println("裁判在等待所有运动员就绪")
	for i := 0; i < 10; i++ {
		go func(i int) {
			fmt.Printf("运动员%d开始热身。\n", i)
			m.Lock()
			time.Sleep(1 * time.Second)
			ready <- struct{}{}
			for !isReady {
				c.Wait()
			}
			m.Unlock()
			fmt.Printf("运动员%d出发。\n", i)
			ready <- struct{}{}
		}(i)
	}

	//c.Broadcast() false op
	//c.Signal() false op

	for i := 0; i < 10; i++ {
		fmt.Printf("%d位运动员已就绪\n", i)
		<- ready
	}
	isReady = true
	fmt.Println("收到所有运动员就绪，裁判发令")
	c.Broadcast()
	for i := 0; i < 10; i++ {
		<- ready
	}
}
