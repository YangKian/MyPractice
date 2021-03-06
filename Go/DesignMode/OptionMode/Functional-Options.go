package main

import "fmt"

type House struct {
	Material string
	HasFirePlace bool
	Floors int
}

func NewHouseOld() *House {
	const (
		defaultFloors = 2
		defaultHasFirePlace = true
		defaultMaterial = "wood"
	)

	h := &House{
		Material: defaultMaterial,
		HasFirePlace: defaultHasFirePlace,
		Floors: defaultFloors,
	}
	return h
}

type HouseOption func(*House)

func WithConcrete() HouseOption {
	return func(h *House) {
		h.Material = "concrete"
	}
}

func WithoutFirePlace() HouseOption {
	return func(h *House) {
		h.HasFirePlace = false
	}
}

func WithFloors(floors int) HouseOption {
	return func(h *House) {
		h.Floors = floors
	}
}

// 批量传参，且不用考虑顺序问题和 API 参数过多的问题
func NewHouse(opts ...HouseOption) *House {
	const (
		defaultFloors = 2
		defaultHasFirePlace = true
		defaultMaterial = "wood"
	)

	// 实现默认配置
	h := &House{
		Material: defaultMaterial,
		HasFirePlace: defaultHasFirePlace,
		Floors: defaultFloors,
	}

	// 用自定义配置修改默认配置
	for _, opt := range opts {
		opt(h)
	}
	return h
}

func main() {
	// 显示指定配置，代码语义更明确，且将指定的配置定义好后，可以重复使用，减少了
	// 复杂配置修改时可能导致的出错问题
	h := NewHouse(WithConcrete(), WithFloors(4), WithoutFirePlace())
	fmt.Printf("%#v", h)
}
