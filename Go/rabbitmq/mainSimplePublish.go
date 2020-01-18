package main

import (
	"fmt"
	"rabbitMQ/rabbitmq"
)

func main() {
	mq := rabbitmq.NewRabbitMQSimple("imoocSimple")
	mq.PublishSimple("Hello imooc!")
	fmt.Println("发送成功！")
}
