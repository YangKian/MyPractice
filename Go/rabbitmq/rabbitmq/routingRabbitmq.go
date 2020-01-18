package rabbitmq

import (
	"fmt"
	"github.com/streadway/amqp"
	"log"
)

func NewRabbitMQRouting(exchangeName, routingKey string) *RabbitMQ {
	return NewRabbitMQ("", exchangeName, routingKey)
}

//路由模式下的生产者
func (mq *RabbitMQ) PublishRouting(message string) {
	//1.尝试创建交换机
	if err := mq.channel.ExchangeDeclare(
		mq.Exchange,
		"direct", // 类型设置为direct
		true,
		false,
		false,
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "创建交换机失败")
	}

	//2.发送消息
	if err := mq.channel.Publish(
		mq.Exchange,
		mq.Key, //要设置routingkey
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		}); err != nil {
		mq.failOnError(err, "发送消息失败")
	}
}

//路由模式下的消费者
func (mq *RabbitMQ) RecieveRouting() {
	//1.尝试创建交换机
	if err := mq.channel.ExchangeDeclare(
		mq.Exchange,
		"direct", // 类型设置为direct
		true,
		false,
		false,
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "创建交换机失败")
	}

	//2.尝试创建队列
	q, err := mq.channel.QueueDeclare(
		"",
		false,
		false,
		true,
		false,
		nil,
	)
	if err != nil {
		mq.failOnError(err, "创建队列失败")
	}

	//3.绑定队列到交换机
	if err := mq.channel.QueueBind(
		q.Name,
		mq.Key,
		mq.Exchange,
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "绑定队列失败")
	}

	//4.消费信息
	msgs, err := mq.channel.Consume(
		q.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)

	forever := make(chan bool)
	go func() {
		for d := range msgs {
			log.Printf("Received a message: %s", d.Body)
		}
	}()
	fmt.Println("[*] Waiting for messages. To exit press CTRL+C")
	<-forever
}
