very informal

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
   ...: xdistances.levenshtein_simd(b'Thorkel', b'Thorgier')
   ...:
   ...:
549 ns ± 10.8 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)
```

C seems hard to beat

note that these were run on an M1 macbook pro and that they may not be representative of intel architecture. especially the SIMD, don't think M1 supports that?