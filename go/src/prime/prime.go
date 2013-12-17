package prime

import (
	"math"
)

// Simple function to check if n is prime. Naive (i.e. slow) implementation.
func Isprime(n int) bool {
	for i := 2; i <= int(math.Sqrt(float64(n))); i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// Generate array of ints containing all primes up to value n
func Genprimes(n int) []int {
	l := []int{2}
	for i := 3; i <= n; i = i + 2 {
		if Isprime(i) {
			l = append(l, i)
		}
	}
	return l
}

// Fast prime seive. Returns array of bools
func PrimeSieve(n int) (primes []bool) {
    primes = make([]bool, n+1)
    for i, _ := range primes {
        primes[i] = true
        if i == 0 || i == 1 {
            primes[i] = false
        }
    }
    for j, v := range primes {
        if v == true {
            multlim := n/j
            for i:=2;i<= multlim;i++{
                primes[j*i] = false
            }
        }
    }
    return primes
}

// Generate the first n primes
func GenNprimes(n int) []int {
	l := []int{2}
	for i := 3; len(l) < n; i = i + 2 {
		if Isprime(i) {
			l = append(l, i)
		}
	}
	return l
}

//returns the prime factors of n
//requires a prime sieve to be passed
func PrimeFacts(n int, primes []int) []int {
    facts := make([]int, 0, 10)
    for i:=0; i < len(primes); {
        p := primes[i]
        if n % p == 0 {
            facts = append(facts, p)
            n = n/p
            if n == 1 {
                break
            }
            continue
        }
        i++
    }
    return facts
}
