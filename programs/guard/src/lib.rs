use anchor_lang::{prelude::*, solana_program::program_pack::Pack};
use anchor_spl::token::spl_token;
pub mod input;
declare_id!("3SmBMUQe5QUpLc7wMrm97CRs3kXBSFtMZvPw8CDwZvUi");


use input::*;

#[error_code]
pub enum MyErrorCode {
    #[msg("parse base amount error")]
    InvalidBaseAccount,
    #[msg("Not Profit")]
    NoProfit,
}

pub fn unpack_token_account_ammount(account: &AccountInfo) -> Result<u64> {
    if account.data_len() >= spl_token::state::Account::LEN {
        let mut amount_buffer = [0u8; 8];
        let account_data = account.data.borrow();
        amount_buffer.copy_from_slice(&account_data[64..64 + 8]);
        let amount = u64::from_le_bytes(amount_buffer);
        return Ok(amount);
    }
    msg!("{}", account.key);
    return Err(error!(MyErrorCode::InvalidBaseAccount));
}

#[program]
pub mod guard {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

    use super::*;

    pub fn arb_process_32_account(
        ctx: Context<CommonAccountsInfo32>,
        max_in: u64,
        min_profit: u64,
        market_type: Vec<u8>,
        market_flag: Vec<u8>
    ) -> Result<()> {
        let before_amount = ctx.accounts.user.lamports() 
                                + unpack_token_account_ammount(&ctx.accounts.user_token_base)?;
        let mut cpi_discriminator: Vec<u8> = vec![
            198,
            43,
            70,
            199,
            55,
            193,
            203,
            81
          ];
        cpi_discriminator.append(borsh::to_vec(&max_in).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&min_profit).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&market_type).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&market_flag).unwrap().as_mut());

        let mut metas = ctx.accounts.to_account_metas(None);
        for meta in metas.iter_mut() {
            if meta.pubkey == ID {
                meta.is_writable = true;
                meta.is_signer = false;
                meta.pubkey = *ctx.accounts.arb_program.key;
            }
        }
        
        let ix = Instruction {
            program_id: *ctx.accounts.arb_program.key,
            accounts: metas,
            data: cpi_discriminator,
        };

        invoke(&ix, &ctx.accounts.to_account_infos())?;

        let after_amount = ctx.accounts.user.lamports() 
                                + unpack_token_account_ammount(&ctx.accounts.user_token_base)?;

        if after_amount < before_amount {
            return Err(error!(MyErrorCode::NoProfit));
        }
        Ok(())
    }

    pub fn arb_process_64_account(
        ctx: Context<CommonAccountsInfo64>,
        max_in: u64,
        min_profit: u64,
        market_type: Vec<u8>,
        market_flag: Vec<u8>
    ) -> Result<()> {
        let before_amount = ctx.accounts.user.lamports() 
                                + unpack_token_account_ammount(&ctx.accounts.user_token_base)?;
        let mut cpi_discriminator: Vec<u8> = vec![
            39,
            254,
            194,
            218,
            233,
            22,
            71,
            203
          ];
        cpi_discriminator.append(borsh::to_vec(&max_in).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&min_profit).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&market_type).unwrap().as_mut());
        cpi_discriminator.append(borsh::to_vec(&market_flag).unwrap().as_mut());


        let mut metas = ctx.accounts.to_account_metas(None);
        for meta in metas.iter_mut() {
            if meta.pubkey == ID {
                meta.is_writable = true;
                meta.is_signer = false;
                meta.pubkey = *ctx.accounts.arb_program.key;
            }
        }

        let ix = Instruction {
            program_id: *ctx.accounts.arb_program.key,
            accounts: metas,
            data: cpi_discriminator,
        };

        invoke(&ix, &ctx.accounts.to_account_infos())?;

        let after_amount = ctx.accounts.user.lamports() 
                                + unpack_token_account_ammount(&ctx.accounts.user_token_base)?;

        if after_amount < before_amount {
            return Err(error!(MyErrorCode::NoProfit));
        }
        Ok(())
    }

    
}
