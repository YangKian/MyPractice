package exercise

import (
	"fmt"
	"testing"
)

func TestInorderRepeat(t *testing.T) {
	// done chan 用于优雅退出，返回的 sig chan 用于同步
	repeat := func(done <-chan interface{}, name string) chan interface{} {
		sig := make(chan interface{})
		go func() {
			defer close(sig)
			for {
				select {
				case <-done:
					fmt.Printf("close %s channel\n", name)
					sig <- struct{}{} // 这里其实可以用 waitGroup
					return
				case <-sig:
					fmt.Println(name)
					sig <- struct{}{} // 同步
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
			<-ch // 同步
		}
		fmt.Printf("======Turn %d\n", i)
	}
	close(done)

	// 这部分可以用 waitGroup 实现，因为创建了 sig chan 就复用了
	for i := 0; i < len(channels); i++ {
		<-channels[i]
	}
}
