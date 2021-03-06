package main

import (
	"math/rand"
	"testing"
)

func TestPipeline(t *testing.T) {
	t.Run("获取无限生成数列中的前 n 个数", func(t *testing.T) {
		repeat := func(done <- chan interface{}, values ...interface{}) <- chan interface{} {
			valueStream := make(chan interface{})
			go func() {
				defer close(valueStream)
				for { // 无限循环执行遍历语句，直到 return
					for _, v := range values {
						select {
						case <- done:
							return
						case valueStream <- v:
						}
					}
				}
			}()
			return valueStream
		}

		take := func(done <- chan interface{}, valueStream <- chan interface{}, num int) <- chan interface{} {
			takeStream := make(chan interface{})
			go func() {
				defer close(takeStream)
				for i:= 0; i < num; i++ { //执行 num 次以后会阻塞
					select {
					case <- done:
						return
					case takeStream <- <- valueStream:
					}
				}
			}()
			return takeStream
		}

		done := make(chan interface{})
		defer close(done)

		for num := range take(done, repeat(done, 1, 2, 3, 4, 5), 10) { //无限循环生成数列，但是最终只执行了num次，生成 num 个数
			t.Logf("%v", num)
		}
	})

	t.Run("test", func(t *testing.T) {
		repeatFn := func(done <- chan interface{}, fn func() interface{}) <- chan interface {} {
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
				for i:= 0; i < num; i++ {
					select {
					case <- done:
						return
					case takeStream <- <- valueStream:
					}
				}
			}()
			return takeStream
		}

		done := make(chan interface{})
		defer close(done)

		rand := func() interface{} {return rand.Int()}

		for num := range take(done, repeatFn(done, rand), 10) {
			t.Log(num)
		}
	})
}
