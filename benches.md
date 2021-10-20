## informal levenshtein benchmarking

very informal, not scientific, etc

## mac m1

```
In [2]: %%timeit
   ...:
   ...: _ = Levenshtein.jaro('Thorkel', 'Thorgier')
120 ns ± 0.899 ns per loop (mean ± std. dev. of 7 runs, 10000000 loops each)

In [3]: import xdistances

In [4]: %%timeit
   ...:
   ...: _ = xdistances.jaro('Thorkel', 'Thorgier')
191 ns ± 1.2 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

In [7]: %%timeit
   ...:
   ...: _ = xdistances.levenshtein('Thorkel', 'Thorgier')
208 ns ± 2.57 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

In [8]: %%timeit
   ...:
   ...: _ = Levenshtein.distance('Thorkel', 'Thorgier')
104 ns ± 0.22 ns per loop (mean ± std. dev. of 7 runs, 10000000 loops each)

In [3]: %%timeit
   ...: _ = xdistances.levenshtein_simd('Thorkel', 'Thorgier')
   ...:
   ...:
549 ns ± 10.8 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)
```

C seems hard to beat

note that these were run on an M1 macbook pro and that they may not be representative of intel architecture. especially the SIMD, don't think M1 supports that?

## intel

on a 8 core intel, which should have avx512?

```
In [3]: %%timeit
   ...: _ = xdistances.levenshtein_simd('Thorkel', 'Thorgier')
   ...: 
   ...: 
   ...: 
437 ns ± 0.547 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

In [4]: %%timeit
   ...: _ = xdistances.levenshtein('Thorkel', 'Thorgier')
   ...: 
   ...: 
   ...: 
368 ns ± 1.68 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

In [2]: %%timeit
   ...: _ = Levenshtein.distance('Thorkel', 'Thorgier')
   ...: 
   ...: 
175 ns ± 0.27 ns per loop (mean ± std. dev. of 7 runs, 10000000 loops each)
```

wild that every command is so much slower on the intel than the m1 (thank you apple), but also weirdly the simd heuristic is not performing better. 

## parallelism

```python
import xdistances

n_iters = 1_000_000

test_left = [('Thorkel')] * n_iters
test_right = [('Thorgier')] * n_iters

%timeit _ = xdistances.levenshtein_parallel(test_left, test_right)

%timeit _ = [xdistances.levenshtein('Thorkel', 'Thorgier') for i in range(n_iters)]
```

with n_iters = 1000

```
34 µs ± 2.34 µs per loop (mean ± std. dev. of 7 runs, 10000 loops each)
204 µs ± 726 ns per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

with n_iters = 10000

```
18 µs ± 7.92 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
2.17 ms ± 40.1 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)
```