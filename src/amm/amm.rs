use crate::{clmm::PoolState, serialization::minimal_deserialize_pool_state};
// use crate::serialization::deserialize_pool_state;
use jupiter_amm_interface::{
    Amm, AmmContext, KeyedAccount, AccountMap, Quote, QuoteParams, SwapParams, SwapAndAccountMetas, 
};
use anyhow::Result;
use solana_program::pubkey::Pubkey;

pub struct SwapIoClmmAdapter {
    pool_key: Pubkey,
    pool_state: PoolState,
    token_a_decimals: u8,
    token_b_decimals: u8,
    program_id: Pubkey,
}

impl SwapIoClmmAdapter {
    fn new(
        pool_key: Pubkey,
        pool_state: PoolState,
        program_id: Pubkey,
    ) -> Self {
        let token_a_decimals = pool_state.mint_decimals_0;
        let token_b_decimals = pool_state.mint_decimals_1;
        Self {
            pool_key,
            pool_state,
            token_a_decimals,
            token_b_decimals,
            program_id,
        }
    }
    
    // Make these getters public for testing
    pub fn token_a_decimals(&self) -> u8 {
        self.token_a_decimals
    }
    
    pub fn token_b_decimals(&self) -> u8 {
        self.token_b_decimals
    }

    pub fn pool_state(&self) -> &PoolState {
        &self.pool_state
    }
}

impl Amm for SwapIoClmmAdapter where Self: Sized {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let pool_key = keyed_account.key;
        let pool_data: &[u8] = keyed_account.account.data.as_ref();

        // Check if we have the 8-byte discriminator at the beginning
        if pool_data.len() < 8 {
            return Err(anyhow::anyhow!("Account data too short"));
        }
        
        // Debug information about the account data
        println!("Account data length: {}", pool_data.len());
        
        // For Anchor programs, we need to skip the 8-byte discriminator
        let data_without_discriminator = &pool_data[8..];
        println!("Data without discriminator length: {}", data_without_discriminator.len());
        
        // Check size requirements
        println!("PoolState size: {}", std::mem::size_of::<PoolState>());
        
        // Deserialize pool_state
        println!("About to unpack PoolState");
        // Use the minimal deserialization function
        let pool_state = minimal_deserialize_pool_state(data_without_discriminator)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize pool state: {:?}", e))?;
        
        let program_id = keyed_account.account.owner;
        Ok(Self::new(pool_key, pool_state, program_id))
    }
    
    fn label(&self) -> String {
        "SWAP-IO-CLMM".to_string()
    }
    
    fn program_id(&self) -> Pubkey {
        self.program_id
    }
    
    fn key(&self) -> Pubkey {
        self.pool_key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.pool_state.token_mint_0, self.pool_state.token_mint_1]
    }
    
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        todo!()
    }
    
    fn update(&mut self, _account_map: &AccountMap) -> Result<()> {
        todo!()
    }
    
    fn quote(&self, _quote_params: &QuoteParams) -> Result<Quote> {
        todo!()
    }
    
    fn get_swap_and_account_metas(&self, __swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        todo!()
    }
    
    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        todo!()
    }
    
    fn has_dynamic_accounts(&self) -> bool {
        false
    }
    
    fn requires_update_for_reserve_mints(&self) -> bool {
        false
    }
    
    fn supports_exact_out(&self) -> bool {
        false
    }
    
    fn get_user_setup(&self) -> Option<jupiter_amm_interface::AmmUserSetup> {
        None
    }
    
    fn unidirectional(&self) -> bool {
        false
    }
    
    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        std::vec![]
    }
    
    fn get_accounts_len(&self) -> usize {
        32 // Default to a near whole legacy transaction to penalize no implementation
    }
    
    fn underlying_liquidities(&self) -> Option<std::collections::HashSet<Pubkey>> {
        None
    }
    
    fn is_active(&self) -> bool {
        true
    }
}