use std::{collections::BTreeMap, vec::Vec};

use anyhow::Context;
use miden_objects::{
    account::AccountId,
    block::{BlockInputs, ProposedBlock},
    testing::account_id::ACCOUNT_ID_PUBLIC_FUNGIBLE_FAUCET,
    transaction::ProvenTransaction,
};

use crate::tests::utils::{
    generate_batch, generate_executed_tx_with_authenticated_notes, generate_fungible_asset,
    generate_tracked_note_with_asset, generate_tx_with_expiration,
    generate_tx_with_unauthenticated_notes, generate_untracked_note, setup_chain,
    ProvenTransactionExt, TestSetup,
};

/// Tests that we can build empty blocks.
#[test]
fn proposed_block_succeeds_with_empty_batches() -> anyhow::Result<()> {
    let TestSetup { chain, .. } = setup_chain(2);

    let block_inputs = BlockInputs::new(
        chain.latest_block_header(),
        chain.latest_chain_mmr(),
        BTreeMap::default(),
        BTreeMap::default(),
        BTreeMap::default(),
    );
    let block = ProposedBlock::new(block_inputs, Vec::new()).context("failed to propose block")?;

    assert_eq!(block.affected_accounts().count(), 0);
    assert_eq!(block.output_note_batches().len(), 0);
    assert_eq!(block.created_nullifiers().len(), 0);
    assert_eq!(block.batches().len(), 0);

    Ok(())
}

/// Tests that a proposed block from two batches with one transaction each can be successfully
/// built.
#[test]
fn proposed_block_basic_success() -> anyhow::Result<()> {
    let TestSetup { mut chain, mut accounts, mut txs, .. } = setup_chain(2);
    let account0 = accounts.remove(&0).unwrap();
    let account1 = accounts.remove(&1).unwrap();
    let proven_tx0 = txs.remove(&0).unwrap();
    let proven_tx1 = txs.remove(&1).unwrap();

    let batch0 = generate_batch(&mut chain, vec![proven_tx0.clone()]);
    let batch1 = generate_batch(&mut chain, vec![proven_tx1.clone()]);

    let batches = [batch0, batch1];
    let block_inputs = chain.get_block_inputs(&batches);

    let proposed_block = ProposedBlock::new(block_inputs.clone(), batches.to_vec()).unwrap();

    assert_eq!(proposed_block.batches(), batches);
    assert_eq!(proposed_block.block_num(), block_inputs.prev_block_header().block_num() + 1);
    let updated_accounts =
        proposed_block.updated_accounts().iter().cloned().collect::<BTreeMap<_, _>>();

    assert_eq!(updated_accounts.len(), 2);
    assert_eq!(updated_accounts[&account0.id()].transactions(), &[proven_tx0.id()]);
    assert_eq!(updated_accounts[&account1.id()].transactions(), &[proven_tx1.id()]);
    assert_eq!(
        updated_accounts[&account0.id()].final_state_commitment(),
        proven_tx0.account_update().final_state_commitment()
    );
    assert_eq!(
        updated_accounts[&account1.id()].final_state_commitment(),
        proven_tx1.account_update().final_state_commitment()
    );
    // Each tx consumes one note.
    assert_eq!(proposed_block.created_nullifiers().len(), 2);
    assert!(proposed_block
        .created_nullifiers()
        .contains_key(&proven_tx0.input_notes().get_note(0).nullifier()));
    assert!(proposed_block
        .created_nullifiers()
        .contains_key(&proven_tx1.input_notes().get_note(0).nullifier()));

    // There are two batches in the block...
    assert_eq!(proposed_block.output_note_batches().len(), 2);
    // ... but none of them create notes.
    assert!(proposed_block.output_note_batches()[0].is_empty());
    assert!(proposed_block.output_note_batches()[1].is_empty());

    Ok(())
}

