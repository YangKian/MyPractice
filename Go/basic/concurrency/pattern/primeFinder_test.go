package main

import "testing"

func TestPrimeFinderr(t *testing.T) {
	t.Run("test NaviePrimeFinder", func(t *testing.T) {
		NaviePrimeFinder(10)
	})

	t.Run("test FanOutPrimeFinder", func(t *testing.T) {
		FanOutPrimeFinder(50)
	})
}

func BenchmarkPrimeFinder(b *testing.B) {
	b.Run("benchmarkNaviePrimeFinder", func(b *testing.B) {
		b.StartTimer()
		NaviePrimeFinder(b.N)
	})

	b.Run("benchmarkFanOutPrimeFinder", func(b *testing.B) {
		b.StartTimer()
		FanOutPrimeFinder(b.N)
	})
}