use anchor_lang::prelude::*;

declare_id!("6e8f8Gbnq1qd5ApwXTh5gJq6GEMWi1G1Kvz5TsakiT56");

#[program]
pub mod dem {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
