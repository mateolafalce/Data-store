use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use decenwser::state::DecenwserAccount;
use decenwser::state::CSS;

pub fn css_store(
    program: &Program,
    css: String,
    web_name: String
) -> Result<()> {
    let (main_account, _bump) = Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let account: DecenwserAccount = program.account(decenwser)?;
    let (css_store, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::CssStore {
            main_account,
            decenwser,
            css_store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::CssStore {
            css
        })
        .send()?;
    let css_account: CSS = program.account(css_store)?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA: {}", css_store);
    println!("------------------------------------------------------------");
    println!("Css data: {}", css_account.css);
    println!("------------------------------------------------------------");
    println!("Total updates: {}", account.total_updates);
    println!("------------------------------------------------------------");
    Ok(())
}