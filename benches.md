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
import Levenshtein
# from joblib import Parallel, delayed

n_iters = 100_000

test_left = [('Thorkel')] * n_iters
test_right = [('Thorgier')] * n_iters

print("parallel rust")
%timeit -n20 _ = xdistances.levenshtein_parallel(test_left, test_right)

print("sequential rust")
%timeit -n20 _ = [xdistances.levenshtein('Thorkel', 'Thorgier') for i in range(n_iters)]

print("sequential rust - eddie")
%timeit -n20 _ = [xdistances.eddie_levenshtein_distance('Thorkel', 'Thorgier') for i in range(n_iters)]

print("parallel rust - eddie")
%timeit -n20 _ = xdistances.eddie_levenshtein_distance_parallel(test_left, test_right)

print("sequential Python")
%timeit -n20 _ = [Levenshtein.distance('Thorkel', 'Thorgier') for i in range(n_iters)]

print("parallel Python")
# apparently this is not threadsafe.
%timeit -n20 _ = Parallel(n_jobs=-1)(delayed(Levenshtein.distance)('Thorkel', 'Thorgier') for i in range(n_iters))
```

run with `n_iters = 100_000` on m1

```
parallel rust
5.73 ms ± 169 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
sequential rust
21.8 ms ± 302 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
sequential rust - eddie
24.9 ms ± 713 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
parallel rust - eddie
6.69 ms ± 610 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
sequential Python
12.6 ms ± 360 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
parallel Python
207 ms ± 2.62 ms per loop (mean ± std. dev. of 7 runs, 20 loops each)
```

run with `n_iters = 100_000` on intel

```
parallel rust
11.3 ms ± 90.2 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
sequential rust
40.6 ms ± 1.17 ms per loop (mean ± std. dev. of 7 runs, 20 loops each)
sequential Python
20.8 ms ± 63.3 µs per loop (mean ± std. dev. of 7 runs, 20 loops each)
```

```
import xdistances

n_iters = 100_000

test_right = [('Thorkel')] * n_iters
test_left = [('Thorgier'), ('Thornulf'), ('Thoracic')]

print("parallel rust")
%timeit -n20 _ = xdistances.levenshtein_max_similarity(test_left, test_right)

%timeit -n200 _ = min(xdistances.levenshtein_parallel(["hamming"] * 3, ["hammers", "hammer", "ham"]))
43.1 µs ± 2.86 µs per loop (mean ± std. dev. of 7 runs, 200 loops each)
%timeit -n200 _ = xdistances.levenshtein_min_similarity(["hamming"], ["hammers", "hammer", "ham"])
433 ns ± 37.8 ns per loop (mean ± std. dev. of 7 runs, 200 loops each)
```

```python
import timeit
import random
import string
import xdistances

# Function to generate a random string of a given length
def random_string(length):
    return ''.join(random.choice(string.ascii_lowercase) for _ in range(length))

# Generate random data
num_targets = 10  # Number of target strings
target_length = 8  # Length of each target string
num_sources = 100  # Number of source strings
source_length = 8  # Length of each source string
target_str = random_string(target_length)

targets = [target_str] * num_targets
source_strings = [random_string(source_length) for _ in range(num_sources)]

# Benchmark functions using generated random data
def levenshtein_parallel_benchmark():
    a = min(xdistances.levenshtein_parallel(targets, source_strings))

def levenshtein_min_similarity_benchmark():
    _ = xdistances.levenshtein_min_similarity([target_str], source_strings)

# Number of loops for timeit to run
num_loops = 200

# Measure time for levenshtein_parallel
parallel_time = 1000 * timeit.timeit("levenshtein_parallel_benchmark()", setup="from __main__ import levenshtein_parallel_benchmark", number=num_loops)
print(f"Time taken for levenshtein_parallel: {parallel_time:.6f} milliseconds")

# Measure time for levenshtein_min_similarity
min_similarity_time = 1000 * timeit.timeit("levenshtein_min_similarity_benchmark()", setup="from __main__ import levenshtein_min_similarity_benchmark", number=num_loops)
print(f"Time taken for levenshtein_min_similarity: {min_similarity_time:.6f} milliseconds")


```

this is probably not a fair comparison vs parallel python since delegating to workers has a fixed overhead that potentially is amortized over more iterations. but this is pretty sweet, easy 2x improvement.
