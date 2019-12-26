package main

//示例来自 sort 包内的文档
import (
	"fmt"
	"sort"
)

type earthMass float64
type au float64

type Planet struct {
	name string
	mass earthMass
	distance au
}

//By定义了参数排序的方法
type By func(p1, p2 *Planet) bool

//Sort是函数类型变量By的方法，封装了sort.Sort的调用
func (by By) Sort(planets []Planet) {
	ps := &planetSorter {
		planets: planets,
		by: by,
	}
	sort.Sort(ps)
}

//sort接口实现的载体，保存了待排序的变量和排序的方法
type planetSorter struct {
	planets []Planet
	by func(p1, p2 *Planet) bool //使用在Less方法中的闭包。定义了排序的方法
}

func (s *planetSorter) Len() int {
	return len(s.planets)
}

func (s *planetSorter) Swap(i, j int) {
	s.planets[i], s.planets[j] = s.planets[j], s.planets[i]
}

func (s *planetSorter) Less(i, j int) bool {
	return s.by(&s.planets[i], &s.planets[j])
}

var planets = []Planet {
	{"Mercury", 0.055, 0.4},
	{"Venus", 0.815, 0.7},
	{"Earth", 1.0, 1.0},
	{"Mars", 0.107, 1.5},
}

func main() {
	name := func(p1, p2 *Planet) bool {
		return p1.name < p2.name
	}
	mass := func(p1, p2 *Planet) bool {
		return p1.mass < p2.mass
	}
	distance := func(p1, p2 *Planet) bool {
		return p1.distance < p2.distance
	}

	By(name).Sort(planets)
	fmt.Println("Byname: ", planets)
	By(mass).Sort(planets)
	fmt.Println("Bymass: ", planets)
	By(distance).Sort(planets)
	fmt.Println("Bydistance: ", planets)

}
