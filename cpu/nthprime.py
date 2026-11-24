def nth_prime(n):
    primes = [2]
    x = 3
    while len(primes) < n:
        is_prime = True
        for p in primes:
            if p * p > x:
                break
            if x % p == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(x)
        x += 2
    return primes[n - 1]


print(nth_prime(1000000))
