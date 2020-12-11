package main

import (
	"fmt"
	"time"
)

// 对外暴露的业务功能接口
type UserService interface {
	Login()
}

// 原始类，专注实现业务逻辑
type User struct {
}

func (u *User)Login()  {
	// do login service
	time.Sleep(1 * time.Second)
}

// 代理类，增加了数据收集的逻辑
type UserProxy struct {
	user User
}

func NewUserProxy(user User) UserProxy {
	return UserProxy{ user }
}

func (p *UserProxy)Login() {
	// 执行新增的数据收集逻辑

	start := time.Now()

	p.user.Login() // 原始类的业务逻辑

	fmt.Printf("Elapsed time: %v\n", time.Since(start).Seconds())
}

func main() {
	proxy := NewUserProxy(User{})
	proxy.Login()
}




