// Copyright (c) 2022 MASSA LABS <info@massa.net>

use std::collections::{BTreeMap, VecDeque};

use bitvec::prelude::*;
use massa_models::{
    constants::POS_MISS_RATE_DEACTIVATION_THRESHOLD, prehash::Map, Address, Amount, Slot,
};
use num::rational::Ratio;

/// Final state of PoS
#[derive(Default)]
pub struct PoSFinalState {
    /// contiguous cycle history. Front = newest.
    pub cycle_history: VecDeque<CycleInfo>,
    /// coins to be credited at the end of the slot
    pub deferred_credits: BTreeMap<Slot, Map<Address, Amount>>,
}

/// State of a cycle for all threads
#[derive(Default, Debug, Clone)]
pub struct CycleInfo {
    /// cycle number
    pub cycle: u64,
    /// whether the cycle is complete (all slots final)
    pub complete: bool,
    /// number of rolls each staking address has
    pub roll_counts: Map<Address, u64>,
    /// random seed bits of all slots in the cycle so far
    pub rng_seed: BitVec<Lsb0, u8>,
    /// Per-address production statistics
    pub production_stats: Map<Address, ProductionStats>,
}

/// Block production statistic
#[derive(Default, Debug, Copy, Clone)]
pub struct ProductionStats {
    /// Number of successfully created blocks
    pub block_success_count: u64,
    /// Number of blocks missed
    pub block_failure_count: u64,
}

impl ProductionStats {
    /// Check if the production stats are above the required percentage
    pub fn satisfying(&self) -> bool {
        Ratio::new(self.block_success_count, self.block_failure_count)
            >= *POS_MISS_RATE_DEACTIVATION_THRESHOLD
    }
}

/// Recap of all PoS changes
#[derive(Default, Debug, Clone)]
pub struct PoSChanges {
    /// extra block seed bits added
    pub seed_bits: BitVec<Lsb0, u8>,

    /// new roll counts for addresses (can be 0 to remove the address from the registry)
    pub roll_changes: Map<Address, u64>,

    /// updated production statistics
    pub production_stats: Map<Address, ProductionStats>,

    /// set deferred credits indexed by target slot (can be set to 0 to cancel some, in case of slash)
    /// ordered structure to ensure slot iteration order is deterministic
    pub deferred_credits: BTreeMap<Slot, Map<Address, Amount>>,
}

/// Selections of endorsements and producer
#[derive(Clone)]
pub struct Selection {
    /// Choosen endorsements
    pub endorsments: Vec<Address>,
    /// Choosen block producer
    pub producer: Address,
}
