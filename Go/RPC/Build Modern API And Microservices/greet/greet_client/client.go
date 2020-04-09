package main

import (
	"context"
	"fmt"
	"gRPC_Via_Udemy/greet/greetpb"
	"google.golang.org/genproto/googleapis/rpc/errdetails"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"io"
	"log"
	"time"
)

func main() {
	fmt.Println("Hello I'm client")

	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure(),
		/*添加拦截器*/grpc.WithUnaryInterceptor(greetUnaryClientInterceptorfunc),
		grpc.WithStreamInterceptor(greetClientStreamInterceptorfunc))
	if err != nil {
		log.Fatalf("could not connect: %v", err)
	}
	defer conn.Close()

	c := greetpb.NewGreetServiceClient(conn)

	// 通过 metadata 传递信息
	// 定义元数据
	md := metadata.Pairs(
			"timestamp", time.Now().Format(time.StampNano),
			"kn", "vn",
		)

	// 将元数据放入 context 中
	// 方式一: 创建一个携带元数据的新的上下文
	mdCtx := metadata.NewOutgoingContext(context.Background(), md)
	// 方式二：在原有的上下文中追加元数据信息
	ctxA := metadata.AppendToOutgoingContext(mdCtx, "k1", "v1", "k1", "v2", "k2", "v3")

	//doUnary(c)S
	//doServerStreaming(c)
	//doClientStreaming(c)
	//doBiDiStreaming(c)
	//doUnaryWithDeadline(c, 5 * time.Second)
	//doUnaryWithDeadline(c, 1 * time.Second)

	// 在 Unary RPC 中使用元数据
	//doUnaryRPCWithMetadata(mdCtx, c)
	// 在 Stream RPC 中使用元数据
	doBiDiStreamingWithMetadata(ctxA, c)
}

func doUnary(c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a Unary RPC...")
	req := &greetpb.GreetRequest{
		Greeting: &greetpb.Greeting{
			FirstName: "Stephane",
			LastName:  "Maarek",
		},
	}
	res, err := c.Greet(context.Background(), req)
	if err != nil {
		errHandle(err)
		log.Fatal("error while calling Greet RPC")
	}
	log.Printf("Response from Greet: %v", res.Result)
}

func doServerStreaming(c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a Server Streaming RPC...")
	
	req := &greetpb.GreetManyTimesRequest{
		Greeting:             &greetpb.Greeting{
			FirstName:            "Stephane",
			LastName:             "Maarek",
		},
	}
	resStream, err := c.GreetManyTimes(context.Background(), req)
	if err != nil {
		errHandle(err)
		log.Fatal("error while calling GreetManyTimes RPC")
	}

	for {
		msg, err := resStream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("error while reading stream: %v\n", err)
		}
		log.Printf("Response from GreetManyTiems: %v\n", msg.Result)
	}
}

func doClientStreaming(c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a Client Streaming RPC...")

	requests := []*greetpb.LongGreetRequest {
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Stephane",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "John",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Lucy",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Mark",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Pipe",
			},
		},
	}
	stream, err := c.LongGreet(context.Background())
	if err != nil {
		errHandle(err)
		log.Fatal("error while calling LongGreet RPC")
	}

	for _, req := range requests {
		fmt.Println("Sending req: ", req)
		stream.Send(req)
		time.Sleep(100 * time.Millisecond)
	}

	res, err := stream.CloseAndRecv()
	if err != nil {
		log.Fatalf("error while receiving response from LongGreet：%v", err)
	}
	log.Printf("LongGreet Response: %v\n", res)
}

func doBiDiStreaming(c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a BiDi Streaming RPC...")

	//创建一个stream
	stream, err := c.GreetEveryone(context.Background())
	if err != nil {
		errHandle(err)
		log.Fatal("Error while creating stream")
		return
	}

	requests := []*greetpb.GreetEveryoneRequest {
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Stephane",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "John",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Lucy",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Mark",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Pipe",
			},
		},
	}

	waitc := make(chan struct{})
	//发送信息
	go func() {
		for _, req := range requests {
			fmt.Printf("sending message: %v\n", req)
			stream.Send(req)
			time.Sleep(100 * time.Millisecond)
		}
		stream.CloseSend()
	}()

	//接收信息
	go func() {
		for {
			res, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				close(waitc)
				log.Fatalf("Error while receiving: %v", err)
				return
			}
			fmt.Printf("Received: %v\n", res.GetResult())
		}
		close(waitc)
	}()

	<- waitc
}

func doUnaryWithDeadline(c greetpb.GreetServiceClient, timeout time.Duration) {
	fmt.Println("Starting to do a UnaryWithDeadline RPC...")
	req := &greetpb.GreetWithDeadlineRequest{
		Greeting: &greetpb.Greeting{
			FirstName: "Stephane",
			LastName:  "Maarek",
		},
	}
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	res, err := c.GreetWithDeadline(ctx, req)
	if err != nil {
		statusErr, ok := status.FromError(err)
		if ok {
			if statusErr.Code() == codes.DeadlineExceeded {
				fmt.Println("Timeout was hit! Deadline was execeeded")
			} else {
				fmt.Printf("unexpected error: %v", statusErr)
			}
		} else {
			log.Fatalf("error while calling GreetWithDeadline RPC: %v", err)
		}
		return //notice here we need to return because if we go into the ok clause, we just print the err and not
		// deal with it, so the res.Result will error
	}
	log.Printf("Response from GreetWithDeadline: %v", res.Result)
}

