package main

import (
	"context"
	"fmt"
	"gRPC_Via_Udemy/greet/greetpb"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"io"
	"log"
	"net"
	"strconv"
	"time"
)

type server struct {}

// Unary
func (*server) Greet(ctx context.Context, req *greetpb.GreetRequest) (*greetpb.GreetResponse, error) {
	fmt.Printf("Greet function was invoked with %v", req)
	firstName := req.GetGreeting().GetFirstName()
	result := "Hello " + firstName
	res := &greetpb.GreetResponse{
		Result:               result,
	}
	return res, nil
}

// Server Streaming
func (*server) GreetManyTimes(req *greetpb.GreetManyTimesRequest, stream greetpb.GreetService_GreetManyTimesServer) error {
	fmt.Printf("GreetManyTimes function was invoked with %v\n", req)
	firstName := req.GetGreeting().GetFirstName()
	for i := 0; i < 10; i++ {
		result := "Hello " + firstName + "number" + strconv.Itoa(i)
		res := &greetpb.GreetManyTimesResponse{
			Result:               result,
		}
		stream.Send(res)
		time.Sleep(1000 * time.Millisecond)
	}
	return nil
}

// Client Streaming
func (*server) LongGreet(stream greetpb.GreetService_LongGreetServer) error {
	fmt.Println("LongGreet function was invoked with a streaming request")
	result := ""
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return stream.SendAndClose(&greetpb.LongGreetResponse{
				Result:               result,
			})
		}
		if err != nil {
			log.Fatalf("Error while reading client stream: %v", err)
		}

		firstName := req.GetGreeting().GetFirstName()
		result += "Hello " + firstName + "! "
	}
}

// BiDi Steaming
func (*server) GreetEveryone(stream greetpb.GreetService_GreetEveryoneServer) error {
	fmt.Println("GreetEveryone function was invoked with a streaming request")

	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			log.Fatalf("Error while reading client stream: %v", err)
			return err
		}
		firstName := req.GetGreeting().GetFirstName()
		result := "Hello " + firstName + "! "
		if err := stream.Send(&greetpb.GreetEveryoneResponse{
			Result:               result,
		}); err != nil {
				log.Fatalf("Error while sending data to client: %v", err)
				return err
		}
	}
}

func (*server) GreetWithDeadline(ctx context.Context,
	req *greetpb.GreetWithDeadlineRequest) (*greetpb.GreetWithDeadlineResponse, error) {
	fmt.Printf("GreetWithDeadline function was invoked with %v", req)
	for i := 0; i < 3; i++ {
		if ctx.Err() == context.Canceled {
			fmt.Println("The client canceled the request.")
			return nil, status.Error(codes.Canceled, "The client canceled the request.")
		}
		time.Sleep(1 * time.Second)
	}

	firstName := req.GetGreeting().GetFirstName()
	result := "Hello " + firstName
	res := &greetpb.GreetWithDeadlineResponse{
		Result:               result,
	}
	return res, nil
}

