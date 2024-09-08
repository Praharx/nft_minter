use anchor_lang::prelude::*;

declare_id!("CpdEbG4y8cF9fSoy5RQuPnn4sK8XJhbtiZWWPAQjQqUd");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
