use felt::Felt;
use stark::stark_proof::stark_verify::traces_decommit::TracesDecommit;
use swiftness_proof_parser::transform::MONTGOMERY_R;
use types::funvec::FunVec;
use types::swiftness::air::trace::config::Config as TraceConfig;
use types::swiftness::air::trace::Commitment as TraceCommitment;
use types::swiftness::air::trace::Decommitment as TraceDecommitment;
use types::swiftness::commitment::table::config::Config as TableConfig;
use types::swiftness::commitment::table::types::Commitment as TableCommitment;
use types::swiftness::commitment::table::types::Decommitment as TableDecommitment;
use types::swiftness::commitment::vector::config::Config as VectorConfig;
use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::StarkCommitment;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;
mod fixtures;
use types::swiftness::stark::types::StarkProof;

#[test]
fn test_traces_decommit() {
    let mut stack = BidirectionalStackAccount::default();

    // Data from traces_decommit_log.txt
    let height = Felt::from_hex("0x14").unwrap(); // 20
    let n_verifier_friendly_layers = Felt::from_hex("0x64").unwrap(); // 100

    let queries_hex = [
        "0x3982a", "0x52d42", "0x585a8", "0x7c3cc", "0x8af7f", "0x8e6f3", "0x97846", "0x9e330",
        "0xa9b57", "0xfa009",
    ];

    let queries = queries_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

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

    // Decommitment values from traces_decommit_log.txt (70 values for original, 30 for interaction)
    let original_decommitment_values_hex = [
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

    let interaction_decommitment_values_hex = [
        "0x5bf29218811908115445900bc8f367dcf29cf113bfe0e29f3c669b396e12da6",
        "0x84945c17d78c44d588525e6012070172989c66a12f876d40def2eabd025352",
        "0x73dd90541f088a8be7da7ff68e532d4af514312e6c367913e2d93aebded6888",
        "0x6f925f5f494f60aae00761a87d21aa2d29b807514e30407a6049618c4d54db8",
        "0x7d3e24eb2c919340e3e8573cdbf8330830b040dc6cd255497435039c9fff343",
        "0x1e33d04a094e6d4dc2ccbac5f94effdf5800ec814d42f4b4b30642a1b537c5d",
        "0x554156d17b1d78c4503b5d9ac0b5aef51cf1048c18c017318955654b81e106b",
        "0x112cbcd71ebd64a09832ba3b69a8ac48af5176386e09e908b5d7118adfacdad",
        "0x8fff861ec8a2f0216e8161a3a0dff2a6384f9dd9eb6c1c3cbf39f2c7a8ebf6",
        "0x2e3dae8f2a70ca35f49f5ebe4a33613fbb8bcc59c42e9681e81c007aa139f1",
        "0x1c9a20fc0184009000e8a93b983d6016117e8279dcd4afeef8b0231313a4068",
        "0x3f111d9ad96bfdfc7d467e2ff0b79fe3a7e8bd5e2736a58b4296ab8e910d25f",
        "0x3ddbf3767e51556ca3914783920d508c993d7c4b80b7a9232cc132674f4124e",
        "0x35a280058cf32f47441a42db2dd0a41848ad8560e2a7e2ae806196006f1d306",
        "0x3807fe3c5ccb585140ad0e58ab11f20ccf3d614684b8014a3deab4f607f4a8",
        "0x3312b1827bf625d9269f7a2b8597f598b2ab729e2406f40dd089155cdfedeb0",
        "0x1085a4ee6cea111d74c9ca967e5b558625bb9e93bfa4d1617051cb66378c57d",
        "0x18b9b375d9da086d8825ba6f683a19f1a6afb30d3cd8b3ab8a5b242040d21c7",
        "0x718e58a2f2f48f6bfecc1263b2ceb43d26fd1aa86bf12f2c75cc564c592108c",
        "0x60c3dbaea374e6b68e8363f5769023ece533a0581a098aacd185fbe657b5a10",
        "0x164e1191ba919ecf586655e920bae73134c7b8db66416579074725b71154177",
        "0x4a283c8426e01924982760c0180e9b4dad90e3dce99f017eea4e62a43ec1e7a",
        "0x72fe1e3b53b3c59593a7f77afd48966ba8a2cbc7fafa7b2ec80ee929d080134",
        "0x54e169ac57ec327fc0fc6bfa109ccbf0877335becd6bab57e53e2c5d2963452",
        "0xafd9d0db759ab20cfaab153a2e79339dfe6a73e0349059cf0ce17c52603dc3",
        "0x23d84e00e69b16f915492c474a2b503eb2ff556c5b68fd3319cca404d7a3c6e",
        "0x60ad0242b3f839ce26a48fab179628f81a02768082dfb09c48db6e44e6dd111",
        "0x1feaa9ecc39d6ad84bc3bb79327b39731545e636ee2ca7ace1677cb8b1f6887",
        "0x6cab97e59dae1d257f420aea18b44d84d40cc76cc6bbf81bb4307f5e28c88af",
        "0x461c77600d1fee552f0b2ae8cb786c17c02360bcde0c1b2c568590792af759c",
    ];

    let original_decommitment_values = original_decommitment_values_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    let interaction_decommitment_values = interaction_decommitment_values_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    for index in queries.iter().rev() {
        // Query: index first, then value
        stack.push_front(&index.to_bytes_be()).unwrap();
    }

    // 2. Push queries (length + query data)
    let queries_length = Felt::from(queries.len() as u64);
    stack.push_front(&queries_length.to_bytes_be()).unwrap();

    let vector_config = VectorConfig {
        height,
        n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
    };

    let original_commitment_hash =
        Felt::from_hex("0x2a588e8517b956684162e05e373dc6891146c1853c82d3984fbc707ae937972")
            .unwrap();
    let interaction_commitment_hash =
        Felt::from_hex("0x7171ffc67e24fcbb2a7d1acd6244fa91c54dff15c96ca26d193907b716ce2c5")
            .unwrap();

    let vector_commitment_original = VectorCommitment::new(vector_config, original_commitment_hash);
    let vector_commitment_interaction =
        VectorCommitment::new(vector_config, interaction_commitment_hash);

    let n_columns_original = Felt::from(7 as u64);
    let n_columns_interaction = Felt::from(3 as u64);

    let table_config_original = TableConfig {
        n_columns: n_columns_original,
        vector: vector_config,
    };
    let table_config_interaction = TableConfig {
        n_columns: n_columns_interaction,
        vector: vector_config,
    };

    let _trace_config = TraceConfig {
        original: table_config_original,
        interaction: table_config_interaction,
    };

    let _table_commitment_original =
        TableCommitment::new(table_config_original, vector_commitment_original);
    let _table_commitment_interaction =
        TableCommitment::new(table_config_interaction, vector_commitment_interaction);

    let original_decommitment_values_vec = FunVec::from_vec(original_decommitment_values.clone());
    let original_montgomery_values = FunVec::from_vec(
        original_decommitment_values
            .iter()
            .map(|x| x * MONTGOMERY_R)
            .collect(),
    );
    let interaction_decommitment_values_vec =
        FunVec::from_vec(interaction_decommitment_values.clone());
    let interaction_montgomery_values = FunVec::from_vec(
        interaction_decommitment_values
            .iter()
            .map(|x| x * MONTGOMERY_R)
            .collect(),
    );

    let table_decommitment_original =
        TableDecommitment::new(original_decommitment_values_vec, original_montgomery_values);
    let table_decommitment_interaction = TableDecommitment::new(
        interaction_decommitment_values_vec,
        interaction_montgomery_values,
    );

    let mut proof = StarkProof::default();
    let mut stark_commitment = StarkCommitment::<InteractionElements>::default();

    // Set up the proof with data from traces_decommit_log.txt
    proof.witness.traces_decommitment = TraceDecommitment {
        original: table_decommitment_original,
        interaction: table_decommitment_interaction,
    };

    // Set up witness data from the log
    let original_authentications = FunVec::from_vec(authentications.clone());
    proof.witness.traces_witness.original.vector.authentications = original_authentications;

    // Set up interaction authentications from the log
    let interaction_authentications_hex = [
        "0x76b789ba37b249ed446240d80a657fd3a5216fdef370ac2e67966da1781f111",
        "0x364dd8b93f77e8fe4a4220cb995c4b80079891290789864fa3c100ab21bec05",
        "0x3cc20832ca880bc6ad2eaacb38e0ffea115901137c46c31fe838a4238514049",
        "0x348907f2e55dce07e27d77e7deab058a8f501647dcb9eedee3e250615a04884",
        "0xc301771c274048c2e4cf9a80cba0f5f6c6b55ad24f7f88c74cdfec6d9a40eb",
        "0x181abc85910722d6eb29e740b06d2c45c21dae09a007c0f0df98de5cba3b7d2",
        "0x202effed198702786efe1dc32cde6df6fa54278bae9c4d801e05da6292c0f11",
        "0xc9e7ce3c1c9ddf651f57010923085513b541a85001eae3d4a08f1ca1f63298",
        "0xdaa9ce9b236f5d9f5b07937d8d042a354fe1e2f63c10000b0df1693bd8ce82",
        "0x5a09b9f8b632d9d548b473a53d9b0c2412d5c36382bf6f8d13ba59a7475f455",
        "0x2913951a49284de65d5ab35c06dab42cae6e92ac073035d0af51d261a6fc08e",
        "0x35464b46b9239eda631924a70a930e9f8d08cf1defe97a9ba33b2453a5b6291",
        "0x70eb30da3695280edfba8b540a75732aa6dc840dcc99a0b730f81c58f4bd4d0",
        "0x4ba37beb9e6735d2e47788d96e841e10fe506e5a6b88f275a70ce7beb2a9967",
        "0x4d2703aa177413be9e1e3b4580d7921eb0df767adef8aa0e0dfb43d3a11464",
        "0x2d9bfc4a97be1ad887ca557e1063cf7cb83af174b61d977c3950eca104685b3",
        "0x90a12c9a8964cea0a0062997bec48724249543e13ee96d120cbcbf3f31a56b",
        "0x496b8770873e2597d658f6d918be7b6545d1dd2f85fe19fb4a8fd416e8b513d",
        "0x69b9eb7973b0fcaddfcb3c8a6ea0c5361375f7c7ae16385cf9c887d30bdf2cb",
        "0x287a3ec04cda03b1ba5b4c117af129033e5571356e78d4527f2cc670d022856",
        "0x20d7ac2523a5c88f689a4da0f18fd7394fdac8c34f567d2951fec771fa984d8",
        "0x2f445fd061f2ba231c04fb1e32baa117563c07034cfcb13c1b3fb79b07a79b6",
        "0x6eb3c9fb9dd1434531abf6e291f701e06c13c987d1c3ba3f643d4e45064742d",
        "0x17afd828f62de9cc5ee017618c80e7e064f66b9a631ba13dce6ef7949b6bfc8",
        "0x38d895b73ff320fec7098cdf7c1a1b68280a184b3db3f5b452d5019f5e5361",
        "0x492c1da796bb73bb3029eb41451c8b779c7cb92b80c589d9094861c065a916c",
        "0x679b2916f02ceaceef3c3ed8c3f068229c6435316a2ce19b9ab463ea67a373b",
        "0x3bf581f4d94a39b2b41494919584ef328f3bff01e7e58b4c9015a3502a737cc",
        "0x7c934170516c3836f2ce10df2a3cf5339cdaecd2353ab97e89a77cc6aeb14bb",
        "0x43c46699d2ec397138b261d4bcdab798903f2cf2890730476011135826c9a2f",
        "0x42cde356b7bbe38893c983635932cfd0a5ed77eb6a77f3788bf388f67d60af7",
        "0x3f25a361d52b2ff8fe8267f07650c9506b1de04cc9ed82d84f2102af0fc04c5",
        "0x696d504019609ed98b34337eae1e5d4590c3c7ff8d7c333644e400734a63162",
        "0x30bff263f7ce997b2a79c0bf2f6e0598632d7290a721a15edd725be2c69df29",
        "0x3d45d035c364e5f1f28236c55284427305a30afc602feb34e0e8b9df582bca3",
        "0x4a714cac0662c1a852d712a26e4cbeeeacb9f7116b769dd11fc6684e2efddf4",
        "0x362768880673b2913007dc8606dbe556457191f2afdf607764b93954b024281",
        "0x480ab8e6165c76254bd56054b8a8c3ce6d5de60cb3084a2669af5ec8f36f75",
        "0x7fd76d34699b64be7a980af9c8bde3737c61df11b38fc5fa39fa71cc1d86c5f",
        "0x4c248805022a02f16a9f650e015f829d1c89d1c459af7157f14bcc1f46bfa1",
        "0x58288d9c1b845ddde62bbad387ef2d6960ec43ff3412c67ca0720e9bd63614e",
        "0x40871f884f21fb25bfa97e01ee9a2bd4bfa689e0c34a86e907d1f5f6a0abec9",
        "0x49cdacb2c7bca4996af15fa53d6bb45862fd34986d48f217cc3047ddf6c4d45",
        "0x503de266db2ecd69cc2ed3a2410be913e777179891a49aba0f931c165a2420b",
        "0x3573f6fce688a827535a2e82ea94b515c13d26e456fb719d2a2f4ad85eea13f",
        "0x6a2904e5ef366202b5c2c263a4aa230b80cd44d5e037faf7f3d130061ad20f6",
        "0x4759e31171ca140476a936f4dce9d15c412cd99044839c0913d2784f1125b5d",
        "0x251ef225175f621ec7d6cfc275100997d019544bc3d0b8262e9abdbea6e4bad",
        "0x709fe6fe7a173da6f301e8ca6a95f32c0d8964070f58ccb059c57a919baed62",
        "0x5d34845bd23ac1fdfb64d6f00c8c0a3eb52f13ca1af0a773d91330cb29ed206",
        "0x6d0dac6849e3c1b12aa08d641e8647954383b1375d258b490317060f314f614",
        "0x6c8c76d1fef8b75cbabd284f621f49e76cf60db0c6739d3f1120ee5fb6427a",
        "0x2a06dd43d9c6b756330563bbaade7f47898924787aae724f6deade207d02811",
        "0x3d2e609634b4d1379703bf2d9d54bdfdfee3ae14e1a2f94502366695b83f536",
        "0x294f7d5a2984cf078d5946973f41dc80a6dfe3d5bf89a24791d85f7ff3bb9c5",
        "0x3d88ce09803c433f5442d75cbd3d7d35b00912329664b8fede98627febd77d",
        "0x12bf84d3ed4b26d5c5de2fe84469663f717905dbfd614e4551bf71cf82ff4a5",
        "0x4fbfec4a2da7ab549af92264fadfcf01c26d2591904cfbb6e02dd714d705e89",
        "0x687fda78e7de0256d84f04eb7f94b9be5bcd017f6618edfa51858bb32d510dd",
        "0x6087d5ea70a6389469e8fa2094785812f75368e86faee9bce58ab74a17277f6",
        "0x1cc1ea86d5772b9a7397ad41989f2e73e76eedd7605177ad3131cd8a150126b",
        "0x27f2f556a6e391fe6328aa105917e8c6d1dcbb2e4c7ea92de6c173757395a28",
        "0x69d1b243bcecd8b1c8abad30db6b8fa74856970db0843669a805edd2b14c00e",
        "0x1d9a02e2319e56be57a3cc2be932d9fee602960a6229547571b2e1b92eadf50",
        "0x4bd8ed28820db1ce055f4b79809d9e11a99c1e1f766777dde07ec452892041b",
        "0x79fe4421d71a83ff347c92b6ff8d231b6e74b43c5f2a0c098bdd7ea4bb30b7c",
        "0x49d6aebc099ac97edcc682cc0b76e22b4a8c93303e458b5d9b6e4e13a158fa8",
        "0x7cb5ef83e467ab67f9795ad86bd45fd55487cb68de0c316ed16cf4daa4f1505",
        "0x7d281768a3b3492df15855e1080411807790f9b89286277ac0da55ab13e3176",
        "0x7b8052c7ef750eadb16ac7e87527ab76fe331006f61752ca7b9dfdca97bf493",
        "0x2af715167fefed6fa0887799d9798ebe99b256fa2eddc94b43c6c9146f5090a",
        "0x18796cd4ed88e1750e229851cc7861bf1ea23d0f318b5605d4712b2580d5e1f",
        "0x4e408e992bae49aab8b3b66696af3dfb886beaec391fb412e07eda5f65941f1",
        "0x7c6c81a18bca75581a49438eae384067c07ed270b3d68a6e48348265e49e6b3",
        "0x1ca7da8b0480c44aeece37621be633fcee5c4636a979000abd383ccf9d224f1",
        "0x70f86634c6abc920496946dce41c0d5e4db8118fd83d63bee0fe94f35d6a36b",
        "0x7ef6623b4cb856b176b3a18dad4bca6735b5ab73c0f4a0e05ad0747ea73006a",
        "0x2a193cb6a0683e8a19cd4b59f9ca501cfad289be86b80ec5c35b4f85bb10132",
        "0x25fdcaffcebdc6bad5c375e9936dfe096c2a77b86059a86aa4ec6e8cb5c4216",
        "0x3ea2279958ff5852eeb4e64d80f862fef7d10e17eeecf8d9c7e9d56fa9c6430",
        "0x4c0db58654eb85c6c576963d5318659f6bb24784c995d200c6fbfb630eed95e",
        "0x46d53379d0a45e9bb282186624beed25ed6a5071609b35a394e9df575094b08",
        "0x2f260e73f0e718a49a5668cc9f43b9e0766e58692ecdf8461857e043dab2b7c",
        "0x4e9cb4e481222e66f386530a57ec2ecd0451fda60ae2888f4450b3c55006073",
        "0x26bf87fbdb5e542c9d918347faf9f4a3985519f1f7ef4f76bdb121016b26de8",
        "0xd224688a01b4489679359f96d88f3dbe7905beb5127535dac5948fe725fb07",
        "0x46997f693222169d68fdd91d920e60bd72ebaf0a6767718242198b1c370c796",
        "0x1d170d2e6be529b17a8a7f31867fe76ab89ddd45321866bc3dcb8b92fc519a6",
        "0x1157e7065cd177bbc01cb84d1d7a6e03f974a9047139972f9c25e1bef27214a",
        "0x36fbfc9c34a54cb96e770b3dd8ac439673e9793e26ed5a220382567bf7bd77a",
        "0x531bf470f6c585696173467c023cb9c5f5fc44a38c5df75db366b8cdceea9cd",
        "0x3840da2268d57a48cd43834e90d8d0d693fb567cfd1dff5d6a2193717859fd3",
        "0x3410a3b1e98235e5d44e8bf65221d3cf9e4cf6f37fd18a296f3bf7c23069125",
        "0x458d0ce8934f2e68b5a240e88db9093d2afc49f1534c9fda46652a04f1138b5",
        "0x6e23116750b4494e3331dfb20b71ae5de2a2903cdf0cca6d2fd7667bf98c3d3",
        "0x575a6eee8491b3f94f5a0bdc8d69b11114f1ffa778d0ed456fe916497dff86f",
        "0x75ccf0e20b9bc788de40efc15e207f2ea100b1ad222aa88e708f3065f42e02b",
        "0x302f0e3dd5518420328c09910b7b0c4e7583276bef4f44cdce91f2bd642dd5b",
        "0x4a3ac75b61c5b5cb3064eda42c4e3d236c20530f5a847e0aa19d477974306fe",
        "0x580ffaae66cfd97404ee879c52902a30f4c8de26d660df820f78f95dbe48dd0",
        "0x3ef650c119753a23c13fec26d43cc5ec950043a36ec58c425d5f2b1c76f078d",
        "0x4483e16171bd32594824649dd4c271ef9d3ffeccb94a52ba9f9ae63e83afd8e",
        "0x596d276a006d16d98fedc2a0c8f5b44cdb68a9742feeb83d25a2c3a43ac3a91",
        "0x4d15b8fe7cc7be20ad80b9815fed60bf1c9e192cb481b9793fd964de29d2831",
        "0x7455767862cbcec3e835abf1533cdb6da3940549bd2923614fae97dd2e15628",
        "0x3242c7b9abc0e845e5789bffcdc074b38ec1aef8ec29ea06dc9b6a9bf72727a",
        "0x1193f34976cb21fdbeda00f8f6016e0ed5d235589a6cd12a33ac335e7ee3451",
        "0x53a712804fb598609746cb9ddc53adf71ce01e8f6646e87e20576613b3ab893",
        "0x1f485222649b25d762b6f580b3c7408dd51d4a41887cb3bd11ffa5e007b0192",
        "0x47ad45c0dbb6b27545cd95ddc7c4316e91a5ae7615d87d6646e728379c715a8",
        "0x180374fe491b2b7439dabe9ee9ad0f4b3e1fa5ab50d3b137ef6aa4cf3fd71b3",
        "0x3559b1b89d680c86863ab37b25535e877c6bf88beefcba745e483159077ddb5",
        "0x2724cd76d2abfcc4f293b7894621ddfb7adecdbf1e4ed32d3a4e43bead7c3cc",
        "0x1a0029238afcd7a0779e39a8158fda0942eb14b2734f9921428b96638ee8f3e",
        "0xcd7f8ec7105c1a67b2c1b9b7de0d1b53d820ea4b5a108a3bcf76f5580d71f9",
        "0x1c3ebbea383770c83c8eff8d24aff4ef6c75cbe60caf27794db0bc0ae069352",
        "0x45d5c5c1eaf7b3cc85e6ddf6b4f8308603b51bf01e2731ad9eddd0b6f97f173",
        "0x6cfb748aa36ddefbbafedd60df791e550298023f9cabde71ca9a7b3b12590c3",
        "0x5da9e3f1e07085fb498e9acaca82ae002f347845dfd292f6ace331d9efde939",
        "0x5ca30e4aab922d075b5381044d90dc3a9055e2f3d74334db41c878cbe50a3f9",
        "0x3b600e2f955c36e52a123047736afbe3668092e8da0d87e4499ca2d33e14723",
        "0x2fcd621cd66fa329296e7494ddffb13131b2edcc0d2e23e16e4583831deab9b",
        "0x1414487139a8c1ce17740db3931d82bd8cea317d27977ee0b844b0317c5c8e5",
        "0x67b9fef9446991016a899cd5fed72896fabf2e04c77107d267733ad58f524a1",
        "0x6e33d20e800e78463a51dd4550112d8e5fec1b010a77c1e00e4a087b381e611",
        "0x58dd95737a203c3981ab8a6dc97957f557c4cbbdcb752ae3168ed6590f32b82",
        "0x468f3c0fde415a9c5d2aa36f1a012cb9cfb34d308bba8b7187c7ab6e25d2eb9",
        "0x481e7e3d48be9918c1722bd598049aa7dd3336d6bb3d1d6f625bbd9dfe51aa0",
        "0x229b56d9c6e88a2f968227234dd3b0566253b952df03d7d09a93d1a76e927a9",
        "0x33443ebf07007f6dea85ff4d506cc3edc02cb316ddff09d77cb754a14e03b34",
        "0x46ba51410bc8e371d1a75ef0069301819bac23dd3e1041fb4fa97fcbcd561c8",
        "0x4bfebfc9f423838dfd4cb28b6d70a53646ea57002ce4afc91e371e6815b3c4d",
        "0x3d809e239ad34363d350a1af2a2b0e0ef9968147357985497d50de7da341a64",
        "0x3e43249e901c2d86ace654b820a4a9c86ab22e792a74f3593066cbeeeb028bc",
        "0x63b92167d491f8d3ca069ad069553961059cab349317f5c8b5f2f38f30c8f39",
        "0x2b14efdd8315aab3f3fc8e72b151131a3f5e68eebd6b710773a83b96a906967",
        "0xff9d7b63a123f19014da4be885187928ac982e6605de9b8a23cbd4ec59b929",
        "0x38cd76ca81385261a7b2687cca02395d325ac759ef35f07354e706c95c3675f",
        "0x6e3cc6ce52df290ebc16600c5b2358c195f7138e1e8a77e1c8717facd5943de",
        "0x6f5237cf2ff3d24c6c2a5172890e9734b34a15430d4327dfef99655e09c1715",
        "0x5eec4358e0dd031543e035da38030f9389d1f517b576450d1c7bba3cbce370f",
        "0x3760b1561e21245e82dca98edcd6f0d32dd98e9deb253e0ed56d542ad393ed5",
        "0x74a631cf2a833c70e1388c23a2c9d12e6ccc18169fb111c910a0b3119ae6aa9",
        "0x5794890aef3be791ae9619b98d45bde617134519b933cf9c7327cdec92991f7",
        "0x7600f8a366a525a42b38da4da5a6880d0683e27413f8d7f700fdb806856fba1",
        "0x1b3f2bd4523c83536ca69293a9d412acc3ab3b1e3ca90c0c1a7fa52799583ec",
        "0xf9f931aa1e228a15eac2f97c43c045d26604331d829afb40a8d1f68c00e17b",
        "0x50f96bad1eed06805a2eb595f2935672dde616e68fcb21530fe13c45019cb5",
        "0x50a55442b12ab08a66eba124f4eadb678d7e47be6bc3762aa41000e652af7c4",
        "0x7f1b6af56db7871e9eb4c4bc70694d0aacc31b9ac4a76f079fc4ed8d7cdf47a",
        "0x1a84ce8e1e09ff9a37ce8827f7a9eda4680343c23adbb13e669eb8874b74558",
        "0x7030ad79dae6c0fb2ff37c6940ed5d5c427ca6e40d66a4b92ba45aa418b7876",
        "0x58cee4467ffcb3a8c615e75eed63da75f9ce0070fd56c54038fe7287c162193",
        "0x33488d700e4f77d7f67f64ed336ef8cf669bc6bb71c0b0e798d4b9ac431f5f",
        "0x1819dddd855e88e064337c6e6157f38b86867304f27eec9df48b4dfe67fab6b",
        "0x4d783b6bb8704ad7160508cfee86f790552c368e20e9d0a0919266164de74be",
        "0x79c5ef984a8e33b3ccd3539ef8d73f4617363cc93fbe909d088e0943749c795",
        "0x72f53cc49512032ed82e1a53aa5ab61f94b540404327fc433f752ee75247594",
        "0x724a99ba8eeb2811abed127c86b74ed92e051812cd58e4ea4a15f709d29b850",
        "0x39ba80681bf41d02a2af546a30ca8a2c22386d2c3059006da659f8a5ef4c917",
    ];

    let interaction_authentications = interaction_authentications_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    proof
        .witness
        .traces_witness
        .interaction
        .vector
        .authentications = FunVec::from_vec(interaction_authentications);

    // Set up stark commitment with data from the log
    let table_commitment_original =
        TableCommitment::new(table_config_original, vector_commitment_original);
    let table_commitment_interaction =
        TableCommitment::new(table_config_interaction, vector_commitment_interaction);

    let interaction_elements = InteractionElements {
        memory_multi_column_perm_perm_interaction_elm: Felt::from_hex(
            "0x33dd9c083eee682a168398d8189bf14bc503febe8d298c7b254ff5d74868c39",
        )
        .unwrap(),
        memory_multi_column_perm_hash_interaction_elm0: Felt::from_hex(
            "0x6a7aed3206de9c6a434484cc1f499801fc2df433783aef27b67ee8010b9a6c5",
        )
        .unwrap(),
        range_check16_perm_interaction_elm: Felt::from_hex(
            "0x645541a20083f184215fcb20d8014bed0546f6ae292f9eb7ee3de254117c597",
        )
        .unwrap(),
        diluted_check_permutation_interaction_elm: Felt::from_hex(
            "0x11d978931f41d88e9e8930b42c84b6471782ca8c565464dd6f468c2b4804a22",
        )
        .unwrap(),
        diluted_check_interaction_z: Felt::from_hex(
            "0x1e22e58ff299d68ff2d5823796c232bd70a8e700d70379417849503417a478a",
        )
        .unwrap(),
        diluted_check_interaction_alpha: Felt::from_hex(
            "0x3b74ba4892c5814cd3d16f5f28df48e8a6dc70a3d67f18b1b4921510fe88278",
        )
        .unwrap(),
    };

    let trace_commitment = TraceCommitment::<InteractionElements>::new(
        table_commitment_original,
        interaction_elements,
        table_commitment_interaction,
    );

    stark_commitment.traces = trace_commitment;

    stack.proof = proof;
    stack.stark_commitment = stark_commitment;

    // Push the VectorDecommit task
    stack.push_task(TracesDecommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Executed {} steps", steps);
    println!("Final stack size: {}", stack.back_index - stack.front_index);
    assert!(steps > 0, "Should have executed at least one step");
    assert!(stack.is_empty_front(), "Front stack should be empty");
    assert!(stack.is_empty_back(), "Back stack should be empty");
    println!("Test completed successfully");
}