// Unary 服务端拦截器
func greetUnaryServerInterceptor(ctx context.Context, req interface{},
	info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {

	// 可以通过 Info 参数获取相关参数信息
	// 此阶段发生在 RPC 调用前，可以执行自己想要添加的逻辑
	log.Println("========== [Server Interceptor]", info.FullMethod)

	// 调用 handler 正常执行一个 Unary RPC
	m, err := handler(ctx, req)
	log.Printf("Post Proc Message: %s\n", m)

	// 返回 RPC 调用的结果
	return m, err
}

// 实现服务端 Stream 拦截，可以同时拦截接收流和发送流
// wrappedStream 通过组合实现 ServerStream 接口，通过重写 RecvMsg 和 SendMsg 方法来加入
// 用户逻辑
type wrappedStream struct {
	grpc.ServerStream
}

// 在接收到流信息时的处理逻辑
func (w *wrappedStream) RecvMsg(m interface{}) error {
	log.Printf("============[Server Stream Interceptor Wrapper] " +
		"Receive a message (Type: %T) at %s\n", m, time.Now().Format(time.RFC3339))
	return w.ServerStream.RecvMsg(m)
}

// 发送流信息时的处理逻辑
func (w *wrappedStream) SendMsg(m interface{}) error {
	log.Printf("============[Server Stream Interceptor Wrapper] " +
		"Send a message (Type: %T) at %v\n", m, time.Now().Format(time.RFC3339))
	return w.ServerStream.SendMsg(m)
}

func newWrappedStream(s grpc.ServerStream) grpc.ServerStream {
	return &wrappedStream{s}
}

// Stream 服务端拦截器
func greetServerStreamInterceptor(srv interface{}, ss grpc.ServerStream,
	info *grpc.StreamServerInfo, handler grpc.StreamHandler) error {
	log.Println("========== [Server Interceptor]", info.FullMethod)

	// 调用 handler 正常执行一个 Stream RPC，通过传入自定义的wrappedStream来实现客户化逻辑
	err := handler(srv, newWrappedStream(ss))
	if err != nil {
		log.Printf("RPC failed with error %v", err)
	}
	return err
}

// 元数据的交互
// Unary RPC
func (s *server) GreetWithMetadata(ctx context.Context, in *greetpb.GreetRequest) (*greetpb.GreetResponse, error) {
	fmt.Println("Function was invoked for exchange metadata")
	// 读取元数据
	md, metadataAvailabe := metadata.FromIncomingContext(ctx)
	if !metadataAvailabe {
		return nil, status.Errorf(codes.DataLoss, "UnaryEcho: failed to get metadata")
	}
	// 处理元数据
	if t, ok := md["timestamp"]; ok {
		fmt.Printf("timestamp from metadata:\n")
		for i, e := range t {
			fmt.Printf("====> Metadata %d. %s\n", i, e)
		}
	}

	// 创建和发送 header 或/和 trailer
	// 另一种创建元数据的方式，使用 map
	header := metadata.New(map[string]string{"location": "San Jose", "timestamp": time.Now().Format(time.StampNano)})
	_ = grpc.SendHeader(ctx, header)
	trailer := metadata.Pairs("status_code", codes.OK.String())
	_ = grpc.SetTrailer(ctx, trailer)

	res := &greetpb.GreetResponse{
		Result:    "Received the metadata",
	}

	return res, nil
}

// Stream RPC
func (s *server) GreetEveryoneWithMetadata(stream greetpb.GreetService_GreetEveryoneWithMetadataServer) error {
	fmt.Println("GreetEveryoneWithMetadata function was invoked for exchange metadata")

	fmt.Println("Function was invoked for exchange metadata")
	// 读取元数据
	md, metadataAvailabe := metadata.FromIncomingContext(stream.Context())
	if !metadataAvailabe {
		return status.Errorf(codes.DataLoss, "UnaryEcho: failed to get metadata")
	}
	// 处理元数据
	fmt.Println("Get metadata from client: ")
	for k, v := range md {
		fmt.Println(k, v)
	}

	defer func() {
		trailer := metadata.Pairs("timestamp", time.Now().Format(time.StampNano))
		stream.SetTrailer(trailer)
	}()

	header := metadata.New(map[string]string{"location": "MTV", "timestamp": time.Now().Format(time.StampNano)})
	_ = stream.SendHeader(header)

	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			log.Fatalf("Error while reading client stream: %v", err)
			return err
		}
		firstName := req.GetGreeting().GetFirstName()
		result := "Hello " + firstName + "! "
		if err := stream.Send(&greetpb.GreetEveryoneResponse{
			Result:               result,
		}); err != nil {
			log.Fatalf("Error while sending data to client: %v", err)
			return err
		}
	}
}


func main() {
	fmt.Println("Hello world")

	lis, err := net.Listen("tcp", "0.0.0.0:50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	// 不使用拦截器时的注册方式
	//s := grpc.NewServer()

	// 使用拦截器，可以同时加上 Unary 拦截器和 Stream 拦截器
	s := grpc.NewServer(grpc.UnaryInterceptor(greetUnaryServerInterceptor),
		grpc.StreamInterceptor(greetServerStreamInterceptor))
	greetpb.RegisterGreetServiceServer(s, &server{})

	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to server: %v", err)
	}
}
