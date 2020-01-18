package main

import (
	"fmt"
	"rabbitMQ/rabbitmq"
	"strconv"
	"time"
)

func main() {
	imoocOne := rabbitmq.NewRabbitMQRouting("exImooc", "imooc_one")
	imoocTwo := rabbitmq.NewRabbitMQRouting("exImooc", "imooc_two")
	for i := 0; i <= 100; i++ {
		imoocOne.PublishRouting("Hello imooc one!" + strconv.Itoa(i))
		imoocTwo.PublishRouting("Hello imooc Two!" + strconv.Itoa(i))
		time.Sleep(1 * time.Second)
		fmt.Println(i)
	}
}
