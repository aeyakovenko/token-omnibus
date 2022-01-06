use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_omnibus {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: SHA256) -> ProgramResult {
        if ctx.accounts.account_set.initialized {
            return Err(ErrorCode::AlreadyInitialized.into());
        }
        ctx.accounts.account_set.root = data;
        ctx.accounts.account_set.initialized = true;
        Ok(())
    }
    pub fn deposit_to(ctx: Context<DepositTo>, data: RequestArgs) -> ProgramResult {
        if !ctx.accounts.account_set.initialized {
            return Err(ErrorCode::NotInitialized.into());
        }
        let pda_bump = [data.pda_bump.clone()];
        let pda_seeds = [ctx.accounts.account_set.to_account_info().key.as_ref(), ctx.accounts.mint.to_account_info().key.as_ref(), pda_bump.as_ref()];
        let omnibus_account = Pubkey::create_program_address(&pda_seeds, &ctx.program_id)?;
        if ctx.accounts.omnibus.owner != omnibus_account {
            return Err(ErrorCode::InvalidOmnibusAccount.into());
        }
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.source.to_account_info(),
            to: ctx.accounts.omnibus.to_account_info(),
            authority: ctx.accounts.source.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, data.amount)?;

        Ok(())
    }
    pub fn withdraw_to(ctx: Context<WithdrawTo>, _data: RequestArgs) -> ProgramResult {
        if !ctx.accounts.account_set.initialized {
            return Err(ErrorCode::NotInitialized.into());
        }
        Ok(())
    }
}

pub type SHA256 = [u8; 32];

#[account]
pub struct AccountSet {
    root: SHA256,
    initialized: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RequestArgs {
    ///  Proof must start at SHA256(destination owner, amount)
    proof: [SHA256; 20],

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
pub struct DepositTo<'info> {
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

#[derive(Accounts)]
pub struct WithdrawTo<'info> {
    #[account(mut)]
    pub account_set: Account<'info, AccountSet>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub omnibus: Account<'info, TokenAccount>,
    pub source: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[error]
pub enum ErrorCode {
    #[msg("Already initialized.")]
    AlreadyInitialized,
    #[msg("Not initialized.")]
    NotInitialized,
    #[msg("Invalid Omnibus Account.")]
    InvalidOmnibusAccount,
}
