use anchor_lang::prelude::*;
use anchor_lang::system_program::ID;

#[program]
pub mod emitter {
    use super::*;

    pub fn log_events(_ctx: Context<LogEvents>, row_length: u64, row_count: u64) -> Result<()> {
        let large_string = "A".repeat(row_length as usize);
        for _ in 0..row_count {
            msg!("{}", large_string);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogEvents {
    // #[account(mut)] maybe?
    // pub signer: Signer<'info>,
}
