use anchor_lang::prelude::*;

declare_id!("2wQRM1r6oS5DZxVMcrz4MNRZ3CDqcFGuojrXDivnfAXW");

#[program]
pub mod nft_marketplace_in_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
