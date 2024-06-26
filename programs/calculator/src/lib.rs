use anchor_lang::prelude::*;

declare_id!("6RmZxF5CSUWq2FjREhGNoA8BQxbeHVvt7qZfxXWtsDU9");

#[program]
pub mod calculator {
    use super::*;

    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        ctx.accounts.counter_account.authority = ctx.accounts.authority.key();
        ctx.accounts.counter_account.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter_account.counter += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.counter_account.counter -= 1;
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub counter: u64,
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter_account: Account<'info, Counter>,
    // the signer account must be mutable because it will be paying fee
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter_account: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = authority)]
    pub counter_account: Account<'info, Counter>,
    pub authority: Signer<'info>,
}
