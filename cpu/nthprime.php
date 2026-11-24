<?php
function nth_prime(int $n): int {
    $primes = [2];
    $x = 3;
    while (count($primes) < $n) {
        $is_prime = true;
        foreach ($primes as $p) {
            if ($p * $p > $x) { break; }
            if ($x % $p == 0) { $is_prime = false; break; }
        }
        if ($is_prime) { $primes[] = $x; }
        $x += 2;
    }
    return $primes[$n - 1];
}
echo nth_prime(1000000), "\n";
