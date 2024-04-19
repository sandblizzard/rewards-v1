use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;

use anchor_spl::token::Mint;

use anchor_spl::token::Token;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// creator is the owner of the protocol
    /// should become a smart wallet over time
    #[account(mut)]
    pub protocol_owner: Signer<'info>,

    /// CHECK: metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer=protocol_owner,
        seeds=[
            BOUNTY_SEED, // only one protocol config
        ],
        space=8 + size_of::<protocol::Protocol>(),
        bump
    )]
    pub protocol: Account<'info, protocol::Protocol>,

    /// mint to be used to distribute rewards
    #[account(
        init,
        seeds=[
            MINT_SEED,
        ],
        bump,
        payer=protocol_owner,
        mint::decimals=9,
        mint::authority=protocol,
        mint::freeze_authority=protocol,

    )]
    pub sand_mint: Account<'info, Mint>,

    /// create treasury account to hold the protocol's funds
    pub token_program: Program<'info, Token>,

    /// CHECK: token metadata program
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

/// Initialize protocol
///
/// creates a new protocol config and sets who gets to
/// control it
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("initialize protocol");
    let protocol = &mut ctx.accounts.protocol;
    let sand_mint = &ctx.accounts.sand_mint;

    let protocol_bump = ctx.bumps.get("protocol").unwrap();

    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: &ctx.accounts.token_metadata_program,
        metadata: &ctx.accounts.metadata,
        mint: &sand_mint.to_account_info(),
        mint_authority: &protocol.to_account_info(),
        payer: &ctx.accounts.protocol_owner.to_account_info(),
        update_authority: (&protocol.to_account_info(), true),
        system_program: &ctx.accounts.system_program,
        rent: Some(&ctx.accounts.rent_sysvar.to_account_info()),
        __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data: mpl_token_metadata::types::DataV2 {
                name: METADATA_NAME.to_string(),
                symbol: METADATA_SYMBOL.to_string(),
                uri: METADATA_URI.to_string(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    }
    .invoke_signed(&[&[BOUNTY_SEED, &[*protocol_bump]]])?;

    // Initialize collection
    protocol
        .initialize(
            ctx.bumps.get("protocol").unwrap(),
            &ctx.accounts.protocol_owner.key(),
            &mut ctx.accounts.sand_mint.key(),
        )
        .unwrap();

    Ok(())
}
