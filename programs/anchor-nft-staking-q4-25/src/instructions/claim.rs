use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
//TODO
     #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"rewards", config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
    //TODO
    let points = self.user_account.points;
        if points == 0 {
            return Ok(());
        }

        let amount = (points as u64).checked_mul(1_000_000).unwrap();

        let seeds: &[&[u8]] = &[b"config", &[self.config.bump]];
        let signer = &[seeds];

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);

        mint_to(cpi_ctx, amount)?;

        self.user_account.points = 0;

        Ok(())
    }
}
