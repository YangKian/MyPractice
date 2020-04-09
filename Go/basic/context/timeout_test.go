package main

import (
	"context"
	"testing"
	"time"
)

func TestTimeout(t *testing.T) {
	ch := make(chan int, 1)

	ctx, cancel := context.WithTimeout(context.Background(), 150 * time.Millisecond)
	defer cancel()

	go func() {
		time.Sleep(50 * time.Millisecond)
		ch <- 1
	}()

	select {
	case d := <- ch:
		t.Log("work complete: ", d)
	case <- ctx.Done():
		t.Log("work cancelled")
	}
}
