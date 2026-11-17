//! Detection of scripts from Unicode codepoints.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A context.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Context {}

/// A script.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Script {
    /// The ISO 15924 code.
    pub code: &'static str,
    /// The proper name in English.
    pub name: &'static str,
    /// The number of codepoints found in the script.
    pub count: usize,
    /// The total number of codepoints in the script.
    pub total: usize,
}

impl Context {
    /// Detect scripts given ranges of Unicode codepoints.
    ///
    /// Each range is closed, containing both the first and the last codepoint. A single codepoint
    /// is given as a range whose ends coincide. The ranges should not overlap.
    ///
    /// The threshold is the minimum share of the codepoints of a script, from zero to one, that
    /// should be found for the script to be reported. The scripts are sorted by code.
    pub fn detect<T>(&self, codepoints: T, threshold: f64) -> Vec<Script>
    where
        T: IntoIterator<Item = [u32; 2]>,
    {
        let mut counts = vec![0; DATA.len()];
        for [start, end] in codepoints {
            for (count, (.., ranges)) in counts.iter_mut().zip(DATA) {
                // The ranges are sorted and do not overlap; skip those lying before.
                let index = ranges.partition_point(|(_, upper)| *upper <= start);
                for (lower, upper) in ranges[index..].iter().cloned() {
                    // The rest lies beyond.
                    if end < lower {
                        break;
                    }
                    *count += ((end + 1).min(upper) - start.max(lower)) as usize;
                }
            }
        }
        DATA.iter()
            .zip(counts)
            .filter(|((.., total, _), count)| {
                *count > 0 && *count as f64 / *total as f64 >= threshold
            })
            .map(|(&(code, name, total, _), count)| Script {
                code,
                name,
                count,
                total,
            })
            .collect::<Vec<_>>()
    }
}
