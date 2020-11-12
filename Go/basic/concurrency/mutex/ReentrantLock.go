package main

import (
	"fmt"
	"runtime"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
)

type ReentrantLock struct {
	lock *sync.Mutex
	owner int64
	holdCount int
}

func NewReentrantLock() sync.Locker {
	return &ReentrantLock{
		lock: &sync.Mutex{},
	}
}

// 获取 goroutine id，有运行时开销
func getID() int64 {
	defer func()  {
		if err := recover(); err != nil {
			fmt.Printf("panic recover:panic info:%v\n", err)     }
	}()

	var buf [64]byte
	// 获取当前栈帧信息，里面包含有 goroutine id
	n := runtime.Stack(buf[:], false)
	// 得到id字符串，buf 中数据的格式 goroutine 1 [running]: ......
	idField := strings.Fields(strings.TrimPrefix(string(buf[:n]), "goroutine "))[0]
	id, err := strconv.ParseInt(idField, 10, 64)
	if err != nil {
		panic(fmt.Sprintf("cannot get goroutine id: %v", err))
	}
	return id
}

func (l *ReentrantLock) Lock() {
	gid := getID()
	// 如果是锁持有者重入，则计数加 1
	if atomic.LoadInt64(&l.owner) == gid {
		l.holdCount++
		return
	}

	// 如果是获取一把无主的锁，则更新 owner，设置计数为 1
	l.lock.Lock()
	atomic.StoreInt64(&l.owner, gid)
	l.holdCount = 1
}

func (l *ReentrantLock) Unlock() {
	gid := getID()

	if atomic.LoadInt64(&l.owner) != gid {
		panic(fmt.Sprintf("Wrong owner %d call Unlock", l.owner))
	}

	l.holdCount--
	if l.holdCount != 0 {
		return
	} else {
		atomic.StoreInt64(&l.owner, -1)
		l.lock.Unlock()
	}
}
