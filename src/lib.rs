//! Detection of scripts from Unicode codepoints.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use interavl::IntervalTree;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A context.
#[derive(Clone, Debug)]
pub struct Context {
    scripts: Vec<(&'static str, &'static str, usize, IntervalTree<u32, ()>)>,
}

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
    /// should be found for the script to be reported. The scripts are sorted by this share in
    /// descending order with ties broken by code.
    pub fn detect<T>(&self, codepoints: T, threshold: f64) -> Vec<Script>
    where
        T: IntoIterator<Item = [u32; 2]>,
    {
        let mut counts = vec![0; self.scripts.len()];
        for [start, end] in codepoints {
            for (count, (.., ranges)) in counts.iter_mut().zip(self.scripts.iter()) {
                for (range, _) in ranges.iter_overlaps(&(start..end + 1)) {
                    *count += (range.end.min(end + 1) - range.start.max(start)) as usize;
                }
            }
        }
        let mut scripts = self
            .scripts
            .iter()
            .zip(counts)
            .filter(|((_, _, total, _), count)| {
                *count > 0 && *count as f64 / *total as f64 >= threshold
            })
            .map(|(&(code, name, total, _), count)| Script {
                code,
                name,
                count,
                total,
            })
            .collect::<Vec<_>>();
        scripts.sort_by(|one, other| {
            (other.count * one.total)
                .cmp(&(one.count * other.total))
                .then_with(|| one.code.cmp(other.code))
        });
        scripts
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            scripts: DATA
                .iter()
                .cloned()
                .map(|(code, name, total, ranges)| {
                    let mut tree = IntervalTree::default();
                    for (start, end) in ranges.iter().cloned() {
                        tree.insert(start..end, ());
                    }
                    (code, name, total, tree)
                })
                .collect(),
        }
    }
}
