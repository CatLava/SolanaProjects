use anchor_lang::prelude::*;

declare_id!("FWrTEDWZCLJh1KbrvbwVwyZa23AbUVSvHARRcBDVZ7ma");

#[program]
pub mod messengerapp {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        // Solana accounts persist data between calls
    // accounts pay rent in the form of lamports
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    
    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone(); 
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    
}

// Context<T> is the solana accounts array and program ID
// this function we keep track of the data calls and the history of data

#[derive(Accounts)]
pub struct Initialize<'info> {
    // init is designating owner of the program
    // must designate payer and space to create the info
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    // authority of who signed the transaction
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    pub data_list: Vec<String>,
}
