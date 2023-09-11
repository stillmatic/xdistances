# xdistances

Python wrapper on [strsim](https://crates.io/crates/strsim) a [Rust](https://www.rust-lang.org) implementations of [string similarity metrics]:

- [Hamming]
- [Levenshtein] - distance & normalized
- [Optimal string alignment]
- [Damerau-Levenshtein] - distance & normalized
- [Jaro and Jaro-Winkler] - this implementation of Jaro-Winkler does not limit the common prefix length
- [Sorensen-Dice]

The normalized versions return values between `0.0` and `1.0`, where `1.0` means
an exact match.

## Installation

`pip install git+https://github.com/stillmatic/xdistances`

## Usage

### Examples

Compute pairwise distances

```python
>>> import xdistances
>>> xdistances.hamming("hamming", "hammers")
3
>>> xdistances.hamming("hamming", "hammer")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: Lenght mismatch
>>> xdistances.levenshtein("kitten", "sitting")
3
>>> xdistances.normalized_levenshtein("kitten", "sitting")
0.5714285714285714
>>> xdistances.osa_distance("ac", "cba")
3
>>> xdistances.damerau_levenshtein("ac", "cba")
2
>>> xdistances.normalized_damerau_levenshtein("levenshtein", "löwenbräu")
0.2727272727272727
>>> xdistances.jaro("Friedrich Nietzsche", "Jean-Paul Sartre")
0.39188596491228067
>>> xdistances.jaro_winkler("cheeseburger", "cheese fries")
0.9111111111111111
```

Compute zipped pairwise distances

```python
>>> import xdistances
>>> xdistances.levenshtein_parallel(["hamming", "hamming", "hamming"], ["hammers", "hammer", "ham"])
[3, 3, 4]
>>> xdistances.levenshtein_parallel(["humming", "hummer", "humor"], ["hammers", "hammer", "ham"])
[4, 1, 3]
```

Here, `hamming` is repeated 3x and compared against `hammers`, `hammer`, and `ham`. In the second example, `humming`, `hummer`, and `humor` are compared against `hammers`, `hammer`, and `ham`. These are pairwise and not all combinations.

It's often useful to compute the distance between a single string and a list of strings (using min/max)

```python
>>> import xdistances
>>> min(xdistances.levenshtein_parallel(["hamming"] * 3, ["hammers", "hammer", "ham"]))
3
>>> xdistances.levenshtein_min_similarity(["hamming"], ["hammers", "hammer", "ham"])
3
```

This is a toy benchmark but the optimized Rust version is about 40-100x faster than the Python version.

## Contributing

If you don't want to install Rust itself, you can run `$ ./dev` for a
development CLI if you have [Docker] installed.

Benchmarks require a Nightly toolchain. Run `$ cargo +nightly bench`.

## Credits

strsim: [crates](https://crates.io/crates/strsim) - [Github](https://github.com/dguo/strsim-rs)

## License

[MIT](https://github.com/OvalMoney/xdistances/blob/master/LICENSE)

[string similarity metrics]: http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]: http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]: http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]: http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]: http://en.wikipedia.org/wiki/Hamming_distance
[Sorensen-Dice]: http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient
[Optimal string alignment]: https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance
[Docker]: https://docs.docker.com/engine/installation/
