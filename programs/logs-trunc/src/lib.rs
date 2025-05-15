use anchor_lang::prelude::*;

declare_id!("BT29poq1aygVaHLCA7Dz64mK7nvxgCifPw9zKDXuc36v");

#[program]
pub mod logs_trunc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
