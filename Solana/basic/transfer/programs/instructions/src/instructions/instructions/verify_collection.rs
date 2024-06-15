use {
    anchor_lang::prelude::*,
    anchor_spl::metadata::Metadata,
    mpl_token_metadata::instructions::{VerifyCollectionV1Cpi, VerifyCollectionV1CpiAccounts},
};
pub fn verify_collection(ctx: Context<VerifyCollectionV1CpiAccount>) -> Result<()> {
    let verify_cpi = VerifyCollectionV1Cpi::new(
        &ctx.accounts.program_id.to_account_info(),
        VerifyCollectionV1CpiAccounts {
            authority: &ctx.accounts.collection_authority.to_account_info(),
            delegate_record: None,
            metadata: &ctx.accounts.metadata.to_account_info(),
            collection_mint: &ctx.accounts.collection_mint.to_account_info(),
            collection_metadata: Some(&ctx.accounts.collection_metadata.to_account_info()),
            collection_master_edition: Some(
                &ctx.accounts.collection_master_edition.to_account_info(),
            ),

            system_program: &ctx.accounts.system_program.to_account_info(),
            sysvar_instructions: &ctx.accounts.sysvar_instructions.to_account_info(),
        },
    ).invoke(); // invoke_signed use this
    Ok(())
}

// ///////////////////////////////////////////////////////
#[derive(Accounts)]
pub struct VerifyCollectionV1CpiAccount<'info> {
    pub program_id: Program<'info, Metadata>,
    pub collection_authority: SystemAccount<'info>,

    pub collection_master_edition: UncheckedAccount<'info>, // where to find this
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    // pub collection_master_edition: UncheckedAccount<'info>,
    // pub collection_authority_record: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub sysvar_instructions: UncheckedAccount<'info>, // Sysvar1nstructions1111111111111111111111111
}

/////////////////////////////////////////////////////
// // pub fn pdd(_ctx: Context<MintAccount>) -> (Pubkey, u8) {
// //     let pda = Metadata::find_pda(mint);
// //     msg!("{},------- {}", pda.0, pda.1);
// //     pda
// // }

// // #[derive(Accounts)]
// // pub struct GG<'info> {
// //     pub mint_account: UncheckedAccount<'info>,
// // }
