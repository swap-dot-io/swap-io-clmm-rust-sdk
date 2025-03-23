// use crate::types::PoolState;
// use anyhow::Result;

// pub fn deserialize_pool_state(data: &[u8]) -> Result<PoolState> {
//     // Пропускаем дискриминатор Anchor (8 байт)
//     let pool_data = &data[8..];
//     PoolState::try_from_slice(pool_data)
//         .map_err(|e| anyhow!("Failed to deserialize PoolState: {}", e))
// }