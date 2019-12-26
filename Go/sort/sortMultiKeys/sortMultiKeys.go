package main

//示例来自 sort 包内的文档
import (
	"fmt"
	"sort"
)

type Change struct {
	user string
	language string
	lines int
}

type lessFunc func(p1, p2 *Change) bool

type multiSorter struct {
	changes []Change
	less []lessFunc
}

func (ms *multiSorter) Sort(changes []Change) {
	ms.changes = changes
	sort.Sort(ms)
}

func OrderedBy(less ...lessFunc) *multiSorter {
	return &multiSorter{
		less:    less,
	}
}

func (ms *multiSorter) Len() int {
	return len(ms.changes)
}

func (ms *multiSorter) Less(i, j int) bool {
	p, q := &ms.changes[i], &ms.changes[j]
	var k int
	for k = 0; k < len(ms.less) - 1; k++ { //尝试前 k - 1 个比较
		less := ms.less[k]
		switch {
		case less(p, q): //p < q
			return true
		case less(q, p): //p > q
			return false
		}
		//if p == q, 尝试下一个比较
	}
	return ms.less[k](p, q)
}

func (ms *multiSorter) Swap(i, j int) {
	ms.changes[i], ms.changes[j] = ms.changes[j], ms.changes[i]
}

var changes = []Change{
	{"gri", "Go", 100},
	{"ken", "C", 150},
	{"glenda", "Go", 200},
	{"rsc", "Go", 200},
	{"r", "Go", 100},
	{"ken", "Go", 200},
	{"dmr", "C", 100},
	{"r", "C", 150},
	{"gri", "Smalltalk", 80},
}

func main() {
	user := func(c1, c2 *Change) bool {
		return c1.user < c2.user
	}
	language := func(c1, c2 *Change) bool {
		return c1.language < c2.language
	}
	increasingLines := func(c1, c2 *Change) bool {
		return c1.lines < c2.lines
	}
	decreasingLines := func(c1, c2 *Change) bool {
		return c1.lines > c2.lines // Note: > orders downwards.
	}

	//单个变量排序
	OrderedBy(user).Sort(changes)
	fmt.Println("By user: ", changes)

	//多变量排序
	OrderedBy(user, increasingLines).Sort(changes)
	fmt.Println("By user, <lines: ", changes)

	OrderedBy(user, decreasingLines).Sort(changes)
	fmt.Println("By user, >lines: ", changes)

	OrderedBy(language, increasingLines).Sort(changes)
	fmt.Println("By language, <lines: ", changes)

	OrderedBy(language, increasingLines, user).Sort(changes)
	fmt.Println("By language, <lines, user: ", changes)
}

