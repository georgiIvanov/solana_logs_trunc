use anchor_lang::prelude::*;

declare_id!("HVxkVckuk55bkXeXZoz78aMsVYL5ChTyQ8aiPymicUTf");

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
pub struct LogEvents<'info> {
    pub system_program: Program<'info, System>,
}
