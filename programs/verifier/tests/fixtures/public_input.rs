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
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16"),
                value: Felt::from_hex_unchecked("0x480a7ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1d"),
                value: Felt::from_hex_unchecked("0x482a7ffc7ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1e"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1f"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x20"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x21"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff9",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x22"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x23"),
                value: Felt::from_hex_unchecked("0x25"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x24"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x25"),
                value: Felt::from_hex_unchecked("0x68"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x26"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x27"),
                value: Felt::from_hex_unchecked("0x1ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x28"),
                value: Felt::from_hex_unchecked("0x9ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x64"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x65"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x66"),
                value: Felt::from_hex_unchecked("0x1ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x67"),
                value: Felt::from_hex_unchecked("0x9ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x68"),
                value: Felt::from_hex_unchecked("0xa"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x69"),
                value: Felt::from_hex_unchecked("0x90"),
            },
        ])),
        continuous_page_headers: FunVec::from_vec(vec![]),
    }
}
