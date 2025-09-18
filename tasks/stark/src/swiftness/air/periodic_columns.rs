use crate::swiftness::air::consts::FELT_0;
use felt::Felt;

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_ecdsa_x(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22aac295d2c9dd7e94269a4a72b2fb3c3af04a0cb42ed1f66cfd446fc505ee2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2bc4092c868bab2802fe0ba3cffdb1eed98b88a2a35d8c9b94a75f695bd3323",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf685b119593168b5dc2b7887e7f1720165a1bd180b86185590ba3393987935",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5febf85978de1a675512012a9a5d5c89590284d93ae486a94b7bd8df0032421",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e1b2bc38487c21db3fcea13aaf850884b9aafee1e3a9e045f204f24f4ed900",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x324182d53af0aa949e3b5ef1cda6d56bed021853be8bcef83bf87df8b308b5a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5d4c38bd21ee4c36da189b6114280570d274811852ed6788ba0570f2414a914",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x529414d56e9f6bf4ce8be38c8f79ffab78b185da61d606c411098f981f139a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x66d15398bbd83688bda1d5372e048536a27d011f0f54a6311971822f55f9c07",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x52e5e75be2c96802a958af156a9e171dc7d5cfa7f586d90ed45027e57c5fe92",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb0e39f10e5433b2341ecef312e79ed95d5c8fe5a2e571490dd789dad41a2b9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x703dcca99c0a4f2b2b7f1b653dbbf907dd1958c248de5dcb35be82031f7d170",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x749e86688f11d3d0ef67e4f55535c715a475ceec08547c81d11de8884436d8d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44a14e5af0c3454a97df201eb3e4c91b5925d06da6741c055504c10ea8a534d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6cd537aebc479350e63acbcf7b9da84f4b06c6c26a571d3a7dd416a94a956ca",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5e2909b1136e1d6608663e5cbabb616b28d2fd6f5dfb7cd03c4a7e719b7c53f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x51170abac6896de6a5b478741dd56f52b1d2a1feea59b1f26d060e09ed98b32",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1d0f94ce5d9d3beaa42ebed05a2f172aa2227e9a9fee0bf43a3fb068c1ac345",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x785dc572a88712cb4eddcc8a167bb1b62f9a79282f21ee92a0374af76169344",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x39d9d83e0ac884a5ee0f2d227f9eda71724a55002a41938458e45251e121308",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7fdc637318ea00385719f9ce50848d13cc955eef9f36a90b87e646dac85e3aa",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x72d82458ba49cd6c638f89d2e3a68e49944f486cdfb7d2848e51aa9f99292a4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4c484b2cc04747d8d812180ec716f779302231983fa17971b575274c0a9c378",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62773dee1773834dbb324c4c0d48dcdf9bbf0511547feb1b2ab0f7af7fa2dc2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5b0343972ee9e17afaf76adc54e6797d54e6e47a7ea1167654ce076e3c6c360",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3ca8d84242dd2bd2a5d6e644fa1dc9f5082ee6131b6f0db8fd7d4f87109098b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6f4ab1f3bccea47669a4c93da36db05bd6f5197945b5ab29191a703312ed3a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2d626ebcfae2d3618e350c190fc636495fbb04dd4a4e563680fb961a3d30d8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x42f0a74ce045e8194b7a5cac4e882b1f1a9face49c38fb3383cfd3d960806c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x34b7ebee71c5876183407c57610a0a8a33d3138ccd6ae416651cd505e5761d9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x150c633a21f3cfa157978e9561161f3953e180b9588347a0c819e4173afcfa8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4f8cecab5f743c7227a63fa7f320930ffa7cc52b0fff6c351d3e9d4c22f9f9a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x98ad9c2080ba0663fb302025e6224cff41d1d30c5c9101ad77a48a71d8ac",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x58a6d8229d82c192f190e55d28489f621cbcc64e4ef10c1ec5663c5384e60f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7850ac1ef437d1b99c026a910b2437c1b877242e605c8f31a456f10e2f78743",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x337092590652e19c23b48de3629ae0bd4157a5a72ecd3fcd17bb93f05814716",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x95fd265a2a87c42af5a20a199e6730ee3f0e3352a38a5e7e84ef46c621903d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38ada3df52cd03154d66b7da4a8a01835a461e61a76ac9576649d8c00013610",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc1bbae3cf2d414dc12119a0c746e3c10e148f8b522d574eff757d44d8b3a14",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7122e4b28d4ee35902b7f7b8ad5f525b6c70a2f2bb6b4ee4b9f0008845ffacf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x75275c33b919425b271966642fabd9ea7c917e70e96eda669040935b1d49db6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x70af32c484244d3435bb65b0ed076f48d06abb45b7765de9c6f26c1c8e9156d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47c78a993a13204796a2fca3b20c0f02c0601e7cc59f84570fa026c65796dc9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x55713c4cc9f91e9f158f70683238853d0bb7cbd8358ff72b01fb60808b5c1de",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x412fcd2551c0516392f685a62b54fb82b9a73bcffd42abecea4482b65aeea47",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6925415cd4dbae0ea5e9f41edcb503ff6f668da1cb13ec73eab6a99cd96752a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a2a902a0e43ab33c19459984fe116fb215796cb40c48e254de6126b55e9c3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x552e18bfefab6c3362cec587f0a7433a914f1359e5767b4fe883f1ad902dd13",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5643c5a69044bb8e86d10d3248ea3f50f8598732b0c517b256fe108294e09f3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x542f931640d9010e906b7e1e375cd0481740157eb51500ea1e10afe77f26265",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a2811098d68a747bebe9ca2eae06b604bb307e5f51a9bdac1636f380feabb5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x57d53073d66a528c88f24e40011321f74ce5bdbecd6ca319e5e770ae29b21da",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a4393bce3924d765902469c715fedeea69adca566859b4c8c412b7d7cb566d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6dfc1fb08b981f73911dc43811caa0ed99749c2f0903f87f389c9a0e2a88126",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x224fe4f546c8f999947a5864ed0dbcd64fcac6f774ebce11667c2bbb7d8603",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1f7d548c5a6f2bc70ff6f8ee47f38221ae25dcb4f9b068054ee66227494f87",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x735f4476c2b51acb4f0dd9dbc4306108e37543538b2cd3cd2327ae5377a2e5d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x66ec70c796374a71b6aec5520467ebed547f645d1670b990dfa680a1b415cd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x14ac38a4b82b4c65e4993726b58f32c74988997b8e8f7729fe9032cf187896d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x27092905558602aec9af09947b70bb974caa3dd7cb1cb991810e15d75194aa6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44e3645cc1b135410b2a52a5b92bcb454985033615453a51ac46377885c4309",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x352b933e5d853527d2a4317db613d07117fad8115948957515bc07d72e161f5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x268c1e10f6f9969291b1d2f54289371a2f40a14cc67b3736e04eb891c1824ed",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xaa81707e389769aeb31cc8b45276af0370dd702ac79461bae0a4078cefb5df",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x67dec5ad6ddb1761ec61d2820533f7a2bb56d66f2fb8ecff9cbe28218990061",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x106911de08ef437acabf58d178db7c81ff4d7de25f3ef5cd2582f44176d449e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x356591a80d5c2e14c3d8a180c030a9529a8580a4f3be00a5a9eea83d0d585f0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40a3ea8c4059a1b9138884234381d6d383e66dd48eac1bf05f5fcddd593c881",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x699e679a8f38a1ecb14c6695a2848c6abbab8a05003e43aa5cf4a9c6e6058f2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6b86f825e41b2c9934f71cc2cb08787d1bd4f2eefd2be9c44e37bf387b35940",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4eb2786b11bc602bbf773564eb9b057d7dc02daaf4359c015295d97b74e72bb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7b0ed28b968689517aaa216c0203e57f1cf56b22ff1213561499ae140d37fa2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x367ea925556a875faedf4d61bd2a95a31067bde6e682c50035bb3310cc54b03",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x781cf0ea1c0ba9cf908656aa2c5a9403d54c26c8ece401a2c13be8d3090f9c1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x20ffc2b4c6c318bee0cdfdca40b2c10f2c629d3b52472b17c1bfd909cb7b85a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xe5e89fde76daa211fadf1178785f0c25a94d47a468cda257a895b871a928c2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x164344bae5b9dca8f384612e7351fecde28adee3d245c98dc2f65509b181d8e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4063a6202df9488fe5384aaf7be7610b3e88a9c01486c1b88767ca36355340",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x546f65cf3367a004f10e9a4e47d71f6ec80086cb2be19d7b225825e01eb323",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x72c11bd84cd54152607e4c6e558a28e480a6487e374b865682c167484f8c29b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7a5d11f284ee7db72bed2338784d6467e05cae85f333e05c5610c018a57c2a7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x64c71feb673d2655bb1865f9c4bdfb16b1bcd0f278a911363056674dacb812f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62334e7d6094be4431aeebefc420f7e656459d6fc2cb10455123ede054f4cdf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5dd4b3dd252fa7eda7b46674369a2f8c5b00a891cf01ada0ea5aada8bfbf6d4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3373dcd7d0f0f8bb31ec396e1ec67e1f121121356dba549bce9fd4d3bbfbaad",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x42a6c571001e263b1ec8168805bf4d6cb65935cd0687c696ae3a6968fd28378",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16f35b8d34d425a85fe48e66632d3e4af27d5d65cb180cb99047fdc2b908ea6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x24327b5849aaae0d313870c10e8010a115b70a99cf6b92925f51d2f05686287",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x45ba7e524d75c65ab27b57a6e0b90458c9b0eb651935f84898a5d3cd0db9b8e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x767d8839373a2e97b7e3de1be6f4c18df648806920e92fcc4da9ab6bd8525ce",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x741b0f4e1bf8ed4d6318f5dc5ebba8529089f5ef4a84cd727564c60cc11a96f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68682814e1b4dd639cf396a9f60efe5ca035c6ccd75054b8911e8a15230efa7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68edfc809bfa6534b583624db421a2cb885d2ce888e6f95eae85ad9cb38249d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5d6575134d1b37e610f25e65bc8b0b1ad7fd0cdcaa56fe573142a09707640b5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x327bd35b3ec38fb121c039f777669426d3d60df3922e688a408a06d4e7ee3a1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x29a66c93ef1fa5ac4b6f96ed329810085b294a7ab8e16c61b1e225fd7406236",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xac01d3129d24fe9b9209df8bfeb2526bc27e9c27d78f69eac16ce151b13540",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a967c407600baaac716275b8fa16a08c22e928d895c762b2843d00496b3390",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d111629c799fb16f602183ae372aee382e0b401312951eefe77a1674575242",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xa8a00bb9874fbb44ee3411814dfb9d4d6048f5e3af6f7f09fff4e9f0263901",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3abd943152451107f59aa81194e7bbbe37c4a86a6b41e20a02f8145dd32fa87",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x580bd7107af3afc93d0cfd1f0bd39f78f06ebe3a900f5d79943c25e980e5653",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7a615360e826e937db0c91cc1c9196086a3fd608cb01d20186ba1ce856904ed",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21df6648e6f783b7361a20191b8d399a4373dcbcc83f6b4a9a40bf11956219c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2c82b2a99d198138ca2c4229a1929d044b113c1b0f693659712318ca7e7f804",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6dd74321080cc46d816a963c8a6f5dac42cb11e66c79831efba77433cce0d23",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e54c3a5a3beca7932090ff58784aa43261075950feaab0e2a840f3801b81b9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3360af40b57c0a951da3219025643a76516f85119dfbb05f61874eb3b56b130",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x164d44fb88efb41e301934bf2c61a20e41c9bcb3f8e784ac5857063b4fc3d5a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x46efbcd0bd7f06d59a430ddeb9f239d66a24ce1fa72f5dbcc2bab48b707b2dd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5ae517bdefe7b6785680842685de0b5cd972a22dae9ceb50a6ea3665feb06f0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x51f4698c121db3db4a5244334c5180cfba256dc80a59689e2c0f1f8d946e6c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x67d2681fae96c0b4bf22d10a73a1882c5bf4a5440f8d0458394d514ff7bd18b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3dab30754623b91aec7a165cc167e9003269ebab3e551781e4c8cfb73402de7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44be18892438118a0b3fc099da7489a89cffd4206678abfd37b1e649ad19178",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xdb0ad3bd8a33b8daf1d53ff8604bbe5259b6620e3b547d5c6f392dbc10ccd5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a36f20817da4dc0c2e8b62fa08ce15cd3cb50419acf5211d6948bd6b28c8ce",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5fa6f7f2a7a527880a5b58911dd7f3a491fc702f481cee30e67c4980092f851",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6f682eebabbcbfa3e7084b47b2a01acb693865749df222b4b8dee0ec41903cb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1fd7088411b30cb5762147b1d6749942485b36c68ea32f60ab83fdcbe987d83",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7172b43d0c88348e5453b0b26d54d4a7ad7e99e6b0c4b787341c8d89936197e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x34369f479f013d44dd5bb0d79d8a9effdb2ca36ce8b3d7e759bf707233c5bbe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3054d35b59baf5b0a2078c23322de031b383033837cd6b978b6c060120b7fb3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x23f0124cd1c3f3605fa1ec36dc4d6cb6e229f8ba8998b138a44595f96f3bf21",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2300892e3f3c180333d091901ba99ab9e23c7947309b9e88ad47025847ec3a0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4182bea2ea16dcacb0194876cd5fe8c79e1a55836aff8aa6074d235af5f7b29",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50f3e383aaf3533fc91b9633386542798abd69b79af893f47f6603d3cc35ea4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc37f91c81a7006d6681cb511dab2e4d83928ccb78d1dc72c4c556e4cd72db8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2693f31fd4bb5a1ef9cacdc4f2b33c3d6d965b76e7bf289020ab1b6c6660d70",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2bebc90c59dc0e37e28c7c7d8254520ce08894637bf1a089aed26012690d119",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17626d3869adf0fdd3fedd48e9fe1266bb33419bfe9046df43c6409b440980e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x734438bc30566591da45df9366f936415d29eaeaeab392488bcccb9acf0edcf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22a7b1c897f54da39a1db61b345b234969e36ef6ba0ea02f8d8b3e83b5c6242",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22eef827b9d0b57649233c5d527b4641decab31df78347a20da21c705df093b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x193185be6e02dc0a07c0dced4ed031bf0a406219cce325e76408123406c318b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb10494024548b14df121b738abc7babe56c12acc0490699443426a52f3a4f9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x375ce3766894524209e2043a150f10ad0bf4f726e3dc5453c3c757e56943a51",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1b1c82e5c561dc42f8c9c2a9f7db6bacd729b2646892a8ecfae9ead9a338aa6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4b540d0085be455b24f014bf51dc7d0eceb8c93bb644a5208fa02dc58c718ae",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17f2709d2719458a9bf72a2b04463f0a6529fd9368a47715c628ba4e006cea",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x20e468bb2828fb774d5ab538ff7f93ada201c2e392936e05cec29cd5a7a462d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x29cc816e6be353f6ad5e2c390f37ed3940b0dd67610a7eeb0bcded94bdcf920",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d3b0654fd74862a92aa716af33b5ad5ac20dc0460c724d95ca94fe6d8a9d7e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7ba5194da963f8224987db2720f16baa604ff62351e66a63c0c9dba00fbc7c4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x31d335bd885c9cdf2adc68ab45b8eecd2d3588cf85b93206896b2626eb1e369",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4efcba706a8b7868e32f363efac2696ad0625d046a3ef97917c710515016386",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x56017977a273ad0e91c7c26a702ae4508343e97968295b08447b3cc7f20522f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16416cc193a5ced6ff213fc18c86bd6f08d17c576f26b9ebd00d2653bbd6444",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4237c41364975eb79919303fc0a381b934befe871fdbd72c18f97627292923e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1b389d976c22a3bfb42424896c9b135a3794048724c729968f81e04ce414194",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62fc206aa283139f7451e54cdac873fe86b6e7e89214a3c0318fbcaf6016fa4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x88f6e5a835dfda9fa2e2ff248d9378352f4a89b6bf5935700da390baebadb7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4f9e975176d3aacd79c322d013c854c4b8829d1e469c9b242461f35e8dc6fed",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6e1143b147dd1bcc56dd43e6a3616c9a4016d6887cf0009ebf9f9796efc944a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x284c547c04ca83fdb01020cfc797eb362838317f09e5d25e1e4eef353ab7a7f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x60c327ef73c8468805ecace45a33ccc375fc91ffbf01b4b10a01ffd4b7aaefe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59cd87f8751437900e984a009c63fdf7461b177067760f30d4f648ab271660a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xdaf5a68420fa7ad811f6dc75c5b4e92173a5d89255dc75accb8cec80a9cd91",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1f573af6e3ad146eeaa582f540de6a8db237ff2f28423660de998a4275bf4d0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2830a6edb344b7fa86506557a0b2b0bd900429218fb35e7990951fe4fe869c6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x58f2e18613b3b25529935a623e7d5c8318ca9ff3fb180f16f7454ca9e348e35",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x42c8f0b5507417eb48ffeb1a7df8808633f193c27df8e2f44ee7bd62cb2c3bf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50d603bf9c2a456b828ae476092affde072ecd878877ec3f99ba8f574d263a2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x413fda31150aa8462deae8a6043fc5624599fb7f638c4d5c5f89472e1223c28",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x13fe84c8ecc2e3fd289560c0ada7a251fdd5fba24c076be4be465feec4262e6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2b2a0768e9a5f59e7f33ea449690794c8b409bacd1c808f7ee8065ed9d8648c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3030332e9cf430f72159914e59ab9af532bdfdafedc1be39691256c8084954e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16617a52bfe5d2fd0eedb0d6411f5fafeb14a4ac17da0cc828c914acb500ce9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x553f8ab49053432bab53835480b6f4c416eeffb3470fb6bcf122741cac3d71d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3939ef0e572dcc3b67f0cb819fffc521df26e50814281621fa6982b1465f786",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x520b18e79de342aa7095ffe56be6222b0d2e44fc3c676a5c994f24e427b45e2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3f3ae3871460ac578f5030d925e91c138f3290f8f3cb6d4b560b4b16fbacd64",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xbffb0e4f7ccfff0cee519edd1004eefbc47024f92c4409bbdf688c133ad285",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62d6874b6dcb1c4dc8ed797b9158da4359c6c49f27af4851a12908ecad2092e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x511c0ad7c0bfdcfcfaf925895a8ef5e8c5e0d147e29c9cdae45fbc998fce346",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x25199c11f7193e07191cd9b9108aa8b440ce1972dd1cbe5f0cc33b7783203a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3cf3b95ba351a72019ed1bcadab32116adcf079e72800a9d88f15244e7743e0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x19cf240d04f4859941f9b6af4a7088729aa10307cd08aa75f01cb22e872543d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cd528d070930aef19e0f928fc744e79ff57e227b6aa1bbfce15a79166aefd8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x475f8af086f7aa4ec3739f754f7dd291dc50decc7c7fb03de8aee3cf06824f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x28f86fe2d71f9410e14c17195ae19c2c5e623c525c979f4f74dec3ef8848eb5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x508243aa19e23cdb8ca0154055c05130462908c6a2691ae522e37ab9d6168f2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x37cf9640e321e7bccf1926d5fea92918d6888c5805e27193722995233a4adc5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x66336e2e2eeb939818f861fa4aa9b2576936470f511786f8fa3417850a6c2d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x19a0ff21908842e412addb744b0ca384a54bdde819f6337c4c672f682fea9cb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44147236daf669f8a94b7ea353c3dd7e64312ece01ccc1d4dad67916591d50b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1d6cb5a655919a581078aa2f8a21d300425026ccd7d047302443d78dbc67abd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d8d9b92b38a45147bc9c87c071672edd93cbf5bdc8d85e608f26f1d82d172b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4acd125e74056ca611a1b07369166eb5c02af7a4cbf387b2bd584a362fa9e60",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3b4fdc8d965de1761e445ee88cb406f707f9d0b1ea3c069d12084c0ccba9b44",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1f27c20f47daaf01d4627d5e9bee0e9bd2aa5b75807064cd60ed87e307f677a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4758304a75f149e24563c2b22459151389b86d36108f5dfe11ea1fc7a64fd7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1c1216fe648d287c2645dfc5152e171f25483df5ef112b745c2e59b5d9ee07c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x24adf288d61c113e28d9a298d2642eb67586019adcb952abf274ebe1d30e24a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3e09706cb43c83143c9dc46f97e0e1ab4327de19ced69badaa8b2c80f68fb9b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x589a2e11637d0c90fe91bb9f4d55a80cd1a2df7f3431e8b8bdce8fe7d35126c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5f506aaae7ce6d94712c9e0ab02bd2a4ae09600608d54a8ca381b8e96222cf7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3f64b3a307276c6a7169c54297bb12aaeebadec98df6ba1184492a82effe353",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc3e0400cbde1da659381240d9c84b977eef3cd70e3e4a1a8763a05e682eb3b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1fdb038204ac50e87e3e7239d8c1c0572893ba98e031c982e545e6de64cb8e0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xe0b21e37008355c35f7aee295a8b2b72465866b2bd68e72d36f032c34b38a0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2cc90219912af16cf9a39f57f8b8c514f797dd5d49dfed5eabdc278e31106a2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xab2147a23a826d5f7c6fea5bf889eaafb5531721f31ee0a9f02fd58f09f65c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xa5d4606609371577b0d17fadcd85ce659885b00245a67b038f902176d99a7c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xfc76b77f717a5b3ecafafadf29e7f886c8ae67a3a2bb30467c440472349953",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40fb948f8a4a10d2b2e928a5d77b481f8d3068b47fa388a3ee65609aade1a41",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x13d322a0ecbe1e785921a7aa6f4d1135e0798e72f4c055226205314b8348144",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x20096a7aa30c6c42f1d5f1ed88de275d1d1610f2548711a75fbbd72d373a50e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x18f9cfeaf2c33e21d7c6fd9e15a3601a2fb3905588868167566e8c1f1dd30fa",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3e3aa48bb5db9e2b0dc6d294009ecd5d4ff6255dfcdde3f5b4e545032ea9b68",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x425b03b0356b92e66ca816869a76110d68862a0d8ad76f950fdb1d5c03279d1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7cdb723061223f33289237c7476e737ef0bbc5e2c1ed9a70566511fc2036ba5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x18b8b8d0f393950c9a2e674052150a328d214618049c7e2f58cbad76adbfbd5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf19faf3accc43b56369dccdec35dc7b49c5b8f8976764886bd16dd2e155f92",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xe08853aabc9eb934b4470bb4ae1dbbe90c61d2093516df998ca7adc98afe10",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7736291268c775a82caea06004d53edb829be2566fc7c4053b1d850a8116cac",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1bc1186238f0d39e1c56185a8d2bf00c90c9c89647917d60a5b762932856524",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6cf772fa8050ad8eb87bc8f0c8fc511622b416fdb084cbc93b79501c96b0bda",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7417da24519b4c55ec0d698ecaceeb49711aa1e7f7d907102351e73388a0fa5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x66e1e25d1bcea87acd136f2c33498e3223fbf78bc6cc816ad6aaf68e961da0d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x60db5bf6f060d82c169a1c4ed6c548d5e8cdb6cfd2e3257c155bf11f48ca609",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2d5447623584d3a19e9993814622d6369248bc61813f067c4825c9b0a81551f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xac068a1aae938e26e125b35c88a87130044bf3637bf1acd797103e7388b33a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3ab2d353537697d4de9c5c4c0bc31e5e776cb93181029144f6c6d4b5ea4317b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6ca2dd473297a2852e68ea2b83faf8f71e5cb471adcc74a858132c6a823f0c0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ec5264a5287f1c6de79b3df3adbfa157e8430e594078c3fba7002a077db447",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50ee695deb5a4e63c5dd6de35621d1c0c5a496bf41fecbaa929b2b3e23f174a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x36f446f7e5a51114cbdd3b460431bacb5a42cd61f4690cf5e9d9f13e488318d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x195f98a85cfe403a7d229a6eb4533a1fea641c331db75a5807711fdf1e27dac",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x63d4964faab567e795024a17032ec564ff221a421bd2e42632d3770c73dbba1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68d729620eca6b4d904198a0e6d241953b9b8c874a10b5ede5596146d560979",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1137975bab819ce0cbc73714305030fcd4a185f71d46c169908460390d56d18",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf639bcd7777c1ffd41a693ac9f5a051bd124b7edce3d568f14304c9fd90a67",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2322f8d96071356feee538e0c53d857b1924134b94377af20ed5d0e8b3925b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22cf65c6bbbf76765555748cc1ae91c83ea93ca2c8b34a59332567b5b3b0cd2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xd62eb553de83e5d51f78ddd9480d65870dc426f61153e732eb6cd62cee09cd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6afb39d46d5a846e9d58a6ae27e6cdd83bee29c72754cd4cd3d3cae423f5c9d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x364889e46da58b66c827835a0c2807338eeb4431f2099f490d13bbad0777a01",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xaea6f7f915e4aec612029a9d02316baa3f6297ea4cfd38897f4c9859ec485e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5c180e2fbb2b51e053941d0e1611424fe60ced6d439115dd98530c8d79cca4a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ab93f16e576b6a54598582eff5e2cfc33baeeb607826579680636b05046d16",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4de47e973af27fde9ad29f812de8a04855110118eb73fcdb46865390486a287",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50be25e516e30f96d8b420a7c494506d2cd21d64f4d5ecb67d58c2ae99bf5e0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2aa45ec320ea12beb804e35af3684dc981324dc9bd044592d1c408c052a4322",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x26701dfe3cc76754a4ab893fef59886a43013ea6ba648efd82fd03941fa2910",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1773ba95dbeaab6e5e9fc79ac153d46be1e57828e92287d698a3f4f87ef4984",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e92d5f575fcaac9adedb4e0c3549dc18f61bc40e3752e3506f3761c32c6e3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x580f9d95c2bd746c9210a87b0f9ed275afee1dde7a41d9ad5e69861ec0e43f6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3e2dbef5f162784e13b5ff4c33bcbc444ad1546922b293d6783b5de5c5aba78",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x596f89b6ca79194eb6a87c17692aa491f5b014da3cc7e5f05caf4fc1779c2dc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7e84842d5fff1666e01505f62661bcc822dd3fa530ebd1e4089230a4045a04f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5626d2ae9581d1d335bfc3863a4eaf3568ec8e70fcdae93f50a15b0cf601b6b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68371fc7cb3e0670a73eb3a7e773ddb63f231c26bf25bb1fc1fe6e93a7e3bd0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d215dd42f87632a9cce2cb95081dc731e36796c3d2847dc96a3554231c6aef",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x421fac0e48da8e6355c07f6a64bcea96384848e8ea9a7113ab45f15b1dd15aa",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x679061e5f453c8bb1855dce8f7d61f2cb64b15d2c4e70b969ec4ead3fc6a226",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_ecdsa_y(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x23a2994e807cd40717d68f37e1d765f4354a81b12374c82f481f09f9faff31a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x49d16d6e3720b63f7d1e74ed7fd8ea759132735c094c112c0e9dd8cc4653820",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6c930134c99ac7200d41939eb29fb4f4e380b3f2a11437dd01d12fd9ebe8909",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x347dfb13aea22cacbef33972ad3017a5a9bab04c296295d5d372bad5e076a80",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62e62fafc55013ee6450e33e81f6ba8524e37558ea7df7c06785f3784a3d9a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x23b940cd5c4f2e13c6df782f88cce6294315a1b406fda6137ed4a330bd80e37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf524ffcb160c3dfcc72d40b12754e2dc26433a37b8207934f489a203628137",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x603e3a8698c5c3a0b0b40a79ba0fdff25e5971f0ef0d3242ead1d1a413e443b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xa401d8071183f0c7b4801d57de9ba6cda7bd67d7941b4507eab5a851a51b09",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x37d720cf4c846de254d76df8b6f92e93b839ee34bf528d059c3112d87080a38",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5057b804cff6566354ca744df3686abec58eda846cafdc361a7757f58bd336e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76b4883fd523dff46e4e330a3dd140c3eded71524a67a56a75bd51d01d6b6ca",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1058ff85f121d7902521abfa5f3f5c953fee83e0f58e069545f2fc0f4eda1ba",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4eac8ffa98cdea2259f5c8ad87a797b29c9dccc28996aed0b545c075c17ebe1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6e7240c4a94fa3e10de72070fd2bf611af5429b7e83d53cfe1a758dee7d2a79",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59fef071cf1eeff5303f28f4fe10b16471a2230766915d70b525d62871f6bc6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x480d06bb4222e222e39ab600b8aadf591db4c70bae30fe756b61564eec6c7e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7d2292c8660492e8a1ce3db5c80b743d60cdaac7f438b6feab02f8e2aade260",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5a593d928542a100c16f3dc5344734c9ef474609bd7099257675cef0392fab8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x761717d47600662a250116e2403b5115f4071de6e26e8dc231840eeb4484ec3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4b74b468c4ef808ddcc6e582393940111941abece8a285da201171dc50525c7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x14ef999212f88ca277747cc57dca607a1e7049232becedf47e98aca47c1d3fe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47b2a5ef58d331c30cfcd098ee011aaeae87781fd8ce2d7427c6b859229c523",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59bd7fe1c9553495b493f875799d79fc86d0c26e794cce09c659c397c5c4778",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3ede75d46d49ceb580d53f8f0553a2e370138eb76ac5e734b39a55b958c847d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6e0bed1b41ee1cf8667c2924ebd460772a0cd97d68eaea63c6fa77bf73f9a9e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x739edb8cdd16692deaba7fb1bb03f55dd417891bacb39c7927969551f29cb37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x247573f2f3fbd5386eac2d26851f9512cd57ad19773b8ca119d20852b9b6538",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2f6efb89f27d2c0a86ec1e6f231b225caf2af9be01aca173a15fa02b11fdf24",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x75a0f99a4dec1988f19db3f8b29eeef87836eb0c3d8493913b7502cfedcef28",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x278a7c68986adbe634d44c882a1242147e276fee7962d4c69ca4c8747b3e497",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x675532b80f5aaa605219de7fe8650e24fee1c3b0d36cdf4fb605f6215afacee",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5599e790bd325b322395d63d96cd0bd1494d4648e3d1991d54c23d24a714342",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x43545892bb5a364c0b9acd28e36371bede7fd05e59a9dcd875c44ff68275b2b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38db61aa2a2b03053f5c51b155bc757b0634ce89baace113391369682fc1f74",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x71b2b6b03e8cc0365ac26c4dbf71e8d426167d79f8bd1af44738890c563062a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a663fc27ec3ad56da89d407089bcec0971cebcb3edf0c393112501919643d7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1030d58878296e14b1c5bcafe7e817ebe4aa1039aa96b9d0dd7fc915b23f42a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xcb3335374cc2a2350fe53d2389f04952c4d634f489031742dfccca17be2e09",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6ae3ee97ea5dcfbb7c36cffd89665baf114fae391c0367be688db09861a8ca1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7b16c33c4a8ffcecbd83f382469e1d00a340ceab5e7d9c0bd4fd010b83f4310",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x10f236430f20aafda49d1c3e3759c510fdf0c0c19f89df6d5d71deac88b547b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x104b04e96151f5103118c4eb556cd79899148fd6656e73cb62f41b41d65e4d8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x253bf2869135f4bda4029cae2819b2f468ae88530f3ea771090b2727814c494",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x65d50aa3c1d84a3deee14057eec98656a1296cdcbe32250bfdaa50ffac4c5dc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76323f8567119897f10d58e1552c98f5a62f03a16d3737e20fc2b0a31a3a843",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xdaee1c7b34ecb34717b7313dc4a299dd1a161447e2e0249426a6fc33a72289",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2bfd1294f111a5a90842d19cffb97481aefbc09ab6c47d7dcf91ba228019c07",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4f63db02e10fbe428a5dda8d9093feef46cc19568a3c8ad2fce7e7519004095",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x761a240cd8aa2f135daf0760bfc2c9d5e896e93a45426571cdad9118722e2b0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x30a2e8ac9e6605fd722dffb4caca8c06dd4a8968a7bf41a5371cb1a07d11c00",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5ba89e0eb3830039d0f8a9ca00acef15db22374c965b01abc49dee46270a7d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7e8659c39d7a102a198f0e7c3814060926ec0410330dd1a13dfadeab4e74593",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5a00feeb391114d7b976654ab16ddf8360f05671b34d4a97da278c0aef34d76",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x33ff2d848bf237f536524da818598ae0f2516ebee526b77957448973eefacd3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e0a5dd802deed7cb8d06527beb15dad32547bae77141c32473f4c8148912e3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x776459dfedbbdfcef7a31e0f60c6480fc0676b280fdb6290859fe586d6e6106",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59d0d8ca9ecda81081dfcae7580ab3c08a72195438c1556000c0c1dbdc08174",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6eb66d366da57e4ae717307dfc3351579fe857c51aa82b95044473c9ed14377",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xa758a70ba6a0cbcbc65abfeca51359904f790752c3df55d42707253d8dea70",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2046e1b4fd4c108e8f832f5bcc4dd46abf0d19ef0237beaec29d6c12fb9832e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x472d99d1a6e1a6aef339eab1af3d53af7a8326e4d0a6bac73c3a159031c3686",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1b0fa36439192f135c239918bf47ad14b55ced699f4582d929a60dd227b34ff",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x728dd423dbf134972cbc7c934407424743843dd438e0f229afbcca6ce34d07d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e42531395d8b35bf28ccc6fab19ea1f63c635e5a3683ac9147306c1640e887",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ea9bd78c80641dbf20eddd35786028691180ddcf8df7c87552dee1525368ba",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68a8c6f86a8c1ebaeb6aa72acef7fb5357b40700af043ce66d3dccee116510a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x25c5f348c260177cd57b483694290574a936a4d585ea7cf55d114a8005b17d0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x339b405bffb6dbb25bc0432e9c726b7f94e18cf1332ec7adfeb613345e935ab",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x23590dabe53e4ef12cba4a89b4741fcfaa232b7713d89df162031c8a627011e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x534a4f3cf71c93023e473f12e407558b6c24b712204fd59ddc18c7bcddd571e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2e1b2a3c32aebc0be30addd8929c01714783aaf01be8a1d35e830646e8a54f0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x605a244f646a825602891bf9ddffef80525010517b32625759b0bf5a7f2c386",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4f81a946bb92416d212e4d54f2be5fa8043be6fa482b417d772bfa90be4e273",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x655038ca08eba87484bc562e7fd50ce0584363278f9d716e31c650ee6989a2b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44938959c2e944eb6e5c52fc4ee40b34df37905fa348fa109f6875c1aa18000",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x30b11c32e8aab0c5908651a8d445395de52d5ce6a1efe75f2ad5e2c8c854a30",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4a92733a733f225226a3d7f69297e7ff378b62c8a369e1bbf0accfd7fb0977e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1345876a6ab567477c15bf37cc95b4ec39ac287887b4407593203d76f853334",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x580550e76557c8ff3368e6578a0e3bed0bac53b88fefdde88f00d7089bc175d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7d7faca17be1da74cf132dda889a05fce6e710af72897a941625ea07caa8b01",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62be425458d26cfedf8ec23961cdfd9f4abeb21f1debbe87bd51469013358fe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xd77a8e8eed7ce4931a6d2a4774c21864e2c9f468d080af9aba6756433a1a8d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3e850e31c0345726c1ace38537dd88a50c85d6819ae98add1bbd62b618f7a1c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x375a5d9b11c83d06a04dc9f1908b8183adc6f04e5b2ceeaa23d3b68c973ee77",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76640613af9ed1a125624e0c38252bee457ce87badb24fc4f961e55883d9077",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5428ff423f2bbabcb5f54aafa03d99a320b4b255115351f50b229eae5522178",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6dcfc3a99563a5ba4368ac4f11f43e830c5b620a7273330e841bedec0bfb5a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2652523cbbec2f84fae1a17397dac1965127650479e1d5ccfc6bfbfcbb67996",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xa737d6916aa6a869252d8ff294a55706e95e0844e6b047755704e37d978e09",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2833391a62030808228d14437d6f91b31c0038c14988a23742b45e16f9b84b5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x284f7815a7eabc1dcf56da511f7d739f1a199f8ffaf3474f645d2fc93327dc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e141c5429a369996563573bf61d7f713cb7d25baadff636ba2756c65a910ee",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x60bdb98c079bd5cef216803b056afce03f6ea41934275c965d6e196240fb953",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7f2abefac9e7f8109b0a2d25d0bd297059e45dd66798ac8b299f0a3e442dd2c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x41776c662b44a36c7075097c14b6010cb321591a4eca2866d58252eaf9471ac",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x573b13b32161c11c9b16eff7cf93fa770a3ef667547a27503e39092aeabf73e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x327319fcc0d34a0d64f5acab00244b43674a60bef754844fb2920c87c90cff0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x755f0e4c374e2fa4aa7eda10041e2139a4a7793eea44f415c73ad4fcba1758",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7b9cd3b277f00a75a17961d2d8e46e6a1838c8500c569cdcad08bd4e0cbae84",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21f5ea8660d290f28b9300e02ed84e110d7338a74503b369ad144a11cf79f63",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5e4b0ecc6a6c15ed16c1c04e96538880785ff9b5bff350f37e83b6fed446f14",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3d8506e792fa9ac86ac9739d3d5bf63cfc13c456a99c8581adf590c8d9b72eb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e3b6498f0daba2fd99c2ac65461c3fa519cb738b53cd6f002e97199fa4161c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x70930735d913d54915fba20c97f07cba8f33eb8f4f81fd869699a10e83264cd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16a36769ee50227c564bebce3d9cd7c4ca55702a7c7ccf403075f68f05a0c2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3aa748723229eb8b33354e0901f50ad052b6c1006916790c979133c4442be90",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5db8c52b6adb520496f9edd7105c92df67e8605ff4e0cc59992c3eb651ac7a4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4b2222d0aee638c7e5efd8ada791638ac155a01b78f3b532283574653998bb2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x26a4b2a61f40c1ad77737b99cb27d2f3118622be64f0120907e2589d2f25ebf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5820792f23a13d58ddef0607950d422598bb1f21888dace88929fbe7d4828c4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3678de28b6896959edf5c9dc0caec59b02dfbbf54811f87939b32d0523f58bb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3cd13f84bb7ae6eeccc1012837d2f3e017f069e66cf047172bc70371f5aed38",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7af8995e2ceed8841e34d44365c7ca14f5980a6a5c67b9813fa7bfd74a9c1b1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6d7af6524127a117184a0c12a6ff30d28b14933a4e96bb3b738d2a36db72e84",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x73200d12e733294b5cbb8ffe7fb3977088135d0b0e335135f9076d04a653c58",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x229d7fc2a1bcfbe00d5773f8dadd70a2641d8578fa73e66263b3512d3e40491",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x29889daac66c404d6491ec3a435d810a2877d885df1a3a193697b79b4af39c4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x171f0638dedf0b69655fa9930bcbc91b257e299a6717bd8ea23ef550c8faff5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xded0f75cd0a6a5401a954d26880eaf12050ce6458d3254c9dd6354bf66278",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7fc7d854c9d0b3bfbf826c384b3521af0f29f975613e8ea6dc14f37d8beb54c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6d1c3edcf1de16a4e0ad7d8aa099a31fa2cfbf81f6d1a5798bd1ef93ff906af",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3444c0f008988c8f600270b365ff926f016e49a54ab35bac4f3b3a42a5879b1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a48058c77edcd75dd4323d9bb9eccb854009b1184fd716a8202f8627bb5447",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x56cbe248ebbc2f57ca8b943b219ba245791592f687815293a4499ef598fa9b7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x658160ea7b654d786dc624b258c691f594e080610c2d41d6ebea0d8e3396849",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6fcc261ded0ba97b4defc7c9bcd32b5dac89e4c08cb55cef98c6b50f5a3a289",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7b74edd15d97b289da4040272cfc573f69a8c9a8b36d05e3e50b598508b7f9d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x19637a12aa8b822c4a3f3551ef6c538043371a12a962de1dc25d67e0a5ee561",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4c05a7abaaf08f21d93b2257d4f4a3ab2b44f4ac44ce0444418c864ca18470b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x657060a10db73c4a9b6aa6288dd6164e0b50a4e6efbc2ee599a0cf4fda33b81",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x688c61ee887c1497ffcef82163f1a81bf7778f2c314ffbd325627bf0b25dc5a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x54ab13ae1984dcc7d38c867a47f4a8cf786079ee07cc94ab5ec1962c21f638b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xccee381472bb7dcae008316038c87a44fd9295f730e389eff14e86442c41b8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x610bf9b7ea4557d72411ec90fb677f9a2ccb84c76f003954da4e7f439c9a84c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x51d6322f7d582892421e977464b49c4e6e64af2438da9a7f21a061c77712dc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7d92a463e2aec09eb86f4647dc9ec241904135b5eb53ea272e809e58c0a271e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17ab90241b58bd3bd90b8a5c7f30aa9e5afeedbe1c31f21ca86c46c497b573c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x199d80ad30b4b330fc8a063d1e87307993e1d98822a1729488ba8a586045691",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x601a139ed75acbecf557cd6513171385a119087585111c30bbc1b65cd6d30d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x77b10e23b08892ab18cc6b14dfda6f4be5c2fec94a12e3622622376edd0d6a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a17a5c34f9f598deb5bec334fde606eaa5601df908eb5825ecf70f9cecec3f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7e176a66dcfd58e240c4546cd760b7e5ad02e4f0265c6a2f38d710bbdf99d55",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x27e76848780aba5b12061bffefff1710995586618a2f32792d62771d31ed519",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x43f51dfe0f1cf290c9a522e2a5e734f79d220be80348438c676295c3d429e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf1f93c3d919653f02fba06fcba1ab89497fff53eceff6a7d129887d5a9e3b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x79fd6f5f9b042ece36af6b10eae2eef9de9c9dd18752eb66868a0c301015dd9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1958435eb08883bd69b6a56a8f3103c22f8ae206a3d4deaf4a04118b4dd6a6c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x329230075f64ffbf631eb0c40b97d71b4dc38a08bd18b638f57e5644680068c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x219557f1604be8622e697e986c03d2a49e40cce558a264bf4f1ebe06493eceb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7238f034b8c57c8b59b0f744ababf9da8229152a051d4f3b3c4995233ac1111",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x201019c76d9aa29a00e6b18a4eeac7b1322b44285c57cf4c0b68a87120b1d31",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x69d95f3c7892a1cf65b45c324be2294c4c5459e05e0feaa0b8bb98cd8bc958f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x78aafbe80fa5ee9a846e991bf35b81567a6dcbb1b190e7ee47e53fc66422e84",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x15ba3c5a882d4dfe3e23db18368ade6b2d10ef52e34f12ce0d62e7183c10f7e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a4bdaf2bff969eff8cef73e762b6346492b8d0f17b2e42956c526f625241ea",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4adaabee9ab3c6ee7fc67a2ddc09c5185755dcc76cc3b814a6b71aa7ae542ea",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2f47cde744314dc0502faffb0387a2e765e4354b0516ee9ab0b97a1b6c33ec2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x90b2b18b3fc2919a55b71ad6d6fa67dda752bd02c985b59e6554f557fe4a2e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6eba866251e1dca38a21c8b3fad0aa3c22a45dd89884c4c68bd7ef67de64f52",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb8dd33ef8726747fb368aedf80c2f4a720bc1b5220f4a3f0e56e2fafb7e243",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1fac2f441d05a3b483675200cb1ebc6f4ca6ecc5ae60118fe8745f95217bf8b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6d28879c6f75c4ede18e1b94ffff964d08c79038fd9ba2e7873cbefb5f323db",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x28b38e0334fc06af4c94ec4f9434923d4149cc51817526597423fd4692c59ad",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x84add7269e2e41ea57aaed996f4c012ba7003ea2b994670cc0d554b7a8bd2a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x64d672ca00300ddd5e9c9d2db433d7623bb54c8eb2db51b235a07616f1517e5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7f71cb5526600d15d3413ec971ee3b133718224b3cbdc68171a53d7c8684382",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38e5702bb10256e1856a5bfb03a06b231b89a36e2f84af80bcd2d027153d847",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a8d4b2044b8e03b325c353f3f92283013920b92f479064b6e93159d2ed3ba0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68384718bd3bb23f32999f1edcb2dbddd8136259e676c4492d0cafe80ffd856",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x31a77aa370bb597dbdd0422612a7dd947aae09a5b0b17d1996f13a85103d150",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40a9cea0394d15ef057c2923d4185f290fe2347e00529d92f927ef506e3b5e7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a427d70a34b6b5237894f065ef5d60a9872ba444d47d98648b080b8ddb2a68",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xe505592d606917f898c54a7afc45b328be3cd48121aee2e8f05185a3e23e5f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x45b4e74f19b293bc3d3d172a101e344558fcf4ccfe5eecefe31f45a45614df7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68486394265c9dc8fae42c8fd39605d3179c981cb44cbe33740a3deb907bc59",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2868a08eae382c069047152ee964ac5ebd242b44267e97e578802440ef764f5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3159144c85f2c515eb806e5aedd908553057b69c556d226adc6e4511a35423c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4387edee6899d4a85883d2f8524978a4634ff82779f150b7b0c861bb315ed3f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68c5830832f6270a189b074d7675fcbc1d1c5cc06ce9c478bf8f4d5ac1bf40",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x399c00b8ebb398248bb1f52528d5241e7366b73c2d89f57a11dc82c530cc57c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3238aeb8f6bea8bcaaa1bdd5b4f917ccfad8eab031785ccdc648b47d7ea4be8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x357bf5d87c973292381fa4320114551a837a1d6cb6e2bb0eeba534fb2e01742",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x360274f27df6eeec0b7b65fbb227a8214ac3e55cb37b1970e18489ef5b574e1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cb6e2fba23730f5bf9d8e726569b6e8bf6b5ffe8520339503c5469cc3713a2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3924324af1994280f87f289fdae0b9a2d8cb9914ec37d319c18daf029211815",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3c4ad04a5a057e4411487858dbe16af8e3fc065ef7400749ffdc248bdb25bc5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50c92b3e6848a21001be2a268615e1e26cb4918ecb09640efaaf1d8b71568fb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47d21828025d0cbab84084965a49dd14c7833aac562b55de808a94777df2ea3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6207c6a2fd70c19a10430566c9efaad95eab8cbddf308f0057c81f3155a25a0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2d4acebd804035257147ad8d8419a5f5762b4b543c4846ef9acf41856e672ee",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x78f49c214872b5cce18ead0207a165fb741ea818a69cfe9647737323f70f4f5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22aa8c5c5ff26f9a0edc768ae32ff4f71a71205b4e83cfa0cc687a1e02566ba",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5dd2e0680c7eff25211f31d3c30a9f454500d6eb09d46d87a75a42b190203cb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5ac4bcdb9c14634ab83c13a30822ddbabc54248cf1177b11cc2aed24d2d32f5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x77dee5f03389585fad0d1f2a8accfa4cb985344891b8befaee42f3462cb48a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x737dba18eb055a12d842bfae32fd146dcd2d7bb932a2591aa864458d6d652",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x23bf372b0b59abf250463697ef4b2096eb1c9674613918b4d0c79aa10d9fd59",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x73724274fdd351c378e597da1615dc51058e14994464cb7b318766199ac2a35",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2e14e83be58cde3ed5f3fec8ba6462493a4a2f0f7d6c846006220eccd49ef25",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4846d310812d81ffda3731e8289005e2f0e05411e76b1c84332c3ee9e831afb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x160abeb38bc4f22af5fe618c19c77c39903007900722bdbdeaee059f31544c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x264a535ae10091157ed59b04955dff66897af74cae20456bb830336b803ae47",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x316ce6b23e720b8302e2d4bd968c0f140f69930e46a54784a7cee7e0b8a0c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x775d95a0beb287c98663a3f9a9c577ffc67c1fe6fbe2db5b08829a2c3eac922",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2353c4a418bdc1e461be162140cc69c26eb9d99f08924991f85058f87f6df41",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x181ef9cde124459dc0e2aaf93512abd49a10328fb93dfc4d49ab671db64bbc4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7ff76956e0cd2b490b47a0a0497df5f874cf47f54c45f08101256429b48460",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x318e5a52d685eaa06e0f39159a344b3d97b52688b671d133954aeff0bc17707",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7616cfc6834643d4b95ed1cfec036f816a7c3d3b9800f301f98ddf341712ebf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59869515fb57ea7733567e5d849bcaa00c00e0f86f4ebbd2c7a6f4c0c77692b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb806f4e19770279fab5427b8eaf5bc68bf984d6ccea1e878a7aaf32c9975d9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4fb0c93fe30da048576fe5e839483636218dfdda3d05f1d68847a4c0167597f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x685af2d7bbf30cd0c5c3d41c430a8657eeafeeb4596165faaa73d802087ad80",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6f617dce150ea148cb8c7488fe4caa920b2000bc8122cce1891e4b76cddc9d4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47f02fc512b153462379f4f793c7cab9e659bfdb07d3439d29039f566b7236d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4ce0a14a5a9c30a38062eb8870eeb4ff3562db743c0f3eede2e3d3862a2eb7c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7b077d27c7007656025224fa4e528b4c4261f43c3da1e42bd1349403af55cbb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50f5f6adbf0b9abc6e231b855018f4ec806a4f199cc511bed5c423ebef298e4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5fcfeb78685abb1ce610e516ab7e2aa210fd90844c8d1c89cd798f3d71bbcb3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4255a568f4597862e1dfe0c391b97059d179d7eb4d868f61364835e5028f9dd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x206d7f23d0fe1b1c0967486ebb792d7fdf5b1691d2c2f9306e211d3b849526b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc61c74cc988663ee09f4c725d5b1f04549bd342d3550ce17427ac75592b637",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x175a904681c7a91856bf7fcf8410d2c19eb8705267914489664a1ea2af5b8fe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1bd842a4ec97e1489ceb542bd3161e5a00ce431547bfadfbced954d993b0a11",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x14899e0f97aac917d46ce5e9ddf11194fb846d2c52726af4085f27c570a98a9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x842955243a56778a332ba9be0b22b2af62efaa50068d3078675fb76c225e76",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6dff267c3bbce68474294da908df4f5cf2a4160c638f7cb45c098057e968f44",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x72c0dd24a576b47a84cdd1a20227773b5621f85b781c288625e3368e1cf738a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x728771890334d0c9b0f400543bdc13ea6890497bc87c509a04f8014916c13a5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x30632b3865a272a1a00270430744ee90b40ff16e1fc44515876ce8e36215ca0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76d656560dac569683063278ea2dee47d935501c2195ff53b741efe81509892",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1dbdc2ea2e555309578eeb2352fbc47c8fd5ed77cc09903b577700f9a4d1be1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x119bcf6402ad9953851bac8e318d50af699b0cc75e2597aff0a2cc521975aa4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7c3234094dff9a45064a5b9abd0667c04dd76c62722984f7f8475e7cc344c06",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1495d40cf3f13c5fc90653c2b2f02e0b833790c07576286d3127f745ea920ae",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40f019a18b8097235264cb8efee7d149321a199ccd32ffac43b5a778dfadda1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e23809ce49747990e43b2d976083dc84d67e75cf22e5a76ad5b7a2dca50b3d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7f0a3bec1d34f2fd632993a3d9c6432401cec25ad9d6196b909f3672980bd05",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x9460aa25f77fc10cfcc4579e2011e39ce477a32a768aa553201e556ed2bbe1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x611384709c407d85c93256b6aff04c4ac515450c70cf507994165abfe2347b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2065bc7a4aa38d5fe86f9b593ccd060f8d4a5a19a9ca8b182c32199a4bd27be",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x9969a08d753e885857a5696d1cafd39f62bb193acc99089df76c240acd2fc0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6df73a948c95439f3230282814ba7e26203cfdc725901e4971ad9cff4db4396",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cdf0446663046f35c26d51e45a5233a93c51f4f7f1985dfe130dd67addefa3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59cbe680183d1dc3161ee7f945f38ab9461a5293748b2b7be84899e62c9860b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5030fda0c29a929e6cd634b9f3d1bf975c363012cfb439cae13495f8ce10225",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x49aadcf98ef59c0e5d2097845949988862b96194abc8c5453f056f232482892",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x319c68159cdf104c2543486ff784860f302187d77effb9a5fefe4e16f0ddc2c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x575531b404cdba72a63dbbd17aef7d9ae00f73eca7c6dcdaf5e0778c921be41",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x35ca7fa56aa38486833a976804899ba3c97fdaa0a23056cd2dc9bfdbcdd2e31",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47dc0e209ee8d0b67f63d9e63837ff2ab462c4839bc14a1a3e802327ff0e31f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x744bdf0c2894072564f6eca2d26efc03ef001bc6e78b34bf6be3a1a91fd90fc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x73c57ecea0c64a9bc087e50a97a28df974b294c52a0ef5854f53f69ef6773af",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7dd14b0299ff6064a96fe97e086df3f64a4c7e8b4a58a5bd5fe1b9cf7c61e7c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x562f636b49796e469dfe9e6748c4468f340e8f69e3f79cfe6925a261198dbb3",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key0(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x25257ecfcf301b18da64254e59b151a549668b8c150b39f71eb3e3508224fbb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x13342c1639906e1ef17847fdf16d032ea663fdc4fed28da92f88381ebfa626b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x12125a111c43e7a0819677f13fcfb3d15bf9921cb098a11db3cac22813437ce",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x15c58abecd948a965601dc451075b0c1dcbfe1ad91c086e0f43865ebfeca111",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x73960e722af010fc2cbc3012de9ae4bc357e8728e7d6a15b0cb10b12bbebeeb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5cb77e4fe2751fb9cf145aea5d07503c518440d7bc7b85c91b6799f14fa0b82",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x624263f4821e9fd3af946554a890b0e20ee21c5db0eb4742f547f049d7265a5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x601b3d8d8eeed4743cab5f121b1c30874e95ad5883c331aa0fef0b8d9c4b17a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50e3e5f09b36a9055c9992f6b89471bd122b785b6071483279bf6c45263f18b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6687d6c2e62115cff5db20d04e5fbcdf28e85d6acfab9e7b8869f46f388b16d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3fb1f146e0af9750e5d66c46244aa96306b4c4518e6311ccc4a730f289d4493",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4ea9551cbcfe5f18614ae86e485adba9f70255f312eb4386020432c1169a03b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cae942c39310c3913c33d6e0c10c4db313621d4e70baf4ab17a4b71a0a2336",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x186c97cc34ee93808f32336d2afe9ca74ce30b932b416ed662b0287c7db52eb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x754e8d183db530c59c19c447e1b0fa66cac465db9ed0bec3576fe8723a88d5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5bbf43b8bcde1f233ee3b725e768ddbef1d6ee4e272c2cc365cf9d58b2cec06",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x322981f5cfa5a7aac59428b3062bad8dc08145c68082d791cce8fdf2287fbd2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3318691be977dcb75f1fac814c8d42814e8fae2640bb483f5ce5fab2fa9f063",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a70f05c50491d642354e1df25c2b04cc06b790375c5dc7309838f0d17a9fc0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5d870d9e97b879e273beec4b5ea076338bebca91d4104ff4a5fa8a51cab1348",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3fb0c6c8b229297ff6644fdf9bb36a659c4a91dd060d65f243d23308fd2765a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x749e7d5e4eee9f1220ed1c8991d70dae19b3f742417b23c5acec221e971ff46",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2fe4c27271112cbdc354a0ede4e3cf78b16a52a77c3cce55cdeed2504192c65",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a3cd9702057cb220459e07f39c2db92ef07599d1016c02a8f028c6a13572ab",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x53ad84fad4ed87e5606af2364d96d04735dba9a7a1f235a13b68a7496e22a73",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2b91e359964f3f5cc62de8673b400cd968e983184fccf3d2f71da3d4a0f442b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21202fc82523fa85255e596d816cc3476bd24f5be58e592577b74fbd4a6966d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x462ecdd9bc89bf4cd186564983426380d0ef409db04df04e971d2e59f2bf32d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb981951722e737e9e029e1148b76ad3ffa4d71a21426c17065ac1482f379a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x315b9e0e2e5cd6c0490268bd5b95853a50961e5584289a0ee467e5f6bdee9dd",
        );
    res = res * point + Felt::from_hex_unchecked("0x1111111111111111");
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key1(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x46c20a61e3dd5f8e8d4085bfcf8e52c9c50152285742b9ba783a4edb6608353",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62db2649e22e5c49b68354e9a3801a9b695f7d1c8d03854b1edfa0e322ef902",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3420050a4dc6c773aa43eb3a5c8b2b88b3e33bc739775bc375933e2ff8e3846",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4bc9ed4ab2622c0cad2c8111a9a5598ddac8d9e1a89766c0bf0bfd7b750538c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2df8e6211399d12f3d33874172e235e92aae15d062516b5c1641ce309354f2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb63ac706fb9d207b59fa7456f07c71fb193932810c68fd454a98d89d96e97a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x523ed890c4e0e53b280afa7ecfc9d32182829c79f4f90ee5352b2b1671933a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x401b3d8d8eeed4303cab5f121b1c30874e95ad5883c331aa54334fd1e08f5be",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44670acb048727d509f4189ff2322439252d747f2a78570bb8c20b28f8e8c1b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5c6bd13190fe93c3c7535bade15bf685833e3f846aafbe531aedf5e448bf4f6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4e9d1c063aa65b8d301ab440c23f7489eddf4d4d1d049fd6f817fbea9b9b843",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x73531922feb3a6f79490f130de30f0c2fadd1afe367970c43540a32112a5e8c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40dfd1db116746967a4792685c8eb61ea90a2de3f9aa80975d4c6dc52e7025b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a3b463bdc8408051f1aac5b7c31cecefcd32c6a6dc13b9b9cfee72fb3dad69",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xadc55da564cf8859d120b02ab9fb09ed6ae934330c42084a392a9688723630",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4481ca3e932177631ec3ceed8ffc6372aa9cf54186ed4be4ff4641a7807c127",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x592a0b7fa0c98ab8930a5ca88ac1cbac0dfa16ae244d58745171ce70fda046",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6cc4bd7e3e47730c6f03f139e48f03db875a3e1302c00af6caabfe34f7cbf37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2392a163f404deefe6cf4481b3b183464db5160b40d56f427af356cc7ad1d37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x740152f0bfd3a106dcae496a23399b3195eb3242ec4154529c8bb14120ccbb4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a6899691636dd2f8e27428b7478962bcf7938103e12c462ebb4b1a0bed9d28",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x309e897530d9e68ebd3e9938bf89553a7fa2c5f8a26090b3cf280a70dd89e77",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xfe4c27271112c79c354a0ede4e3cf78b16a52a77c3cce561233169485d70a9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x20c3b86201349cc6d6c20bd29a2ad219855bcbc3c6db925113aa6d6dd0b57d2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3b935e02ce81c807eba625e65ee894418cd957563514ad82c0d77a87f5a37a9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5c83a35a80e818de30d8bf353017d8c6135384ab53836885cc65b88059b67af",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1d50582e5ae5502bd773493bc4783268dca4f514e019b93890c00896fd830b3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17d0b504a0d0e37a287c72997d21dec2143836cf446b2563784e32746ee2c13",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6f8b7db461cffbbaaa70ed8b33dd11c847c952fb3458a9845a5e22e6ba27d89",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6640c6366c5b918d0369115b919baa50eabbbe778b22d0cacad4a7b7bb357be",
        );
    res = res * point + Felt::from_hex_unchecked("0x1111111111111111");
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key3(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4a025b231a318123295e4fc2ad69f9552eec69c64066fac264b1cfcdd7d3158",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5460b40a6b82eea400c5f65223403b791182933281ab2654f3762718ea7272a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x577c87daa8b61681534eabd0626b9e6b73e5152876c214cd16515b620f42de",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x42cae55f34098448653815d656a3fe1e71f0b843a3a978717f2f02f69ec80c7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x54dbe71eb9ae525a52abca43b7aadeeac9664a351d711d8d43ed87957535f19",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x57276cf88395452ddfb9ecf810e6b6407e3f757e0186097c3484de9195304d7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ea216e7fa1188997bf294ce196ff23a97bed74a3a74820555d2aee1819c839",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x601b3d8d8eeed4743cab5f121b1c30874e95ad5883c331aa0fef0b8d9c4b17a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x75d25626915653ec90adc5178d4a32f65ed1857d70c0a737b9fb5535837c9d0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x559b4e49955b69058393ead11679baa4ceebadafcd17d1214cd1fb92c9a67de",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x844af79594ffe6ea50d5c9e6ee3287cd7871390cca63a0797c3f2d2d3f53f3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc52210e7d0c4dcddc855c6c31324c3263b53c9c3b675f1352fb7b83c6c0f2e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4fdd57886595821090659b2a53a9dc377a2a90315c901d58677c17a0a4cfb60",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x77b6d8762a4c0870d48f1bf7d4ef0674e1b51ca1a5f23186babfc02837bbac0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x71b4d4ca13e1018d2b54993a3950f29b7ffbd7f317e8186338bf32e552c2ae2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1000000000000021ffffffffffffffffffffffffffffffffdddddddddddddde",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6bd6af7759bad5fe5748f2ffe9d4cb5319aef5a5e59df200aa3df034cc06941",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xdd3d6e7af0c08df08799e48b8fadc745d590aba8fb7199d89ffc2e9d87bf31",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1b0a352b349c7cf1a5cb6b4faec5eaca91a91b99e1f633c145c994cd3cf8a58",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6cc8246a903b2e8ca81a6de13ceb3fc453b8925a4d49c0e5daae686c89a52ef",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x24bb40b69d54b9ce773c98f82908afd7181b1187d5051669025874cb5401c72",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x66a4081561dbc5b317067e6d12de31d212e4ec94ed17b69109c0f327639d08c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59926196b781e5f56e60c792946b8853ef6ce8c17b5f3da6fcc8ede489bafb3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2fe4c27271112cbdc354a0ede4e3cf78b16a52a77c3cce55cdeed2504192c65",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x705bc2bf83594f7602f344e78e5f6f8774d43933d67103708446fabf41e59cb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x683026c45015a1db732c80940d4b2d6dc238b4632185eeebf173d6262f26d85",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x341a62cac79ab855999592a767fa1e3270800864554795a6a6405796a23b15b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x641ad527beaf01c116281fdc3b3e75ead6a178c5d3a567950ee2d4d4cc8d8da",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59aa70b54354dfd8ac0ebc473fd8a9a613658b3025a989f4fd0af711bddb694",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6a7db27bf042f02234b078a3074c11788d26814b6b700e6bc2b629da8b32b9c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7a0f8f36b1ef9bbfd60fbda365eb2bafdc9749e3c50d90852773205d38a9f04",
        );
    res = res * point + Felt::from_hex_unchecked("0x1111111111111111");
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key7(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x279b1137fbe58a227ed345e6b3136ea36aa5eabb7ebaac5f226c318ef599e60",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x63355b4be0dccf93a1fa0f44f29a9ddd83c010b942d979a530cecd6c74ba8c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a1210c6928cd6a942ae59da3af2423896f30acfc1174f1611cd90922303f40",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2950c0942b252300fdf68871a455ed05549929a79037834d6faa1d2dab1e731",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5b2ede5b73c60003b027071c1f36a59ff339069eccddc28f143ad374c13c67a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2bfbed668244b9211d0023eaf227f0a8a9a8aea53ba93b84d35be4acaf5e0b7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6d8c656cba303fd802db54d6b43bfcec1e4db34d0ee27d5bdd23686e5f50760",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xf9309c9c444b1810d5283b7938f3de2c5a94a9df0f339579e21afa76cb17f9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5b5d5cc26ebb21f4a1851a39f0e5f5df99c9304ceacdb7f10d81344a3d781dd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21cf87a7632c50d1d9c0477a319cf769e77bb36db2f4848848c7caea274d289",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x734b0fdf3a990e5ccec5ff6f72d41000033dfbd57586901e7ce5a9ce071992",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76af3f6bd4dade530209778e5baa12faab66d6586fc87cb24c119e8e109d48c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x48639d021d476bb451319356bf88a82456f69947e4f11c446d5e847136e5ad8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xeff2fa639b2281167458555e9a07a0feb1b8d33ce88c64d2a84fa742c11b71",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x493fd6f738b14dbb2866f8dfb966aa9963463ab1edfae1b39749c44b8713519",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x60000000000000cc00000000000000000000000000000000444444444444445",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e9cf906f9fbdf8e75a43cdad43c3f7e2aa6645eedb45f02d6c87332ca5b8a4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xeff2fa639b2281167458555e9a07a0feb1b8d33ce88c64d2a84fa742c11b71",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7df59f05447f800e840bac0aeba642399305955df706f73e5f11e645104258d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x571c35a210962caff4b6f3d6c81ad517e5bd8bba7ed5435ad0121108c60deb5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16e29756eb5894d70e07ac474fd109610d3721dcd6dc8020d85a5c9832a1d7b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21cf87a7632c50d1d9c0477a319cf769e77bb36db2f4848848c7caea274d289",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7a49136d756e1c2773a7c78b098e94291c2d1d10608723b18b334f988feb421",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x506cf6363bbb4f4af2ad7c486c70c21d3a56b5620f0cc6a8a622949cd792c4c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5e6a98fe9b63767a6a03630487ca5bfed0ea8098a8c33cacf94a26f40292721",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2bfbed668244b9211d0023eaf227f0a8a9a8aea53ba93b84d35be4acaf5e0b7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50c39f36354a1a826c599a23e23a3a8dd5d38014f08950a9a7522e8bec481a3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x48e3ca5def69d4a40b490c2937e52ae81a427445812abca4eba9aab2f5add08",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x458aed4b839a0190f09fb945d16fa8daa8993e3c7754a10ba60c4b81d53c135",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x63355b4be0dccf93a1fa0f44f29a9ddd83c010b942d979a530cecd6c74ba8c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4eeab02e97b059756115eabe88cec451623ef4f0a29b7d3f005f83ad89b0f69",
        );
    res = res * point + Felt::from_hex_unchecked("0x1111111111111111");
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key15(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x24ec7e1a7b8bcb5b138102f4131d7fa12dd5fc5278251892aa929a46b50dec4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6fab9476126a16ec91fb0071a673f7c8b65b1dbd3d04502cbc915a3db9e8a1f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x12f9d780e40408e85cfd757fc76df0883a6aeb39adb7626b2ff9b582c551d3e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x76e0a1037b5bac3527a824fe3e4eb1f1841619c97aad3b04461cd97010cd690",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x51d69b95d84bf6cb03ae28cd575fd8b1c2fde0263a9cbb33b0c2d281d6593e5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x637549dd8a31e6eb931f74c374ae94c1344727397e1baa82702b01ca6765a5f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x259a0178365605d693f8d60441444981aa3782845295b3c795504192c100bd5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x501b3d8d8eeed4523cab5f121b1c30874e95ad5883c331aa32112dafbe6d39c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4cf3d65c0b542463ffbade568e57a5562c15810c7050d7fa1cffd718c535167",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xe735942acb168306ebc477871555c572af3741c59332ca97f034aaa040eef3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x231af732a4a2d73dd564d92560c1b4387d641e999f139c35a49dce4b95c40a8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6bc84bb37162e162131f843a2a2e6f1bfaf80796e8203e6c98b9007613eceea",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x34701ae77a897e00aecf2472198c4bbb5d7e5568c3b4439be3a5872607c4674",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2b785b28d1c810d0ae9d9ff6be5830d87f9eab4199991d2982faacbfee350f1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x9e00b3dfda8e20537eb42e591413bf4f6b41ff1af4904634a41461a23c0033",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a3f52e03867160939f19be537d98bcc8972a660111194aa12ac42a7ec61d96",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x388bb770abf3374034623c1e4f7fde7ed6b8a6192abda75187874c71404ac37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2fc8f5dccdd25a5950fbc1514105c89273300b1a94541a69944e88356b5a664",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x78e8e3e166c6ae785f011cdd8b78ecffdebe8b857dcc61a7339ea90e5036019",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5b858dbd345e2432e03866bcde0f9bc4e720c6c79f303f462219f77397d4815",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x728d7717199c70f029fe3331a50a2998e9f0feafd8440d09345598bfadbd37b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2d5b04f1e90c33a30180d67ba63c239c6b5eee519be4e624aa37fbd973a73b2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1fe4c27271112c9bc354a0ede4e3cf78b16a52a77c3cce55f010f47263b4e87",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x544d4edf7c744af8a57fff189322115956f891a315854171657d7a2f0a87826",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x49555ad694f14bc2cae67bf798b6cd6147f8c80d3f0adbd83ce40ea701bdab9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59b545396dcb787789f473c12a59d08f9aaa35b00fd4204cced53d3d3fdad4a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x646e2f67ac7ac7a8663739ea0c09f1f2a233531a1f6624e76502f48302871e8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ea0b1fbb487b7c25ff7c84c1d7501eb32b9b90b718b88928939dd1afb3a379",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7e84e3e28a699a839444b81427ef10cd62292ed510072b4ad884b8b5fca8138",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x62bdf821a73997783fed94521acd94cfb95eb9d6532f9b07adf1c5baade31da",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x60000000000000cc00000000000000000000000000000001555555555555556",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key31(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5852cb7312b6299275e11e7bf04e6a604038b1398d93ed46cdfa41cb9503116",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3645c0d194b9983bc9c138a6e57651551a16c071ad1f5b0fc37946aa5b6a2cf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x40c0d849a4523e6379cddf0bb9f7220b4ad778b81a7e873876700bc9d276ec4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x211f5efc84a4550dd857db01c1b14e0e7be9e6368552c4fb86aff35cbbff63e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x59c8aeb2a0ac85c5c13da60be839d4171a4a2c8808ecdf75ba6490bb81a9079",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22aeb7c2e8c9fc5f1819601d752141f60e5e894d854be8a2bc8cd9791bb115e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x691f1c1adfc25c7591030cc775b01c76d305e385ef6528a6b2f29d656d8d181",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x851b8a8accc7b09b6021d3651549195ebc108098b4994fe630055dc081479f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1a8f201c55f7facb0cc93566ad5194662a91b7372cf67f0fd9c6bc400b89de1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a3014183373f80fc4185542958defd2e934de274c76e6e4faf6922f7c26ca",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5471f4be94ccf3bb7be211193c1827b4070e4f2eea1ed7e828d4d4ca4a703f8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2c37b44c8e9d1fe0ece07bc5d5d190e40507f86917dfc1933413cc56b8dfde4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3ef97a843574486d4a4ca50dfea986f767c7fdd2a8ba4c9217eae4746ef650",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x507518e91a5aa1f164583e8bc6dfa7713e2bccb306b992611ac085e76070526",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5646153d03a487c880a5e3b7005a996b45570b60a5be6c4a0a8f545fa9900f0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x800000000000010ffffffffffffffffffffffffffffffffeeeeeeeeeeeeeef",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x32a29bfc37f6ff30148ef2f00017fecc0b8c87f6f22227f97ee3df56d231b3d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5d887530f485c860ea1cdab3580755179545c78a97de1c891f4d2598b28606c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x265b35f9701f2ec4b9f0c2ebc4aa23c969d376930985a63729470320ba328df",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1f171c1e993952caa0fee322748713002141747a82339e58992e23be7c96cb5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x198a2a2108558f4d2a79ee44f9de100d3c12f49fb5d3614a63091db177c83fd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x67bd1f15f0224840b783710d5d0dac3eed90d875f3d84ece0501b738a170b52",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4b95badd4e47e63f49ff64ccced083d700959b8009f8dceda07f2b04cc47ed7",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x77ae47575333860649fde2c9aeab6e6a143ef7f674b66b019cffaa23f7eb862",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x167f58c665f83f3d48f50c40164294afe64889203f9bb2731e2b7cce7890efe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x88ec8bbf38961364fe0675199297a9622102a21463b19f88945e655b6091b8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x86e1cac50243f817030f9dc914c0034e7a747fe0594b3e4f2a3c21a3d96b36",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3391d0985385399a99c8c615f3f60e0d5dccace5e099db1867c9d849ca45ae6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xd939b9e847fe38544d046dc62e475644b3450374885610484442e103f3105d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x451f103e0cb91bd2cc0af04966f16a59c5e4d1898022362ddf6ca5229e299e8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x702f07b05d77fca39f2b0536963dd44eceff845d2f124793dece58719d09399",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x180000000000003300000000000000000000000000000000dddddddddddddde",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_keccak_round_key63(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x13c65642d180be37bf6db1ca47e2884a87a9c17aaa35744910c1b0fccc4c50f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4dff8ed2557436c17e52a86eb24e65fcef37d2d5d0df503f27351b9fb114deb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5f4717961bc9bb0c1996dabb3c74988f05f5107081b80ff8c4c4299d22048f0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x581b3d8d8eeed4633cab5f121b1c30874e95ad5883c331aa21001c9ead5c28b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x216dc3dbb9063124640a3d5f603ca702781ff5264ef23cc5dfcec751f4a33d1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3ad1ff12fccb50a5de8e7672380cda75ed5bf08fe3d3cbecd1195b32d881025",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1d5c1c42d8ec54a42b1632618314889f63c11a24b80b67b9f587da4d671640f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38367b1b1ddda7c77956be243638610e9d2b5ab10786635430ef282c49a7404",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6cc4bb82d7803f17da904038c2212dcef110ebbaa6234e67c5718db62a14207",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x69204e331102a9fadf461be4a4a5b34cf06aa4ce4167589c0dd4f22cc888ffe",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x45d4b9d9009554d081839e925dfddc46033bb5347b940cb21555dd388c66a56",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x27e4c27271112cacc354a0ede4e3cf78b16a52a77c3cce55deffe36152a3d76",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1ad6ca03d6d5422e58017c5e3cc08f74ddc9ad8bcca0774dd78b592dc16b98b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x722178334a9327c73bca71705e5c459530198bdca5be25f1d95c98d7190bf96",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6b62a10a0b8411be1c13352de351dbe55e625cf2569f2c61613d6c57d7039cc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x800000000000010ffffffffffffffffffffffffffffffffeeeeeeeeeeeeeef",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x477aaaf598b4391c108cc2b3cf8fd238e20c8224820c59847541d59d2b5c101",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5bfdb03906bd7492c48faf9c33f8dba8f2900740d8c0f835344a49d639c843e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x653211d3237934d10c7bbcffcba0673cf8919f3d5695d32c1d40749e82bfc3f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x581b3d8d8eeed4633cab5f121b1c30874e95ad5883c331aa21001c9ead5c28b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e2c6706b58c375bedd4d71b850b928686714f3c74987aced96858048d7f05f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cf6b50644d7634d5b144cfdd1ff4f4285904b99b46a3c94eacdfe1e2d4b987",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x715d1152348922f2f7f84c0dbd3622bf0537cda35bb2dc2dc23bb5c6a103680",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x57c984e4e222596a86a941dbc9c79ef162d4a54ef8799cabaceeb5b194369db",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3bf31fc435aed4a4412cfe877384108788f7b288c0644c5f2314977430c3576",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6ce272c192cbade0ddd78c1075130b0d2dcd811b14f85eef96aba85d4c99ddc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38d10cd0c01529a85ec684600e23388d6d4f123c20cdeb6c6f2e4b5ab6739b6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x27e4c27271112cacc354a0ede4e3cf78b16a52a77c3cce55deffe36152a3d76",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x22701b06baaaeab34fc2b47969c12262b49396f2fb24f5d9802c38244c4b085",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3615d3b373ca26658a92cb1f979790b25cfa37f9c203d18c6abc0dd7e1276c0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x41eb54e16fa27062d52699248f4bdfe254e5d96302d92721efeccf4745da594",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x68000000000000dd00000000000000000000000000000001444444444444445",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key0(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7ed6ec4a18e23340489e4e36db8f4fcebf6b6ebd56185c29397344c5deea4c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5f3e9a55edfd3f6abac770ff5606fca5aaf7074bedae94ade74395453235e8e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x11eaccb2939fb9e21a2a44d6f1e0608aac4248f817bc9458cce8a56077a22b1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1b8c9c9cfe3c81279569f1130da6064cbf12c4b828d7e0cf60735514cf96c22",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7865d89fa1e9dce49da0ac14d7437366bd450fb823a4fd3d2d8b1726f924c8f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x712a2cab5d2a48c76a95de8f29a898d655cc216172a400ca054d6eb9950d698",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2574ea7cc37bd716e0ec143a2420103589ba7b2af9d6b07569af3b108450a90",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47da67f078d657e777a79423be81a5d41f445f9455b207ec9768858cfd134f1",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key1(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x143ce163d9e857b549efa236512d839954411bc04e888aa114215f991ee8a57",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4430620ab3eb75b8b2c3ee9c8bafd3408efbe93661f670002b3f96d354c2bc0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x16ba64f5ffc9bcb3a71b49f79a1c26ce608e33f1b6ce5fdfeae1c732b5d0b5",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x26315e8a17d10270d98790f94772ab99b185baeab1e0ec64e783de5c5b35859",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x21052369229137423604dbda64cdab20290c4da86882c0444750eaf0687d1c8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x67fa64d83009acfaae5a7a0e910d322b5d4dbc825090c1239dc68cd18338ed4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17190a2c4fe2fb2a1c4061a3aaa8d89e8a363f653a905e43ab819ff47516c67",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x587584d86e310744ac2167594e87c72847cc1018d766c61b29b572ba4552a80",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key2(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x565a88ff293c0a9c48cb67be157ad800604990d390e1b173e9bdc09abf9f788",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6217cc4bd0f62fec8a25f305b3914f3c6c2df7701aee105c60cd37ef815239a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x27a6021b1b06d9adf868d5ba9b068ecdee5e65fe62163095b96f7f4c2fa6c3e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x71273291cc9fb7c500b008872a8890e1e3917ea2b954d1f4a9af67427323126",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2fa9daffc6ffa8c6dd8cf633aa7c2d2a113a885f4ba935ff7f0198a4ea056cf",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x71a637fccbfdcc8da4828cb4734b6887fe9ebd78725ceb92d2756ea4e4c86fb",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x646004831088eedddafcec3518108e2033e3e613eb2b2b0ca972f75946901ba",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7d384f90e1f21f53dbafb1648ecdb97d8c020dbad501b0d79a491587484fefa",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_partial_round_key0(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3413bee8966e47edad4d25455e74664d547713650ae8ef6f7f4bd1d56077b55",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x624b1ba9e7d45d86f0a2ef7896a159e8e3d418234f3950ae2c1a1106b4d8e64",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xaba2f20ea6ee9cae2a9a5ffab6bb531cae756025a2039dbb3fdc7f6a7ea66a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x18e5bd14d527406ed33ef180f4351d66ba350fd42a210f14b13774666960edd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6fba7ab30e117b743f154c4c1ef96007fbbff3b8cddbffbaa3cf1620dad0df2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x641cd514114aa297433e1ebb6f6fe8cf4c5b3816df09b39b38bf3851328781b",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x52208d8264d42061c7107f7945857541692a87bb1b4b4307c17d43193be3ad",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x44e2813694e35f41733099371352f930e87366ded64841028c54de5ae0cf86e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x58372f1bada3f7d38dee566363d48fc45a542d57a2357a00006f8c4508f3858",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x592bcd7384ba517197075eca669701a6d8eac3bdf21af499e3defd891fc8787",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xaa7db6d9cd63141d64bc671099b444013d3ac056afb7223fdf97319f7bd76f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1e8db0feaf54299f9e0daa802e5a00c5b43dc189f622dc9d0d8039fc8f4eb16",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1125b5ff47f1e0c4105a6b62e2a6dcf3d71812409c77b4c708825299e70bcfd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7fdffd1f06d45f58c50609eddb9e4dcdf9845c3e13ae29fa3e6a4134615463a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x770f2c3dcb1befd2dbdd3e874a40ec38860828877139317823bc60ed3b69be4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x54beab500732d6102d1d501adac8f41fd04cf465e580d8664009c12e28fc5ed",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xc1e6049a1a088b613f8cb972734a8c4ee6d4bc5a359d5ebf272eff71312c01",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x24c477665b5e4b3843749877bcce106ac76c085f15b0759fe9d8f1d04b723be",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d2b779ec13ff444eafa96e2e505999c3f79b06939f6ec492378d2ccb49c3dc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6429e4ca7107ea29d85b4c45f4926f82d9d72206fdf33d7e499243b6a9ca81f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x42240cb9baca23c27a0eb13e654a5af7a490b95b51a152b8a2da5f0752226",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3c53eb4b33fc6cd4e86c4f3fbe866d358233a54b0f7c626f0ef3164ac48b189",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5cd8a4ce2b3274c77469ce2c328d9f56ed2bafe7992707f64ce99d42968f648",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2b23cce09410c815c33da25e53f0204d5d6f474f5f784647a19e9114e4cf753",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x398c6094de25847f31d6458f8bb9c6952ba9092ba7abc54d08050017ae2db64",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3a8053abe10aed5567dd7d40517596eb747cb829760fbc06f5bc322a0911c84",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x324ea07796ce3412e6f938ef1a2974abcb3f8ff7114fef8e0fef438b6e69b89",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4d5422ca4881cbaf9e99fe864068190670a6a1074e21de1382759182177eaf1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6db31dad71bfece85b88afc622cfdeaa557d4bfb3d3a313eaa4235dc7ec4ac9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0xb375c79888613ea49838515cb5f6842dea48d273b9699855c67d0978f13925",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a584b677c86b2a15d48c57df9dad7188545a3a994fef603e86ac16ce1facfa",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4291c5f5cb048e49b20c5b3caa1fa12b99ef81488aa83663110b12abfe704d8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6f2eda70c3c0c744df7d7bdc1ded3d80f290f951649456874904374564edf90",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x489dffafde7fd6ef39e1542159c9d49bfdefe802fe6b358d6ddd1f28942ba69",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x163df55208e1561da127d03f6b63d46e0aa05a1ef3321cfd5711eb4d3fb3ff1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x293681f3dfad87cd19bd1cdf5c6244a5f943e411d7a035121621f8692fa77f9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x50c9a8d62edbd150d6090cf1f0831c066282b324ca794df5aca0fbc9e71714d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x18e0bd645ba4fec89f9991a934891217ba872651494fc08589186d6e6dda88d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x8bce38b2895c04a0c7620adef3a51a8319fc4e151359a52809b1509f48f662",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5add69d3f68df10a5d626eed04e8c34e83780c2c3eb9e07bc49ce7f1fe9f618",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x90916a638846883fc6bbc6c241dd630e4346567e5520bffeb17d0b05a17cbc",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6ec11adaf8da159cac400a273fdd7765371056001451e6441a6cc9da18beb31",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x58e3b2dc12d9ffe27bf5dc6c28a216e5612a7a0775f902c537806d2f60f4226",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3b09364e6fc149b3063a5442b78165712343e075297108206e246e0de596874",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2a05aa150252d7f810276589f79dbd0aa619289cd283f72ae0d34f141635a13",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7e9c35a05ef0ccb7cfe93272e0b46324e97e1512fa4c6e1d30ca2c00dc207b8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4a2ae027e432c0863e1749b62e8533e649ed78091e11155d341cfe47168bb0",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7f610dcb987484937a18dbae85c5b46f0bbc5f2cd845487501b2f1f7bf9357c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x41b07a5f9241075a4ff1b76a9a529c9315f2435f79bab965fce61c8f616badd",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x301f8e1e5f31d9f0546da692c88e007789002e56c4ccf68f3bd5fba12db838f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x410d9eaa6c615c482f890e4c738e555ac3e4892272617bc7a0ca80613e27fc9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x583d1f426394c7610a252cae8485a3e6fe2f5fcadc19fb5097a5c55c0787fd4",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5bd0655433a76820184b6dd6fa4f3a67ebc321c75d1f9bc7422fac69074e2ff",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x74155a89a923ea1e2a23985156091d435b5b815ae1e9fa573330f01d880e52f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7523eb70ea4b345b7be4f151bfff9cdbfd589120b63d0b7a21a5cdc3d36aed9",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4dda9dce889ee4e52e93f3ccd9f32bbfbad5a8e1768aecab88170f78d5f0de1",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1cdad5777ab21cdea2c8f5994456ce2253e8b020ef32d4d12714106b7d2f632",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1871b013899aedb3e2551a73c9f7f4189e86dddd5dfb8db56965e67812ace0a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4eecc6622ccb897afcd651f5bb655b47101430a53a29bf743f5b1041ac8ff13",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5432e64cc316b7f386cf5467af442acb9d986873c5c513bdbdd133259ad54e3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1da26a447725d6a61f31012a81300349baf580ddfaada24630d03ed293da28a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3bd2aeed72b102694fc4a99c25a5250c234c91b03680ef4212885989ba9267f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4ddf2d7e380560d571e765deec379fec8ae4d909848b18b5389ec295140687d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x47237ffdabc0cba010385bf48714bb06a6a2b9316394603c450330e743124ce",
        );
    res
}

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_partial_round_key1(point: Felt) -> Felt {
    let mut res = FELT_0;
    res = res * point
        + Felt::from_hex_unchecked(
            "0x26f2aa4059eb10ba60302d001cdf4a5482d43e2d7d05bd2b5486cd8c52ab9be",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x722090545903a2f0b654199a04a5db8fc128eb36cbad8255818bf1d5db2736d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7510614da9b9ad318575990ca2107d7b8b4e66622a28b08499b7444a86e0d37",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7658d45c2170beb301fdad273c8aea07d4add3b02890567fa38c0f6b5c1689e",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x38ea9c051a4621f17bf1d34344272953018e378f8b587aabac79157963d7a11",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x794689eb6cd1d1acb82b7d5741d61a961b28a3f5468403a1981ddbc21eca96d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4883c98a63a118749cc26ac47607af9d17fb8fd36ccb743e2b6dc13f202a42f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x8729cd967a805126fa9fd4136a390051c690dfc413f1de62f6fc13123f9586",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x17dae5cd6089cc03cafa39762a14985af1e7a05e9bbf55d3952c86839098c06",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7406ca984b25f47732349b87565103d2bbf220ebab93085c063ce5ef28e7337",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x30e1c6b719648866af8220a2220904dd632b089e54ca459dcab5d853043fc25",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x26993661e64b45b5787fd8b923ecfb6f681b554191429fbfd96f7010aba3115",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x57455541d2426a546ddc818e996e5e4120a233416ce5da3422b065b60c287d3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6ae8114b8a4b0e360f3108b4c4679c6e51d7870c05c3cc5504007a29c118b53",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x5acb7c9ee9cc689cf9ed6c611a1bd730f43c4ea34b94e07ed804fb6d2bb8d4f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x29a15985dd04254ad523298f35de868c8f4538f2d800d6005634b3a32bb00f2",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x292b8bad037db0033c816ef6752c1bb9d551215a498452832f721cd95519372",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x31a3edaa5ab567b05861b16a6e0da76ea8e159108d2fe83eb73ad7b8f86ef7a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2d35fd2bf29729904a91cc5ebd7d79362c34828e0c37e09aa4907de26a45fb3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3b1f311a53410f51c90fd8a9189465059ef46149b8fb7930963ead8eabaf53a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x230279d6296ebacdcc9cfa5bf60e5a1d4ebe3ff0ac8f8bf8318c988f5c99bf8",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x2369e96b64fea009a1f66290a5dfe08010918b4ce3bfc9066739a4dbe133a0d",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x13816f7acb88c6bf0356430faf0c4fed6972a9498b29919af38d9d5f5ae440a",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x787d78882592b85f1de17e47bad43712e69d0899fc94beff77d62d2c4a1375c",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x1619514ce1cabc2996036ec8d3e3476a8a2d9e83be3e8aa7a020ad11b548622",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x176e131cb6830fe0570f692f5cce9f3f37d3444e647a318f35d1138bb580133",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4a300aa8b63feafeec79c07fe87991c0e85737ae1fbe3aa7f60d285bcf89cf3",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x7845d76e25e1fe6f884a162b379bad9bd7e421befbd911bfd1810a6973ca552",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x32e59ba3c11289dfbca64ae5646d50270c6f78f070e0ed6f1b24f45ce6832a6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x3779baab792c4c306342b4e6640c4a1c315ff9d08e0fcc97576ef889dd657b6",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x6712fc14a35a8b515bfc975d8cf356c749b04d5d7abc78ba6b2aa0924d6146f",
        );
    res = res * point
        + Felt::from_hex_unchecked(
            "0x4b7fdbd1ae93d05ecb29e4702e1715f462cce519dba31b9f4b87107ada27016",
        );
    res
}
