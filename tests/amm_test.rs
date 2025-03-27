use swap_io_clmm_jupiter::amm::SwapIoClmmAdapter;
use jupiter_amm_interface::{Amm, AmmContext, KeyedAccount};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_from_keyed_account_with_real_pool() {
    // 1. Establish RPC connection
    let rpc_url = "https://api.mainnet-beta.solana.com"; // or any other solana RPC endpoint
    let client = RpcClient::new(rpc_url);
    
    // 2. Public key for real pool
    let pool_pubkey = Pubkey::from_str("HR1xNcU5XPHpEZDsEknw22oPFELk1VGyBzoSaCJrL926").unwrap();
    let program_id = Pubkey::from_str("SWPammPnp7L9qFgV436u3CSPmcxU6ZQm6ttawzDTRuw").unwrap();
    
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
    let amm_interface: &dyn Amm = &adapter;
    // Check that the adapter program_id is equal to the pool program_id
    assert_eq!(amm_interface.program_id(), program_id);
    // Check that the adapter key is equal to the pool_pubkey
    assert_eq!(adapter.key(), pool_pubkey);

    
    // Here you can add additional checks for pool_state fields
    println!("Successfully created adapter from real pool data");
    println!("Pool key: {}", amm_interface.key());
    println!("Program ID: {}", amm_interface.program_id());
    println!("Token A decimals: {}", adapter.token_a_decimals());
    println!("Token B decimals: {}", adapter.token_b_decimals());
    println!("Config: {:?}", adapter.pool_state().amm_config);
    
    // Additionally, you can check what other methods return
    let reserve_mints = amm_interface.get_reserve_mints();
    println!("Reserve mints: {:?}", reserve_mints);
    // fail test
    assert_eq!(1, 2);
}
