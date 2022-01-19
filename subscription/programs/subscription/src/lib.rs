/*
THis is a subscription based program on Solana. Simple set up is this
1. User deposit funds locked into a vault
2. Depositor receives back an NFT (Could be another FT as well)
*/

use anchor_lang::prelude::*;
use anchor_spl::token::accessor::authority;

declare_id!("2zvxkCxLL8nJ87tRc8tbo4JWNhJ66e19aRRq8cDxiUqA");

#[program]
pub mod subscription {
    use super::*;
    pub fn vault(ctx: Context<Vault>) -> ProgramResult {
        let vault = &mut ctx.accounts.deposit;
        vault.count = 0;
        Ok(())
    }

    pub fn submit(ctx: Context<Deposit>) -> ProgramResult {
        let deposit = &mut ctx.accounts.deposit;
        deposit.count += 1;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub deposit: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub deposit: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
