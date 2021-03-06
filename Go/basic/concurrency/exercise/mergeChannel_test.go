package exercise

import (
	"fmt"
	"math"
	"testing"
)

func generator(values ...int) <-chan interface{} {
	out := make(chan interface{})
	go func() {
		defer close(out)
		for _, v := range values {
			out <- v
		}
	}()
	return out
}

func TestMergeGoroutine(t *testing.T) {
	res := MergeGoroutine(generator(1, 2, 3, 4, 5), generator(6, 7, 8, 9, 10),
		generator(10, 11, 12, 13, 14))
	for v := range res {
		fmt.Printf("%d ", v.(int))
	}
}

func TestMergeReflect(t *testing.T) {
	res := MergeReflect(generator(1, 2, 3, 4, 5), generator(6, 7, 8, 9, 10),
		generator(10, 11, 12, 13, 14))
	for v := range res {
		fmt.Printf("%d ", v.(int))
	}
}

func TestMergeRecursive(t *testing.T) {
	res := MergeRecursive(generator(1, 2, 3, 4, 5), generator(6, 7, 8, 9, 10),
		generator(10, 11, 12, 13, 14))
	for v := range res {
		fmt.Printf("%d ", v.(int))
	}
}

func BenchmarkMerge(b *testing.B) {
	merges := []struct {
		name string
		fun  func(...<-chan interface{}) <-chan interface{}
	}{
		{"goroutines", MergeGoroutine},
		{"reflect", MergeReflect},
		{"recursion", MergeRecursive},
	}
	for _, merge := range merges {
		for k := 0.; k <= 10; k++ {
			n := int(math.Pow(2, k))
			b.Run(fmt.Sprintf("%s/%d", merge.name, n), func(b *testing.B) {
				for i := 0; i < b.N; i++ {
					chans := make([]<-chan interface{}, n)
					for j := range chans {
						chans[j] = generator(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
					}
					c := merge.fun(chans...)
					for range c {
					}
				}
			})
		}
	}
}