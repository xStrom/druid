// Copyright 2020 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A fast set, used to track child widgets.

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

const NUM_BITS: u64 = 64;

// the 'offset_basis' for the fnv-1a hash algorithm.
// see http://www.isthe.com/chongo/tech/comp/fnv/index.html#FNV-param
//
// The first of these is the one described in the algorithm, the second is random.
const OFFSET_ONE: u64 = 0xcbf2_9ce4_8422_2325;
const OFFSET_TWO: u64 = 0xe10_3ad8_2dad_8028;

/// A fast set optimized for small values.
///
/// It consists of a simple Bloom filter guarding a full set.
#[derive(Clone)]
pub(crate) struct FastSet<T> {
    bits: u64,
    set: HashSet<T>,
}

impl<T: ?Sized + Eq + Copy + Hash> FastSet<T> {
    /// Create a new set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of entries in the set.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Remove all entries from the set.
    pub fn clear(&mut self) {
        self.bits = 0;
        self.set.clear();
    }

    /// Add an item to the set.
    pub fn add(&mut self, item: T) {
        let mask = self.make_bit_mask(&item);
        self.bits |= mask;
        self.set.insert(item);
    }

    /// Returns `true` if the set contains the value.
    pub fn contains(&self, item: &T) -> bool {
        self.bloom_contains(item) && self.set.contains(item)
    }

    /// Create a new `FastSet` with the entries from both sets.
    pub fn union(&self, other: &FastSet<T>) -> FastSet<T> {
        FastSet {
            bits: self.bits | other.bits,
            set: self.set.union(&other.set).copied().collect(),
        }
    }

    #[inline]
    fn bloom_contains(&self, item: &T) -> bool {
        let mask = self.make_bit_mask(item);
        self.bits & mask == mask
    }

    #[inline]
    fn make_bit_mask(&self, item: &T) -> u64 {
        //NOTE: we use two hash functions, which performs better than a single hash
        // with smaller numbers of items, but poorer with more items. Threshold
        // (given 64 bits) is ~30 items.
        // The reasoning is that with large numbers of items we're already in bad shape;
        // optimize for fewer false positives as we get closer to the leaves.
        // This can be tweaked after profiling.
        let hash1 = self.make_hash(item, OFFSET_ONE);
        let hash2 = self.make_hash(item, OFFSET_TWO);
        (1 << (hash1 % NUM_BITS)) | (1 << (hash2 % NUM_BITS))
    }

    #[inline]
    fn make_hash(&self, item: &T, seed: u64) -> u64 {
        let mut hasher = FnvHasher::with_key(seed);
        item.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: ?Sized + Eq + Copy + Hash> std::fmt::Debug for FastSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FastSet: {:064b}: ({})", self.bits, self.set.len())
    }
}

impl<T: ?Sized + Eq + Copy + Hash> Default for FastSet<T> {
    fn default() -> Self {
        FastSet {
            bits: 0,
            set: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn very_good_test() {
        let mut set = FastSet::default();
        for i in 0..100 {
            set.add(i);
            assert!(set.bloom_contains(&i));
        }
        set.clear();
        for i in 0..100 {
            assert!(!set.bloom_contains(&i));
        }
    }

    #[test]
    fn union() {
        let mut set1 = FastSet::default();
        set1.add(0);
        set1.add(1);
        assert!(!set1.bloom_contains(&2));
        assert!(!set1.bloom_contains(&3));
        let mut set2 = FastSet::default();
        set2.add(2);
        set2.add(3);
        assert!(!set2.bloom_contains(&0));
        assert!(!set2.bloom_contains(&1));

        let set3 = set1.union(&set2);
        assert!(set3.bloom_contains(&0));
        assert!(set3.bloom_contains(&1));
        assert!(set3.bloom_contains(&2));
        assert!(set3.bloom_contains(&3));
    }
}
