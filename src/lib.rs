//! Detection of scripts from Unicode codepoints.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use interavl::IntervalTree;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A context.
#[derive(Clone)]
pub struct Context {
    scripts: Vec<(&'static str, &'static str, Tree)>,
}

/// A script.
#[derive(Clone, Copy, Default)]
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

#[derive(Clone)]
struct Tree {
    count: usize,
    backend: IntervalTree<u32, ()>,
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
            for (code, name, tree) in self.scripts.iter() {
                for (range, _) in tree.backend.iter_overlaps(&(start..end + 1)) {
                    let count = range.end.min(end + 1) - range.start.max(start);
                    let script = scripts.entry(*code).or_insert_with(|| Script {
                        code,
                        name,
                        count: 0,
                        total: tree.count,
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
                .map(|(code, name, ranges)| (code, name, Tree::from(ranges)))
                .collect(),
        }
    }
}

impl From<&[(u32, u32)]> for Tree {
    fn from(ranges: &[(u32, u32)]) -> Self {
        let mut count = 0;
        let mut backend = IntervalTree::default();
        for (start, end) in ranges.iter().cloned() {
            count += (end - start) as usize;
            backend.insert(start..end, ());
        }
        Self { count, backend }
    }
}
