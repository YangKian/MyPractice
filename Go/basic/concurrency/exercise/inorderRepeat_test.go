package exercise

import (
	"fmt"
	"testing"
)

func TestInorderRepeat(t *testing.T) {
	repeat := func(done <-chan interface{}, name string) chan interface{} {
		sig := make(chan interface{})
		go func() {
			defer close(sig)
			for {
				select {
				case <-done:
					fmt.Printf("close %s channel\n", name)
					sig <- struct{}{}
					return
				case <-sig:
					fmt.Println(name)
					sig <- struct{}{}
				}
			}
		}()
		return sig
	}

	done := make(chan interface{})
	catCh := repeat(done, "cat")
	dogCh := repeat(done, "dog")
	fishCh := repeat(done, "fish")
	channels := []chan interface{} {catCh, dogCh, fishCh}
	for i := 0; i < 10; i++ {
		for _, ch := range channels {
			ch <- struct{}{}
			<-ch
		}
		fmt.Printf("======Turn %d\n", i)
	}
	close(done)
	for i := 0; i < len(channels); i++ {
		<-channels[i]
	}
}