/// Tests that account updates are correctly aggregated into a block-level account update.
#[test]
fn proposed_block_aggregates_account_state_transition() -> anyhow::Result<()> {
    // We need authentication because we're modifying accounts with the input notes.
    let TestSetup { mut chain, mut accounts, .. } = setup_chain(2);
    let asset = generate_fungible_asset(
        100,
        AccountId::try_from(ACCOUNT_ID_PUBLIC_FUNGIBLE_FAUCET).unwrap(),
    );

    let account0 = accounts.remove(&0).unwrap();
    let account1 = accounts.remove(&1).unwrap();

    let note0 = generate_tracked_note_with_asset(&mut chain, account0.id(), account1.id(), asset);
    let note1 = generate_tracked_note_with_asset(&mut chain, account0.id(), account1.id(), asset);
    let note2 = generate_tracked_note_with_asset(&mut chain, account0.id(), account1.id(), asset);

    // Add notes to the chain.
    chain.seal_next_block();

    // Create three transactions on the same account that build on top of each other.
    // The MockChain only updates the account state when sealing a block, but we don't want the
    // transactions to actually be added to the chain because of unintended side effects like spent
    // nullifiers. So we create an alternative chain on which we generate the transactions, but
    // then actually use the transactions on the original chain.
    let mut alternative_chain = chain.clone();
    let executed_tx0 = generate_executed_tx_with_authenticated_notes(
        &mut alternative_chain,
        account1.id(),
        &[note0.id()],
    );
    alternative_chain.apply_executed_transaction(&executed_tx0);
    alternative_chain.seal_next_block();

    let executed_tx1 = generate_executed_tx_with_authenticated_notes(
        &mut alternative_chain,
        account1.id(),
        &[note1.id()],
    );
    alternative_chain.apply_executed_transaction(&executed_tx1);
    alternative_chain.seal_next_block();

    let executed_tx2 = generate_executed_tx_with_authenticated_notes(
        &mut alternative_chain,
        account1.id(),
        &[note2.id()],
    );
    alternative_chain.apply_executed_transaction(&executed_tx2);

    let [tx0, tx1, tx2] = [executed_tx0, executed_tx1, executed_tx2]
        .into_iter()
        .map(|tx| {
            ProvenTransaction::from_executed_transaction_mocked(tx, &chain.latest_block_header())
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("we should have provided three executed txs");

    let batch0 = generate_batch(&mut chain, vec![tx2.clone()]);
    let batch1 = generate_batch(&mut chain, vec![tx0.clone(), tx1.clone()]);

    let batches = vec![batch0.clone(), batch1.clone()];
    let block_inputs = chain.get_block_inputs(&batches);

    let block =
        ProposedBlock::new(block_inputs, batches).context("failed to build proposed block")?;

    assert_eq!(block.updated_accounts().len(), 1);
    let (account_id, account_update) = &block.updated_accounts()[0];
    assert_eq!(*account_id, account1.id());
    assert_eq!(
        account_update.initial_state_commitment(),
        tx0.account_update().initial_state_commitment()
    );
    assert_eq!(
        account_update.final_state_commitment(),
        tx2.account_update().final_state_commitment()
    );
    // The transactions that affected the account are in chronological order.
    assert_eq!(account_update.transactions(), [tx0.id(), tx1.id(), tx2.id()]);
    assert!(account_update.details().is_private());

    Ok(())
}

/// Tests that unauthenticated notes can be authenticated when inclusion proofs are provided.
#[test]
fn proposed_block_authenticating_unauthenticated_notes() -> anyhow::Result<()> {
    let TestSetup { mut chain, mut accounts, .. } = setup_chain(3);
    let account0 = accounts.remove(&0).unwrap();
    let account1 = accounts.remove(&1).unwrap();
    let account2 = accounts.remove(&2).unwrap();

    let note0 = generate_untracked_note(account0.id(), account1.id());
    let note1 = generate_untracked_note(account0.id(), account2.id());

    // These txs will use block1 as the reference block.
    let tx0 = generate_tx_with_unauthenticated_notes(&mut chain, account1.id(), &[note0.clone()]);
    let tx1 = generate_tx_with_unauthenticated_notes(&mut chain, account2.id(), &[note1.clone()]);

    // These batches will use block1 as the reference block.
    let batch0 = generate_batch(&mut chain, vec![tx0.clone()]);
    let batch1 = generate_batch(&mut chain, vec![tx1.clone()]);

    chain.add_pending_note(note0.clone());
    chain.add_pending_note(note1.clone());
    chain.seal_next_block();

    let batches = [batch0, batch1];
    // This block will use block2 as the reference block.
    let block_inputs = chain.get_block_inputs(&batches);

    // Sanity check: Block inputs should contain nullifiers for the unauthenticated notes since they
    // are part of the chain.
    assert!(block_inputs.nullifier_witnesses().contains_key(&note0.nullifier()));
    assert!(block_inputs.nullifier_witnesses().contains_key(&note1.nullifier()));

    let proposed_block = ProposedBlock::new(block_inputs.clone(), batches.to_vec())
        .context("failed to build proposed block")?;

    // We expect both notes to have been authenticated and therefore should be part of the
    // nullifiers of this block.
    assert_eq!(proposed_block.created_nullifiers().len(), 2);
    assert!(proposed_block.created_nullifiers().contains_key(&note0.nullifier()));
    assert!(proposed_block.created_nullifiers().contains_key(&note1.nullifier()));
    // There are two batches in the block...
    assert_eq!(proposed_block.output_note_batches().len(), 2);
    // ... but none of them create notes.
    assert!(proposed_block.output_note_batches()[0].is_empty());
    assert!(proposed_block.output_note_batches()[1].is_empty());

    Ok(())
}

/// Tests that a batch that expires at the block being proposed is still accepted.
#[test]
fn proposed_block_with_batch_at_expiration_limit() -> anyhow::Result<()> {
    let TestSetup { mut chain, mut accounts, .. } = setup_chain(2);
    let block1_num = chain.block_header(1).block_num();
    let account0 = accounts.remove(&0).unwrap();
    let account1 = accounts.remove(&1).unwrap();

    let tx0 = generate_tx_with_expiration(&mut chain, account0.id(), block1_num + 5);
    let tx1 = generate_tx_with_expiration(&mut chain, account1.id(), block1_num + 2);

    let batch0 = generate_batch(&mut chain, vec![tx0]);
    let batch1 = generate_batch(&mut chain, vec![tx1]);

    // sanity check: batch 1 should expire at block 3.
    assert_eq!(batch1.batch_expiration_block_num().as_u32(), 3);

    let _block2 = chain.seal_next_block();

    let batches = vec![batch0.clone(), batch1.clone()];

    // This block's number is 3 (the previous block is block 2), which means batch 1, which expires
    // at block 3 (due to tx1) should still be accepted into the block.
    let block_inputs = chain.get_block_inputs(&batches);
    ProposedBlock::new(block_inputs.clone(), batches.clone())?;

    Ok(())
}
