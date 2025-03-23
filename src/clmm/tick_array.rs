use solana_program::pubkey::Pubkey;

use super::REWARD_NUM;

const EXTENSION_TICKARRAY_BITMAP_SIZE: usize = 14;
pub const TICK_ARRAY_SEED: &str = "tick_array";
pub const TICK_ARRAY_SIZE_USIZE: usize = 60;
pub const TICK_ARRAY_SIZE: i32 = 60;

pub struct TickState {
    pub tick: i32,
    /// Amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left)
    pub liquidity_net: i128,
    /// The total position liquidity that references this tick
    pub liquidity_gross: u128,

    /// Fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    /// only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,

    // Reward growth per unit of liquidity like fee, array of Q64.64
    pub reward_growths_outside_x64: [u128; REWARD_NUM],
    // Unused bytes for future upgrades.
    pub padding: [u32; 13],
}

pub struct TickArrayState {
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    pub ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
    pub initialized_tick_count: u8,
    // account update recent epoch
    pub recent_epoch: u64,
    // Unused bytes for future upgrades.
    pub padding: [u8; 107],
}

pub struct TickArrayBitmapExtension {
    pub pool_id: Pubkey,
    /// Packed initialized tick array state for start_tick_index is positive
    pub positive_tick_array_bitmap: [[u64; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
    /// Packed initialized tick array state for start_tick_index is negitive
    pub negative_tick_array_bitmap: [[u64; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
}
