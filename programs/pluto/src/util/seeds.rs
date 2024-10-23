// mainet staging : *_v01
// last devnet: *_xa1
pub const PROTOCOL : &[u8; 12] = b"protocol_v01";
pub const STATS : &[u8; 9] = b"stats_v01";

pub const WSOL_AUTH: &[u8; 13] = b"wsol_auth_v01";

pub const CONFIG_EARN_AUTH : &[u8; 20] = b"config_earn_auth_v01";
pub const CONFIG_EARN : &[u8; 15] = b"config_earn_v01";
pub const VAULT_EARN_AUTH : &[u8; 19] = b"vault_earn_auth_v01";
pub const VAULT_EARN: &[u8; 14] = b"vault_earn_v01";
pub const LENDER_MINT: &[u8; 15] = b"lender_mint_v01";
pub const BORROW_MINT: &[u8; 15] = b"borrow_mint_v01";
pub const LENDER_AUTH: &[u8; 15] = b"lender_auth_v01";
pub const LENDER: &[u8; 10] = b"lender_v01";

pub const CONFIG_LEVERAGE_AUTH: &[u8; 24] = b"config_leverage_auth_v01";
pub const CONFIG_LEVERAGE: &[u8; 19] = b"config_leverage_v01";
pub const VAULT_LEVERAGE_AUTH: &[u8; 23] = b"vault_leverage_auth_v01";
pub const VAULT_LEVERAGE: &[u8; 18] = b"vault_leverage_v01";
pub const LEVERAGE_MINT: &[u8; 17] = b"leverage_mint_v01";
pub const OBLIGATION_AUTH: &[u8; 19] = b"obligation_auth_v01";
pub const OBLIGATION: &[u8; 14] = b"obligation_v01";
pub const POSITION: &[u8; 12] = b"position_v01";

pub const METADATA: &[u8; 12] = b"metadata_v01";
pub const VAULT_SWAP: &[u8; 14] = b"vault_swap_v01";

pub fn get_signer<'a, 'b>(seeds: &'a [&'b [u8]]) -> [&'a [&'b [u8]]; 1] {
    return [&seeds[..]];
}