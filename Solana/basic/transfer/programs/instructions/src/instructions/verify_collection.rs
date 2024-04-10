use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
        token::{Mint, Token},
    },
    mpl_token_metadata::instructions::VerifyCollection,
    mpl_token_metadata::types::{Collection, Creator, DataV2},
};

pub fn verify_collection(ctx: Context<LetsVerifyCollection>) {
    CpiContext::new(program, accounts);
    invo
}

#[derive(Accounts)]
pub struct LetsVerifyCollection<'info> {
    #[account(mut)]
    pub metadata: Account<'info, Metadata>,

    pub collection_authority: UncheckedAccount<'info>,

    pub payer: Signer<'info>,
    pub collection_mint: UncheckedAccount<'info>,
    pub collection: UncheckedAccount<'info>,
    pub collection_master_edition_account: Account<'info, Mint>,
    // pub collection_authority_record: Option<Pubkey>,
}
