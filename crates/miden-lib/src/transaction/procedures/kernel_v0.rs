//! This file is generated by build.rs, do not modify

use miden_objects::{digest, Digest};

// KERNEL V0 PROCEDURES
// ================================================================================================

/// Hashes of all dynamically executed procedures from the kernel 0.
pub const KERNEL0_PROCEDURES: [Digest; 36] = [
    // account_get_initial_commitment
    digest!("0x920898348bacd6d98a399301eb308478fd32b32eab019a5a6ef7a6b44abb61f6"),
    // account_get_current_commitment
    digest!("0x1c294b4d92eedce11252353164d712200adfa5cc443ac3667dd86ef33b303c8c"),
    // account_get_id
    digest!("0x1a5583b3a4011d0ca83ac9633fc12b0c6ec2cba03ee8c5f380ac69fc4f767075"),
    // account_get_nonce
    digest!("0xf1dfe3621b9147803b6668352915be7fb7f85df476c9d18052272270a854fa75"),
    // account_incr_nonce
    digest!("0x726357558767a56aae315c4c8aa406cdf57ad400832222622eec58dc42cce406"),
    // account_get_code_commitment
    digest!("0xbab83830e881bdbee08fa1506d651388c20ebb0cadfb6794189542dd257841aa"),
    // account_get_storage_commitment
    digest!("0xe4cd056f358b2438c3b378a31650ddbbf8d631f135bf966e01a2b57eebd458a5"),
    // account_get_item
    digest!("0xdd8f439cb6f7f3edcda15c9c339e7c2b2dada2fc94952a8199081a197aeebb7a"),
    // account_set_item
    digest!("0x61104ec016c3ed9b49aee53650ddde9e984a72e4c4e13001cbf98b9cef426758"),
    // account_get_map_item
    digest!("0x21237825d10004f77d3e7b32aee9052b519752fd03f839c19440e2010f73132e"),
    // account_set_map_item
    digest!("0x876168292d11aea0e2886ee6d7f9e723a95ec3aa4e467e6f46c898567bfc3604"),
    // account_get_vault_root
    digest!("0x279b4a9e5adca07f01cadf8ecc1303fa3c670003a7a4e69f09506b070c4023df"),
    // account_add_asset
    digest!("0x7ae966cc7da065c7d06248ccdc5cf319eddba55c063900fc97833f39870234f1"),
    // account_remove_asset
    digest!("0x50935b368b7843258ae86f6392df9346721b60dd9563d77a2d548da8ab81e44c"),
    // account_get_balance
    digest!("0xc3385953bc66def5211f53a3c44de8facfb4060abbb1c9708859c314268989e8"),
    // account_has_non_fungible_asset
    digest!("0x4fea67ed25474d5494a23c5e1e06a93f8aa140d0a673c6e140e0d4f1dd8bd835"),
    // faucet_mint_asset
    digest!("0x836ecba4b7caa770b24ffa51115e2c08724ecb7f0854ee3a94848bec5b0ee2ce"),
    // faucet_burn_asset
    digest!("0x37e6b7c80f478c11ea7f12c065013c4bc3a09100fd6ecc4893e17d0abd675254"),
    // faucet_get_total_fungible_asset_issuance
    digest!("0xd2ee4bd330f989165ee2be0f121a4db916f95e58f6fd2d040d57672f2f0cef63"),
    // faucet_is_non_fungible_asset_issued
    digest!("0x04c49c1f9cc628c1447e19d46c307084e995044faa86154353eec6af6b5b5041"),
    // note_get_assets_info
    digest!("0x34e4f1ea83eb4342ab8f5acec89962b2ab4b56d9c631e807d8e4dc8efd270bf2"),
    // note_add_asset
    digest!("0x999a790c639966f85b4420f7b00a5965db163e7a69fa288b059f79162f3f81a8"),
    // note_get_serial_number
    digest!("0x59b3ea650232049bb333867841012c3694bd557fa199cd65655c0006edccc3ab"),
    // note_get_inputs_commitment
    digest!("0x9d4af62050a2024dbd9e1967f2ba9b81f7801e8eb704494498904d3affd74a55"),
    // note_get_sender
    digest!("0x01172024b89517e5da80121cedfa6c19dd2ace0fe4d09a8cde6605103fe62952"),
    // note_get_script_root
    digest!("0x66fb188ca538d9f8bc6fd1aedbd19336bf6e3a1c0ae67b5f725cbc9cb4f7867f"),
    // tx_create_note
    digest!("0xb0ce6d0bd9dfa8b77408fac615eb570381d28769914e7ca0475ba5dd00a2283a"),
    // tx_get_input_notes_commitment
    digest!("0x16cb840dc9131e2fd2b3e83b8d796eb466722ae36f29f27b4b053f1bee2ed473"),
    // tx_get_output_notes_commitment
    digest!("0x0c241940512d130ad36c70c4e946285cb5841f2655c4fe12df001cb834256a29"),
    // tx_get_block_commitment
    digest!("0xe474b491a64d222397fcf83ee5db7b048061988e5e83ce99b91bae6fd75a3522"),
    // tx_get_block_number
    digest!("0x297797dff54b8108dd2df254b95d43895d3f917ab10399efc62adaf861c905ae"),
    // tx_get_block_timestamp
    digest!("0x786863e6dbcd5026619afd3831b7dcbf824cda54950b0e0724ebf9d9370ec723"),
    // tx_start_foreign_context
    digest!("0x6e7ab409b5661e1d03dc73bd7b6e32ca7785a983e49ab2d056e9c786620e5c20"),
    // tx_end_foreign_context
    digest!("0x90a107168d81c1c0c23890e61fb7910a64b4711afd0bf8c3098d74737e4853ba"),
    // tx_get_expiration_delta
    digest!("0x756352beed1624a42d4540c434a4faa986d6d9d08ef8437699d9086fcd9ad9e7"),
    // tx_update_expiration_block_num
    digest!("0x11ca0c8662d20e6b05fbff4a20423bfa52595862b6c7c5c5ef1cc0a917e4cb62"),
];
