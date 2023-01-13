use anyhow::Result;
pub mod functions;

fn main() -> Result<()> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]//cargo test decenwser1 -- --show-output
    fn decenwser1() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN").unwrap());
        functions::decenwser1::decenwser1(
            &program
        )
        .unwrap();
    }
    #[test]//cargo test main_account -- --show-output
    fn main_account() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN").unwrap());
        functions::main_account::main_account(
            &program,
            "app".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test html_store -- --show-output
    fn html_store() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN").unwrap());
        functions::html_store::html_store(
            &program, 
            "dsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfs1sddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfsddsfsdfsdfs".to_string(),
            "app".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test css_store -- --show-output
    fn css_store() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN").unwrap());
        functions::css_store::css_store(
            &program, 
            "dsfsdfsdfsd".to_string(),
            "app".to_string()
        )
        .unwrap();
    }
    #[test]//cargo test js_store -- --show-output
    fn js_store() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN").unwrap());
        functions::js_store::js_store(
            &program, 
            "dsfsdfsdfsd".to_string(),
            "app".to_string()
        )
        .unwrap();
    }
}