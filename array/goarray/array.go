package main

import (
	"fmt"
	"time"

	"github.com/Axect/Numeric/array"
)

func main() {
	start := time.Now()
	A := array.Create(0, 1, 1E+08)
	for i := range A {
		A[i] = float64(i + 1)
	}
	s := 0.
	for j := range A {
		s += float64(j + 1)
	}
	elapsed := time.Since(start)
	fmt.Println(s)
	fmt.Println(elapsed)
}
