use anchor_lang::prelude::*;

declare_id!("9r3XTs1andLuWwTkpuWsr8HvzG9Gi9mXc6MfeN5ePM6n");

#[program]
pub mod mintr {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
