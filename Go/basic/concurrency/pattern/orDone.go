package main

// 同时监控两个 channel，当 done channel 被激活或者 c channel 被关闭时，退出 goroutine
func OrDone(done, c <-chan interface{}) <-chan interface{} {
	stream := make(chan interface{})
	go func() {
		defer close(stream)
		for  {
			select {
			case <-done:
				return
			case v, ok := <-c:
				if ok == false {
					return
				}
				select {
				case stream <- v:
				case <-done:
				}
			}
		}
	}()
	return stream
}
