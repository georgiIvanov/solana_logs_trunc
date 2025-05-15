use anchor_lang::prelude::*;
// use logging_program::logging_program;

declare_id!("DUKnRfntDqsg2jvN5JUvh8otaCAfQe4Q5etkrdm8tE4D");

#[program]
pub mod logs_trunc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, row_length: u64, row_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value += 1;

        // Call the logging program via CPI
        // let cpi_program = ctx.accounts.logging_program.to_account_info();
        // let cpi_accounts = logging_program::LogEvents {
        //     signer: ctx.accounts.user.clone(),
        // };
        
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // logging_program::log_events(cpi_ctx, row_length, row_count)?;

        msg!("Deposit executed: amount = 1234, fee = 1, etc, etc");
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    // pub logging_program: Program<'info, >,
}
