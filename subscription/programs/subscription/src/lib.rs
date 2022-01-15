/*
THis is a subscription based program on Solana. Simple set up is this
1. User deposit funds locked into a vault
2. Depositor receives back an NFT (Could be another FT as well)
*/

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod subscription {
    use super::*;
    pub fn sub_deposit(ctx: Context<Initialize>, deposit_amount: u64) -> ProgramResult {
        // Todo
        Ok(())
    }

    pub fn vault(ctx: Context<Vault>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

pub struct Vault {}
