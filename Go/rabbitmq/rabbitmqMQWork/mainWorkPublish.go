package main

import (
	"fmt"
	"rabbitMQ/rabbitmq"
	"strconv"
	"time"
)

func main() {
	mq := rabbitmq.NewRabbitMQSimple("imoocSimple")
	for i := 0; i <= 100; i++ {
		mq.PublishSimple("Hello imooc!" + strconv.Itoa(i))
		time.Sleep(1 * time.Second)
		fmt.Println(i)
	}
}
