use anchor_lang::prelude::*;

declare_id!("7Be3MS2o8A46F8UftQ1hj1q6bAKNxyjwvmUQPKG2JnVu");

#[program]
pub mod spl_50 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
