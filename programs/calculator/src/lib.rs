use anchor_lang::prelude::*;

declare_id!("6RmZxF5CSUWq2FjREhGNoA8BQxbeHVvt7qZfxXWtsDU9");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
