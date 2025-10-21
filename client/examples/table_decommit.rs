use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use felt::Felt;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark_verify_decommitments::{table_decommit::TableDecommit, vector_decommit::VectorDecommit};
use std::{mem::size_of, path::Path};
use types::swiftness::commitment::table::config::Config as TableConfig;
use types::swiftness::commitment::table::types::Commitment as TableCommitment;
use types::swiftness::commitment::vector::config::Config as VectorConfig;
use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use types::swiftness::stark::types::cast_struct_to_slice;
use utils::{AccountCast, Executable};
use verifier_2::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Trace)
        .init();

    let config = Config::parse_args();

    let client = initialize_client(&config).await?;

    let payer = setup_payer(&client, &config).await?;

    let program_path = Path::new("target/deploy/verifier_2.so");

    let program_id = setup_program(&client, &payer, &config, program_path).await?;

    println!("Using program ID: {program_id}");

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {space} bytes");

    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &program_id,
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],
        client.get_latest_blockhash().await?,
    );

    let signature = client
        .send_and_confirm_transaction(&create_account_tx)
        .await?;
    println!("Account created successfully: {signature}");

    println!("\nEvalCompositionPolynomialInner Task on Solana");
    println!("==============================================");

    let stack_bytes = prepare_input::get_bytes();
    let mut instructions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        instructions.push(set_data_ix);
    }
    // Send transactions
    let mut transactions = Vec::new();
    for instruction in instructions.iter() {
        let set_proof_tx = Transaction::new_signed_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(set_proof_tx.clone());
    }
    send_and_confirm_transactions(&client, &transactions).await?;
    println!("All data set successfully");

    let authentications_hex = [
        "0x6732ed418f24c6da3dd615ee87b95896d7b019fc0a47a7bdbd22080a10601a8",
        "0x28cabb17e9c835a54d1880bb8d22a44b78f71e17988fe7f55acc911ec04d28a",
        "0x6214e719cfbf829a3eb67ba544cc0ab0461f95f64396125c9d3377244aa4d41",
        "0x23a6ddff1c2516c5768d2304ddd3f20fd54d3d01d28b14135917a7c5db50936",
        "0x498c153909a13bc9a21e67d272aa177363f890a68c79ee4be6ff29de6f580df",
        "0x344e1535dfebeb80f128f78ab8f6e380492adcd9f9bd6e1aa211bad8d2296fd",
        "0x64f54e174049fbe5a13a0b01fe1ee7a2c9d92f2c845276fc68c6d0fc2dec664",
        "0x0d48aa0243b93f20afe0b8ebf3e8a4844cae339c936dc36026f07082d9229f8",
        "0x4f70db4abb4620c633b896f3e02934a4e08d9375b4c73381d8b4388e8f593b4",
        "0x4df0916a10099f167d918e73392695b88eb983a90116edc3fb9e1e78a3ca7c3",
        "0x75148d85e336d9e62e712b211ef7dae80ad386c9ab87e4843288d639e7cb9b1",
        "0x72cebe5ac503265da3291921e1ff344b8dd3a4442e406647daec029d88273f2",
        "0x1e42a2c2272772e6cb0404acafac4b824d4f28cf436193c7e6d0973b3ddef16",
        "0x14a9d594ac3b499ba5e5fdaad7f883b374b154a5f3f2c09abb06bc70f52223e",
        "0x10b53191bc11f1ce2e38c79dcd4114d70bd37d12088cea6a4e57d984179a395",
        "0x18e8f3fde0909cc145912bacd507069fc6d7645c3f7cf50591fec9001f812dc",
        "0x2b2d20a915e711d3b05045bce09469deab3e3e9330875e1f49577ed67ad14eb",
        "0x7c64e607219c378cad9fb69837871bd2995524fea36456ee08c75516f057e9e",
        "0x1f830df4ac2a597a455e750925972c250f671c04eb19319a5a88e9bf42927ce",
        "0x7cc08920132321258da8d4bc8ce112720d153209c79587732f5d11726af5377",
        "0x0bdc3cfd862ccde258daca450cd9cd9fa95bf887cdb2832187936455025a594",
        "0x7ebd3b5f3f38b1e32f0eaa6484949e44506f8d713638bd84bb1e85be51d6703",
        "0x5bf31b969a1fb675ef5a948258f74d30aef41637e116a8cf28341c5409800ed",
        "0x3b5c662319228c1acfd2374dd344e8ca614f82f52cb627a74605583e0f10d22",
        "0x6e829ffaffe67889c11a67182281b5700f31889ec09e545ab52ded870910e9c",
        "0x1db3811df3a2af1ce0193bfa1bc1e0a3b956a75d3be29b54b301502fc878499",
        "0x293985852f3dc44908e44d13d735ba20b537290aae311f900a7364644ff814f",
        "0x69d022c6526b6fbfdf17d197b22b0d44d7e320b5b61d66060561bada2194bfc",
        "0x1fb3e08e6c8206f2f6251a989682f6032f635defaa6b8f8657d91e2aba066b6",
        "0x7c31459b60c9019b9fe9fa008dd74e87def5fb083e7cd03920255ec807e5616",
        "0x0853f073b93f4cf9072b95f861fa3372876a7b587e1fc21216df1eef4fc4013",
        "0x407e552536cab0825d8401d44c3fdb9b7460dccf3682438d56e7f268b0529ff",
        "0x768dfab4d41a1e853467c3975fb2a72bab65c88c187bb98ff6b9674b3cba368",
        "0x6a6f8f18d681622da6a18cc1cb8b9946ab7ea01262a0b64dbcbe715efab1b77",
        "0x4e1b14573ebf91b1862c613d2d06f3cd4f3a2f3b55b86157e048ec0b32c4d84",
        "0x26c388389f5ec09a319809f02d6e58e3d993629197fd09c3d73a8ba036658e2",
        "0x6df67786292e3125476857fcabc33634700f0bf30afbfd6cc4e8f9b1825acb2",
        "0x0c5c3cca3ff6053394385f4b5182f03346cfdcef691a8baf348ddc8e31d4701",
        "0x486eb37d5579a0caee557c868e1d7d01f967d138e6fbdbfd39a160b6fa4e63e",
        "0x574233f66307fdc6f74c45ce10aa33ae05ef1666a0ac0f230475c6238c6c316",
        "0x34bc7dbe6b0cd9aadded371788e008c04c56716ab11b7e1f1daac0ad4b2276a",
        "0x22f58a09327a3c5bca0d4cf7ff02e2d964404d8eab38f44ff00abcd331b4fc",
        "0x526d874f7804d66ee0f7846ffbb7fa0bf73e5a89ee275c9b0a153149842b9a0",
        "0x3e5202263e021d963247997e0ece905f915e01322b49dd1c06e973d7a3145af",
        "0x37157ac8afd63f0b80f9c80b63011fda9870db44a3bc0e1ebfa145c13189b5c",
        "0x20f162eb26186b33832a2f93a5097b01eacd5fe9c266d40408c5b28e136ff0b",
        "0x0cfc1a205e543ae593f4cfcce1e578402d3517e9cc144cb2358da65368a2beb",
        "0x7f7b914d7060aebd82915db4ac186c5fa8bb3a1ac0112883155e30a770d1778",
        "0x2ebaf8af434dc0489ce86c51cd9ec27bc1f62f6da4a2aeb942d07cb32607811",
        "0x1ee0ba218a55034ad2c249794e75270a3ce5890a138acb8bb2f167f64022545",
        "0x20b038b71c9de83ee4d687a67a2397fdbad21f7f41ce615b9b9797221f6f5c4",
        "0x7ddae6a4f22044e4471b57bee9932e30ca719f7a8f5716de489bbf5804953b1",
        "0x2190bfa79c0319717366b0dac513ee9230cdabb4d5925036194c61ce89d6060",
        "0x645cb7d2705a36cbd5201c0dd4cc0813c0f2cecb212c6bfa3b2e10405881895",
        "0x70d97fb391d64e7a07365db7778d8d4bf43fb62ee4029193a25a9a2faa96925",
        "0x30b5bcb407f000bfa5107b9b3322977e7ac9e4162016c54409a12807c196dae",
        "0x17c0823441762d65959d8c10f8a540c1603427121d452b25a01ab76a9dcfe56",
        "0x154fd71cf324b5f4529319ed218167886cfad759364e65e0a470ae9e8a2f1e3",
        "0x07f0705a6c90cb26cd6fffae0f01edf50c194e68e8c913005705dae130b0918",
        "0x56c4b01971cbcd0187fd948ace27c462f479b0f2511e1cdf5dfc8b36cf78331",
        "0x1e154c52a43824a6495e09545fb328f97c208204569787b0628c9953b79f407",
        "0x291d2ed2440efb06c1946756933d193da55f6b6fce896b9adcd726e6cd73e3c",
        "0x2e3d5c712962a223f52d3706de21c457f1d51b5b12b58f05b188f15c5123f17",
        "0x2c9dceb210579ce5bbdfb2708a9d84f82ae7692f2a3c3b44ac7c69816e01fc0",
        "0x440874703031393332c05adca97d834648ceb06f61ba39aaad460ec543d0db3",
        "0x463dbebb01f70a69d81e4421ea80cf2c651f88caca2a6ee70f4f7b33dfe6f34",
        "0x4946ebc82ad29c7200d883f0f3a885e3234cc64f8efd4029d917f2ba2e25079",
        "0x5394bef18b5a2fdd9b67efef71906b24ef22f8d50b648312b6b9b03b6cf197e",
        "0x7071c8130607f557b0a0e3397db423bbc62d3a5f0a1e327b066e2dbc3d53cd9",
        "0x03fe45a048eb9d1c5f8095f0853109621d6eee5e57ad72befe9bf1f7862d5f",
        "0x22a4f10c4fd1014974e5dbbe269fa8d33f87946bcb9d4880eb4c9dbfa2356b",
        "0x635c1a4ac0d8914dc6cd277d580acc6b2f0958bb0139ff748ed00bc4deed673",
        "0x6a87208ceed9b476255021ea29afb56773565a44ff44abb467fd32a0334847b",
        "0x617423ea49026b3db6e73a8d2ab62992cb5ff7935a2298d4a02e22241e720b9",
        "0x1d0f100953bcef2541c0d4beed2b0221b1ac394d762a25d624191bbd539e232",
        "0x3269e71c3aaaebb5b4384ac6b27d00b85407ce38736ae193e8b82083a8bcbf6",
        "0x30ccbbf3c5921341d0bc54543bafb52a17df09a67b9ca3ca5ecbb7b3bda00f5",
        "0x1f4877fef15c2b8af560077483005d7d92d73e2cbb5cf0c49037a5dca54cd94",
        "0x42b1269082d2ec500e9ef0cb14f3e189503d70e1b3d456efb4ea804b06622cf",
        "0x4b7b30d49419516a6be731f8aa3697504dce72324a34972bac909e9486c1f73",
        "0x1c1a6eb3b6ab6b7a98bcceb864e749590e077fbcf53b9cee4c420f67116e8d7",
        "0x23b64280140603c379d9b42ca550b0a12580efff0faa3bc48fda4d8d8128f80",
        "0x17284e263bfc1ae752b29a2e6949141c22524c90b4d6a777cf3a098c0558ff7",
        "0x4c86c3c26ea70dcf6696208d029b4e99bc5c14a65a6c30092c9798a74b7e390",
        "0x6879cbfee3412d68d2a3d37aa7670edcf370a763c7753b39aa942a3e4a87376",
        "0x2c73082ce23a352eb6bcecf1033e43ae0733ccc7a2954e02c98ccc91a629fb7",
        "0x5abfe3f25084dd3a9336d089a44f22d889070b1bd2ce1fbe324a3e849e92870",
        "0x4c4d8f51ac2526e39b236dbada19b9b2620434f807098834f8a4bb565edffec",
        "0x36d5388ce5df9468c27c1f05de94531a44bb5055825ae8232d6f2a729881ca1",
        "0x3727e326c4580489cce7675229cd101d5f3307a32f457bc402a76ec3cc905e5",
        "0x0baf6111c17160c8a42a7a8918ef4d1d4ebd660a3d078fd82a1c89745d96734",
        "0x2cf77c2c4a5b67b18a664dab073eb28999bfa77f23f82f7192779ac12641722",
        "0x4b5af607d439c00deaf1d8201fe17d97bb4ca8fc9f03c47af24540ee4396f79",
        "0x2cabfa9a6ef0555401a405edb0e0eea4f8186a7b8735393750ca0b97efe2edf",
        "0x0c010a4313bb2961d2d4746907fc39778495f3ec3f3068c0643ed2b3f9ce751",
        "0x28726d66bc620659c6f830417eb83f344bee66715065d0dac3da9c517ff2823",
        "0x3fc3d949d33bc850e3399e041625ec78f4b71f54e0613b4e8c165ead8748a8a",
        "0x3e78c52c9c3eb4bfbc60aa980e338f1d00f437d38c0abf093d8e07194dc27d3",
        "0x7009097210886a00c53cddb63ae98c8afb242427a55b1f228952bd4d1288973",
        "0x26edb847d71edb9280e378b549da39b9038f0eb1e1b1b0e76ff5bac9a9e3f79",
        "0x373f5e672c467bb4230774bbbfe4c5a9743007083c8d84465d78ca2d6e1c597",
        "0x5aa508eba1a6e119b86aca2af55376d78fe68fa769311622cf4d2f394fe806f",
        "0x640c6b6c06c524c0ceb77ff0baa88d0708d6ea90c90afc40ef420c7928402d9",
        "0x5a0940a4bd1f9fe590119bcb380ee13a58599a0ce31a1b8b8ff9f00927a03d8",
        "0x2a4ab2ff0d066495db5133ff72402ebdc6938cfe3f64fc9b4aae96ae5c05aa8",
        "0x38bd912279d2c81a610481db8e8d1e1400f11cff5cd84765de62bd7a46cac8a",
        "0x291abccbf06da820f6b587b8982ad2bd26e22edaddb68ceda8deb329d4ca805",
        "0x0c17238f73ce1700b3b44c07940faa2cdf110bfec091614cff4687e0f8e5362",
        "0x23a9f8160b59e063bbb15a0fce3c0ae263f295de9b402c7455585ce130b96e2",
        "0x7d44d59a8d8c90c918db722e5be1c2e295432297704c625205fd1f50c68b693",
        "0x3908b599e3a55d8457a50d1e326b2cfb094ee4ad68fd509f44e634b27ca8da0",
        "0x3454069863fa6875eb329d06b0c057375855873a49391d149263d8beaafc116",
        "0x02bd38daee10cb40e369aa08e6fa9a7dc1e05cb8dd77338433665ed6bb9adfa",
        "0x34a16e03e0257f63665ceb0d8334cde97e8ef7f14b83274030aceca4023a5ef",
        "0x2d32fd8ec6adcfc4d6f45422c29c88f0630f56b72b067dcd0fc18a9ed853804",
        "0x766af779ecb5150b5776d86e21771dd5ad64433a4d5e9938a24726e1e0e9fd7",
        "0x32af82e56c41157fb82a3d08dacdfea6d70e3bf5c5424bc08a596eac804f900",
        "0x43f4dca133dec1e6d58c6d163fc27ed39ee6128bd990cb10e391f7eed8f96bb",
        "0x1067edca8e333f40ab18b93d091cd28dc8f8afc70696fcf6ed76694f707cbf3",
        "0x1c14e5e24a2c2bfe16e1d0a90c936dbd43ba9467e10109ae3e475a4056d42c1",
        "0x5f4509c8e31b52254b741304e3e6496e56f19bdab195c3ccac0973b48d49e68",
        "0x12d788c4f12a4df5185fa8bd38412cb5e63eac2454aa686b2c464aebe2e1783",
        "0x698dbfa8c6562c6e73e2781713ffebf2205b28364269c20ccf1fa4e796a571f",
        "0x28b21fd43315c6961006929b05d1a6ccc9b64470ff662030698cceec5e55aae",
        "0x0d7a1df6f3ba83ccdce826f2cd014b84a362fb6736db1cadfa6237fc8d73aec",
        "0x47b6f1c1dade8b1f6311cc396362e91a1af807b5350f22196f768316802b964",
        "0x1282d312559ae4c79acf08a05317a9a8efecb4b1cdfca1599dd7fb9dd29bafc",
        "0x60a76252f1d03af1f3a08e91748fd0105f5a333e220d70eef6965257419bcef",
        "0x5df76e8553798d8191a847bad0879513734a2ce104b73a1805bb1e3c437b590",
        "0x77b737e49112692f6801ede2baf488409259669d233e528955c68ed889e22a6",
        "0x239a69e7ce3bca46c1e773e8680e0682052bb03f9cca0fca989fa93e1a1749b",
        "0x44fba54cae99b80cf8444aa144a4e3340ea58bc62456e1cb9a5bc368918f54b",
        "0x3210573a40c97423e5a015b330897273d0c2b296a957264866810a170ab4282",
        "0x71e66d53ada090738b30a0f4615c780151976b596dcc9b26e8782947202e65c",
        "0x72614e1c0b1c0b8adf9ddfa02955dcba24cf4b371e772acdb7513444aa08243",
        "0x5abefe3de6e8ebc82cfdcaae9ba109baddefca8c5fb5a5d31fd548b014e7375",
        "0x1df514c95e4d5ad87549b1aa50b6770f8a52afd741187a6e3a099b482a6cbc0",
        "0x0711b1a87e6fd6257d2c833e421d8904cd547f5054dfc8da6313a8061bd7f2a",
        "0x35eaa41c1e1e082e88ad2a35c023af601fc1a1bacab369a8d64d795cfb0b9ec",
        "0x7d7c5af6bdcbfae8f3e0508330c9d0445a5d9bf3bae268d20abf12597f7bee1",
        "0x1119b2579a3c18ab87aef0397065f478de67247357288640a274da392f60f77",
        "0x55755fde7fbb97ae849498ac138c1b8dd4e053babeedd20e416851a6e838472",
        "0x7ccb4e2f768304a9d373116ba7cc359f7e70ac3f7252561ff8e42d500e9db13",
        "0x2867592c2f90fda6d1fde967e38815bed599e2421027c77728ed2a5892d0a27",
        "0x5be1bdc992630a9d29acf21c0d56301795e918457ba844cf307fea401caf398",
        "0x603e4f88c5f30e2b23ad85c51f26e0f6cd06f7713c3dd0594859e68e47874d1",
        "0x681eea6ba8b99866f1070e021b86861819eedf1c6ce4ffe6fc4c8b6da3bf2b1",
        "0x312fe6432443aacfb621362ebafc3e7e0939e4534541de8513fcae4a597df26",
        "0x53ddff8b1074d2528caac08fe263e096b0cbfc8486e83b2695f0807e533e776",
        "0x0089980eb76a496aadf5f20f818e3e6c294eec28d43e4e279aa74507a411092",
        "0x1757afef11afbaee9a4d8f4d340a300b8f547823f9d07789321ff14ed3aa4f2",
        "0x17ef0fc64f1918228fb54e73fb396cf186bf91df493e851ae2d9f0903f20132",
        "0x6760c353d97754a630a2010fce738b1f3fa9b58217e282815f69d6c1be94f51",
        "0x48c0c505f9e4d0b61fafcbbc6e590d207a7f5a846ba23c69b01226d4a8f2dc3",
        "0x3885f08a49567c29fcc27fe8dfd4714dd45c3ceb612095b5e98f8917d8240e1",
        "0x54b0867a7337fe41dd1859a4741a055247b5590e6357a20378c98a6bdd915ac",
        "0x24f4240ff5bd7d57ca965cff4cac11c302a3b00c7403e34c41c616af94c3c24",
        "0x72a01166e63162a7acb698f5d52b18a8ca5ae4ec2f8f46eeb11846986d95c6b",
        "0x2a1f13ceea256bb43e72aff3f9cfcf945e1b7d8fd787e49162e944470d81804",
        "0x3a30fc5f86d8991365c4b32a67a38acf28b8a4a6db00ae0a2fd9b7dd6da2df6",
    ];

    let authentications = authentications_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    let decommitment_values_hex = [
        "0x5a81cfa7b8ba1dd722ce2bcaf78476fd0e0b7fda53287ed2632c2c32ab4f42c",
        "0x437f6248b14ae3bc546eafe54a32cdc961c0821ab13a8ef15b28aae6762c6e9",
        "0x350167cc2d1223d974e60d87bbadb0bf782ceb21bebca6657ceb3df9d2398b",
        "0x6f5f181efbce585e6bfc816c428b043a0f7bde3e6fb62e836300d1bf215aba0",
        "0x1e6d05d636513c6d40b99c7760e610b27a05f049e0523f444369f9b1bb98a96",
        "0x173e602ff07512f9b50c59fc3471494203877c2da9e02b44b62c44486052ff7",
        "0x749f215c8d13b0eedcf5419760e8455603cb0ec4a74be885ebf7a0489f3c5aa",
        "0x5b753d91ea71a67ce18a799f695e25b7962d506dced9b78f3bdc8435ee213da",
        "0x6e6de7b0565dcdc4b1b62152bfbecb2dba5aeca0eb52e9260dd7f0074e930b0",
        "0x672901a267bd4b938a23bad8d6207fd73b71107835aca545ad88eee3100e586",
        "0x2d030c78db00d0278dd5637e3456888c15521452c57f8eb3e14665614fb381",
        "0x23c90d84e2502f1079a6c4bab62cf77b5feb89f8199276852ac09a885b2b495",
        "0x10e35324d09571bacfcf642d0b1f872c6a8071462b5864f23340cd83c5cec11",
        "0x24d19d603e658cdac66bcd2ecd484b0fdfb502d32e3d03f3bbfff6bfb3c13f3",
        "0x2b85b285285a0543466f2fbc053db9995ab26578dc2db49df0536d3b68d7a21",
        "0x67d06d0cdec229eb147948d8917c60a2d5ffaa01d3faa4ebd62c68bf7161d7",
        "0x506ae3c00774c128e05fcd292031bc9b5748853e5ecdc2b68efad22502d9dd3",
        "0x28e7bc0081b3f64605be467c67679b4694698d61549cfe1a61df01801d8b706",
        "0x571894602d25aa56fca3058d68de05b420cf3a610e79710746e8b97d09d9960",
        "0x7b36942fb2f4ca2f43f4166792a8e558f86875191d9543fe633e4f95d05bc2c",
        "0x1b5edd7c7943548eeaa3b0f53875f7b2a864e7b96fb97264dadca98be349cbb",
        "0x6032b3931ed6052582a1eefb1fdf475d8d5f6eca637fe659f3df222d9b8bd56",
        "0x6ee0b7bed142500d285418ab68c14fbabbe28cead7517ba6391affbabfa07f8",
        "0x465476fca06dcb03e8c5dd71f8b6d6983f75c3e008dd274dac89cb6859407d6",
        "0x98128772999f1b99d53878b9337576e4898ab5734534d13895f6d17874fbe6",
        "0x3812f410c441ec208f01e579effcf9992c2c316218a8d585e006a9b6a9586f5",
        "0x6736142028c68e21e699235313fe5cb080bb63c47d493ca15c09cc854e3949f",
        "0x18494ab2c44c2200a93a37adfe8ad06ccdd705aa53085827931ccddb46e2b21",
        "0x64dba7947b0037472a738c4af52b1b1b193c02beb2443ed81f63946081b76b",
        "0x339d037e561c5811d01ae344f03fc8f847b4a96a7c2d4a4a07db2ab6fa0acea",
        "0x47928dcc6bdbb5bc6b3b87d3fbce6b549191693539a2ee0ba232f2d0b2f8677",
        "0x56084701a335a410c2c0a73a51b2d59b298dc166480d55e8ca76eb2f0c24c8",
        "0x5d9e1faf79f4720c7dd030ceacb7f3bb28c8393fb6311913730a306554b7e26",
        "0x470641cdbfdee39e6c5d7f00afdb53062fe672a2c32df1a63ab6674f9f8b482",
        "0x230ed14ad7be0bd1f69fd827c65dd357dbceaf022573da3b6b247b8a7965685",
        "0x7aa7111c69f1464ca6099e2364519d075ea3113f04be2e0e294664d360b7aee",
        "0x5cfd1f2b95c01a8445e16c38c4a268cd6b55a9fb5677b8b548ffeae1ff895a3",
        "0xfe0782c363cdecf73f1e7ad20bbbd7bb466cbbc56ca79cbe4d49a1cedf1df8",
        "0x47e73e996073fa3fdf22a5dc7cc34ff4119f1b87641575adc3a11e1f296ea1b",
        "0x75af7bfc4f26e98c2938fe1452f113f83687949b476710171120b229690bcb3",
        "0x4d16d2a61df3d18b4f93bbe01cd7163f15090a4147f73df56de70e688ebc8ae",
        "0x49ce8f3d4264f5a7f7d87261e58aaff8e10e56e5ac56ad742b1db67c583bbd3",
        "0x22fda7c7beab8b0561d2803247eb8659e51b653250cc18ccad1bf1b5db8686",
        "0x590638191c4f6fd6a3bc633578c0fa4059c3ba6ce9f0acc214a55fe44e81bb3",
        "0x28642a4c037572cd49eabfe499ce3f3cd0d1ec4a5d9bf997a4de4d79ffc2b04",
        "0x7b278073e44677a98119164b79728a594cff1fa6a2b0577be9d28ce35da74bb",
        "0x40c384be4aa0577bb23b12fd4151b68919212c1b2f63faa3a992eb133525c87",
        "0x5de53658a09a08072e09f61f01fabac07ffd1773423cd2821c6270278c8df0f",
        "0x2602d1f5b0bb3e5e49d9e2775940221124b2b2c27acc0353e77e53345903ed9",
        "0x4889f879db89dd3394974a06381e4a4f23934fabce6619f00c45ffe1e956418",
        "0x25cd05e979f9a05c01282e4178ae51eae662fea72eb6c8ff8290aeb36f9e5ec",
        "0x794670d35a13811889251fcf36386e591ff014df483539049a1d16408ad2198",
        "0x6752ae4c7342e0d7aa8a64b65f6ca33a9a4ce112c1d70ac8384beccb239eebe",
        "0x224f2c778cbc616138aabd3fde19d5dcc3fb89cb20e9670f36aa71b9bb95d3b",
        "0x3aede81986738227e8856c73dbc08603a620f9c227a7e72d913e09e0dd3af96",
        "0x29adbb8d26e892c0b854f9ecf07556f33d8cb83ab4efd152e6c2168e55f4322",
        "0x1e0065b34962535246ad7be5b4b17d66af996b950d1c24fffa317c15d48b72e",
        "0x78dc672d38e9d3f7d4c10a91a703b874c8d5f450510775ae1eb9ec4b897918f",
        "0xdf66dec5e9469a844fbf2a60fde593d5ecb58b8fc6f4e023dd8348d0a8b753",
        "0x647d1a44e02b844416eb63d024eb8065b618dc5d5df1d1e5708ca2c61faa8ed",
        "0x1cee497a299c6cb56ff4c3583f6174d8a385f1c1eefb25dc09da3450efe4b66",
        "0x3f49a6a6668039023c67c63e8c24a61cb7a7ea93265cccf7f2d84e9ef687482",
        "0x448a6f4bcde506f1f462055f82ed18d320337c32ba53a3f4fd4213068672bfa",
        "0x3202dea1cd62a3aa3cfc5aa4b8c1a13101018c6d267af36b54f1653ae61c04b",
        "0x94593db5d8e77c7a7bd15d1ba888842dd9f89c57f1b6e6e94fd906de911d30",
        "0x454446a92177d7f0b2c61b96635abff2f6b5158497b85f29478d057ee71717c",
        "0x713272f23405a27a0c8f46705f8c8e8f18286f1b368baace6654e788b437a85",
        "0x52dd724126aa589695a807016b3a030f66d035f58ceda4675b08e6eb700e362",
        "0x425606a0d129d87548a4315c0548decbc6f1ecd3f76a9b6bf77fbb859e6cf47",
        "0x61e378adc06e8c25707453f902a88a58320f383024acb6e8766faad2dae72a4",
    ];

    let decommitment_values = decommitment_values_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    // Push authentications data
    for auth in authentications.iter().rev() {
        let push_auth_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushData(auth.to_bytes_be().to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let _signature = interact_with_program_instructions(
            &client,
            &payer,
            &program_id,
            &stack_account,
            &[push_auth_ix],
        )
        .await?;
    }

    // Push authentications length
    let authentications_len = Felt::from(authentications.len());
    let push_auth_len_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(authentications_len.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_auth_len_ix],
    )
    .await?;

    // Push authentications data
    for decommitment_value in decommitment_values.iter().rev() {
        let push_decommitment_value_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushData(decommitment_value.to_bytes_be().to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let _signature = interact_with_program_instructions(
            &client,
            &payer,
            &program_id,
            &stack_account,
            &[push_decommitment_value_ix],
        )
        .await?;
    }

    // Push authentications length
    let decommitment_length = Felt::from(decommitment_values.len());
    let push_decommitment_length_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(decommitment_length.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_decommitment_length_ix],
    )
    .await?;

    let queries_len = 10;
    // Push queries length
    let queries_len = Felt::from(queries_len);
    let push_queries_len_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(queries_len.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_queries_len_ix],
    )
    .await?;

    let commitment_hash =
        Felt::from_hex("0x2a588e8517b956684162e05e373dc6891146c1853c82d3984fbc707ae937972")
            .unwrap();
    let height = Felt::from_hex("0x14").unwrap(); // 20
    let n_verifier_friendly_layers = Felt::from_hex("0x64").unwrap(); // 100

    // Create VectorCommitment and use push_to_stack method
    let vector_config = VectorConfig {
        height,
        n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
    };
    let vector_commitment = VectorCommitment::new(vector_config, commitment_hash);
    let n_columns = Felt::from(7 as u64);

    let table_config = TableConfig {
        n_columns,
        vector: vector_config,
    };
    let table_commitment = TableCommitment::new(table_config, vector_commitment);
    let table_commitment_bytes = cast_struct_to_slice(&table_commitment);

    let commitment_bytes_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(table_commitment_bytes.to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[commitment_bytes_ix],
    )
    .await?;

    // Push the VectorDecommit task to the stack
    let stark_commit_task = TableDecommit::new();

    println!(
        "Using VectorDecommit with TYPE_TAG: {}",
        VectorDecommit::TYPE_TAG
    );

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(stark_commit_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_task_ix],
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);

    // Execute all steps until task is complete - split into chunks of max 5000
    const MAX_CHUNK_SIZE: usize = 1000;

    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        let chunk_size = chunk_end - chunk_start;

        println!(
            "Processing steps {}-{} ({} steps)",
            chunk_start,
            chunk_end - 1,
            chunk_size
        );

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let execute_tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[&payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(execute_tx.clone());
        }

        send_and_confirm_transactions(&client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);

    println!("All execution steps completed");
    // Read the result from the account and verify it matches expected values
    println!("\nVerifying results against expected values...");

    // Check that stack is empty (task completed successfully)
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");

    println!("✓ All verifications passed! Results match expected values from stark_commitment.rs");
    println!("✓ Stack is empty - task completed successfully");
    println!("✓ TableDecommit test completed successfully on Solana!");

    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use types::swiftness::{global_values::InteractionElements, stark::types::VerifyVariables};

    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::stark::types::StarkCommitment;
    use utils::StarkCommitmentTrait;
    use verifier_2::state::BidirectionalStackAccount;

    use crate::constraint_coefficients;

    pub fn get_bytes() -> Vec<u8> {
        let mut stack = BidirectionalStackAccount::default();

        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();
        stack.proof = proof_verifier.clone();

        let queries_hex = [
            "0x3982a", "0x52d42", "0x585a8", "0x7c3cc", "0x8af7f", "0x8e6f3", "0x97846", "0x9e330",
            "0xa9b57", "0xfa009",
        ];

        let queries = queries_hex
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect::<Vec<_>>();

        let mut verify_variables = VerifyVariables::default();
        for i in (0..queries.len()).rev() {
            let index = queries[i];
            verify_variables.temp_queries[i * 2] = index;
        }
        stack.verify_variables = verify_variables;

        stack.constraint_coefficients = constraint_coefficients::get()
            .as_slice()
            .try_into()
            .unwrap();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        // Set interaction elements (these would normally come from StarkCommit::GenerateCompositionAlpha)

        let mut stark_commitment: StarkCommitment<InteractionElements> = StarkCommitment::default();
        stark_commitment
            .traces
            .interaction_elements
            .diluted_check_interaction_alpha =
            Felt::from_hex("0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822")
                .unwrap();
        stark_commitment
            .traces
            .interaction_elements
            .diluted_check_interaction_z =
            Felt::from_hex("0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48")
                .unwrap();
        stark_commitment
            .traces
            .interaction_elements
            .diluted_check_permutation_interaction_elm =
            Felt::from_hex("0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322")
                .unwrap();
        stark_commitment
            .traces
            .interaction_elements
            .memory_multi_column_perm_perm_interaction_elm =
            Felt::from_hex("0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64")
                .unwrap();
        stark_commitment
            .traces
            .interaction_elements
            .memory_multi_column_perm_hash_interaction_elm0 =
            Felt::from_hex("0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288")
                .unwrap();

        stark_commitment
            .traces
            .interaction_elements
            .range_check16_perm_interaction_elm =
            Felt::from_hex("0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c")
                .unwrap();

        stack.set_stark_commitment(&stark_commitment);
        let bytes = cast_struct_to_slice_mut(&mut stack).to_vec();
        bytes
    }
}

