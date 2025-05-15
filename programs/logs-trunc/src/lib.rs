use anchor_lang::prelude::*;

declare_id!("FXjLSRRVSFJXXmvNa29RivshADPLjFxJb4g7AwbvHYWN");

#[program]
pub mod logs_trunc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        for i in 0..11 {
            // Create a string with repeating digits based on the iteration number
            let digit = i.to_string();
            let large_string = digit.repeat(1_024_000);
            
            // Log the large string
            msg!("{}", large_string);
        }

        msg!("Deposit executed: amount = 1234, fee = 1, etc, etc");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Deposit {}
