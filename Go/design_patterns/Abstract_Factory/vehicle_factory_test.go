package abstract_factory

import "testing"

func TestMotorbikeFactory(t *testing.T) {
	MotorbikeF, err := BuildFactory(MotorbikeFactoryType)
	if err != nil {
		t.Fatal(err)
	}

	motobikeVehicle, err := MotorbikeF.Build(SportMotorbikeType)
	if err != nil {
		t.Fatal(err)
	}

	t.Logf("Motorbike vehicle has %d wheels\n", motobikeVehicle.NumWheels())

	sportBike, ok := motobikeVehicle.(Motorbike)
	if !ok {
		t.Fatal("Struct asseertion has failed")
	}
	t.Logf("Sport motorbike has type %d\n", sportBike.GetMotorbikeType())

}

func TestCarFactory(t *testing.T) {
	carF, err := BuildFactory(CarFactoryType)
	if err != nil {
		t.Fatal(err)
	}

	carVehicle, err := carF.Build(LuxuryCarType)
	if err != nil {
		t.Fatal(err)
	}

	t.Logf("Car vehicle has %d wheels\n", carVehicle.NumWheels())

	luxuryCar, ok := carVehicle.(Car)
	if !ok {
		t.Fatal("Struct asseertion has failed")
	}
	t.Logf("Luxury car has %d doors.\n", luxuryCar.NumDoors())

}
