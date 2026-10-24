# Pin benchmarks

The same programs written in [Pin](https://github.com/pin-language/pin), Rust, Go, PHP and Python, plus the
same JSON API in Pin, Go and PHP. Every result is checked: all languages must print the same answer.

## CPU (`cpu/`)

Apple M2 (8 cores), macOS 14 · Pin 0.1.0, Rust 1.95 (`-C opt-level=3`), Go 1.26, PHP 8.5.3 (with and without
tracing JIT), Python 3.15 · best of 3 runs · 2026-10-24

| benchmark | Pin | Rust | Go | PHP + JIT | PHP | Python |
|---|---|---|---|---|---|---|
| sieve — primes below 20M | **0.048s** | 0.074s | 0.046s | 0.642s | 1.385s | 4.420s |
| collatz — longest chain below 1M | **0.163s** | 0.133s | 0.174s | 0.715s | 3.223s | 6.370s |
| pi — 100M Leibniz terms | **0.098s** | 0.096s | 0.093s | 0.943s | 2.260s | 13.265s |
| listsum — build 5M-item lists ×10, sum | **0.116s** | 0.141s | 0.178s | 0.459s | 1.210s | 5.280s |
| fib(35) — recursion* | 0.028s | 0.028s | 0.031s | 0.228s | 0.956s | 0.806s |

\* clang optimises this recursion in ways that say little about the language; do not quote it.

Read with care:
- These are small CPU loops, not applications. They show that Pin compiles to code as fast as Rust and Go,
  and 4–13× faster than PHP with JIT.
- macOS cannot pin a process to a CPU core, so differences under ~20% are noise (Rust's sieve took 0.047s in
  an earlier run and 0.074s here).
- Peak memory is in `cpu/results.json`, but on macOS it is unreliable for `listsum`: three runs of the same
  binary gave 106–204 MB for Pin and 120–236 MB for Rust. On Linux the same program stays at 40 MB (Pin and
  Rust) — see `cpu/results-linux-2core.json` (2-core x86-64 container, earlier Pin version).
- Pin's `+ - *` do not check for int overflow (neither do Rust release builds or Go).

Run: `python3 cpu/run.py` (or `python3 cpu/run.py sieve pi`). It builds every version first; it needs
`pin` on the PATH (or `PIN=/path/to/pin`), `rustc`, `go`, `php`, `python3` and a C compiler.

## HTTP (`http/`)

The same 3-endpoint JSON API (`/hello`, `/user/42`, `/primes/20000`) in Pin, Go `net/http`, PHP with
FrankenPHP worker mode, and classic `php -S`. Measured once on a 2-core Linux container: each server pinned
to core 0, `wrk` with 64 connections pinned to core 1, 10 seconds; every server returned identical bytes.

| endpoint | Pin | Go net/http | FrankenPHP worker | php -S |
|---|---|---|---|---|
| /hello | 132,268 req/s* | 41,696 | 10,439 | 4,703 |
| /user/42 (JSON) | 94,897 | 37,999 | 10,887 | 4,441 |
| /primes/20000 (CPU) | 17,019 | 14,295 | 3,518 | 2,324 |
| server memory | 2.3 MB | 13 MB | 102 MB | 96–102 MB |

\* the load generator was the bottleneck (Pin's server at 80% CPU).

Read with care: Pin's server does far less than Go's (no TLS, HTTP/2, header API or timeouts), which explains
part of the gap on `/hello` and `/user`. `/primes` is the fairest language comparison (Pin 1.2× Go, 4.8×
FrankenPHP). FrankenPHP 1.13.2 / PHP 8.5.12 crashed on `/primes` with `opcache.jit=tracing`, so it ran with
`jit=function`. `php -S` closes every connection (no keep-alive) and is a development server, a weak stand-in
for PHP-FPM. These numbers predate Pin 0.1.0's safety changes; they have not been re-measured.

Run: build `http/server.pn` with `pin build http/server.pn -o http/server` and `http/server.go` with
`go build -o http/server-go http/server.go`, start the four servers on ports 8081–8084 (see the top of
`http/loadtest.py`), then `python3 http/loadtest.py /path/to/wrk`. On macOS there is no CPU pinning; say so
next to any numbers.

## License

MIT — see [LICENSE](LICENSE).
