use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use std::io::{Cursor, Read};

use crate::clmm::PoolState;

pub fn minimal_deserialize_pool_state(data: &[u8]) -> Result<PoolState, ProgramError> {
    println!("Minimal deserialization of PoolState");
    let mut pool_state = PoolState::default();
    
    // Minimum set of fields required for the adapter
    if data.len() < 300 {
        return Err(ProgramError::InvalidAccountData);
    }
    
    let mut cursor = Cursor::new(data);
    
    // Function to read Pubkey
    let read_pubkey = |cursor: &mut Cursor<&[u8]>| -> Result<Pubkey, ProgramError> {
        let mut buffer = [0u8; 32];
        cursor.read_exact(&mut buffer).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(Pubkey::new_from_array(buffer))
    };
    
    // bump (1 byte)
    let mut bump = [0u8; 1];
    cursor.read_exact(&mut bump).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.bump = bump;
    
    // amm_config (Pubkey - 32 bytes)
    pool_state.amm_config = read_pubkey(&mut cursor)?;
    
    // owner (Pubkey - 32 bytes)
    pool_state.owner = read_pubkey(&mut cursor)?;
    
    // token_mint_0 (Pubkey - 32 bytes)
    pool_state.token_mint_0 = read_pubkey(&mut cursor)?;
    
    // token_mint_1 (Pubkey - 32 bytes)
    pool_state.token_mint_1 = read_pubkey(&mut cursor)?;
    
    // token_vault_0 (Pubkey - 32 bytes)
    pool_state.token_vault_0 = read_pubkey(&mut cursor)?;
    
    // token_vault_1 (Pubkey - 32 bytes)
    pool_state.token_vault_1 = read_pubkey(&mut cursor)?;
    
    // observation_key (Pubkey - 32 bytes)
    pool_state.observation_key = read_pubkey(&mut cursor)?;
    
    // mint_decimals_0 (u8 - 1 byte)
    let mut decimals = [0u8; 1];
    cursor.read_exact(&mut decimals).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.mint_decimals_0 = decimals[0];
    
    // mint_decimals_1 (u8 - 1 byte)
    cursor.read_exact(&mut decimals).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.mint_decimals_1 = decimals[0];
    
    // tick_spacing (u16 - 2 bytes)
    let mut buffer_u16 = [0u8; 2];
    cursor.read_exact(&mut buffer_u16).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.tick_spacing = u16::from_le_bytes(buffer_u16);
    
    // liquidity (u128 - 16 bytes)
    let mut buffer_u128 = [0u8; 16];
    cursor.read_exact(&mut buffer_u128).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.liquidity = u128::from_le_bytes(buffer_u128);
    
    // sqrt_price_x64 (u128 - 16 bytes)
    cursor.read_exact(&mut buffer_u128).map_err(|_| ProgramError::InvalidAccountData)?;
    pool_state.sqrt_price_x64 = u128::from_le_bytes(buffer_u128);
    
    // Sufficient for basic adapter functionality
    // Other fields are left with default values
    
    println!("Read minimal set of fields:");
    println!("  token_mint_0: {}", pool_state.token_mint_0);
    println!("  token_mint_1: {}", pool_state.token_mint_1);
    println!("  mint_decimals_0: {}", pool_state.mint_decimals_0);
    println!("  mint_decimals_1: {}", pool_state.mint_decimals_1);
    println!("  tick_spacing: {}", pool_state.tick_spacing);
    println!("  liquidity: {}", pool_state.liquidity);
    println!("  sqrt_price_x64: {}", pool_state.sqrt_price_x64);
    
    Ok(pool_state)
}