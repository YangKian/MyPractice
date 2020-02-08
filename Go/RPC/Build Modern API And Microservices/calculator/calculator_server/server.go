package main

import (
	"context"
	"fmt"
	"gRPC_Via_Udemy/calculator/calculatorpb"
	"google.golang.org/grpc"
	"io"
	"log"
	"net"
)

type server struct {}

func (*server) Calculate(ctx context.Context,
	req *calculatorpb.CalculateRequest) (*calculatorpb.CalculateResponse, error) {
	fmt.Printf("Calculate function was invoked with %v", req)
	sum := req.FirstNum + req.SecondNum
	res := &calculatorpb.CalculateResponse{
		Result:               sum,
	}
	return res, nil
}

func (*server) ManyCalculate(req *calculatorpb.ManyCalculateRequests,
	stream calculatorpb.CalculateService_ManyCalculateServer) error {
	fmt.Printf("ManyCalculate function was invoked with %v", req)
	num := req.FirstNum
	if num < 2 {
		stream.Send(&calculatorpb.ManyCalculateResponses{Result:0})
	}
	var k int32 = 2
	for num > 1 {
		if num % k == 0 {
			stream.Send(&calculatorpb.ManyCalculateResponses{Result:k})
			num /= k
		} else {
			k += 1
		}
	}
	return nil
}

func (*server) AverageCalculate(stream calculatorpb.CalculateService_AverageCalculateServer) error {
	var temp, count float64 = 0, 0
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return stream.SendAndClose(&calculatorpb.AvgCalculateResponse{
				Result:               temp / count,
			})
		}
		if err != nil {
			log.Printf("Error while reading client stream: %v", err)
		}

		temp += req.GetNum()
		count++
	}
}

func (*server) FindMax(stream calculatorpb.CalculateService_FindMaxServer) error {
	var max int32 = 0
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			log.Fatalf("Error while reading client stream: %v", err)
		}

		if max < req.GetNum() {
			max = req.GetNum()
			if err := stream.Send(&calculatorpb.FindMaxResponse{Result:max}); err != nil {
				log.Fatalf("Error while sending data to client: %v", err)
				return err
			}
		}
	}
}

func main() {
	lis, err := net.Listen("tcp", "0.0.0.0:50051")
	if err != nil {
		log.Fatalf("listen err: %v", err)
	}

	s := grpc.NewServer()
	calculatorpb.RegisterCalculateServiceServer(s, &server{})

	if err := s.Serve(lis); err != nil {
		log.Fatalf("Server err: %v", err)
	}
}
