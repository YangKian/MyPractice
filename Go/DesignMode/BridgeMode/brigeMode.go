package main

import "fmt"

type Sender interface {
	send(string)
}

// TODO: 并不优雅的实现方式，各报警器类有各自的状态，怎么保存这些状态实例 ？

// 将各类报警信息分别以单独的类实现，类中保存了各自需要的信息以及业务逻辑，
// 抽象一个报警器，在遇到不同级别的警报时，传入对应的报警信息类完成 notify
func notify(sender Sender, msg string) {
	sender.send(msg)
}

type telephoneMsg struct {
	phoneNum []string
}

func (t *telephoneMsg) send(msg string)  {
	// 打电话
	fmt.Println("telephoneMsg send")
}

type emailMsg struct {
	emailAddr []string
}

func (e *emailMsg) send(msg string)  {
	// 发邮件
	fmt.Println("emailMsg send")
}

type wechatMsg struct {
	wechat []string
}

func (w *wechatMsg) send(msg string)  {
	// 发微信
	fmt.Println("wechatMsg send")
}


