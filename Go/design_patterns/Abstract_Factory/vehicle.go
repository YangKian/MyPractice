package abstract_factory

import (
	"errors"
	"fmt"
)

type Vehicle interface {
	NumWheels() int
	NumSeats() int
}

const (
	LuxuryCarType = iota + 1
	FamilyCarType
)

type CarFactory struct{}

func (c *CarFactory) Build(v int) (Vehicle, error) {
	switch v {
	case LuxuryCarType:
		return new(LuxuryCar), nil
	case FamilyCarType:
		return new(FamilyCar), nil
	default:
		return nil, errors.New(fmt.Sprintf("Vehicle of type %d not recognized\n", v))
	}
}

const (
	SportMotorbikeType = iota + 1
	CruiseMotorbikeType
)

type MotorbikeFactory struct{}

func (c *MotorbikeFactory) Build(v int) (Vehicle, error) {
	switch v {
	case SportMotorbikeType:
		return new(SportMotorbike), nil
	case CruiseMotorbikeType:
		return new(CruiseMotorbike), nil
	default:
		return nil, errors.New(fmt.Sprintf("Vehicle of type %d not recognized\n", v))
	}
}
