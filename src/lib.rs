//! Detection of scripts from Unicode codepoints.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

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
    /// is given as a range whose ends coincide.
    pub fn detect<T>(&self, codepoints: T) -> BTreeMap<&'static str, Script>
    where
        T: IntoIterator<Item = [u32; 2]>,
    {
        let mut scripts: BTreeMap<&'static str, Script> = Default::default();
        for [start, end] in codepoints {
            for (code, name, total, ranges) in self.scripts.iter() {
                for (range, _) in ranges.iter_overlaps(&(start..end + 1)) {
                    let count = range.end.min(end + 1) - range.start.max(start);
                    let script = scripts.entry(*code).or_insert_with(|| Script {
                        code,
                        name,
                        count: 0,
                        total: *total,
                    });
                    script.count += count as usize;
                }
            }
        }
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
