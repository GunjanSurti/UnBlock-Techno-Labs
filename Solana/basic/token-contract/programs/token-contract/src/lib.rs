use anchor_lang::prelude::*;

declare_id!("6zRvZSDvHbzXRS1zSuD3dZ6BCbaVqMJtUrorvwg7hG3Q");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
