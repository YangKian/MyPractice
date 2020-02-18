package creational

import "testing"

func TestBuilderPattern(t *testing.T) {
	manufacturingComplex := ManufacturingDirector{}

	t.Run("test carbuilder", func(t *testing.T) {
		carBuilder := &CarBuilder{}
		manufacturingComplex.SetBuilder(carBuilder)
		manufacturingComplex.Construct()

		car := carBuilder.GetVehicle()

		if car.Wheels != 4 {
			t.Errorf("Wheels on a car must be 4 and they were %d\n", car.Wheels)
		}

		if car.Structure != "Car" {
			t.Errorf("Structure on a car must be 'Car' and they were %s\n", car.Structure)
		}

		if car.Seats != 5 {
			t.Errorf("Seats on a car must be 5 and they were %d\n", car.Seats)
		}
	})

	t.Run("test bikebuilder", func(t *testing.T) {
		bikeBuilder := &BikeBuilder{}
		manufacturingComplex.SetBuilder(bikeBuilder)
		manufacturingComplex.Construct()

		bike := bikeBuilder.GetVehicle()
		bike.Seats = 1

		if bike.Wheels != 2 {
			t.Errorf("Wheels on a bike must be 2 and they were %d\n", bike.Wheels)
		}

		if bike.Structure != "bike" {
			t.Errorf("Structure on a bike must be 'bike' and they were %s\n", bike.Structure)
		}

		// if bike.Seats != 5 {
		// 	t.Errorf("Seats on a bike must be 5 and they were %d\n", bike.Seats)
		// }
	})

}
