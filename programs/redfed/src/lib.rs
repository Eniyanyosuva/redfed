use anchor_lang::prelude::*;

declare_id!("GLi1SSdbhQndoRYECdwHBfJixY7g2cUjpbLj3YKVi1aU");

#[program]
pub mod redfed {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
