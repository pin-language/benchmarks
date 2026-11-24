package main
import "fmt"
func nthPrime(n int) int {
	primes := []int{2}
	x := 3
	for len(primes) < n {
		isPrime := true
		for _, p := range primes {
			if p*p > x { break }
			if x%p == 0 { isPrime = false; break }
		}
		if isPrime { primes = append(primes, x) }
		x += 2
	}
	return primes[n-1]
}
func main() {
	fmt.Println(nthPrime(1000000))
}
