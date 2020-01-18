package main

import (
	"fmt"
	"rabbitMQ/rabbitmq"
	"strconv"
	"time"
)

func main() {
	mq := rabbitmq.NewRabbitMQPubSub("newProduct")
	for i := 0; i < 100; i++ {
		mq.PublishPub("订阅模式生产第" +
			strconv.Itoa(i) + "条" + "数据")
		fmt.Println("订阅模式生产第" +
			strconv.Itoa(i) + "条" + "数据")
		time.Sleep(1 * time.Second)
	}
}
