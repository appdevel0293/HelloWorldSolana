use anchor_lang::prelude::*;

declare_id!("Har9vLr7WFKf1XdPZzuzNTcCXbJ1Qx9AMu9v5iMuTXPV");

#[program]
mod hello_world {
    use super::*;
    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello World!");       
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}