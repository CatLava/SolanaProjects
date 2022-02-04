use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token {
    use super::*;
    pub fn proxy_transfer(ctx: Context<ProxyTransfer>, amount: u64) -> ProgramResult {
        token::transfer(ctx.accounts.into(), amount)
    }

    pub fn proxy_mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> ProgramResult{
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn proxy_burn(ctx: Context<ProxyBurn>, amount: u64) -> ProgramResult {
        token::burn(ctx.accounts.into(), amount)

    }

    pub fn proxy_set_authority(ctx: Context<ProxySetAuthority>,
        authority_type: AuthorityType,
        new_authority: Option<Pubkey>,
    ) -> ProgramResult {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType{
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount
}
#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
} 


#[derive(Accounts)]
pub struct ProxyMintTo {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
} 


#[derive(Accounts)]
pub struct ProxyBurn {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
} 

#[derive(Accounts)]
pub struct ProxySetAuthority {
    #[account(signer)]
    pub current_authority: AccountInfo<'info>,
    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}
