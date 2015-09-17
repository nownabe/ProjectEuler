package main

import "fmt"

func fibonacci() func() int {
	last1, last2 := 0, 1
	return func() (fib int) {
		fib = last1 + last2
		last1, last2 = fib, last1
		return
	}
}

func main() {
	sum := 0
	f := fibonacci()
	r := f()
	for ; r <= 4000000; r = f() {
		if r%2 == 0 {
			sum += r
		}
	}
	fmt.Println(sum)
}
