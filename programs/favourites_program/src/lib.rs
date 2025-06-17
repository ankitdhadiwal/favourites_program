use anchor_lang::prelude::*;

declare_id!("CehY6n7M7feYHkt87vnmB21RE9dewFHoWhCAa8h4ZyFs");

#[program]
pub mod favourites_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
