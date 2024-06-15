use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
        token::{Mint, Token},
    },
    mpl_token_metadata::types::{Collection, /*Creator,*/ DataV2},
};
// uri = https://qn-shared.quicknode-ipfs.com/ipfs/QmQFh6WuQaWAMLsw9paLZYvTsdL5xJESzcoSxzb6ZU3Gjx
pub const PREFIX: &str = "metadata";

pub fn find_metadata_account(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>, // this is collection mint account

    #[account(
        init,
        payer = payer,
        mint::decimals = 0, // for NFTs
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),

    )]
    pub mint_account: Account<'info, Mint>,

    // CHECK: Address validated using constraint
    #[account(
        mut,
        
        address=find_metadata_account(&mint_account.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>, // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    pub token_metadata_program: Program<'info, Metadata>, // metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s
    pub system_program: Program<'info, System>,           // 11111111111111111111111111111111
    pub rent: Sysvar<'info, Rent>, // SysvarRent111111111111111111111111111111111
}
// https://arweave.net/O27ikLNE5nSWNd-oDvpmZW46VYiwnsD96JYi_v89Uss token uri use it wisely
pub fn create_token(
    ctx: Context<CreateToken>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    msg!("Creating metadata account");

    // Cross Program Invocation (CPI)
    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: 0, // amount of roalty
            creators: None,             // For roalty to creators
            collection: None,           // for collection metadata
            // collection: Some(Collection {
            //     verified: false,
            //     key: ctx.accounts.collection_mint.key(),
            // }),
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None,  // Collection details
    )?;

    msg!("Token created successfully.");

    Ok(())
}