// 实现元数据的交换
func doUnaryRPCWithMetadata(ctx context.Context, c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a Unary RPC with metadata")
	req := &greetpb.GreetRequest{
		Greeting: &greetpb.Greeting{
			FirstName: "Stephane",
			LastName:  "Maarek",
		},
	}
	// 创建存储 header 和 trailer 的变量
	var header, trailer metadata.MD

	// 发送一个不带任何请求，只携带元数据信息的 Unary RPC
	_, err := c.GreetWithMetadata(ctx, req, grpc.Header(&header), grpc.Trailer(&trailer))
	if err != nil {
		log.Fatalf("error while calling GreetWithMetadata RPC: %v\n", err)
	}

	// 检查收到的响应
	fmt.Println("============Received the header: ")
	if t, ok := header["timestamp"]; ok {
		fmt.Printf("timestamp from header:\n")
		for i, e := range t {
			fmt.Printf(" %d. %s\n", i, e)
		}
	} else {
		log.Fatal("timestamp expected but doesn't exist in header")
	}
	if l, ok := header["location"]; ok {
		fmt.Printf("location from header:\n")
		for i, e := range l {
			fmt.Printf(" %d. %s\n", i, e)
		}
	} else {
		log.Fatal("location expected but doesn't exist in header")
	}

	fmt.Println("============Received the trailer: ")
	for k, v := range trailer {
		fmt.Println(k, v)
	}
}

func doBiDiStreamingWithMetadata(ctx context.Context, c greetpb.GreetServiceClient) {
	fmt.Println("Starting to do a BiDi Streaming RPC...")

	//创建一个stream
	stream, err := c.GreetEveryoneWithMetadata(ctx)
	if err != nil {
		errHandle(err)
		log.Fatal("Error while creating stream")
		return
	}

	requests := []*greetpb.GreetEveryoneRequest {
		{
			Greeting: &greetpb.Greeting{
				FirstName: "Stephane",
			},
		},
		{
			Greeting: &greetpb.Greeting{
				FirstName: "John",
			},
		},
	}

	waitc := make(chan struct{})
	//发送信息
	go func() {
		for _, req := range requests {
			fmt.Printf("sending message: %v\n", req)
			stream.Send(req)
			time.Sleep(100 * time.Millisecond)
		}
		stream.CloseSend()
	}()

	//接收信息
	go func() {
		for {
			res, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				close(waitc)
				log.Fatalf("Error while receiving: %v", err)
				return
			}
			fmt.Printf("Received: %v\n", res.GetResult())
		}
		close(waitc)
	}()

	<- waitc

	// 检查收到的响应
	fmt.Println("============Received the header: ")
	head, _ := stream.Header()
	for k, v := range head {
		fmt.Println(k, v)
	}

	fmt.Println("============Received the trailer: ")
	for k, v := range stream.Trailer(){
		fmt.Println(k, v)
	}
}

// 实现客户端拦截器

// Unary 拦截器
func greetUnaryClientInterceptorfunc(ctx context.Context, method string, req, reply interface{},
	cc *grpc.ClientConn, invoker grpc.UnaryInvoker, opts ...grpc.CallOption) error {

	// Preprocessor phase
	log.Println("Method: " + method)

	// 调用 RPC
	err := invoker(ctx, method, req, reply, cc, opts...)

	// Postprocessor phase
	log.Println(reply)

	return err
}

// Stream 拦截器的实现
type wrappedStream struct {
	grpc.ClientStream
}

// 在接收到流信息时的处理逻辑
func (w *wrappedStream) RecvMsg(m interface{}) error {
	log.Printf("============[Client Stream Interceptor Wrapper] " +
		"Receive a message (Type: %T) at %s\n", m, time.Now().Format(time.RFC3339))
	return w.ClientStream.RecvMsg(m)
}

// 发送流信息时的处理逻辑
func (w *wrappedStream) SendMsg(m interface{}) error {
	log.Printf("============[Client Stream Interceptor Wrapper] " +
		"Send a message (Type: %T) at %v\n", m, time.Now().Format(time.RFC3339))
	return w.ClientStream.SendMsg(m)
}

func newWrappedStream(s grpc.ClientStream) grpc.ClientStream {
	return &wrappedStream{s}
}

// Stream 客户端拦截器
func greetClientStreamInterceptorfunc(ctx context.Context, desc *grpc.StreamDesc, cc *grpc.ClientConn, method string,
	streamer grpc.Streamer, opts ...grpc.CallOption) (grpc.ClientStream, error) {
	log.Println("========== [Client Interceptor]", method)

	// 调用 streamer 来获取一个 ClientStream
	s, err := streamer(ctx, desc, cc, method, opts...)
	if err != nil {
		return nil, err
	}
	return newWrappedStream(s), nil
}

// 错误处理
func errHandle(err error) {
	errorCode := status.Code(err) //使用 gRPC 的 status 包来捕获错误
	if errorCode == codes.InvalidArgument { // 检查错误的类型（这里只检查了参数错误的情况）
		log.Printf("Invalid Argument Error: %v\n", errorCode)
		errorStatus := status.Convert(err)
		for _, d := range errorStatus.Details() {
			switch info := d.(type) {
			case *errdetails.BadRequest_FieldViolation:
				log.Printf("Request Field Invalid: %v\n", info)
			default:
				log.Printf("Unexpected error type: %v\n", info)
			}
		}
	} else {
		log.Printf("Unhandled error: %v\n", errorCode)
	}
}