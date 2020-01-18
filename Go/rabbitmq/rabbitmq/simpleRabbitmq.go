package rabbitmq

import (
	"fmt"
	"github.com/streadway/amqp"
	"log"
)

//1.创建简单模式下的Rabbitmq实例
func NewRabbitMQSimple(queueName string) *RabbitMQ {
	//exchange为空，使用默认类型，key为空即不传入key
	return NewRabbitMQ("imoocSimple", "", "")
}

//2.创建简单模式下的生产者
func (mq *RabbitMQ) PublishSimple(message string) {
	//1.申请队列：如果队列不存在，则创建新队列，如果存在，则跳过申请
	if _, err := mq.channel.QueueDeclare(
		mq.QueueName, //队列名称
		false,        //消息是否持久化
		false,        //是否自动删除，即最后一个消费者断开连接后，是否删除队列
		false,        //是否具有排他性，排他性：队列只能被创建他的连接访问，连接关闭，队列也会销毁
		false,        //是否阻塞
		nil,          //额外属性
	); err != nil {
		fmt.Printf("创建生产者出错，err: %s\n", err)
	}

	//2.发送消息到队列中
	if err := mq.channel.Publish(
		mq.Exchange,  //交换机
		mq.QueueName, //队列名称
		false,        // 如果为true, 会根据exchange类型和routinkey规则，如果无法找到符合条件的队列，会把消息返回给发送者
		false,        // 如果为true, 当exchange发送消息到队列后发现队列上没有绑定消费者，会把消息返回给发送者
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		}); err != nil {
		mq.failOnError(err, "发送消息失败")
	}
}

//3.创建简单模式下的消费者
func (mq *RabbitMQ) ConsumSimple() {
	//1.申请队列
	if _, err := mq.channel.QueueDeclare(
		mq.QueueName, //队列名称
		false,        //消息是否持久化
		false,        //是否自动删除，即最后一个消费者断开连接后，是否删除队列
		false,        //是否具有排他性，排他性：队列只能被创建他的连接访问，连接关闭，队列也会销毁
		false,        //是否阻塞
		nil,          //额外属性
	); err != nil {
		fmt.Printf("创建生产者出错，err: %s\n", err)
	}

	//2.接收消息
	msgs, err := mq.channel.Consume(
		mq.QueueName,
		"",    //用来区分多个消费者，为空则不区分
		true,  //是否自动应答
		false, //是否具有排他性
		false, //如果设置为true，表示不能将同一个connection中发送的消息传递给这个connection中的消费者
		false, //队列消费是否阻塞，false是设置为阻塞
		nil,   //其他参数
	)
	if err != nil {
		mq.failOnError(err, "接收消息失败")
	}

	//3.处理消息
	forever := make(chan bool)
	go func() {
		for d := range msgs {
			//实现逻辑函数
			log.Printf("Received a message: %s", d.Body)
		}
	}()
	fmt.Println("[*] Waiting for messages. To exit press CTRL+C")
	<-forever
}
