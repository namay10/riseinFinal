use anchor_lang::prelude::*;

declare_id!("EM5u5Lqvcgc1bZHYQh2ykczpjeeUCb4VbppKZi2cNJnJ");

#[program]
pub mod review {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
