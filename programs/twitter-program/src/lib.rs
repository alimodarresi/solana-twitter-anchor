use anchor_lang::prelude::*;

declare_id!("AmTqjpN5LN6DovBsXWkanRBagEkL3tirjZXi65n3hitD");

#[program]
pub mod twitter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
