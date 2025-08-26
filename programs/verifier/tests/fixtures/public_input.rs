use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::air::public_memory::PublicInput;
use stark::swiftness::air::types::{AddrValue, Page, SegmentInfo};

pub fn get() -> PublicInput {
    PublicInput {
        log_n_steps: Felt::from_hex_unchecked("0x18"),
        range_check_min: Felt::from_hex_unchecked("0x0"),
        range_check_max: Felt::from_hex_unchecked("0xffff"),
        layout: Felt::from_hex_unchecked("0x7265637572736976655f776974685f706f736569646f6e"),
        dynamic_params: None,
        segments: FunVec::from_vec(vec![
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1"),
                stop_ptr: Felt::from_hex_unchecked("0x5"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1c6"),
                stop_ptr: Felt::from_hex_unchecked("0x1c43b3"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1c43b3"),
                stop_ptr: Felt::from_hex_unchecked("0x1c43b8"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1c43b8"),
                stop_ptr: Felt::from_hex_unchecked("0x1e7f0e"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1f43b8"),
                stop_ptr: Felt::from_hex_unchecked("0x2032d7"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x2f43b8"),
                stop_ptr: Felt::from_hex_unchecked("0x33d748"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x7f43b8"),
                stop_ptr: Felt::from_hex_unchecked("0x7fc6fe"),
            },
        ]),
        padding_addr: Felt::from_hex_unchecked("0x1"),
        padding_value: Felt::from_hex_unchecked("0x40780017fff7fff"),
        main_page: Page(FunVec::from_vec(vec![
            AddrValue {
                address: Felt::from_hex_unchecked("0x1"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2"),
                value: Felt::from_hex_unchecked("0xb"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4"),
                value: Felt::from_hex_unchecked("0x1b3"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa"),
                value: Felt::from_hex_unchecked("0x482480017ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffff",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12"),
                value: Felt::from_hex_unchecked("0x3"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13"),
                value: Felt::from_hex_unchecked("0x404b800080008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14"),
                value: Felt::from_hex_unchecked("0x400380007ff98001"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15"),
                value: Felt::from_hex_unchecked("0x400380007ffa8002"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16"),
                value: Felt::from_hex_unchecked("0x20780017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a"),
                value: Felt::from_hex_unchecked("0x400380007ffb8001"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b"),
                value: Felt::from_hex_unchecked("0x400380007ffc8002"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c"),
                value: Felt::from_hex_unchecked("0x482680017ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1d"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1e"),
                value: Felt::from_hex_unchecked("0x482680017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1f"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x20"),
                value: Felt::from_hex_unchecked("0x482a80007ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x21"),
                value: Felt::from_hex_unchecked("0x482a80007ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x22"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x23"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x24"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x25"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffea",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x26"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x27"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x28"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffe1",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x29"),
                value: Felt::from_hex_unchecked("0x484680017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2a"),
                value: Felt::from_hex_unchecked("0xa"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2b"),
                value: Felt::from_hex_unchecked("0x482480017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2c"),
                value: Felt::from_hex_unchecked("0xc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2d"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2e"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2f"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x30"),
                value: Felt::from_hex_unchecked("0x48307ffc7ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x31"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x32"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x33"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffdc",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x34"),
                value: Felt::from_hex_unchecked("0x402a7ffc7ffd7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x35"),
                value: Felt::from_hex_unchecked("0x40b7ffd7fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x36"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x37"),
                value: Felt::from_hex_unchecked("0x48297ffb80007ffc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x38"),
                value: Felt::from_hex_unchecked("0x48487ffd80007fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x39"),
                value: Felt::from_hex_unchecked("0x400280007ffa7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3a"),
                value: Felt::from_hex_unchecked("0x482680017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3b"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3c"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3d"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3e"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3f"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x40"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x41"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x42"),
                value: Felt::from_hex_unchecked("0x480280007ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x43"),
                value: Felt::from_hex_unchecked("0x480280007ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x44"),
                value: Felt::from_hex_unchecked("0x480280007ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x45"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x46"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff3",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x47"),
                value: Felt::from_hex_unchecked("0x482680017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x48"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x49"),
                value: Felt::from_hex_unchecked("0x482680017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4a"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4b"),
                value: Felt::from_hex_unchecked("0x482680017ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4c"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4d"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4e"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4f"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x50"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffef",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x51"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x52"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x53"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x54"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x55"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x56"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x57"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x58"),
                value: Felt::from_hex_unchecked("0x480a7ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x59"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5a"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5b"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5c"),
                value: Felt::from_hex_unchecked("0x20680017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5d"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5e"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5f"),
                value: Felt::from_hex_unchecked("0x3a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x60"),
                value: Felt::from_hex_unchecked("0x480080007ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x61"),
                value: Felt::from_hex_unchecked("0x48307fff7ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x62"),
                value: Felt::from_hex_unchecked("0x400080007ffc7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x63"),
                value: Felt::from_hex_unchecked("0x480080017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x64"),
                value: Felt::from_hex_unchecked("0x48307fff7ff88000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x65"),
                value: Felt::from_hex_unchecked("0x400080017ffa7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x66"),
                value: Felt::from_hex_unchecked("0x400080027ffa7ff8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x67"),
                value: Felt::from_hex_unchecked("0x480080037ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x68"),
                value: Felt::from_hex_unchecked("0x480080027ff88000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x69"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6a"),
                value: Felt::from_hex_unchecked("0x400080067ff77fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6b"),
                value: Felt::from_hex_unchecked("0x480080047ff78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6c"),
                value: Felt::from_hex_unchecked("0x480080037ff58000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6d"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6e"),
                value: Felt::from_hex_unchecked("0x400080077ff47fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6f"),
                value: Felt::from_hex_unchecked("0x480080057ff48000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x70"),
                value: Felt::from_hex_unchecked("0x400080087ff37fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x71"),
                value: Felt::from_hex_unchecked("0x480080097ff38000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x72"),
                value: Felt::from_hex_unchecked("0x480080047ff18000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x73"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x74"),
                value: Felt::from_hex_unchecked("0x4000800c7ff07fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x75"),
                value: Felt::from_hex_unchecked("0x4800800a7ff08000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x76"),
                value: Felt::from_hex_unchecked("0x480080057fee8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x77"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x78"),
                value: Felt::from_hex_unchecked("0x4000800d7fed7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x79"),
                value: Felt::from_hex_unchecked("0x4800800b7fed8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7a"),
                value: Felt::from_hex_unchecked("0x4000800e7fec7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7b"),
                value: Felt::from_hex_unchecked("0x4800800f7fec8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7c"),
                value: Felt::from_hex_unchecked("0x480080067fea8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7d"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7e"),
                value: Felt::from_hex_unchecked("0x400080127fe97fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7f"),
                value: Felt::from_hex_unchecked("0x480080107fe98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x80"),
                value: Felt::from_hex_unchecked("0x480080077fe78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x81"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x82"),
                value: Felt::from_hex_unchecked("0x400080137fe67fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x83"),
                value: Felt::from_hex_unchecked("0x480080117fe68000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x84"),
                value: Felt::from_hex_unchecked("0x400080147fe57fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x85"),
                value: Felt::from_hex_unchecked("0x480080157fe58000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x86"),
                value: Felt::from_hex_unchecked("0x480080087fe38000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x87"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x88"),
                value: Felt::from_hex_unchecked("0x400080187fe27fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x89"),
                value: Felt::from_hex_unchecked("0x480080167fe28000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8a"),
                value: Felt::from_hex_unchecked("0x480080097fe08000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8b"),
                value: Felt::from_hex_unchecked("0x48307fff7ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8c"),
                value: Felt::from_hex_unchecked("0x400080197fdf7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8d"),
                value: Felt::from_hex_unchecked("0x480080177fdf8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8e"),
                value: Felt::from_hex_unchecked("0x4000801a7fde7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8f"),
                value: Felt::from_hex_unchecked("0x4800801b7fde8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x90"),
                value: Felt::from_hex_unchecked("0x4800801c7fdd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x91"),
                value: Felt::from_hex_unchecked("0x4800801d7fdc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x92"),
                value: Felt::from_hex_unchecked("0x482480017fda8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x93"),
                value: Felt::from_hex_unchecked("0xa"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x94"),
                value: Felt::from_hex_unchecked("0x482480017fda8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x95"),
                value: Felt::from_hex_unchecked("0x1e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x96"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x97"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffc5",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x98"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x99"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9a"),
                value: Felt::from_hex_unchecked("0x20680017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9b"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9c"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9d"),
                value: Felt::from_hex_unchecked("0x12"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9e"),
                value: Felt::from_hex_unchecked("0x480080007ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9f"),
                value: Felt::from_hex_unchecked("0x48307fff7ff88000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa0"),
                value: Felt::from_hex_unchecked("0x400080007ffb7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa1"),
                value: Felt::from_hex_unchecked("0x480080017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa2"),
                value: Felt::from_hex_unchecked("0x48307fff7ff78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa3"),
                value: Felt::from_hex_unchecked("0x400080017ff97fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa4"),
                value: Felt::from_hex_unchecked("0x400080027ff97ff7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa5"),
                value: Felt::from_hex_unchecked("0x480080037ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa6"),
                value: Felt::from_hex_unchecked("0x480080047ff88000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa7"),
                value: Felt::from_hex_unchecked("0x480080057ff78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa8"),
                value: Felt::from_hex_unchecked("0x482480017ff58000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa9"),
                value: Felt::from_hex_unchecked("0x2"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xaa"),
                value: Felt::from_hex_unchecked("0x482480017ff58000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xab"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xac"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xad"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffaf",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xae"),
                value: Felt::from_hex_unchecked("0x482a7ffc7ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xaf"),
                value: Felt::from_hex_unchecked("0x48307ffb80007fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb0"),
                value: Felt::from_hex_unchecked("0x20680017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb1"),
                value: Felt::from_hex_unchecked("0xb"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb2"),
                value: Felt::from_hex_unchecked("0x482480017ff78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb3"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb4"),
                value: Felt::from_hex_unchecked("0x400080007ffa7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb5"),
                value: Felt::from_hex_unchecked("0x400080017ffa7ff7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb6"),
                value: Felt::from_hex_unchecked("0x400080027ffa7ff8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb7"),
                value: Felt::from_hex_unchecked("0x482480017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb8"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb9"),
                value: Felt::from_hex_unchecked("0x480080037ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xba"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xbb"),
                value: Felt::from_hex_unchecked("0x400680017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xbc"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xbd"),
                value: Felt::from_hex_unchecked("0x480080007ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xbe"),
                value: Felt::from_hex_unchecked("0x48307fff7ff68000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xbf"),
                value: Felt::from_hex_unchecked("0x400080007ff97fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc0"),
                value: Felt::from_hex_unchecked("0x482480017ff68000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc1"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc2"),
                value: Felt::from_hex_unchecked("0x400080017ff87fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc3"),
                value: Felt::from_hex_unchecked("0x400080027ff87ff6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc4"),
                value: Felt::from_hex_unchecked("0x482480017ff88000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc5"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc6"),
                value: Felt::from_hex_unchecked("0x480080037ff78000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc7"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc8"),
                value: Felt::from_hex_unchecked("0x480280007ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc9"),
                value: Felt::from_hex_unchecked("0x48327fff7ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xca"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xcb"),
                value: Felt::from_hex_unchecked("0x480080007ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xcc"),
                value: Felt::from_hex_unchecked("0x48007fff7ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xcd"),
                value: Felt::from_hex_unchecked("0x480080007ffd7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xce"),
                value: Felt::from_hex_unchecked("0x400080017ffc7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xcf"),
                value: Felt::from_hex_unchecked("0x482480017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd0"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd1"),
                value: Felt::from_hex_unchecked("0x482480017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd2"),
                value: Felt::from_hex_unchecked("0x3"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd3"),
                value: Felt::from_hex_unchecked("0x480080027ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd4"),
                value: Felt::from_hex_unchecked("0x40287ffd7ffc7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd5"),
                value: Felt::from_hex_unchecked("0x20680017fff7ffc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd6"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff8",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd7"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd8"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd9"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xda"),
                value: Felt::from_hex_unchecked("0x20680017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xdb"),
                value: Felt::from_hex_unchecked("0xc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xdc"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xdd"),
                value: Felt::from_hex_unchecked("0x480280007ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xde"),
                value: Felt::from_hex_unchecked("0x482680017ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xdf"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe0"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe1"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff73",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe2"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe3"),
                value: Felt::from_hex_unchecked("0x48127ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe4"),
                value: Felt::from_hex_unchecked("0x48127ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe5"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe6"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe7"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe8"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe9"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffe1",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xea"),
                value: Felt::from_hex_unchecked("0x48127ffe7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xeb"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xec"),
                value: Felt::from_hex_unchecked("0x48127ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xed"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xee"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xef"),
                value: Felt::from_hex_unchecked("0x1e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf0"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf1"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff18",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf2"),
                value: Felt::from_hex_unchecked("0x40137ffe7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf3"),
                value: Felt::from_hex_unchecked("0x400380007ff98002"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf4"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf5"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf6"),
                value: Felt::from_hex_unchecked("0x4002800180017fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf7"),
                value: Felt::from_hex_unchecked("0x480280017ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf8"),
                value: Felt::from_hex_unchecked("0x480280077ff98000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf9"),
                value: Felt::from_hex_unchecked("0x480a80017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xfa"),
                value: Felt::from_hex_unchecked("0x480a7ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xfb"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xfc"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffde",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xfd"),
                value: Felt::from_hex_unchecked("0x4002800180027fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xfe"),
                value: Felt::from_hex_unchecked("0x4027800180018003"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xff"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x100"),
                value: Felt::from_hex_unchecked("0x4003800380018004"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x101"),
                value: Felt::from_hex_unchecked("0x482a800480038000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x102"),
                value: Felt::from_hex_unchecked("0x4802800280018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x103"),
                value: Felt::from_hex_unchecked("0x40317fff7ffe8005"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x104"),
                value: Felt::from_hex_unchecked("0x4027800180028006"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x105"),
                value: Felt::from_hex_unchecked("0x2"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x106"),
                value: Felt::from_hex_unchecked("0x40137ffb7fff8007"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x107"),
                value: Felt::from_hex_unchecked("0x400380027ff98008"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x108"),
                value: Felt::from_hex_unchecked("0x400380037ff98009"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x109"),
                value: Felt::from_hex_unchecked("0x400380047ff9800a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10a"),
                value: Felt::from_hex_unchecked("0x400380057ff9800b"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10b"),
                value: Felt::from_hex_unchecked("0x400380067ff9800c"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10c"),
                value: Felt::from_hex_unchecked("0x40137ffc7fff800d"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10d"),
                value: Felt::from_hex_unchecked("0x400380087ff9800e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10e"),
                value: Felt::from_hex_unchecked("0x400380097ff9800f"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10f"),
                value: Felt::from_hex_unchecked("0x4003800a7ff98010"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x110"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x111"),
                value: Felt::from_hex_unchecked("0x4826800180008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x112"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x113"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x114"),
                value: Felt::from_hex_unchecked("0xb"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x115"),
                value: Felt::from_hex_unchecked("0x480a80037fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x116"),
                value: Felt::from_hex_unchecked("0x480a80047fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x117"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x118"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff11",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x119"),
                value: Felt::from_hex_unchecked("0x1088800580018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11a"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11b"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffeef",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11c"),
                value: Felt::from_hex_unchecked("0x402a800480117fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11d"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11e"),
                value: Felt::from_hex_unchecked("0x4826800180008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11f"),
                value: Felt::from_hex_unchecked("0x12"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x120"),
                value: Felt::from_hex_unchecked("0x480a80037fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x121"),
                value: Felt::from_hex_unchecked("0x480a80117fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x122"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x123"),
                value: Felt::from_hex_unchecked("0xb"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x124"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x125"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffeea",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x126"),
                value: Felt::from_hex_unchecked("0x402a800380047fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x127"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x128"),
                value: Felt::from_hex_unchecked("0x4826800180008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x129"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12a"),
                value: Felt::from_hex_unchecked("0x4826800180008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12b"),
                value: Felt::from_hex_unchecked("0x12"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12c"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12d"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12e"),
                value: Felt::from_hex_unchecked("0xb"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12f"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x130"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff0f",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x131"),
                value: Felt::from_hex_unchecked("0x402b8002801d8012"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x132"),
                value: Felt::from_hex_unchecked("0x400380008002801d"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x133"),
                value: Felt::from_hex_unchecked("0x4826800180008000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x134"),
                value: Felt::from_hex_unchecked("0x12"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x135"),
                value: Felt::from_hex_unchecked("0x48127ffe7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x136"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x137"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x138"),
                value: Felt::from_hex_unchecked("0x23"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x139"),
                value: Felt::from_hex_unchecked("0x402780017ff38001"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13a"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13b"),
                value: Felt::from_hex_unchecked("0x400b7ff47fff8002"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13c"),
                value: Felt::from_hex_unchecked("0x400b80007fff8003"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13d"),
                value: Felt::from_hex_unchecked("0x400b7ff67fff8004"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13e"),
                value: Felt::from_hex_unchecked("0x400b7ff77fff8005"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13f"),
                value: Felt::from_hex_unchecked("0x400b7ff87fff8006"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x140"),
                value: Felt::from_hex_unchecked("0x400b7ff97fff8007"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x141"),
                value: Felt::from_hex_unchecked("0x400b7ffa7fff8008"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x142"),
                value: Felt::from_hex_unchecked("0x400b7ffb7fff8009"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x143"),
                value: Felt::from_hex_unchecked("0x400b7ffc7fff800a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x144"),
                value: Felt::from_hex_unchecked("0x400b7ffd7fff800b"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x145"),
                value: Felt::from_hex_unchecked("0x400780017fff800c"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x146"),
                value: Felt::from_hex_unchecked("0x6f7574707574"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x147"),
                value: Felt::from_hex_unchecked("0x400780017fff800d"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x148"),
                value: Felt::from_hex_unchecked("0x706564657273656e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x149"),
                value: Felt::from_hex_unchecked("0x400780017fff800e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14a"),
                value: Felt::from_hex_unchecked("0x72616e67655f636865636b"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14b"),
                value: Felt::from_hex_unchecked("0x400780017fff800f"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14c"),
                value: Felt::from_hex_unchecked("0x6563647361"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14d"),
                value: Felt::from_hex_unchecked("0x400780017fff8010"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14e"),
                value: Felt::from_hex_unchecked("0x62697477697365"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14f"),
                value: Felt::from_hex_unchecked("0x400780017fff8011"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x150"),
                value: Felt::from_hex_unchecked("0x65635f6f70"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x151"),
                value: Felt::from_hex_unchecked("0x400780017fff8012"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x152"),
                value: Felt::from_hex_unchecked("0x6b656363616b"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x153"),
                value: Felt::from_hex_unchecked("0x400780017fff8013"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x154"),
                value: Felt::from_hex_unchecked("0x706f736569646f6e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x155"),
                value: Felt::from_hex_unchecked("0x400780017fff8014"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x156"),
                value: Felt::from_hex_unchecked("0x72616e67655f636865636b3936"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x157"),
                value: Felt::from_hex_unchecked("0x400780017fff8015"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x158"),
                value: Felt::from_hex_unchecked("0x6164645f6d6f64"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x159"),
                value: Felt::from_hex_unchecked("0x400780017fff8016"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15a"),
                value: Felt::from_hex_unchecked("0x6d756c5f6d6f64"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15b"),
                value: Felt::from_hex_unchecked("0x400780017fff8017"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15c"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15d"),
                value: Felt::from_hex_unchecked("0x400780017fff8018"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15e"),
                value: Felt::from_hex_unchecked("0x3"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15f"),
                value: Felt::from_hex_unchecked("0x400780017fff8019"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x160"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x161"),
                value: Felt::from_hex_unchecked("0x400780017fff801a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x162"),
                value: Felt::from_hex_unchecked("0x2"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x163"),
                value: Felt::from_hex_unchecked("0x400780017fff801b"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x164"),
                value: Felt::from_hex_unchecked("0x5"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x165"),
                value: Felt::from_hex_unchecked("0x400780017fff801c"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x166"),
                value: Felt::from_hex_unchecked("0x7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x167"),
                value: Felt::from_hex_unchecked("0x400780017fff801d"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x168"),
                value: Felt::from_hex_unchecked("0x10"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x169"),
                value: Felt::from_hex_unchecked("0x400780017fff801e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16a"),
                value: Felt::from_hex_unchecked("0x6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16b"),
                value: Felt::from_hex_unchecked("0x400780017fff801f"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16c"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16d"),
                value: Felt::from_hex_unchecked("0x400780017fff8020"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16e"),
                value: Felt::from_hex_unchecked("0x7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16f"),
                value: Felt::from_hex_unchecked("0x400780017fff8021"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x170"),
                value: Felt::from_hex_unchecked("0x7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x171"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x172"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffe97",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x173"),
                value: Felt::from_hex_unchecked("0x482480017ffe8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x174"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x175"),
                value: Felt::from_hex_unchecked("0x480a7ff57fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x176"),
                value: Felt::from_hex_unchecked("0x482480017ffc8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x177"),
                value: Felt::from_hex_unchecked("0xc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x178"),
                value: Felt::from_hex_unchecked("0x482480017ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x179"),
                value: Felt::from_hex_unchecked("0x17"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17a"),
                value: Felt::from_hex_unchecked("0x480280007ff38000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17b"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17c"),
                value: Felt::from_hex_unchecked("0x26"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17d"),
                value: Felt::from_hex_unchecked("0x400a80007fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17e"),
                value: Felt::from_hex_unchecked("0x40137ffe7fff8022"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17f"),
                value: Felt::from_hex_unchecked("0x4802800280228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x180"),
                value: Felt::from_hex_unchecked("0x48307ffe80007fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x181"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x182"),
                value: Felt::from_hex_unchecked("0x40"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x183"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x184"),
                value: Felt::from_hex_unchecked("0xe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x185"),
                value: Felt::from_hex_unchecked("0x4802800080228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x186"),
                value: Felt::from_hex_unchecked("0x4802800180228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x187"),
                value: Felt::from_hex_unchecked("0x4802800280228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x188"),
                value: Felt::from_hex_unchecked("0x4802800380228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x189"),
                value: Felt::from_hex_unchecked("0x4802800480228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18a"),
                value: Felt::from_hex_unchecked("0x4802800580228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18b"),
                value: Felt::from_hex_unchecked("0x4802800680228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18c"),
                value: Felt::from_hex_unchecked("0x4802800780228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18d"),
                value: Felt::from_hex_unchecked("0x4802800880228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18e"),
                value: Felt::from_hex_unchecked("0x4802800980228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18f"),
                value: Felt::from_hex_unchecked("0x4802800a80228000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x190"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x191"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x192"),
                value: Felt::from_hex_unchecked("0x5"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x193"),
                value: Felt::from_hex_unchecked("0x400780017fff7ffc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x194"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x195"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x196"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x197"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x198"),
                value: Felt::from_hex_unchecked("0x48307fff7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x199"),
                value: Felt::from_hex_unchecked("0x48317fff80007ffc"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19a"),
                value: Felt::from_hex_unchecked("0x40507fff7fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19b"),
                value: Felt::from_hex_unchecked("0x48127ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19c"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19d"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19e"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19f"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff4",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a0"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a1"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a2"),
                value: Felt::from_hex_unchecked("0x5"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a3"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a4"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a5"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a6"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a7"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a8"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a9"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1aa"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ab"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ac"),
                value: Felt::from_hex_unchecked("0x48127ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ad"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ae"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff42",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1af"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b0"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b1"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b2"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b3"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b4"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffffef",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b5"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b6"),
                value: Felt::from_hex_unchecked("0x480a7ff37fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b7"),
                value: Felt::from_hex_unchecked("0x480a7ff47fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b8"),
                value: Felt::from_hex_unchecked("0x480a7ff57fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b9"),
                value: Felt::from_hex_unchecked("0x480a7ff67fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ba"),
                value: Felt::from_hex_unchecked("0x480a7ff77fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1bb"),
                value: Felt::from_hex_unchecked("0x480a7ff87fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1bc"),
                value: Felt::from_hex_unchecked("0x480a7ff97fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1bd"),
                value: Felt::from_hex_unchecked("0x480a7ffa7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1be"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1bf"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c0"),
                value: Felt::from_hex_unchecked("0x480a7ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c1"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c2"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010ffffffffffffffffffffffffffffffffffffffffffffff77",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c3"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c4"),
                value: Felt::from_hex_unchecked("0x1c6"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c5"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c6"),
                value: Felt::from_hex_unchecked("0x1c43b3"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c7"),
                value: Felt::from_hex_unchecked("0x1c43b8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c8"),
                value: Felt::from_hex_unchecked("0x1f43b8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c9"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ca"),
                value: Felt::from_hex_unchecked("0x2f43b8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1cb"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1cc"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1cd"),
                value: Felt::from_hex_unchecked("0x7f43b8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1ce"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1cf"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1d0"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43a8"),
                value: Felt::from_hex_unchecked("0x1c43b8"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43a9"),
                value: Felt::from_hex_unchecked("0x1e7f0e"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43aa"),
                value: Felt::from_hex_unchecked("0x2032d7"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43ab"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43ac"),
                value: Felt::from_hex_unchecked("0x33d748"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43ad"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43ae"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43af"),
                value: Felt::from_hex_unchecked("0x7fc6fe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b0"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b1"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b2"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b3"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b4"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b5"),
                value: Felt::from_hex_unchecked(
                    "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b6"),
                value: Felt::from_hex_unchecked(
                    "0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c43b7"),
                value: Felt::from_hex_unchecked(
                    "0xb2c58e4eec9b5a8f0c5ba4d15ae59c8ac8a8d96fca443dd591296ba3391aaf",
                ),
            },
        ])),
        continuous_page_headers: FunVec::from_vec(vec![]),
    }
}
