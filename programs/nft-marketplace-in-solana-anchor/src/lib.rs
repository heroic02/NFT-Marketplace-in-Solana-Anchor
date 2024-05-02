use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{self, MintTo};
use mpl_token_metadata::types::Creator;

declare_id!("2wQRM1r6oS5DZxVMcrz4MNRZ3CDqcFGuojrXDivnfAXW");


#[derive(Accounts)]
pub struct MinNFT<'info>{
    /// CHECK: master edition
    #[account(mut)]
    pub master_edition:UncheckedAccount<'info>,

    /// CHECK: metadata
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: mint
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: mint authority
    #[account(mut)]
    pub mint_authority : Signer<'info>,

    /// CHECK: payer
    #[account(mut)]
    pub payer : AccountInfo<'info>,

    /// CHECK: rent
    pub rent: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: token account 
    #[account(mut)]
    pub token_account : UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: token metadata program
    pub token_metadata_program : UncheckedAccount<'info>,
}


#[program]
pub mod nft_marketplace_in_solana_anchor {



    use super::*;
    
    pub fn mint_nft(ctx: Context<MinNFT>, collection_key:Pubkey,metadata_uri: String, metadata_title:String)->Result<()>{
        msg!("Initializing Minting process");
        // Configure Cross-Program-Invokation (CPI) ctx

        let token_program  = ctx.accounts.token_program.to_account_info();
        let token_mint = ctx.accounts.mint.to_account_info();

        let token_mint_id = token_mint.key;

        let accounts = MintTo{
            mint: token_mint,
            to: ctx.accounts.token_account.to_account_info(),
            authority:ctx.accounts.payer.to_account_info(),
        } ;

        let cpi_ctx = CpiContext::new(token_program,accounts);
        token::mint_to(cpi_ctx, 1)?;

        msg!("Your NFT has been minted!");
        msg!(" Token ID {}", token_mint_id);

        // Set Up the associated metadata
        let creators = vec![Creator{
            address:collection_key
            ,share:100,
            verified:false
        },Creator{
            address:ctx.accounts.mint_authority.key(),
            share:0,
            verified:false
        }];

        let token_symbol = ToString::to_string("BEST_OF_BOTH_WORLD!");

        // Invoke the solana program to create the metadata accounts
        // Wrapped nicely in anchor instructions
        msg!("Creators added");

        msg!("Token is {} and uri is {}", token_symbol, metadata_uri);

        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                metadata_title,
                String::from(token_symbol),
                String::from(metadata_uri),
                Some(creators),
                1,
                true,
                false,
                None,
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Metadata account created successfully!");

        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("NFT delivered successfully. Check you wallet");
        Ok(())
    }
 

}

#[derive(Accounts)]
pub struct Initialize {}


