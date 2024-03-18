#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Cqxw8JVEXFfkU7gbscZP22TaP8SjzbDWQs9hKAZF9Rmx");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts<'_>>) -> Result<()> {
        Ok(())
    }
}

// Account validation in Anchor is done using the types and constraints specified in the #[derive(Accounts)] structs
// This is a simple example and does not include all possible constraints and types
#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>, // checks account is signer

    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,
    #[account(mut,owner = id())]
    account_to_change: UncheckedAccount<'info>,
    system_program: Program<'info, System>, // checks account is executable, and is the system program
}

// #[program]
// pub mod cross_program_invocation {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}
