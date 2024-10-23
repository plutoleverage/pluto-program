use anchor_lang::{
    prelude::{AccountInfo, CpiContext},
    Result,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked as SplTransfer};

pub fn transfer_token<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let cpi_accounts = SplTransfer {
        mint,
        from,
        to, authority,
    };

    let cpi_program = token_program;
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount, decimals)?;

    Ok(())
}

pub fn transfer_token_with_signer<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    signer_seeds:&[&[&[u8]]],
) -> Result<()> {
    let cpi_accounts = SplTransfer {
        mint,
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    transfer_checked(cpi_ctx, amount, decimals)?;

    Ok(())
}