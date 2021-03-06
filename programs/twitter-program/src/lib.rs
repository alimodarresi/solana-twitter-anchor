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

#[account]
pub struct Tweet {
    pub author:Pubkey,
    pub timestamp:i64,
    pub topic:String,
    pub content:String,
}

const DISCRIMINATOR_LENGTH : usize  = 8;
const PUBLIC_KEY_LENGHT : usize = 32 ; 
const TIMESTAMP_LENGTH :  usize = 8 ; 
const STRING_LENGTH_PREFIX : usize = 4;
const MAX_TOPIC_LENGTH : usize = 50*4; // limited to max of 50 chars
const MAX_CONTENT_LENGTH :usize = 280*4;

impl Tweet {
    const LEN : usize = DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGHT
    + TIMESTAMP_LENGTH
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH
    + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;
}
