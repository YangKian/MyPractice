package exercise

import (
	"reflect"
	"sync"
)

func MergeGoroutine(channels ...<-chan interface{}) <-chan interface{} {
	wg := sync.WaitGroup{}
	transferCh := make(chan interface{})

	transfer := func(ch <-chan interface{}) {
		defer wg.Done()
		for m := range ch {
			transferCh <- m
		}
	}

	wg.Add(len(channels))
	for _, ch := range channels {
		go transfer(ch)
	}

	go func() {
		wg.Wait()
		close(transferCh)
	}()

	return transferCh
}

func MergeReflect(channels ...<-chan interface{}) <-chan interface{} {
	cases := []reflect.SelectCase{}
	out := make(chan interface{})
	go func() {
		defer close(out)
		for _, c := range channels {
			cases = append(cases, reflect.SelectCase {
				Dir: reflect.SelectRecv,
				Chan: reflect.ValueOf(c),
			})
		}

		for len(cases) > 0 {
			i, v, ok := reflect.Select(cases)
			if !ok {
				cases = append(cases[:i], cases[i + 1:]...)
				continue
			}
			out <- v.Interface()
		}
	}()

	return out
}

func MergeRecursive(channels ...<-chan interface{}) <-chan interface{} {
	switch len(channels) {
	case 0:
		return nil
	case 1:
		return channels[0]
	case 2:
		return mergeTwoChannel(channels[0], channels[1])
	default:
		mid := len(channels) / 2
		return mergeTwoChannel(MergeReflect(channels[:mid]...), MergeReflect(channels[mid:]...))
	}
}

func mergeTwoChannel(ch1, ch2 <-chan interface{}) <-chan interface{} {
	out := make(chan interface{})
	go func() {
		defer close(out)
		for ch1 != nil || ch2 != nil {
			select {
			case v, ok := <- ch1:
				if !ok {
					ch1 = nil
					break
				}
				out <- v
			case v, ok := <- ch2:
				if !ok {
					ch2 = nil
					break
				}
				out <- v
			}
		}
	}()
	return out
}
