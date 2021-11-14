package main

func mergeSort(s []int, start, end int) {
	if start >= end {
		return
	}

	mid := start + ((end - start) >> 1)
	mergeSort(s, start, mid)
	mergeSort(s, mid + 1, end)

	l, r := start, mid + 1
	tmp := make([]int, 0, end - start + 1)
	for l <= mid && r <= end {
		if s[l] <= s[r] {
			tmp = append(tmp, s[l])
			l++
		} else {
			tmp = append(tmp, s[r])
			r++
		}
	}

	if l <= mid {
		tmp = append(tmp, s[l: mid + 1]...)
	}
	if r <= end {
		tmp = append(tmp, s[r: end + 1]...)
	}
	copy(s[start:end + 1], tmp)
}
