use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_omnibus {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>, data: SHA256) -> ProgramResult {
        Ok(())
    }
}

type SHA256 = [u8;32];

#[account]
struct AccountSet {
    root: SHA256,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = account_set, space = 32)]
    pub account_set: Account<'info, AccountSet>,
}
