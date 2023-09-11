#![deny(clippy::all)]

extern crate pyo3;
use pyo3::exceptions;
use pyo3::prelude::*;
use rayon::prelude::*;
use paste::paste;
use eddie::*;
use std::cmp::Ordering;

extern crate strsim;
extern crate eddie;

macro_rules! wrapper {
    ($(#[$doc:meta])* hamming -> $type:ty) => {
        $(#[$doc])*
        #[pyfunction]
        fn hamming(a: &str, b: &str) -> PyResult<$type> {
            match strsim::hamming(a, b) {
                Ok(distance) => Ok(distance),
                Err(_) => Err(exceptions::PyValueError::new_err("Length mismatch")),
            }
        }
    };
    ($(#[$doc:meta])* $name:ident -> $type:ty) => {
        $(#[$doc])*
        #[pyfunction]
        fn $name(a: &str, b: &str) -> PyResult<$type> {
            Ok(strsim::$name(a, b))
        }
    };
}

macro_rules! parallel_wrapper {        
    ($(#[$doc:meta])* $name:ident -> $type:ty) => {
        paste! {
            $(#[$doc])*
            #[pyfunction]
            fn [<$name _parallel>] (left: Vec<&str>, right: Vec<&str>) -> PyResult<$type> {
                Ok(
                    (left, right)
                    .into_par_iter()
                    .map(|(x, y)| strsim::$name(x, y))
                    .collect()
                )
            }
        }
    };
}

macro_rules! max_similarity_wrapper {
    ($(#[$doc:meta])* $name:ident -> $type:ty) => {
        paste::item! {
            $(#[$doc])*
            #[pyfunction]
            fn [<$name _max_similarity>] (targets: Vec<&str>, source_strings: Vec<&str>) -> PyResult<Vec<$type>> {
                let zero = <$type>::default(); // Get zero value for type
                let max_distances: Vec<$type> = (targets.par_iter()
                    .map(|target| {
                        source_strings.par_iter()
                            .map(|source_str| strsim::$name(target, source_str))
                            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                            .unwrap_or(zero)
                    })
                    .collect()
                );
                Ok(max_distances)
            }
        }
    };
}

macro_rules! min_similarity_wrapper {
    ($(#[$doc:meta])* $name:ident -> $type:ty) => {
        paste::item! {
            $(#[$doc])*
            #[pyfunction]
            fn [<$name _min_similarity>] (targets: Vec<&str>, source_strings: Vec<&str>) -> PyResult<Vec<$type>>{
                let zero = <$type>::default(); // Get zero value for type
                let min_distances: Vec<$type> = (targets.par_iter()
                    .map(|target| {
                        source_strings.par_iter()
                            .map(|source_str| strsim::$name(target, source_str))
                            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                            .unwrap_or(zero)
                    })
                    .collect()
                );
                Ok(min_distances)
            }
        }
    };
}


#[pyfunction]
fn eddie_levenshtein_distance (left: &str, right: &str) -> PyResult<usize> {
    let lev: Levenshtein = Levenshtein::new();
    Ok(lev.distance(left, right))
}

#[pyfunction]
fn eddie_levenshtein_distance_parallel (left: Vec<&str>, right: Vec<&str>) -> PyResult<Vec<usize>> {
    let lev: Levenshtein = Levenshtein::new();
    Ok(
        left.iter()
        .zip(right)
        .map(|(x, y)| lev.distance(x, y))
        .collect()
    )
}



wrapper! {
    /// hamming(a, b)
    ///
    /// Calculates the number of positions in the two strings where the characters
    /// differ. Returns an error if the strings have different lengths.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    /// :raises ValueError: if a and b have a different lengths
    hamming -> usize
}

wrapper! {
    /// levenshtein(a, b)
    ///
    /// Calculates the minimum number of insertions, deletions, and substitutions
    /// required to change one string into the other.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    levenshtein -> usize
}

wrapper! {
    /// osa_distance(a, b)
    ///
    /// Like Levenshtein but allows for adjacent transpositions. Each substring can
    /// only be edited once.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    osa_distance -> usize
}

wrapper! {
    /// damerau_levenshtein(a, b)
    ///
    /// Like optimal string alignment, but substrings can be edited an unlimited
    /// number of times, and the triangle inequality holds.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    damerau_levenshtein -> usize
}

wrapper! {
    /// normalized_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Levenshtein algorithm between 0.0 and
    /// 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: float
    normalized_levenshtein -> f64
}

wrapper! {
    /// normalized_damerau_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Damerau–Levenshtein algorithm between
    /// 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: float
    normalized_damerau_levenshtein -> f64
}

wrapper! {
    /// jaro(a, b)
    ///
    /// Calculates the Jaro similarity between two strings. The returned value
    /// is between 0.0 and 1.0 (higher value means more similar).
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: similarity
    /// :rtype: float
    jaro -> f64
}

wrapper! {
    /// jaro_winkler(a, b)
    ///
    /// Like Jaro but gives a boost to strings that have a common prefix.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: similarity
    /// :rtype: float
    jaro_winkler -> f64
}

wrapper! {
    /// sorensen_dice(a, b)
    ///
    /// Calculates a Sørensen-Dice similarity distance using bigrams. See 
    /// http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: similarity
    /// :rtype: float
    sorensen_dice -> f64
}

parallel_wrapper! {
    /// levenshtein(a, b)
    ///
    /// Calculates the minimum number of insertions, deletions, and substitutions
    /// required to change one string into the other. Operates in parallel over two lists of strings.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: distance
    /// :rtype: int
    levenshtein -> Vec<usize>
}

parallel_wrapper! {
    /// osa_distance(a, b)
    ///
    /// Like Levenshtein but allows for adjacent transpositions. Each substring can
    /// only be edited once.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: distance
    /// :rtype: int
    osa_distance -> Vec<usize>
}

parallel_wrapper! {
    /// damerau_levenshtein(a, b)
    ///
    /// Like optimal string alignment, but substrings can be edited an unlimited
    /// number of times, and the triangle inequality holds.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: distance
    /// :rtype: int
    damerau_levenshtein -> Vec<usize>
}

parallel_wrapper! {
    /// normalized_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Levenshtein algorithm between 0.0 and
    /// 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: distance
    /// :rtype: float
    normalized_levenshtein -> Vec<f64>
}

parallel_wrapper! {
    /// normalized_damerau_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Damerau–Levenshtein algorithm between
    /// 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: distance
    /// :rtype: float
    normalized_damerau_levenshtein -> Vec<f64>
}

parallel_wrapper! {
    /// jaro(a, b)
    ///
    /// Calculates the Jaro similarity between two strings. The returned value
    /// is between 0.0 and 1.0 (higher value means more similar).
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: similarity
    /// :rtype: float
    jaro -> Vec<f64>
}

parallel_wrapper! {
    /// jaro_winkler(a, b)
    ///
    /// Like Jaro but gives a boost to strings that have a common prefix.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: similarity
    /// :rtype: float
    jaro_winkler -> Vec<f64>
}

parallel_wrapper! {
    /// sorensen_dice(a, b)
    ///
    /// Calculates a Sørensen-Dice similarity distance using bigrams. See 
    /// http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient.
    ///
    /// :param Vec<&str> a: base strings
    /// :param Vec<&str> b: strings to compare
    /// :return: similarity
    /// :rtype: float
    sorensen_dice -> Vec<f64>
}

max_similarity_wrapper! {
    /// levenshtein_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum Levenshtein distance between each target string
    /// and a list of "known bad strings". The Levenshtein distance measures the minimum
    /// number of insertions, deletions, and substitutions required to change one string
    /// into the other.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<usize>
    levenshtein -> usize
}

max_similarity_wrapper! {
    /// osa_distance_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum optimal string alignment distance between each target
    /// string and a list of "known bad strings". This is similar to Levenshtein distance but
    /// allows for adjacent transpositions.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<usize>
    osa_distance -> usize
}

max_similarity_wrapper! {
    /// damerau_levenshtein_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum Damerau-Levenshtein distance between each target
    /// string and a list of "known bad strings". This algorithm allows for transpositions and
    /// is more flexible than the simple Levenshtein distance.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<usize>
    damerau_levenshtein -> usize
}

max_similarity_wrapper! {
    /// normalized_levenshtein_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum normalized Levenshtein similarity between each target
    /// string and a list of "known bad strings". The score is between 0.0 and 1.0, with
    /// 1.0 meaning the strings are identical.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<f64>
    normalized_levenshtein -> f64
}

max_similarity_wrapper! {
    /// normalized_damerau_levenshtein_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum normalized Damerau-Levenshtein similarity between each target
    /// string and a list of "known bad strings". The score is between 0.0 and 1.0, with
    /// 1.0 meaning the strings are identical.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<f64>
    normalized_damerau_levenshtein -> f64
}

max_similarity_wrapper! {
    /// jaro_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum Jaro similarity between each target string
    /// and a list of "known bad strings". The Jaro similarity score is between 0.0 and 1.0,
    /// where a higher score indicates more similarity.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<f64>
    jaro -> f64
}

max_similarity_wrapper! {
    /// jaro_winkler_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum Jaro-Winkler similarity between each target string
    /// and a list of "known bad strings". This is similar to the Jaro similarity but gives
    /// a boost to strings that have a common prefix.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<f64>
    jaro_winkler -> f64
}

max_similarity_wrapper! {
    /// sorensen_dice_max_similarity(targets, source_strings)
    ///
    /// Calculates the maximum Sørensen-Dice similarity between each target string
    /// and a list of "known bad strings". This uses bigrams to calculate the similarity
    /// score, which is between 0.0 and 1.0, where a higher score indicates more similarity.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of maximum similarities
    /// :rtype: Vec<f64>
    sorensen_dice -> f64
}

// min similarity

min_similarity_wrapper! {
    /// levenshtein_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum Levenshtein distance between each target string
    /// and a list of "known bad strings". The Levenshtein distance measures the minimum
    /// number of insertions, deletions, and substitutions required to change one string
    /// into the other.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<usize>
    levenshtein -> usize
}


min_similarity_wrapper! {
    /// osa_distance_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum optimal string alignment distance between each target
    /// string and a list of "known bad strings". This is similar to Levenshtein distance but
    /// allows for adjacent transpositions.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<usize>
    osa_distance -> usize
}

min_similarity_wrapper! {
    /// damerau_levenshtein_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum Damerau-Levenshtein distance between each target
    /// string and a list of "known bad strings". This algorithm allows for transpositions and
    /// is more flexible than the simple Levenshtein distance.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<usize>
    damerau_levenshtein -> usize
}

min_similarity_wrapper! {
    /// normalized_levenshtein_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum normalized Levenshtein similarity between each target
    /// string and a list of "known bad strings". The score is between 0.0 and 1.0, with
    /// 1.0 meaning the strings are identical.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<f64>
    normalized_levenshtein -> f64
}

min_similarity_wrapper! {
    /// normalized_damerau_levenshtein_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum normalized Damerau-Levenshtein similarity between each target
    /// string and a list of "known bad strings". The score is between 0.0 and 1.0, with
    /// 1.0 meaning the strings are identical.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<f64>
    normalized_damerau_levenshtein -> f64
}

min_similarity_wrapper! {
    /// jaro_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum Jaro similarity between each target string
    /// and a list of known source strings. The Jaro similarity score is between 0.0 and 1.0,
    /// where a higher score indicates more similarity.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<f64>
    jaro -> f64
}

min_similarity_wrapper! {
    /// jaro_winkler_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum Jaro-Winkler similarity between each target string
    /// and a list of "known bad strings". This is similar to the Jaro similarity but gives
    /// a boost to strings that have a common prefix.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<f64>
    jaro_winkler -> f64
}

min_similarity_wrapper! {
    /// sorensen_dice_min_similarity(targets, source_strings)
    ///
    /// Calculates the minimum Sørensen-Dice similarity between each target string
    /// and a list of "known bad strings". This uses bigrams to calculate the similarity
    /// score, which is between 0.0 and 1.0, where a higher score indicates more similarity.
    ///
    /// :param Vec<str> targets: list of target strings
    /// :param Vec<str> source_strings: list of source strings
    /// :return: vector of minimum similarities
    /// :rtype: Vec<f64>
    sorensen_dice -> f64
}


#[pymodule]
fn xdistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hamming))?;
    m.add_wrapped(wrap_pyfunction!(levenshtein))?;
    m.add_wrapped(wrap_pyfunction!(osa_distance))?;
    m.add_wrapped(wrap_pyfunction!(damerau_levenshtein))?;
    m.add_wrapped(wrap_pyfunction!(normalized_levenshtein))?;
    m.add_wrapped(wrap_pyfunction!(normalized_damerau_levenshtein))?;
    m.add_wrapped(wrap_pyfunction!(jaro))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler))?;
    m.add_wrapped(wrap_pyfunction!(sorensen_dice))?;
    // parallel
    m.add_wrapped(wrap_pyfunction!(levenshtein_parallel))?;
    m.add_wrapped(wrap_pyfunction!(osa_distance_parallel))?;
    m.add_wrapped(wrap_pyfunction!(damerau_levenshtein_parallel))?;
    m.add_wrapped(wrap_pyfunction!(normalized_levenshtein_parallel))?;
    m.add_wrapped(wrap_pyfunction!(normalized_damerau_levenshtein_parallel))?;
    m.add_wrapped(wrap_pyfunction!(jaro_parallel))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler_parallel))?;
    m.add_wrapped(wrap_pyfunction!(sorensen_dice_parallel))?;
    // max sim
    m.add_wrapped(wrap_pyfunction!(levenshtein_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(osa_distance_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(damerau_levenshtein_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(normalized_levenshtein_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(normalized_damerau_levenshtein_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler_max_similarity))?;
    m.add_wrapped(wrap_pyfunction!(sorensen_dice_max_similarity))?;
    // min sim
    m.add_wrapped(wrap_pyfunction!(levenshtein_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(osa_distance_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(damerau_levenshtein_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(normalized_levenshtein_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(normalized_damerau_levenshtein_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler_min_similarity))?;
    m.add_wrapped(wrap_pyfunction!(sorensen_dice_min_similarity))?;
    // eddie special-cased
    m.add_function(wrap_pyfunction!(eddie_levenshtein_distance, m)?)?;
    m.add_function(wrap_pyfunction!(eddie_levenshtein_distance_parallel, m)?)?;
    Ok(())
}
