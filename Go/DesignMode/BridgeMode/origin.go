package main

type Level uint64

const(
	Severe Level = iota
	Normal
	Trival
)

type Notification struct {
	emailAddr []string
	phoneNum []string
	wechats []string
}

// 这种实现中，如果每一项的实现逻辑都非常复杂，则代码会变得臃肿
func (n *Notification) notify(level Level, msg string) {
	switch level {
	case Severe:
		// 打电话
	case Normal:
		// 发微信
	case Trival:
		// 发邮件
	}
}
