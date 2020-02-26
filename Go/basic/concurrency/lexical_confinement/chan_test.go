package main

import (
	"bytes"
	"fmt"
	"sync"
	"testing"
)

func TestFunc(t *testing.T) {
	t.Run("使用闭包来保证goroutine安全", func(t *testing.T) {
		chanOwner := func() <- chan int { //返回一个只读通道，限制了对该通道的使用行为
			results := make(chan int, 5) // results 使用闭包来定义，限制了外部程序对其进行写入
			go func() {
				defer close(results)
				for i := 0; i <= 5; i++ {
					results <- i
				}
			}()
			return results
		}

		consumer := func(results <- chan int) {
			for result := range results {
				t.Logf("Received: %d\n", result)
			}
			t.Log("Done\n")
		}

		results := chanOwner()
		consumer(results)
	})

	t.Run("使用词法作用域来限制不安全的数据结构 buffer", func(t *testing.T) {
		printData := func(wg *sync.WaitGroup, data []byte) {
			defer wg.Done()

			var buff bytes.Buffer
			for _, b := range data {
				fmt.Fprintf(&buff, "%c", b)
			}
			t.Log(buff.String())
		}

		var wg sync.WaitGroup
		wg.Add(2)
		data := []byte("Golang")
		go printData(&wg, data[:3])
		go printData(&wg, data[3:])
		wg.Wait()
	})
}
