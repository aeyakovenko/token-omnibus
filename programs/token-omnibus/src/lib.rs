use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use anchor_spl::token::Token;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_omnibus {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>, _data: SHA256) -> ProgramResult {
        Ok(())
    }
    pub fn deposit(_ctx: Context<Initialize>, _data: DepositArgs) -> ProgramResult {
        Ok(())
    }
}

pub type SHA256 = [u8; 32];

#[account]
pub struct AccountSet {
    root: SHA256,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositArgs {
    ///  proof that value is zero
    ///  Proof must start at SHA256(destination owner, amount)
    proof_zero: [SHA256; 20],

    /// amount must be delegated by the source token Account
    amount: u64,

    /// PDA(Token Mint Account, pda_bump)
    pda_bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 32)]
    pub account_set: Account<'info, AccountSet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub account_set: Account<'info, AccountSet>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub omnibus: Account<'info, TokenAccount>,
    pub destination: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

}
