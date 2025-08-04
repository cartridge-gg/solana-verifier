use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::air::public_memory::PublicInput;
use stark::swiftness::air::types::{AddrValue, Page, SegmentInfo};

pub fn get() -> PublicInput {
    PublicInput {
        log_n_steps: Felt::from_hex_unchecked("0xe"),
        range_check_min: Felt::from_hex_unchecked("0x7ffa"),
        range_check_max: Felt::from_hex_unchecked("0x8001"),
        layout: Felt::from_hex_unchecked("0x726563757273697665"),
        // dynamic_params: None,
        dynamic_params: Some(stark::swiftness::air::dynamic::DynamicParams {
            add_mod_a0_suboffset: 1,
            add_mod_a1_suboffset: 2,
            add_mod_a2_suboffset: 3,
            add_mod_a3_suboffset: 4,
            add_mod_a_offset_suboffset: 5,
            add_mod_b0_suboffset: 10,
            add_mod_b1_suboffset: 11,
            add_mod_b2_suboffset: 12,
            add_mod_b3_suboffset: 13,
            add_mod_b_offset_suboffset: 14,
            cpu_decode_mem_inst_suboffset: 100,
            cpu_decode_off0_suboffset: 101,
            cpu_decode_off1_suboffset: 102,
            cpu_decode_off2_suboffset: 103,
            cpu_operands_mem_dst_suboffset: 200,
            cpu_operands_mem_op0_suboffset: 201,
            cpu_operands_mem_op1_suboffset: 202,
            pedersen_input0_suboffset: 300,
            pedersen_input1_suboffset: 301,
            pedersen_output_suboffset: 302,
            pedersen_builtin_row_ratio: 256,
            poseidon_param_0_input_output_suboffset: 400,
            poseidon_param_1_input_output_suboffset: 401,
            poseidon_param_2_input_output_suboffset: 402,
            poseidon_row_ratio: 128,
            range_check_builtin_inner_range_check_suboffset: 500,
            range_check_builtin_mem_suboffset: 501,
            range_check_builtin_row_ratio: 64,
            bitwise_diluted_var_pool_suboffset: 600,
            bitwise_row_ratio: 32,
            bitwise_var_pool_suboffset: 601,
            ec_op_builtin_row_ratio: 512,
            ec_op_p_x_suboffset: 700,
            ec_op_p_y_suboffset: 701,
            ec_op_q_x_suboffset: 702,
            ec_op_q_y_suboffset: 703,
            ec_op_r_x_suboffset: 704,
            ec_op_r_y_suboffset: 705,
            ..Default::default()
        }),
        segments: FunVec::from_vec(vec![
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1"),
                stop_ptr: Felt::from_hex_unchecked("0x5"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x25"),
                stop_ptr: Felt::from_hex_unchecked("0x68"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x68"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x6a"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1ea"),
                stop_ptr: Felt::from_hex_unchecked("0x1ea"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x9ea"),
                stop_ptr: Felt::from_hex_unchecked("0x9ea"),
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
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4"),
                value: Felt::from_hex_unchecked("0x4"),
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
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9"),
                value: Felt::from_hex_unchecked("0x400380007ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe"),
                value: Felt::from_hex_unchecked("0x480a80007fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10"),
                value: Felt::from_hex_unchecked("0x9"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11"),
                value: Felt::from_hex_unchecked("0x400280017ffa7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12"),
                value: Felt::from_hex_unchecked("0x482680017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13"),
                value: Felt::from_hex_unchecked("0x2"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
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
