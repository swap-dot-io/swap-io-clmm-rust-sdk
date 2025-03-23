use swap_io_clmm_jupiter::amm::SwapIoClmmAdapter;
use jupiter_amm_interface::{Amm, AmmContext, KeyedAccount};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_from_keyed_account_with_real_pool() {
    // 1. Establish RPC connection
    let rpc_url = "https://api.mainnet-beta.solana.com"; // или другой RPC эндпоинт
    let client = RpcClient::new(rpc_url);
    
    // 2. Public key for real pool
    let pool_pubkey = Pubkey::from_str("HR1xNcU5XPHpEZDsEknw22oPFELk1VGyBzoSaCJrL926").unwrap();
    
    // 3. get account's data
    let account = client.get_account(&pool_pubkey).expect("Failed to fetch account");
    
    let keyed_account = KeyedAccount {
        key: pool_pubkey,
        account: account,
        params: None,
    };
    
    use jupiter_amm_interface::ClockRef;
    
    let amm_context = AmmContext {
        clock_ref: ClockRef::default(),
    };
    
    let result = SwapIoClmmAdapter::from_keyed_account(&keyed_account, &amm_context);
    
    assert!(result.is_ok(), "Failed to create SwapIoClmmAdapter: {:?}", result.err());
    
    let adapter = result.unwrap();
    
    // 8. Check that the adapter key is equal to the pool_pubkey
    assert_eq!(adapter.key(), pool_pubkey);
    
    // Here you can add additional checks for pool_state fields
    println!("Successfully created adapter from real pool data");
    println!("Token A decimals: {}", adapter.token_a_decimals());
    println!("Token B decimals: {}", adapter.token_b_decimals());
    
    // Additionally, you can check what other methods return
    let reserve_mints = adapter.get_reserve_mints();
    println!("Reserve mints: {:?}", reserve_mints);
}
