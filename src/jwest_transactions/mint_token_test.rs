// https://0xksure.medium.com/mint-tokens-on-solana-using-the-rust-sdk-3b05b07ca842
#[cfg(test)]
mod tests {
    use solana_program_test::{tokio, ProgramTest};
    use solana_sdk::{
        program_pack::Pack, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction,
    };
    use spl_token::{
        id, instruction,
        state::{Account, Mint},
    };

    #[tokio::test]
    async fn test_initialize_mint() {
        let program_test = ProgramTest::default();
        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint_account = Keypair::new();
        let owner = Keypair::new();
        let token_program = &id();
        let rent = banks_client.get_rent().await.unwrap();
        let mint_rent = rent.minimum_balance(Mint::LEN);

        let token_mint_a_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &mint_account.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            token_program,
        );

        let token_mint_a_tx = instruction::initialize_mint(
            token_program,
            &mint_account.pubkey(),
            &owner.pubkey(),
            None,
            9,
        )
        .unwrap();

        // create mint transaction
        let token_mint_a_tx = Transaction::new_signed_with_payer(
            &[token_mint_a_account_ix, token_mint_a_tx],
            Some(&payer.pubkey()),
            &[&payer, &mint_account],
            recent_blockhash,
        );

        banks_client
            .process_transaction(token_mint_a_tx)
            .await
            .unwrap();

        // Create account that can hold the newly minted tokens
        let account_rent = rent.minimum_balance(Account::LEN);
        let token_account = Keypair::new();
        let new_token_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &token_account.pubkey(),
            account_rent,
            Account::LEN as u64,
            token_program,
        );

        let my_account = Keypair::new();
        let initialize_account_ix = instruction::initialize_account(
            token_program,
            &token_account.pubkey(),
            &mint_account.pubkey(),
            &my_account.pubkey(),
        )
        .unwrap();

        let create_new_token_account_tx = Transaction::new_signed_with_payer(
            &[new_token_account_ix, initialize_account_ix],
            Some(&payer.pubkey()),
            &[&payer, &token_account],
            recent_blockhash,
        );

        banks_client
            .process_transaction(create_new_token_account_tx)
            .await
            .unwrap();

        // Mint tokens into newly created account
        let mint_amount: u64 = 10;
        let mint_to_tx = instruction::mint_to(
            &token_program,
            &mint_account.pubkey(),
            &token_account.pubkey(),
            &owner.pubkey(),
            &[],
            mint_amount.clone(),
        )
        .unwrap();

        let mint_to_tx = Transaction::new_signed_with_payer(
            &[mint_to_tx],
            Some(&payer.pubkey()),
            &[&payer, &owner],
            recent_blockhash,
        );
        banks_client.process_transaction(mint_to_tx).await.unwrap();

        // Inspect account
        let token_account_info = banks_client
            .get_account(token_account.pubkey().clone())
            .await
            .unwrap()
            .expect("could not fetch account information");
        let account_data = Account::unpack(&token_account_info.data).unwrap();
        println!("account data: {:?}", account_data);
        assert_eq!(
            account_data.amount,
            mint_amount.clone(),
            "not correct amount"
        )
    }
}