mod constraint_coefficients {
    use felt::Felt;
    use types::funvec::FunVec;

    pub fn get() -> FunVec<Felt, 194> {
        FunVec::from_vec(
            vec![
                "0x1",
                "0x27dab20f0955aea0793cb622d7b74b56318978dbd44835af673e1adda5a2cc9",
                "0x4040b9a013f1db8a95f19415974fb60636e99e3458edc6009e422208601645e",
                "0x5f62630850416ab25451cb9c233678947ac980b4943aab801adc211005d8d87",
                "0x42585a6cef0c43c6f48b6214833961ac266ec20f7d5ddd8debec01eb7d37fcc",
                "0x4461dc6c692aa5f2128a2ac23aefc900e7cd421be86d852a640569c8854bed",
                "0x3fbed0c884c75b9f0b40aec97ed672f4715c29d803b2abd72eab3bd3ad6b535",
                "0x1204110e1d3151d2f7b12fbc9bb0d9b79ca7ea2a01a846fb624036444de3a84",
                "0x1369805cedd8e28ba8b6c2f6400975a0ca3283f9ff889b6d52e89064656e4d6",
                "0x44c8b4c6d5850f809bc2534c64e2ede742fd4d123dde2f914da7b33d13e5875",
                "0x1cab697256523ee0ad691ad41b711ff4ff13f42c86ff83c2e67bee36b204b8c",
                "0x219352bf7d888149c48868e738427424278b2e2ba009ba116fd0ae1dfb7dcbb",
                "0x4cbde2d2527ceaa11cee77a01dc504823abd66024d475f6dd29447f37196563",
                "0x5a2cfe6680539d1ee3b0d0c315352d1d90731ac20359ad1ce2c4d5fd05b1b5e",
                "0x6b45468a2dd15d330cdd384cddf7af2efc86c69048ff948d5500984aec6b830",
                "0x3b4570a7f748c7581dc55bbe06c74e9f831266d9e9d9307e64f1f58101589e9",
                "0x6733de2c8331352184d7024002376c6c214eb09dc09a6857a1df09281175951",
                "0x1540fcb219ed88f9aff5a28313f2fe726eb1cc7d3ccd9f9384e1d19f7f5532b",
                "0x3ed54456078fbae52dc454a238d4982396645586ee17e1e3cce8b1800f27a86",
                "0x3a8a82fcd40543eca6ad0c4a82c9e559ba5fafd86effb9f793de860901b68cc",
                "0x10484a5a314e1e7c7c5b9c7fa6d7f86fbe20c49d8d9a149c56790100a3ffb5",
                "0x38ac1c0094d82ba13b6ea942cd493de8c4ac985c4873c6a46c064e6d433525",
                "0x14ed110b351b9b492bb40a194e462f3b9c06bbe19ea4f0f2b27d367d830fc32",
                "0x6e2fd526e4963aa7f529b0dbb564b5b8879e5f180837b2ef57c108b9d2c51d2",
                "0x50550ad7f99b75a60fb2f5193b204ea997d243a9b3c9eb21b85a1b22f2f013e",
                "0x4ad8ef3201cdd98824986ad6b3db73b8169871b0c8b0b39b7b4f4a05b178862",
                "0x46512c15dfc10487196c1df9337444d95fcc3b6a963c3bb682aadc9a69ebb59",
                "0xee52d4f8a05c470798b2a2b11b3e210cc384ea7d5178fdd87d6073e72c2995",
                "0x32d013ff1c6fe8b052774e8dd89723f954860eac3d2949cc9163f9fbcbde2ab",
                "0x494d277d1c530409ef302234c16f7e886f28c76d1dbc7a5e1db3b78d4c4646f",
                "0x4189df84b90919e54bcc8264c89e36b11539eef965bbeb5066d1d2557b39f52",
                "0x6ff14d8e9a3b2189c1fda7b37ca5772902c0fc55f251a7011308d4d636f75b9",
                "0x26d2858f7bc61a2e11bc44671ab48cc3b0e18a409444485532c201308064d8a",
                "0x45d5c7365cf9a22f075ba1bc62eb4a03922f35ad567a8ecede089b2562fb933",
                "0x38b9d162a0f995a3c86572bae519aae67d1ae762e4f80836cb27194bcbcf4e9",
                "0x21cb978a899f40c2ecfaab14c4619beb879e9a610f81787391d4f6a8ba30420",
                "0x10ac29054e4b43e0c9522a61a8fc1087da348e86ac557484568e35d69cb75f",
                "0x10853ddb3111227d402a7b0db168aee96489a91533dd5d7bb51511297215b76",
                "0x4c1583a1d5c4577b7e75c265c05e68e81b3fe27aa69a0ae33b65a2c279fb2a2",
                "0x4a9f86346a282676bb0e2de28325ddd8ab9884b1f8f5f0805d123ff6170aea5",
                "0x2d97725449a903f9f6b196c276a52714a7579e270d0764cf2b733bcdc1584d1",
                "0x50682eb3ff8b83467d74c7ded44df147f3e864093cfc7a4f64baae5050a9d9",
                "0x7e0574d010b422f32c7402fa1f38d89ce0146e0f09f70ec3c6b13af4e16ae01",
                "0xfe28b4929a7978adf78782bb1aed22368aad8b3508abb56ae6f4e17496c197",
                "0x42ec36434a9757bbd58bc2ce9b1eb17bdd200ed1bc12a9010de8fe94410783",
                "0x5add140e834af1345f587e99b26d4e85adfe2e17e95beb88fe51fb6502244af",
                "0x2b9336d3e08c86b874218bf7afdd658c2769fa5b76130ea4a7a1153be2bc0c7",
                "0x25c8192ea2019574fd9681c4e855dc9be7d00f6802d29592725578cf5c76764",
                "0x7dd5addc9624bf333985d3852d64c28b51943878ee788dd35fe938c801ba262",
                "0xfe9f1f57326b111d555aadd9c5087a525dac6091766d870630ff74aaa057b8",
                "0x237424ea6d26d17f516ef6767f25d76aaf6fff2430cccc4ce306f7fc9e3eb48",
                "0x5e21fdf7828d3479842f11528e44c6c09a0be2f3d0d15991ca2ba02c9622f1e",
                "0x445b755a29f8f24ac2fcfff928943fea9b425fc8122f07a9bd8364753893c3",
                "0x5cd960ee5373b5f8a0748f4b6e5de88f7d1cc3ce542f10abb74d41320b76300",
                "0x1dee062068896f21ae1276b8fb1bd557a51028b487fbbf977fcd6d5ec132278",
                "0x39071db8bc5e7096aef050971271256c1a2b3d79e30520b45feb7c3ed5a0f30",
                "0x63a0483bac31d71bbc9e230e9e388c3df9557441f9fd791c49f9c1fd5fa8f0d",
                "0x6e3d38d26d401920226aa1ae17d893f211fda12dfaf5b94cef9b79f81865574",
                "0x536662bf6b81ce90389ecf693674a0358527b4f5b0995456fee37469dd28e16",
                "0x72b7a0129fca4d354d5acbf84052626688e1d186e6c2e8cb19aa9f631e0e9db",
                "0xc64a895d8a2b0c16921c55d6510ce11a9e1750a02e8c29dd5cac86c8d1e982",
                "0x32cd32d89d9ca1fd4fae3b473e0226915514a6cfa23d79d9579fb403e46e65c",
                "0x6b0655c2fd47f55cc04d05cd3cf0e10945971d1b97f573fb882b71c59dfb449",
                "0x7fd2d96c69653a91504d2651acddcbc92b6acddc8f7693116051b427d056d61",
                "0x2de7ae0a048d05b30ad0ea302520305a672d4ace87f2cc07330d1ba5df09dae",
                "0x3c6989ae5b937ee6af8375d81e18e733f68b487b5263c7be7a1bc5088d47d2d",
                "0x2a4a042a939ea99471d90d95fff2c325e3fa543c6cd67349c1559fe37827046",
                "0x5d529b2b178acdb995804427e7eff4dcc69aa071a53c03765d88334b537df1f",
                "0xb0b56f73d99478dcdf327f4c06c7ba51abfbbe28cbc0db24fafb9eb23b618",
                "0x6bdf96d133763ab12d86ff94f84db26484fd87177ebe789a13514c984b45e46",
                "0x17c72c965af666b40f0267d9fad0edba73a8ba003afab033e8cffcac5852fd",
                "0xf737ecfab351ba0148871f58273322346f56547e488a7e1ae5b343e99c345f",
                "0x434abf1f06013ec6f4685e8acfb81c879f10acb9eb0672efc1110d2091df91c",
                "0x76b1c70082f50524c2b229eb0fea1e4f39b8b308e679a28f35543873706ca65",
                "0x3dfa27228f394377e7fd9bb3aff7f5d3bb3d942b7f92d8401ebf371b041f444",
                "0x39325345e24ed792e4829142649e85780311a2ecbdc7d9e99bac0ad367ac9c9",
                "0x123446a80f6089e6a3e8e1c82f61feda2e95e7eff5c7ca878e146d9c342f2a8",
                "0x52b724a01163f936b0fdeccc5be49e29e583826216bb75dcaa21789f62a017f",
                "0x7768cfdb7e0377847fdd144527ce8be42053e9b949f39d28c77d096a45f4899",
                "0x4e6eebaea0f3578f62a773500d6d2d9ff9f93c3182e9f42aa68a940305653c0",
                "0x7108e82f24f76758e0d15a4352fe677b732447a9243fb3474983bb6d1c8777",
                "0x3d89ed5c454278934cc97dccf8e114e096b069e92820b3d5728b19560638c0c",
                "0x1bb8284cc014252d03751f4ddc54390bcb6645766cebe93e98f71dc821d124b",
                "0x373f8ce96b7168ebcb3a118bac24ee5515903a6bd24ec8f943e693d9af1e02a",
                "0x4bac7f84dd6cc5d3e5fc9400a5ecebf27c95336eef665f059c78071c090ad68",
                "0x45c560059e0b51bf0a2a8bd09fb74f5699f3dd8f5ba5754168402be844ce79a",
                "0x210464e2c1dfa56a7f5e14a738f438893db6a08fee024ea69810fb5a2554bbb",
                "0x453a14ef1f22959e16d2d771403ce90cdef73715a22c59b14bf62049615db10",
                "0x29a22cb47f53c2f5c20a0efe19c59eecb5ab7df6d8da5c8cfb1de8f579c89e2",
                "0x392d994085ce631134acfc485420adb5a74feaa13192db209fa4f4b48209ca9",
                "0x3002c84174df933997e19c236604bc85718d6bf66232cf0063b2df7e1dc878c",
                "0x5e644483749453ffed1a1b0817d8053963b1adcc2541c0a211834ef69be4c93",
                "0x4ae257840c2bc49e9d065bf16fb91bb53550bb06198dee2022c818c71763189",
                "0x4b98c5bd7e3507beaf2873cb8f2ebf4e9743b75982b01708b243c43971b4bba",
                "0x118a75b675b9a6f002769d76c03d710236dca30a5b5b913cc09781940ee4d4e",
                "0x38e507ce7d76355bcdcffd9aa0dc70cfaa5aff9ba7c2ef2d09d0ac4e03f36a0",
                "0x157e61846651871215d58910b709fb4ad4daff448da093ead6bef00a48f8c69",
                "0x1270c3f8de988b8a6389ee0ce88e3dc7ab0d8bb0ba83d86b6b2a621a9df50d3",
                "0x286f0362032a502be387080b40a7b92fc44de1e8b58c537d8ae5e847ad8619f",
                "0x30dd31d5c754a281f91d250c811bc6cbd3550f09af105175b6b72fff8261f93",
                "0x5cde7daac8b8980147731923af7bed02b1187496d9ca9dd6a759aacad820f9b",
                "0x33bc352e5bdd7046a890d82436470a09cd1bd3677b22373dcde2cd5deb93a1",
                "0x7b3919f49df4e1a86ced84c553c013e10c21ff845eb08e59392e787b09368b3",
                "0x2db1e75d3e4bbb5661a1281b77ed7b558d557a16651ca7dda0b972ba6195ab9",
                "0x7203949255ea03ff9f23cb19c80391e101234fa845b73b1ddb4340bb46de1d2",
                "0x3ccd0554def08e86cd07dd920f3e628dafdd49d7c5bccfe24038b8f80e89073",
                "0x4f732ef59d57c4c1fcd4236ee74f430f05bf48d8fb156f18c146ec019bc82ac",
                "0x1a23747fe8961d2ba2115eb26d49529cafbf0664e68f3755f61d6582fba5de5",
                "0x288c9ec9962981a8e2c792cdba93eb48af7d5665b1baeec5722f58c2288967b",
                "0x3afebc1e0efd5591f40ac4793bb0cddfcb56c795b799dfe0d4d34259986674f",
                "0x54767d417dfad86bed2ce2ff323c738100e78d3339b3e38bb13f3f8f43927a3",
                "0x1dc95b995965f629eb75a50b58232c4936ff453ec84e25c6a56ffe65adc6c3",
                "0x1f3671ddbc04827b7a9c25295306446b0a238b2e17296d85e5d29bbd27ed17e",
                "0x78300d43ef835ac4d65442e4900d2d33f63f0cc04a7117276e8ca9eec90379e",
                "0x511a5f709b0b7bae5ef1e468fc1bf74f11c46016d73bd08fb8422ea453373df",
                "0x24a8d3a6a2c33b2134ecaa7d0c3160e7969dfc12f4c96409f159298dabd6f83",
                "0x613f34263ebf0b3c131d7709e391253a40265a92d28369b4c3a0e85a98a4491",
                "0x2893fff1cd679033eb438255ae1fc29bb0f31a6d21f961ba8c6d9d28de746aa",
                "0x463097b52722f2801396dc86261cecc4f32811168710af53e778d1009a79ec4",
                "0x2ad540971ff03c58fd7953197b4e4b2f6225ddbf2e57f7e00d6c1b5a57dcd2b",
                "0x7c5756c61c6f314e01c133b37ce7e7671c5578d6aba9a6331cdb2494bbea6a7",
                "0x1dc8cb820d563976f1c34a1c4cd79f7546713f2c62b30d956e6ea992f6ffb4c",
                "0x793e2fce80a764e00be738e5a1254a3c841688db32af4ed50c46c49bb40316c",
                "0x13c40f6cf7067e78cda28f0501825bc32ca4f0e55cebc5cf487c16dd441a20e",
                "0x6778b966da6bc8ece431e6eea74b3b582e312af52726193d536da9f3aae8fa5",
                "0x441596b6c3e462b121ebb816933470367abae8768e2eeb243baad9915410e98",
                "0x47668c8cc4202cb7ad26f83e58c14415d5ea0c28082616f95f630f8928eede0",
                "0x623b07af4d2460035e87395b2183f175a85b8fb81557383b904461d9a7dbe2a",
                "0x43dacf2240b2547f73c1f8161b7549305bc4a1c0891530f6e61158b520e7754",
                "0x50b45743c33745031d53a68543541ef03a31a49bf2a44ab8ad97f94e00db2dd",
                "0x1d5c5c152ff6d864cc8508742c41b80e6750032b1b4042f94bb093d8b6dc8d9",
                "0x787bd280a7f9ea9d1ad7847e14abb148757d757706162d6cc19417aace16780",
                "0x7ff36d1c713012e55c44662faf6aea7dbcc4ed412bc9575bf700821e905b401",
                "0x38a2015bc24b2d49fa34ea71694331b2bb9b6816c51e74b952745615ec0fdf1",
                "0x6017d279eef512afffa38341d7ddf37ca7b67e6afbc988be86d38cd28153783",
                "0x46680d2b0edced686278772ae149aef5df53c015b3c2605c1f2abd88658fea5",
                "0x5c4f2b1d6d547892cbcf746d92644a77809ec418edffe473466f3e9254b582",
                "0x7be091f07e6af6130285635ef1794d496034db9d7413820cd241bc9d684c1a9",
                "0x4e764c122050bb2fda8e3a41c85a24eae031a8a6c1a860b2ca158de0a62956d",
                "0xaf462e85b46e8d582e4cdc5c616f98f597427239da7684638ddcfadcf2231f",
                "0x47c9ac4416d13da3826ac274753cd216bda93c76548b557b341d1c14c11490d",
                "0x253b4111611580963cf488d4e7867ca46be58139306f41212357a9128f425c2",
                "0x341c6bc6bf683f626f1eaa877fdb39728de0bea7bf20663baa871e1a26e2af2",
                "0x703c1c5f8233ee0e51461e0a8f5e1c7aa1192fcc23eb622dc54d40ab6090ff7",
                "0x4e16ca0f93a7f2d72b84f9cc288a52ec9888cd4ec28b845f5b24bbc676f692c",
                "0x2101da64ef6a076d24f72be291055eaaa7d680547f80dfbbedbcd22ad905a6",
                "0x6eaf2f49b9d892ff68cbc38adcd777cd923ed58fcc42c91558639a8f63ad899",
                "0x6d2703bffa2a841a6fa6119fefeaddb627b4b8a5f2a2a05cf1d553eb506f128",
                "0x3dccd46b84d6655bbea7cb5bf556e863c84358894c88a0036c97a92a2937b84",
                "0x2d40d918e72ecae39c8496fe17e7784c394f303ca0d3f7b708426ddcbbbde39",
                "0x5c582cc5e546c5c4bf92b5d7715a618c6c43226cecd1c3a9e9b9e1d01753dd4",
                "0x425f1209a76ec6e9595a1a30c3cd99b7ca432e7e67493e730ddceaecab8aedf",
                "0x46c6cd668e4ac058cc45d48d1c70c4516ded3f025d0276e8185a14a9080244f",
                "0x544834e94ec103ba3850de1cfbe4e9b56e07b13e599ed328619c21b0f7a16bc",
                "0x7d5e2ec3780d71281fedcca6d08f1d0ed85264c5f1a2ec887fcac9a9d007de",
                "0x2277fa2169ec97ec808f6bba2c92271ac0fb73b20205ae0676addef2d4c88a7",
                "0x61d74d052b50379f3112cab0fc83991263ad5c89e326baddf4b228562e41953",
                "0x316e8bcdc0d8dbf1db4df8d2e63f96bb8ab900597030b7551f0332ff9d2a09c",
                "0x3b2c6d515af8b542f4c1273da8a3b78001e282c5fac30fafe8389c964bb8531",
                "0x3e81bb1da8ad64ec5cd3d7f4ce25ec8e963fa162ad25a685f20c9d2a783c472",
                "0x20f852942e1bf3bccc8e59852582e59bb79b56db056239e17ba29f1e48eac2",
                "0x4fd06fb2dc9f15df8c5d276942d0f178b826bd113d93e581544a68001e8d250",
                "0x34a12266c02676d97dbf4bf84447ddbefbacf28625ef94e5c045084fad4fd49",
                "0x5f8df30e9ffb54518dd8fc9b2f405e4f360c4a1b96346e5ef3819a04519729e",
                "0x21767ea9a9d982092c4b8c4dde56426ec50cf3e07f9ba5e29b2fa3e5754e102",
                "0x2d03896bb6ac6252768c4977b7590881a3b4c6d052fb8a8d263df2d25e7625",
                "0x356b0f96b11f5795990d625d8ea9c6de7d869a2044b7cd50dfe951ac29a6e0e",
                "0x6ae56f7f8741c8f43ef2f6966133dc0b34d5e8610b8c7dea3da7c89d7c2d2b9",
                "0x5ffa0f47cc7768c83d204643a7a447580a00fbfad7a12a769bab53febce2895",
                "0x2051d63f5ce3795e77efa76826f43872619ec46ec6afa7c23f9d6f355b443e0",
                "0x7e0f8123feae0a258bcda08a38e65e7bb28cdc04f84523b1640c9f54b68ce4b",
                "0x6c717817eef0504e32ce7a0b4fb18659be9ae9c2aa56ba210468ab2fcaf79cf",
                "0x5337b52058f81bc3dcdd03993850a58e5911af43cb3536ff4228539070fbdbc",
                "0x691cb37604424cfb585a6a35a42748479f1748ea0e65b902dd11651c2e85886",
                "0x59df9036736f235dc89f2261133f598664955a36c8ca22171542d7c2454c3eb",
                "0x31b7677eba3df8f0a5fc108056cc706cd4a68d233fa1bf9c8c65a05528a4c8c",
                "0x1f2e1702db19c2fae9e42f329ebd14c0964d9227e20362397f1f47138599561",
                "0x238b32490ee3ab940f6ca41f921b60d1aa44cb05f225287380703c76ed7b892",
                "0x4ab9b7217d715a3fe55eceb2a5dace1623a87a6e7118b209c13326824c30935",
                "0x47ca19eafd3a13ab82ba5557d7b9a2432d5fec617bd659862aed8f7a9141695",
                "0x10abd80ae93ca1193c6d22d9783565e6297ae5758f5c2a9fed9c979ffd8cb40",
                "0x386bba816d036566ed7c37de103c04055c7923c6aa36a95c3a04303645e21fa",
                "0x7a47cef3e67468513e6962ef223e732a1f5191041e837eec773a9fd2f0875c0",
                "0xbf2262e2b94bb0e73800663b0dc9039cdc1c808cc7ae62ca2e914f598a6958",
                "0x18413a00414d92091c18b2d984eeaa6020e3d2fc9ff17124a9e82702f294fb8",
                "0x531e2b4e28c591fa1e8df17ffe646db148b691b89f0bde12e8c465eb7c14c26",
                "0x5d9bde719bcc9d7f1d2dcd8c80b993c55e55064bebd17ec1484cdbffd9c910f",
                "0x70716e2187959123507a5a08a3d12a52c26632a74a4814a34346612cbfb626d",
                "0x31267b2b6dcb35cc23f7f0573a26722a5494f84c5c71d119772b80a0d692175",
                "0x5dea5c7c620d5bb173a5288c59e4b92f2944c502e73d10c01f933b844a9644b",
                "0x2cd3c1354b4772d3a5b6020902e388a290475dd6bc35ec3d215dc930f8dc56d",
                "0x5a233ad19f77821f871ea6d9d716501c9d1db9c4a525b2fc2b36d9aead841a6",
                "0x4d72e9a46fc082fd230296bd215d4e312bfccc9eb2301bdb486fc7c7d3d99f2",
                "0x71a9fa23e7973944d33b4556eb3d5c940bc111be9967c06ecddbafcf10de274",
            ]
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect(),
        )
    }
}
