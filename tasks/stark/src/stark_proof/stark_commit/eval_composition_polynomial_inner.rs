use crate::swiftness::air::consts::*;
use crate::swiftness::air::recursive_with_poseidon::consts::FELT_1;
use crate::swiftness::stark::types::StarkProof;
use felt::felt_nonzero;
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// Macro to maintain readability: column_row[col][row] -> mask_values[index]
// Maps column_row notation to flat array indices based on the original pattern
macro_rules! column_row {
    ($mask_values:expr, $col:expr, $row:expr) => {
        // This mapping is based on the original function's access pattern
        // We need to map each column_row access to the correct flat array index
        match ($col, $row) {
            (0, 0) => $mask_values[0],
            (0, 1) => $mask_values[1],
            (0, 2) => $mask_values[2],
            (0, 3) => $mask_values[3],
            (0, 4) => $mask_values[4],
            (0, 5) => $mask_values[5],
            (0, 6) => $mask_values[6],
            (0, 7) => $mask_values[7],
            (0, 8) => $mask_values[8],
            (0, 9) => $mask_values[9],
            (0, 10) => $mask_values[10],
            (0, 11) => $mask_values[11],
            (0, 12) => $mask_values[12],
            (0, 13) => $mask_values[13],
            (0, 14) => $mask_values[14],
            (0, 15) => $mask_values[15],
            (1, 0) => $mask_values[16],
            (1, 1) => $mask_values[17],
            (1, 2) => $mask_values[18],
            (1, 3) => $mask_values[19],
            (1, 4) => $mask_values[20],
            (1, 5) => $mask_values[21],
            (1, 8) => $mask_values[22],
            (1, 9) => $mask_values[23],
            (1, 10) => $mask_values[24],
            (1, 11) => $mask_values[25],
            (1, 12) => $mask_values[26],
            (1, 13) => $mask_values[27],
            (1, 16) => $mask_values[28],
            (1, 42) => $mask_values[29],
            (1, 43) => $mask_values[30],
            (1, 74) => $mask_values[31],
            (1, 75) => $mask_values[32],
            (1, 106) => $mask_values[33],
            (1, 138) => $mask_values[34],
            (1, 139) => $mask_values[35],
            (1, 171) => $mask_values[36],
            (1, 202) => $mask_values[37],
            (1, 203) => $mask_values[38],
            (1, 234) => $mask_values[39],
            (1, 235) => $mask_values[40],
            (1, 266) => $mask_values[41],
            (1, 267) => $mask_values[42],
            (1, 298) => $mask_values[43],
            (1, 394) => $mask_values[44],
            (1, 458) => $mask_values[45],
            (1, 459) => $mask_values[46],
            (1, 714) => $mask_values[47],
            (1, 715) => $mask_values[48],
            (1, 778) => $mask_values[49],
            (1, 779) => $mask_values[50],
            (1, 970) => $mask_values[51],
            (1, 971) => $mask_values[52],
            (1, 1034) => $mask_values[53],
            (1, 1035) => $mask_values[54],
            (1, 2058) => $mask_values[55],
            (1, 2059) => $mask_values[56],
            (1, 4106) => $mask_values[57],
            (2, 0) => $mask_values[58],
            (2, 1) => $mask_values[59],
            (2, 2) => $mask_values[60],
            (2, 3) => $mask_values[61],
            (3, 0) => $mask_values[62],
            (3, 1) => $mask_values[63],
            (3, 2) => $mask_values[64],
            (3, 3) => $mask_values[65],
            (3, 4) => $mask_values[66],
            (3, 8) => $mask_values[67],
            (3, 12) => $mask_values[68],
            (3, 16) => $mask_values[69],
            (3, 20) => $mask_values[70],
            (3, 24) => $mask_values[71],
            (3, 28) => $mask_values[72],
            (3, 32) => $mask_values[73],
            (3, 36) => $mask_values[74],
            (3, 40) => $mask_values[75],
            (3, 44) => $mask_values[76],
            (3, 48) => $mask_values[77],
            (3, 52) => $mask_values[78],
            (3, 56) => $mask_values[79],
            (3, 60) => $mask_values[80],
            (3, 64) => $mask_values[81],
            (3, 66) => $mask_values[82],
            (3, 128) => $mask_values[83],
            (3, 130) => $mask_values[84],
            (3, 176) => $mask_values[85],
            (3, 180) => $mask_values[86],
            (3, 184) => $mask_values[87],
            (3, 188) => $mask_values[88],
            (3, 192) => $mask_values[89],
            (3, 194) => $mask_values[90],
            (3, 240) => $mask_values[91],
            (3, 244) => $mask_values[92],
            (3, 248) => $mask_values[93],
            (3, 252) => $mask_values[94],
            (4, 0) => $mask_values[95],
            (4, 1) => $mask_values[96],
            (4, 2) => $mask_values[97],
            (4, 3) => $mask_values[98],
            (4, 4) => $mask_values[99],
            (4, 5) => $mask_values[100],
            (4, 6) => $mask_values[101],
            (4, 7) => $mask_values[102],
            (4, 8) => $mask_values[103],
            (4, 9) => $mask_values[104],
            (4, 11) => $mask_values[105],
            (4, 12) => $mask_values[106],
            (4, 13) => $mask_values[107],
            (4, 44) => $mask_values[108],
            (4, 76) => $mask_values[109],
            (4, 108) => $mask_values[110],
            (4, 140) => $mask_values[111],
            (4, 172) => $mask_values[112],
            (4, 204) => $mask_values[113],
            (4, 236) => $mask_values[114],
            (4, 1539) => $mask_values[115],
            (4, 1547) => $mask_values[116],
            (4, 1571) => $mask_values[117],
            (4, 1579) => $mask_values[118],
            (4, 2011) => $mask_values[119],
            (4, 2019) => $mask_values[120],
            (4, 2041) => $mask_values[121],
            (4, 2045) => $mask_values[122],
            (4, 2047) => $mask_values[123],
            (4, 2049) => $mask_values[124],
            (4, 2051) => $mask_values[125],
            (4, 2053) => $mask_values[126],
            (4, 4089) => $mask_values[127],
            (5, 0) => $mask_values[128],
            (5, 1) => $mask_values[129],
            (5, 2) => $mask_values[130],
            (5, 4) => $mask_values[131],
            (5, 6) => $mask_values[132],
            (5, 8) => $mask_values[133],
            (5, 9) => $mask_values[134],
            (5, 10) => $mask_values[135],
            (5, 12) => $mask_values[136],
            (5, 14) => $mask_values[137],
            (5, 16) => $mask_values[138],
            (5, 17) => $mask_values[139],
            (5, 22) => $mask_values[140],
            (5, 24) => $mask_values[141],
            (5, 25) => $mask_values[142],
            (5, 30) => $mask_values[143],
            (5, 33) => $mask_values[144],
            (5, 38) => $mask_values[145],
            (5, 41) => $mask_values[146],
            (5, 46) => $mask_values[147],
            (5, 49) => $mask_values[148],
            (5, 54) => $mask_values[149],
            (5, 57) => $mask_values[150],
            (5, 65) => $mask_values[151],
            (5, 73) => $mask_values[152],
            (5, 81) => $mask_values[153],
            (5, 89) => $mask_values[154],
            (5, 97) => $mask_values[155],
            (5, 105) => $mask_values[156],
            (5, 137) => $mask_values[157],
            (5, 169) => $mask_values[158],
            (5, 201) => $mask_values[159],
            (5, 393) => $mask_values[160],
            (5, 409) => $mask_values[161],
            (5, 425) => $mask_values[162],
            (5, 457) => $mask_values[163],
            (5, 473) => $mask_values[164],
            (5, 489) => $mask_values[165],
            (5, 521) => $mask_values[166],
            (5, 553) => $mask_values[167],
            (5, 585) => $mask_values[168],
            (5, 609) => $mask_values[169],
            (5, 625) => $mask_values[170],
            (5, 641) => $mask_values[171],
            (5, 657) => $mask_values[172],
            (5, 673) => $mask_values[173],
            (5, 689) => $mask_values[174],
            (5, 905) => $mask_values[175],
            (5, 921) => $mask_values[176],
            (5, 937) => $mask_values[177],
            (5, 969) => $mask_values[178],
            (5, 982) => $mask_values[179],
            (5, 985) => $mask_values[180],
            (5, 998) => $mask_values[181],
            (5, 1001) => $mask_values[182],
            (5, 1014) => $mask_values[183],
            (6, 0) => $mask_values[184], // inter1_row0
            (6, 1) => $mask_values[185], // inter1_row1
            (6, 2) => $mask_values[186], // inter1_row2
            (6, 3) => $mask_values[187], // inter1_row3
            (7, 0) => $mask_values[188], // inter1_row0
            (7, 1) => $mask_values[189], // inter1_row1
            (7, 2) => $mask_values[190], // inter1_row2
            (7, 5) => $mask_values[191], // inter1_row5
            _ => panic!("Invalid column_row access: column{}_row{}", $col, $row),
        }
    };
}

#[repr(C)]
pub struct EvalCompositionPolynomialInner {
    phase: EvalCompositionPolynomialInnerPhase,
    current_step: usize,
    total_sum: Felt,
    point: Felt,
    trace_generator: Felt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalCompositionPolynomialInnerPhase {
    ComputePowers,
    ComputeDomains,
    ComputeConstraints,
    Done,
}

impl_type_identifiable!(EvalCompositionPolynomialInner);

impl EvalCompositionPolynomialInner {
    pub fn new() -> Self {
        Self {
            phase: EvalCompositionPolynomialInnerPhase::ComputePowers,
            current_step: 0,
            total_sum: Felt::ZERO,
            point: Felt::ZERO,
            trace_generator: Felt::ZERO,
        }
    }
}

impl Default for EvalCompositionPolynomialInner {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for EvalCompositionPolynomialInner {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            EvalCompositionPolynomialInnerPhase::ComputePowers => {
                self.point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.trace_generator = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Get global_values first to access trace_length
                let global_values = stack.get_global_values();
                let trace_length = global_values.trace_length;

                // Get pre-allocated arrays
                let autogenerated_pows = stack.get_autogenerated_pows_mut();

                // Compute powers following the original algorithm
                // pow0 to pow12 (point powers)
                autogenerated_pows[0] = self
                    .point
                    .pow_felt(&(trace_length.floor_div(&felt::felt_nonzero!(FELT_4096))));
                autogenerated_pows[1] = autogenerated_pows[0] * autogenerated_pows[0];
                autogenerated_pows[2] = autogenerated_pows[1] * autogenerated_pows[1];
                autogenerated_pows[3] = autogenerated_pows[2] * autogenerated_pows[2];
                autogenerated_pows[4] = autogenerated_pows[3] * autogenerated_pows[3];
                autogenerated_pows[5] = autogenerated_pows[4] * autogenerated_pows[4];
                autogenerated_pows[6] = autogenerated_pows[5] * autogenerated_pows[5];
                autogenerated_pows[7] = autogenerated_pows[6] * autogenerated_pows[6];
                autogenerated_pows[8] = autogenerated_pows[7] * autogenerated_pows[7];
                autogenerated_pows[9] = autogenerated_pows[8] * autogenerated_pows[8];
                autogenerated_pows[10] = autogenerated_pows[9] * autogenerated_pows[9];
                autogenerated_pows[11] = autogenerated_pows[10] * autogenerated_pows[10];
                autogenerated_pows[12] = autogenerated_pows[11] * autogenerated_pows[11];

                // pow13 to pow50 (trace_generator powers)
                autogenerated_pows[13] = self.trace_generator.pow_felt(&(trace_length - FELT_512));
                autogenerated_pows[14] = self.trace_generator.pow_felt(&(trace_length - FELT_256));
                autogenerated_pows[15] = self.trace_generator.pow_felt(&(trace_length - FELT_4096));
                autogenerated_pows[16] = self.trace_generator.pow_felt(&(trace_length - FELT_4));
                autogenerated_pows[17] = self.trace_generator.pow_felt(&(trace_length - FELT_2));
                autogenerated_pows[18] = self.trace_generator.pow_felt(&(trace_length - FELT_16));
                autogenerated_pows[19] = self
                    .trace_generator
                    .pow_felt(&(trace_length.floor_div(&felt::felt_nonzero!(FELT_2))));
                autogenerated_pows[20] = self.trace_generator.pow_felt(
                    &((FELT_255 * trace_length).floor_div(&felt::felt_nonzero!(FELT_256))),
                );
                autogenerated_pows[21] = self
                    .trace_generator
                    .pow_felt(&(trace_length.floor_div(&felt::felt_nonzero!(FELT_64))));
                autogenerated_pows[22] = autogenerated_pows[21] * autogenerated_pows[21];
                autogenerated_pows[23] = autogenerated_pows[21] * autogenerated_pows[22];
                autogenerated_pows[24] = autogenerated_pows[21] * autogenerated_pows[23];
                autogenerated_pows[25] = autogenerated_pows[21] * autogenerated_pows[24];
                autogenerated_pows[26] = autogenerated_pows[21] * autogenerated_pows[25];
                autogenerated_pows[27] = autogenerated_pows[19] * autogenerated_pows[26];
                autogenerated_pows[28] = autogenerated_pows[21] * autogenerated_pows[26];
                autogenerated_pows[29] = autogenerated_pows[21] * autogenerated_pows[28];
                autogenerated_pows[30] = autogenerated_pows[19] * autogenerated_pows[29];
                autogenerated_pows[31] = autogenerated_pows[21] * autogenerated_pows[29];
                autogenerated_pows[32] = autogenerated_pows[21] * autogenerated_pows[31];
                autogenerated_pows[33] = autogenerated_pows[19] * autogenerated_pows[32];
                autogenerated_pows[34] = autogenerated_pows[21] * autogenerated_pows[32];
                autogenerated_pows[35] = autogenerated_pows[21] * autogenerated_pows[34];
                autogenerated_pows[36] = autogenerated_pows[19] * autogenerated_pows[35];
                autogenerated_pows[37] = autogenerated_pows[21] * autogenerated_pows[35];
                autogenerated_pows[38] = autogenerated_pows[21] * autogenerated_pows[37];
                autogenerated_pows[39] = autogenerated_pows[19] * autogenerated_pows[38];
                autogenerated_pows[40] = autogenerated_pows[21] * autogenerated_pows[38];
                autogenerated_pows[41] = autogenerated_pows[22] * autogenerated_pows[39];
                autogenerated_pows[42] = autogenerated_pows[22] * autogenerated_pows[41];
                autogenerated_pows[43] = autogenerated_pows[22] * autogenerated_pows[42];
                autogenerated_pows[44] = autogenerated_pows[22] * autogenerated_pows[43];
                autogenerated_pows[45] = autogenerated_pows[22] * autogenerated_pows[44];
                autogenerated_pows[46] = autogenerated_pows[22] * autogenerated_pows[45];
                autogenerated_pows[47] = autogenerated_pows[22] * autogenerated_pows[46];
                autogenerated_pows[48] = autogenerated_pows[21] * autogenerated_pows[47];
                autogenerated_pows[49] = autogenerated_pows[21] * autogenerated_pows[48];
                autogenerated_pows[50] = autogenerated_pows[21] * autogenerated_pows[49];

                self.phase = EvalCompositionPolynomialInnerPhase::ComputeDomains;
                vec![]
            }

            EvalCompositionPolynomialInnerPhase::ComputeDomains => {
                let (autogenerated_pows, domains) = stack.get_pows_and_domains_mut();

                // Compute domains using powers from pre-allocated array
                domains[0] = autogenerated_pows[12] - FELT_1;
                domains[1] = autogenerated_pows[11] - FELT_1;
                domains[2] = autogenerated_pows[10] - FELT_1;
                domains[3] = autogenerated_pows[9] - FELT_1;
                domains[4] = autogenerated_pows[8] - autogenerated_pows[47];
                domains[5] = autogenerated_pows[8] - FELT_1;
                domains[6] = autogenerated_pows[7] - FELT_1;
                domains[7] = autogenerated_pows[6] - FELT_1;
                domains[8] = autogenerated_pows[5] - FELT_1;
                domains[9] = autogenerated_pows[4] - FELT_1;
                domains[10] = autogenerated_pows[4] - autogenerated_pows[41];

                // Compute complex domains with temp calculations
                let temp = autogenerated_pows[4] - autogenerated_pows[21];
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[22]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[23]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[24]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[25]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[26]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[28]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[29]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[31]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[32]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[34]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[35]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[37]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[38]);
                let temp = temp * (autogenerated_pows[4] - autogenerated_pows[40]);
                domains[11] = temp * domains[9];

                domains[12] = autogenerated_pows[3] - FELT_1;
                domains[13] = autogenerated_pows[3] - autogenerated_pows[41];
                domains[14] = autogenerated_pows[2] - autogenerated_pows[49];

                let temp = autogenerated_pows[2] - autogenerated_pows[36];
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[39]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[41]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[42]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[43]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[44]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[45]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[46]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[47]);
                domains[15] = temp * domains[14];

                domains[16] = autogenerated_pows[2] - FELT_1;

                let temp = autogenerated_pows[2] - autogenerated_pows[48];
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[50]);
                domains[17] = temp * domains[14];

                let temp = autogenerated_pows[2] - autogenerated_pows[27];
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[30]);
                let temp = temp * (autogenerated_pows[2] - autogenerated_pows[33]);
                domains[18] = temp * domains[15];

                domains[19] = autogenerated_pows[1] - FELT_1;
                domains[20] = autogenerated_pows[1] - autogenerated_pows[20];
                domains[21] = autogenerated_pows[1] - autogenerated_pows[50];
                domains[22] = autogenerated_pows[0] - autogenerated_pows[19];
                domains[23] = autogenerated_pows[0] - FELT_1;
                domains[24] = self.point - autogenerated_pows[18];
                domains[25] = self.point - FELT_1;
                domains[26] = self.point - autogenerated_pows[17];
                domains[27] = self.point - autogenerated_pows[16];
                domains[28] = self.point - autogenerated_pows[15];
                domains[29] = self.point - autogenerated_pows[14];
                domains[30] = self.point - autogenerated_pows[13];

                self.phase = EvalCompositionPolynomialInnerPhase::ComputeConstraints;
                vec![]
            }

            EvalCompositionPolynomialInnerPhase::ComputeConstraints => {
                // Get all references at once to avoid borrowing conflicts
                let (proof, domains, mask_values, global_values, constraint_coefficients) =
                    stack.get_proof_data_references::<StarkProof>();

                println!("global_values {:?}", global_values);
                println!("domains {:?}", domains);

                // Compute intermediate values using column_row! macro directly
                let cpu_decode_opcode_range_check_bit_0 = column_row!(mask_values, 0, 0)
                    - (column_row!(mask_values, 0, 1) + column_row!(mask_values, 0, 1));

                let cpu_decode_opcode_range_check_bit_2 = column_row!(mask_values, 0, 2)
                    - (column_row!(mask_values, 0, 3) + column_row!(mask_values, 0, 3));

                let cpu_decode_opcode_range_check_bit_4 = column_row!(mask_values, 0, 4)
                    - (column_row!(mask_values, 0, 5) + column_row!(mask_values, 0, 5));

                let cpu_decode_opcode_range_check_bit_3 = column_row!(mask_values, 0, 3)
                    - (column_row!(mask_values, 0, 4) + column_row!(mask_values, 0, 4));

                let cpu_decode_flag_op1_base_op0_0 = FELT_1
                    - (cpu_decode_opcode_range_check_bit_2
                        + cpu_decode_opcode_range_check_bit_4
                        + cpu_decode_opcode_range_check_bit_3);

                let cpu_decode_opcode_range_check_bit_5 = column_row!(mask_values, 0, 5)
                    - (column_row!(mask_values, 0, 6) + column_row!(mask_values, 0, 6));

                let cpu_decode_opcode_range_check_bit_6 = column_row!(mask_values, 0, 6)
                    - (column_row!(mask_values, 0, 7) + column_row!(mask_values, 0, 7));

                let cpu_decode_opcode_range_check_bit_9 = column_row!(mask_values, 0, 9)
                    - (column_row!(mask_values, 0, 10) + column_row!(mask_values, 0, 10));

                let cpu_decode_flag_res_op1_0 = FELT_1
                    - (cpu_decode_opcode_range_check_bit_5
                        + cpu_decode_opcode_range_check_bit_6
                        + cpu_decode_opcode_range_check_bit_9);

                let cpu_decode_opcode_range_check_bit_7 = column_row!(mask_values, 0, 7)
                    - (column_row!(mask_values, 0, 8) + column_row!(mask_values, 0, 8));

                let cpu_decode_opcode_range_check_bit_8 = column_row!(mask_values, 0, 8)
                    - (column_row!(mask_values, 0, 9) + column_row!(mask_values, 0, 9));

                let cpu_decode_flag_pc_update_regular_0 = FELT_1
                    - (cpu_decode_opcode_range_check_bit_7
                        + cpu_decode_opcode_range_check_bit_8
                        + cpu_decode_opcode_range_check_bit_9);

                let cpu_decode_opcode_range_check_bit_12 = column_row!(mask_values, 0, 12)
                    - (column_row!(mask_values, 0, 13) + column_row!(mask_values, 0, 13));

                let cpu_decode_opcode_range_check_bit_13 = column_row!(mask_values, 0, 13)
                    - (column_row!(mask_values, 0, 14) + column_row!(mask_values, 0, 14));

                let cpu_decode_fp_update_regular_0 = FELT_1
                    - (cpu_decode_opcode_range_check_bit_12 + cpu_decode_opcode_range_check_bit_13);

                let cpu_decode_opcode_range_check_bit_1 = column_row!(mask_values, 0, 1)
                    - (column_row!(mask_values, 0, 2) + column_row!(mask_values, 0, 2));

                let npc_reg_0 =
                    column_row!(mask_values, 1, 0) + cpu_decode_opcode_range_check_bit_2 + FELT_1;

                let cpu_decode_opcode_range_check_bit_10 = column_row!(mask_values, 0, 10)
                    - (column_row!(mask_values, 0, 11) + column_row!(mask_values, 0, 11));

                let cpu_decode_opcode_range_check_bit_11 = column_row!(mask_values, 0, 11)
                    - (column_row!(mask_values, 0, 12) + column_row!(mask_values, 0, 12));

                let cpu_decode_opcode_range_check_bit_14 = column_row!(mask_values, 0, 14)
                    - (column_row!(mask_values, 0, 15) + column_row!(mask_values, 0, 15));

                let memory_address_diff_0 =
                    column_row!(mask_values, 2, 2) - column_row!(mask_values, 2, 0);

                let range_check16_diff_0 =
                    column_row!(mask_values, 4, 6) - column_row!(mask_values, 4, 2);

                let pedersen_hash0_ec_subset_sum_bit_0 = column_row!(mask_values, 4, 3)
                    - (column_row!(mask_values, 4, 11) + column_row!(mask_values, 4, 11));

                let pedersen_hash0_ec_subset_sum_bit_neg_0 =
                    FELT_1 - pedersen_hash0_ec_subset_sum_bit_0;

                let range_check_builtin_value0_0 = column_row!(mask_values, 4, 12);

                let range_check_builtin_value1_0 = range_check_builtin_value0_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 44);

                let range_check_builtin_value2_0 = range_check_builtin_value1_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 76);

                let range_check_builtin_value3_0 = range_check_builtin_value2_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 108);

                let range_check_builtin_value4_0 = range_check_builtin_value3_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 140);

                let range_check_builtin_value5_0 = range_check_builtin_value4_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 172);

                let range_check_builtin_value6_0 = range_check_builtin_value5_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 204);

                let range_check_builtin_value7_0 = range_check_builtin_value6_0
                    * global_values.offset_size
                    + column_row!(mask_values, 4, 236);

                let bitwise_sum_var_0_0 = column_row!(mask_values, 3, 0)
                    + column_row!(mask_values, 3, 4) * FELT_2
                    + column_row!(mask_values, 3, 8) * FELT_4
                    + column_row!(mask_values, 3, 12) * FELT_8
                    + column_row!(mask_values, 3, 16) * FELT_18446744073709551616
                    + column_row!(mask_values, 3, 20) * FELT_36893488147419103232
                    + column_row!(mask_values, 3, 24) * FELT_73786976294838206464
                    + column_row!(mask_values, 3, 28) * FELT_147573952589676412928;

                let bitwise_sum_var_8_0 = column_row!(mask_values, 3, 32)
                    * FELT_340282366920938463463374607431768211456
                    + column_row!(mask_values, 3, 36)
                        * FELT_680564733841876926926749214863536422912
                    + column_row!(mask_values, 3, 40)
                        * FELT_1361129467683753853853498429727072845824
                    + column_row!(mask_values, 3, 44)
                        * FELT_2722258935367507707706996859454145691648
                    + column_row!(mask_values, 3, 48)
                        * FELT_6277101735386680763835789423207666416102355444464034512896
                    + column_row!(mask_values, 3, 52)
                        * FELT_12554203470773361527671578846415332832204710888928069025792
                    + column_row!(mask_values, 3, 56)
                        * FELT_25108406941546723055343157692830665664409421777856138051584
                    + column_row!(mask_values, 3, 60)
                        * FELT_50216813883093446110686315385661331328818843555712276103168;

                let poseidon_poseidon_full_rounds_state0_cubed_0 =
                    column_row!(mask_values, 5, 9) * column_row!(mask_values, 5, 105);

                let poseidon_poseidon_full_rounds_state1_cubed_0 =
                    column_row!(mask_values, 5, 73) * column_row!(mask_values, 5, 25);

                let poseidon_poseidon_full_rounds_state2_cubed_0 =
                    column_row!(mask_values, 5, 41) * column_row!(mask_values, 5, 89);

                let poseidon_poseidon_full_rounds_state0_cubed_7 =
                    column_row!(mask_values, 5, 905) * column_row!(mask_values, 5, 1001);

                let poseidon_poseidon_full_rounds_state1_cubed_7 =
                    column_row!(mask_values, 5, 969) * column_row!(mask_values, 5, 921);

                let poseidon_poseidon_full_rounds_state2_cubed_7 =
                    column_row!(mask_values, 5, 937) * column_row!(mask_values, 5, 985);

                let poseidon_poseidon_full_rounds_state0_cubed_3 =
                    column_row!(mask_values, 5, 393) * column_row!(mask_values, 5, 489);

                let poseidon_poseidon_full_rounds_state1_cubed_3 =
                    column_row!(mask_values, 5, 457) * column_row!(mask_values, 5, 409);

                let poseidon_poseidon_full_rounds_state2_cubed_3 =
                    column_row!(mask_values, 5, 425) * column_row!(mask_values, 5, 473);

                let poseidon_poseidon_partial_rounds_state0_cubed_0 =
                    column_row!(mask_values, 5, 6) * column_row!(mask_values, 5, 14);

                let poseidon_poseidon_partial_rounds_state0_cubed_1 =
                    column_row!(mask_values, 5, 22) * column_row!(mask_values, 5, 30);

                let poseidon_poseidon_partial_rounds_state0_cubed_2 =
                    column_row!(mask_values, 5, 38) * column_row!(mask_values, 5, 46);

                let poseidon_poseidon_partial_rounds_state1_cubed_0 =
                    column_row!(mask_values, 5, 1) * column_row!(mask_values, 5, 17);

                let poseidon_poseidon_partial_rounds_state1_cubed_1 =
                    column_row!(mask_values, 5, 33) * column_row!(mask_values, 5, 49);

                let poseidon_poseidon_partial_rounds_state1_cubed_2 =
                    column_row!(mask_values, 5, 65) * column_row!(mask_values, 5, 81);

                let poseidon_poseidon_partial_rounds_state1_cubed_19 =
                    column_row!(mask_values, 5, 609) * column_row!(mask_values, 5, 625);

                let poseidon_poseidon_partial_rounds_state1_cubed_20 =
                    column_row!(mask_values, 5, 641) * column_row!(mask_values, 5, 657);

                let poseidon_poseidon_partial_rounds_state1_cubed_21 =
                    column_row!(mask_values, 5, 673) * column_row!(mask_values, 5, 689);

                // Sum constraints.
                let mut total_sum = FELT_0;

                // Constraint: cpu/decode/opcode_range_check/bit.
                let value = (cpu_decode_opcode_range_check_bit_0
                    * cpu_decode_opcode_range_check_bit_0
                    - cpu_decode_opcode_range_check_bit_0)
                    * domains[4].field_div(&felt_nonzero!(domains[0]));
                total_sum = total_sum + constraint_coefficients[0] * value;
                // Constraint: cpu/decode/opcode_range_check/zero.
                let value = (column_row!(mask_values, 0, 0)).field_div(&felt_nonzero!(domains[4]));
                total_sum = total_sum + constraint_coefficients[1] * value;
                // Constraint: cpu/decode/opcode_range_check_input.
                let value = (column_row!(mask_values, 1, 1)
                    - (((column_row!(mask_values, 0, 0) * global_values.offset_size
                        + column_row!(mask_values, 4, 4))
                        * global_values.offset_size
                        + column_row!(mask_values, 4, 8))
                        * global_values.offset_size
                        + column_row!(mask_values, 4, 0)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[2] * value;

                // Constraint: cpu/decode/flag_op1_base_op0_bit.
                let value = (cpu_decode_flag_op1_base_op0_0 * cpu_decode_flag_op1_base_op0_0
                    - cpu_decode_flag_op1_base_op0_0)
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[3] * value;

                // Constraint: cpu/decode/flag_res_op1_bit.
                let value = (cpu_decode_flag_res_op1_0 * cpu_decode_flag_res_op1_0
                    - cpu_decode_flag_res_op1_0)
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[4] * value;

                // Constraint: cpu/decode/flag_pc_update_regular_bit.
                let value = (cpu_decode_flag_pc_update_regular_0
                    * cpu_decode_flag_pc_update_regular_0
                    - cpu_decode_flag_pc_update_regular_0)
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[5] * value;

                // Constraint: cpu/decode/fp_update_regular_bit.
                let value = (cpu_decode_fp_update_regular_0 * cpu_decode_fp_update_regular_0
                    - cpu_decode_fp_update_regular_0)
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[6] * value;

                // Constraint: cpu/operands/mem_dst_addr.
                let value = (column_row!(mask_values, 1, 8) + global_values.half_offset_size
                    - (cpu_decode_opcode_range_check_bit_0 * column_row!(mask_values, 5, 8)
                        + (FELT_1 - cpu_decode_opcode_range_check_bit_0)
                            * column_row!(mask_values, 5, 0)
                        + column_row!(mask_values, 4, 0)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[7] * value;

                // Constraint: cpu/operands/mem0_addr.
                let value = (column_row!(mask_values, 1, 4) + global_values.half_offset_size
                    - (cpu_decode_opcode_range_check_bit_1 * column_row!(mask_values, 5, 8)
                        + (FELT_1 - cpu_decode_opcode_range_check_bit_1)
                            * column_row!(mask_values, 5, 0)
                        + column_row!(mask_values, 4, 8)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[8] * value;

                // Constraint: cpu/operands/mem1_addr.
                let value = (column_row!(mask_values, 1, 12) + global_values.half_offset_size
                    - (cpu_decode_opcode_range_check_bit_2 * column_row!(mask_values, 1, 0)
                        + cpu_decode_opcode_range_check_bit_4 * column_row!(mask_values, 5, 0)
                        + cpu_decode_opcode_range_check_bit_3 * column_row!(mask_values, 5, 8)
                        + cpu_decode_flag_op1_base_op0_0 * column_row!(mask_values, 1, 5)
                        + column_row!(mask_values, 4, 4)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[9] * value;

                // Constraint: cpu/operands/ops_mul.
                let value = (column_row!(mask_values, 5, 4)
                    - column_row!(mask_values, 1, 5) * column_row!(mask_values, 1, 13))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[10] * value;

                // Constraint: cpu/operands/res.
                let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9)
                    * column_row!(mask_values, 5, 12)
                    - (cpu_decode_opcode_range_check_bit_5
                        * (column_row!(mask_values, 1, 5) + column_row!(mask_values, 1, 13))
                        + cpu_decode_opcode_range_check_bit_6 * column_row!(mask_values, 5, 4)
                        + cpu_decode_flag_res_op1_0 * column_row!(mask_values, 1, 13)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[11] * value;

                // Constraint: cpu/update_registers/update_pc/tmp0.
                let value = (column_row!(mask_values, 5, 2)
                    - cpu_decode_opcode_range_check_bit_9 * column_row!(mask_values, 1, 9))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[12] * value;

                // Constraint: cpu/update_registers/update_pc/tmp1.
                let value = (column_row!(mask_values, 5, 10)
                    - column_row!(mask_values, 5, 2) * column_row!(mask_values, 5, 12))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[13] * value;

                // Constraint: cpu/update_registers/update_pc/pc_cond_negative.
                let value = ((FELT_1 - cpu_decode_opcode_range_check_bit_9)
                    * column_row!(mask_values, 1, 16)
                    + column_row!(mask_values, 5, 2)
                        * (column_row!(mask_values, 1, 16)
                            - (column_row!(mask_values, 1, 0) + column_row!(mask_values, 1, 13)))
                    - (cpu_decode_flag_pc_update_regular_0 * npc_reg_0
                        + cpu_decode_opcode_range_check_bit_7 * column_row!(mask_values, 5, 12)
                        + cpu_decode_opcode_range_check_bit_8
                            * (column_row!(mask_values, 1, 0) + column_row!(mask_values, 5, 12))))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[14] * value;

                // Constraint: cpu/update_registers/update_pc/pc_cond_positive.
                let value = ((column_row!(mask_values, 5, 10)
                    - cpu_decode_opcode_range_check_bit_9)
                    * (column_row!(mask_values, 1, 16) - npc_reg_0))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[15] * value;

                // Constraint: cpu/update_registers/update_ap/ap_update.
                let value = (column_row!(mask_values, 5, 16)
                    - (column_row!(mask_values, 5, 0)
                        + cpu_decode_opcode_range_check_bit_10 * column_row!(mask_values, 5, 12)
                        + cpu_decode_opcode_range_check_bit_11
                        + cpu_decode_opcode_range_check_bit_12 * FELT_2))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[16] * value;

                // Constraint: cpu/update_registers/update_fp/fp_update.
                let value = (column_row!(mask_values, 5, 24)
                    - (cpu_decode_fp_update_regular_0 * column_row!(mask_values, 5, 8)
                        + cpu_decode_opcode_range_check_bit_13 * column_row!(mask_values, 1, 9)
                        + cpu_decode_opcode_range_check_bit_12
                            * (column_row!(mask_values, 5, 0) + FELT_2)))
                    * domains[24].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[17] * value;

                // Constraint: cpu/opcodes/call/push_fp.
                let value = (cpu_decode_opcode_range_check_bit_12
                    * (column_row!(mask_values, 1, 9) - column_row!(mask_values, 5, 8)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[18] * value;

                // Constraint: cpu/opcodes/call/push_pc.
                let value = (cpu_decode_opcode_range_check_bit_12
                    * (column_row!(mask_values, 1, 5)
                        - (column_row!(mask_values, 1, 0)
                            + cpu_decode_opcode_range_check_bit_2
                            + FELT_1)))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[19] * value;

                // Constraint: cpu/opcodes/call/off0.
                let value = (cpu_decode_opcode_range_check_bit_12
                    * (column_row!(mask_values, 4, 0) - global_values.half_offset_size))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[20] * value;

                // Constraint: cpu/opcodes/call/off1.
                let value = (cpu_decode_opcode_range_check_bit_12
                    * (column_row!(mask_values, 4, 8) - (global_values.half_offset_size + FELT_1)))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[21] * value;

                // Constraint: cpu/opcodes/call/flags.
                let value = (cpu_decode_opcode_range_check_bit_12
                    * (cpu_decode_opcode_range_check_bit_12
                        + cpu_decode_opcode_range_check_bit_12
                        + FELT_1
                        + FELT_1
                        - (cpu_decode_opcode_range_check_bit_0
                            + cpu_decode_opcode_range_check_bit_1
                            + FELT_4)))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[22] * value;

                // Constraint: cpu/opcodes/ret/off0.
                let value = (cpu_decode_opcode_range_check_bit_13
                    * (column_row!(mask_values, 4, 0) + FELT_2 - global_values.half_offset_size))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[23] * value;

                // Constraint: cpu/opcodes/ret/off2.
                let value = (cpu_decode_opcode_range_check_bit_13
                    * (column_row!(mask_values, 4, 4) + FELT_1 - global_values.half_offset_size))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[24] * value;

                // Constraint: cpu/opcodes/ret/flags.
                let value = (cpu_decode_opcode_range_check_bit_13
                    * (cpu_decode_opcode_range_check_bit_7
                        + cpu_decode_opcode_range_check_bit_0
                        + cpu_decode_opcode_range_check_bit_3
                        + cpu_decode_flag_res_op1_0
                        - FELT_4))
                    .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[25] * value;

                // Constraint: cpu/opcodes/assert_eq/assert_eq.
                let value = (cpu_decode_opcode_range_check_bit_14
                    * (column_row!(mask_values, 1, 9) - column_row!(mask_values, 5, 12)))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[26] * value;

                // Constraint: initial_ap.
                let value = (column_row!(mask_values, 5, 0) - global_values.initial_ap)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[27] * value;

                // Constraint: initial_fp.
                let value = (column_row!(mask_values, 5, 8) - global_values.initial_ap)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[28] * value;

                // Constraint: initial_pc.
                let value = (column_row!(mask_values, 1, 0) - global_values.initial_pc)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[29] * value;

                // Constraint: final_ap.
                let value = (column_row!(mask_values, 5, 0) - global_values.final_ap)
                    .field_div(&felt_nonzero!(domains[24]));
                total_sum = total_sum + constraint_coefficients[30] * value;

                // Constraint: final_fp.
                let value = (column_row!(mask_values, 5, 8) - global_values.initial_ap)
                    .field_div(&felt_nonzero!(domains[24]));
                total_sum = total_sum + constraint_coefficients[31] * value;

                // Constraint: final_pc.
                let value = (column_row!(mask_values, 1, 0) - global_values.final_pc)
                    .field_div(&felt_nonzero!(domains[24]));
                total_sum = total_sum + constraint_coefficients[32] * value;

                // Constraint: memory/multi_column_perm/perm/init0.
                let value = ((global_values.memory_multi_column_perm_perm_interaction_elm
                    - (column_row!(mask_values, 2, 0)
                        + global_values.memory_multi_column_perm_hash_interaction_elm0
                            * column_row!(mask_values, 2, 1)))
                    * column_row!(mask_values, 6, 0)
                    + column_row!(mask_values, 1, 0)
                    + global_values.memory_multi_column_perm_hash_interaction_elm0
                        * column_row!(mask_values, 1, 1)
                    - global_values.memory_multi_column_perm_perm_interaction_elm)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[33] * value;

                // Constraint: memory/multi_column_perm/perm/step0.
                let value = ((global_values.memory_multi_column_perm_perm_interaction_elm
                    - (column_row!(mask_values, 2, 2)
                        + global_values.memory_multi_column_perm_hash_interaction_elm0
                            * column_row!(mask_values, 2, 3)))
                    * column_row!(mask_values, 6, 2)
                    - (global_values.memory_multi_column_perm_perm_interaction_elm
                        - (column_row!(mask_values, 1, 2)
                            + global_values.memory_multi_column_perm_hash_interaction_elm0
                                * column_row!(mask_values, 1, 3)))
                        * column_row!(mask_values, 6, 0))
                    * domains[26].field_div(&felt_nonzero!(domains[1]));
                total_sum = total_sum + constraint_coefficients[34] * value;

                // Constraint: memory/multi_column_perm/perm/last.
                let value = (column_row!(mask_values, 6, 0)
                    - global_values.memory_multi_column_perm_perm_public_memory_prod)
                    .field_div(&felt_nonzero!(domains[26]));
                total_sum = total_sum + constraint_coefficients[35] * value;

                // Constraint: memory/diff_is_bit.
                let value = (memory_address_diff_0 * memory_address_diff_0 - memory_address_diff_0)
                    * domains[26].field_div(&felt_nonzero!(domains[1]));
                total_sum = total_sum + constraint_coefficients[36] * value;

                // Constraint: memory/is_func.
                let value = ((memory_address_diff_0 - FELT_1)
                    * (column_row!(mask_values, 2, 1) - column_row!(mask_values, 2, 3)))
                    * domains[26].field_div(&felt_nonzero!(domains[1]));
                total_sum = total_sum + constraint_coefficients[37] * value;

                // Constraint: memory/initial_addr.
                let value = (column_row!(mask_values, 2, 0) - FELT_1)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[38] * value;

                // Constraint: public_memory_addr_zero.
                let value = (column_row!(mask_values, 1, 2)).field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[39] * value;

                // Constraint: public_memory_value_zero.
                let value = (column_row!(mask_values, 1, 3)).field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[40] * value;

                // Constraint: range_check16/perm/init0.
                let value = ((global_values.range_check16_perm_interaction_elm
                    - column_row!(mask_values, 4, 2))
                    * column_row!(mask_values, 7, 1)
                    + column_row!(mask_values, 4, 0)
                    - global_values.range_check16_perm_interaction_elm)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[41] * value;

                // Constraint: range_check16/perm/step0.
                let value = ((global_values.range_check16_perm_interaction_elm
                    - column_row!(mask_values, 4, 6))
                    * column_row!(mask_values, 7, 5)
                    - (global_values.range_check16_perm_interaction_elm
                        - column_row!(mask_values, 4, 4))
                        * column_row!(mask_values, 7, 1))
                    * domains[27].field_div(&felt_nonzero!(domains[2]));
                total_sum = total_sum + constraint_coefficients[42] * value;

                // Constraint: range_check16/perm/last.
                let value = (column_row!(mask_values, 7, 1)
                    - global_values.range_check16_perm_public_memory_prod)
                    .field_div(&felt_nonzero!(domains[27]));
                total_sum = total_sum + constraint_coefficients[43] * value;

                // Constraint: range_check16/diff_is_bit.
                let value = (range_check16_diff_0 * range_check16_diff_0 - range_check16_diff_0)
                    * domains[27].field_div(&felt_nonzero!(domains[2]));
                total_sum = total_sum + constraint_coefficients[44] * value;
                println!("\nrange_check16/diff_is_bit. {:?}", value);
                println!("total sum of range_check16/diff_is_bit.{:?}\n", total_sum);
                // Constraint: range_check16/minimum.
                let value = (column_row!(mask_values, 4, 2) - global_values.range_check_min)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[45] * value;

                // Constraint: range_check16/maximum.
                let value = (column_row!(mask_values, 4, 2) - global_values.range_check_max)
                    .field_div(&felt_nonzero!(domains[27]));
                total_sum = total_sum + constraint_coefficients[46] * value;

                // Constraint: diluted_check/permutation/init0.
                let value = ((global_values.diluted_check_permutation_interaction_elm
                    - column_row!(mask_values, 3, 1))
                    * column_row!(mask_values, 7, 0)
                    + column_row!(mask_values, 3, 0)
                    - global_values.diluted_check_permutation_interaction_elm)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[47] * value;

                // Constraint: diluted_check/permutation/step0.
                let value = ((global_values.diluted_check_permutation_interaction_elm
                    - column_row!(mask_values, 3, 3))
                    * column_row!(mask_values, 7, 2)
                    - (global_values.diluted_check_permutation_interaction_elm
                        - column_row!(mask_values, 3, 2))
                        * column_row!(mask_values, 7, 0))
                    * domains[26].field_div(&felt_nonzero!(domains[1]));
                total_sum = total_sum + constraint_coefficients[48] * value;

                // Constraint: diluted_check/permutation/last.
                let value = (column_row!(mask_values, 7, 0)
                    - global_values.diluted_check_permutation_public_memory_prod)
                    .field_div(&felt_nonzero!(domains[26]));
                total_sum = total_sum + constraint_coefficients[49] * value;

                // Constraint: diluted_check/init.
                let value = (column_row!(mask_values, 6, 1) - FELT_1)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[50] * value;

                // Constraint: diluted_check/first_element.
                let value = (column_row!(mask_values, 3, 1)
                    - global_values.diluted_check_first_elm)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[51] * value;

                // Constraint: diluted_check/step.
                let value = (column_row!(mask_values, 6, 3)
                    - (column_row!(mask_values, 6, 1)
                        * (FELT_1
                            + global_values.diluted_check_interaction_z
                                * (column_row!(mask_values, 3, 3)
                                    - column_row!(mask_values, 3, 1)))
                        + global_values.diluted_check_interaction_alpha
                            * (column_row!(mask_values, 3, 3) - column_row!(mask_values, 3, 1))
                            * (column_row!(mask_values, 3, 3) - column_row!(mask_values, 3, 1))))
                    * domains[26].field_div(&felt_nonzero!(domains[1]));
                total_sum = total_sum + constraint_coefficients[52] * value;

                // Constraint: diluted_check/last.
                let value = (column_row!(mask_values, 6, 1)
                    - global_values.diluted_check_final_cum_val)
                    .field_div(&felt_nonzero!(domains[26]));
                total_sum = total_sum + constraint_coefficients[53] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/last_one_is_zero.
                let value = (column_row!(mask_values, 5, 57)
                    * (column_row!(mask_values, 4, 3)
                        - (column_row!(mask_values, 4, 11) + column_row!(mask_values, 4, 11))))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[54] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones0.
                let value = (column_row!(mask_values, 5, 57)
                    * (column_row!(mask_values, 4, 11)
                        - FELT_3138550867693340381917894711603833208051177722232017256448
                            * column_row!(mask_values, 4, 1539)))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[55] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit192.
                let value = (column_row!(mask_values, 5, 57)
                    - column_row!(mask_values, 4, 2047)
                        * (column_row!(mask_values, 4, 1539)
                            - (column_row!(mask_values, 4, 1547)
                                + column_row!(mask_values, 4, 1547))))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[56] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones192.
                let value = (column_row!(mask_values, 4, 2047)
                    * (column_row!(mask_values, 4, 1547)
                        - FELT_8 * column_row!(mask_values, 4, 1571)))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[57] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/cumulative_bit196.
                let value = (column_row!(mask_values, 4, 2047)
                    - (column_row!(mask_values, 4, 2011)
                        - (column_row!(mask_values, 4, 2019) + column_row!(mask_values, 4, 2019)))
                        * (column_row!(mask_values, 4, 1571)
                            - (column_row!(mask_values, 4, 1579)
                                + column_row!(mask_values, 4, 1579))))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[58] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_unpacking/zeroes_between_ones196.
                let value = ((column_row!(mask_values, 4, 2011)
                    - (column_row!(mask_values, 4, 2019) + column_row!(mask_values, 4, 2019)))
                    * (column_row!(mask_values, 4, 1579)
                        - FELT_18014398509481984 * column_row!(mask_values, 4, 2011)))
                .field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[59] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/booleanity_test.
                let value = (pedersen_hash0_ec_subset_sum_bit_0
                    * (pedersen_hash0_ec_subset_sum_bit_0 - FELT_1))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[60] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/bit_extraction_end.
                let value = (column_row!(mask_values, 4, 3)).field_div(&felt_nonzero!(domains[21]));
                total_sum = total_sum + constraint_coefficients[61] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/zeros_tail.
                let value = (column_row!(mask_values, 4, 3)).field_div(&felt_nonzero!(domains[20]));
                total_sum = total_sum + constraint_coefficients[62] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/add_points/slope.
                let value = (pedersen_hash0_ec_subset_sum_bit_0
                    * (column_row!(mask_values, 4, 5) - global_values.pedersen_points_y)
                    - column_row!(mask_values, 4, 7)
                        * (column_row!(mask_values, 4, 1) - global_values.pedersen_points_x))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[63] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/add_points/x.
                let value = (column_row!(mask_values, 4, 7) * column_row!(mask_values, 4, 7)
                    - pedersen_hash0_ec_subset_sum_bit_0
                        * (column_row!(mask_values, 4, 1)
                            + global_values.pedersen_points_x
                            + column_row!(mask_values, 4, 9)))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[64] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/add_points/y.
                let value = (pedersen_hash0_ec_subset_sum_bit_0
                    * (column_row!(mask_values, 4, 5) + column_row!(mask_values, 4, 13))
                    - column_row!(mask_values, 4, 7)
                        * (column_row!(mask_values, 4, 1) - column_row!(mask_values, 4, 9)))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[65] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/copy_point/x.
                let value = (pedersen_hash0_ec_subset_sum_bit_neg_0
                    * (column_row!(mask_values, 4, 9) - column_row!(mask_values, 4, 1)))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[66] * value;

                // Constraint: pedersen/hash0/ec_subset_sum/copy_point/y.
                let value = (pedersen_hash0_ec_subset_sum_bit_neg_0
                    * (column_row!(mask_values, 4, 13) - column_row!(mask_values, 4, 5)))
                    * domains[20].field_div(&felt_nonzero!(domains[3]));
                total_sum = total_sum + constraint_coefficients[67] * value;

                // Constraint: pedersen/hash0/copy_point/x.
                let value = (column_row!(mask_values, 4, 2049) - column_row!(mask_values, 4, 2041))
                    * domains[22].field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[68] * value;

                // Constraint: pedersen/hash0/copy_point/y.
                let value = (column_row!(mask_values, 4, 2053) - column_row!(mask_values, 4, 2045))
                    * domains[22].field_div(&felt_nonzero!(domains[19]));
                total_sum = total_sum + constraint_coefficients[69] * value;

                // Constraint: pedersen/hash0/init/x.
                let value = (column_row!(mask_values, 4, 1) - global_values.pedersen_shift_point.x)
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[70] * value;

                // Constraint: pedersen/hash0/init/y.
                let value = (column_row!(mask_values, 4, 5) - global_values.pedersen_shift_point.y)
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[71] * value;

                // Constraint: pedersen/input0_value0.
                let value = (column_row!(mask_values, 1, 11) - column_row!(mask_values, 4, 3))
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[72] * value;

                // Constraint: pedersen/input0_addr.
                let value = (column_row!(mask_values, 1, 4106)
                    - (column_row!(mask_values, 1, 1034) + FELT_1))
                    * domains[28].field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[73] * value;

                // Constraint: pedersen/init_addr.
                let value = (column_row!(mask_values, 1, 10) - global_values.initial_pedersen_addr)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[74] * value;

                // Constraint: pedersen/input1_value0.
                let value = (column_row!(mask_values, 1, 2059) - column_row!(mask_values, 4, 2051))
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[75] * value;

                // Constraint: pedersen/input1_addr.
                let value = (column_row!(mask_values, 1, 2058)
                    - (column_row!(mask_values, 1, 10) + FELT_1))
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[76] * value;

                // Constraint: pedersen/output_value0.
                let value = (column_row!(mask_values, 1, 1035) - column_row!(mask_values, 4, 4089))
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[77] * value;

                // Constraint: pedersen/output_addr.
                let value = (column_row!(mask_values, 1, 1034)
                    - (column_row!(mask_values, 1, 2058) + FELT_1))
                    .field_div(&felt_nonzero!(domains[23]));
                total_sum = total_sum + constraint_coefficients[78] * value;

                // Constraint: range_check_builtin/value.
                let value = (range_check_builtin_value7_0 - column_row!(mask_values, 1, 139))
                    .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[79] * value;

                // Constraint: range_check_builtin/addr_step.
                let value = (column_row!(mask_values, 1, 394)
                    - (column_row!(mask_values, 1, 138) + FELT_1))
                    * domains[29].field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[80] * value;

                // Constraint: range_check_builtin/init_addr.
                let value = (column_row!(mask_values, 1, 138)
                    - global_values.initial_range_check_addr)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[81] * value;

                // Constraint: bitwise/init_var_pool_addr.
                let value = (column_row!(mask_values, 1, 42) - global_values.initial_bitwise_addr)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[82] * value;

                // Constraint: bitwise/step_var_pool_addr.
                let value = (column_row!(mask_values, 1, 106)
                    - (column_row!(mask_values, 1, 42) + FELT_1))
                    * domains[10].field_div(&felt_nonzero!(domains[7]));
                total_sum = total_sum + constraint_coefficients[83] * value;

                // Constraint: bitwise/x_or_y_addr.
                let value = (column_row!(mask_values, 1, 74)
                    - (column_row!(mask_values, 1, 234) + FELT_1))
                    .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[84] * value;

                // Constraint: bitwise/next_var_pool_addr.
                let value = (column_row!(mask_values, 1, 298)
                    - (column_row!(mask_values, 1, 74) + FELT_1))
                    * domains[29].field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[85] * value;

                // Constraint: bitwise/partition.
                let value = (bitwise_sum_var_0_0 + bitwise_sum_var_8_0
                    - column_row!(mask_values, 1, 43))
                .field_div(&felt_nonzero!(domains[7]));
                total_sum = total_sum + constraint_coefficients[86] * value;

                // Constraint: bitwise/or_is_and_plus_xor.
                let value = (column_row!(mask_values, 1, 75)
                    - (column_row!(mask_values, 1, 171) + column_row!(mask_values, 1, 235)))
                .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[87] * value;

                // Constraint: bitwise/addition_is_xor_with_and.
                let value = (column_row!(mask_values, 3, 0) + column_row!(mask_values, 3, 64)
                    - (column_row!(mask_values, 3, 192)
                        + column_row!(mask_values, 3, 128)
                        + column_row!(mask_values, 3, 128)))
                .field_div(&felt_nonzero!(domains[11]));
                total_sum = total_sum + constraint_coefficients[88] * value;

                // Constraint: bitwise/unique_unpacking192.
                let value = ((column_row!(mask_values, 3, 176) + column_row!(mask_values, 3, 240))
                    * FELT_16
                    - column_row!(mask_values, 3, 2))
                .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[89] * value;

                // Constraint: bitwise/unique_unpacking193.
                let value = ((column_row!(mask_values, 3, 180) + column_row!(mask_values, 3, 244))
                    * FELT_16
                    - column_row!(mask_values, 3, 130))
                .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[90] * value;

                // Constraint: bitwise/unique_unpacking194.
                let value = ((column_row!(mask_values, 3, 184) + column_row!(mask_values, 3, 248))
                    * FELT_16
                    - column_row!(mask_values, 3, 66))
                .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[91] * value;

                // Constraint: bitwise/unique_unpacking195.
                let value = ((column_row!(mask_values, 3, 188) + column_row!(mask_values, 3, 252))
                    * FELT_256
                    - column_row!(mask_values, 3, 194))
                .field_div(&felt_nonzero!(domains[9]));
                total_sum = total_sum + constraint_coefficients[92] * value;

                // Constraint: poseidon/param_0/init_input_output_addr.
                let value = (column_row!(mask_values, 1, 266)
                    - global_values.initial_poseidon_addr)
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[93] * value;

                // Constraint: poseidon/param_0/addr_input_output_step.
                let value = (column_row!(mask_values, 1, 778)
                    - (column_row!(mask_values, 1, 266) + FELT_3))
                    * domains[30].field_div(&felt_nonzero!(domains[12]));
                total_sum = total_sum + constraint_coefficients[94] * value;

                // Constraint: poseidon/param_1/init_input_output_addr.
                let value = (column_row!(mask_values, 1, 202)
                    - (global_values.initial_poseidon_addr + FELT_1))
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[95] * value;

                // Constraint: poseidon/param_1/addr_input_output_step.
                let value = (column_row!(mask_values, 1, 714)
                    - (column_row!(mask_values, 1, 202) + FELT_3))
                    * domains[30].field_div(&felt_nonzero!(domains[12]));
                total_sum = total_sum + constraint_coefficients[96] * value;

                // Constraint: poseidon/param_2/init_input_output_addr.
                let value = (column_row!(mask_values, 1, 458)
                    - (global_values.initial_poseidon_addr + FELT_2))
                    .field_div(&felt_nonzero!(domains[25]));
                total_sum = total_sum + constraint_coefficients[97] * value;

                // Constraint: poseidon/param_2/addr_input_output_step.
                let value = (column_row!(mask_values, 1, 970)
                    - (column_row!(mask_values, 1, 458) + FELT_3))
                    * domains[30].field_div(&felt_nonzero!(domains[12]));
                total_sum = total_sum + constraint_coefficients[98] * value;

                // Constraint: poseidon/poseidon/full_rounds_state0_squaring.
                let value = (column_row!(mask_values, 5, 9) * column_row!(mask_values, 5, 9)
                    - column_row!(mask_values, 5, 105))
                .field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[99] * value;

                // Constraint: poseidon/poseidon/full_rounds_state1_squaring.
                let value = (column_row!(mask_values, 5, 73) * column_row!(mask_values, 5, 73)
                    - column_row!(mask_values, 5, 25))
                .field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[100] * value;

                // Constraint: poseidon/poseidon/full_rounds_state2_squaring.
                let value = (column_row!(mask_values, 5, 41) * column_row!(mask_values, 5, 41)
                    - column_row!(mask_values, 5, 89))
                .field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[101] * value;

                // Constraint: poseidon/poseidon/partial_rounds_state0_squaring.
                let value = (column_row!(mask_values, 5, 6) * column_row!(mask_values, 5, 6)
                    - column_row!(mask_values, 5, 14))
                .field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[102] * value;

                // Constraint: poseidon/poseidon/partial_rounds_state1_squaring.
                let value = (column_row!(mask_values, 5, 1) * column_row!(mask_values, 5, 1)
                    - column_row!(mask_values, 5, 17))
                    * domains[15].field_div(&felt_nonzero!(domains[6]));
                total_sum = total_sum + constraint_coefficients[103] * value;

                // Constraint: poseidon/poseidon/add_first_round_key0.
                let value = (column_row!(mask_values, 1, 267)
                        + FELT_2950795762459345168613727575620414179244544320470208355568817838579231751791
                        - column_row!(mask_values, 5, 9))
                        .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[104] * value;

                // Constraint: poseidon/poseidon/add_first_round_key1.
                let value = (column_row!(mask_values, 1, 203)
                        + FELT_1587446564224215276866294500450702039420286416111469274423465069420553242820
                        - column_row!(mask_values, 5, 73))
                        .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[105] * value;

                // Constraint: poseidon/poseidon/add_first_round_key2.
                let value = (column_row!(mask_values, 1, 459)
                        + FELT_1645965921169490687904413452218868659025437693527479459426157555728339600137
                        - column_row!(mask_values, 5, 41))
                        .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[106] * value;

                // Constraint: poseidon/poseidon/full_round0.
                let value = (column_row!(mask_values, 5, 137)
                    - (poseidon_poseidon_full_rounds_state0_cubed_0
                        + poseidon_poseidon_full_rounds_state0_cubed_0
                        + poseidon_poseidon_full_rounds_state0_cubed_0
                        + poseidon_poseidon_full_rounds_state1_cubed_0
                        + poseidon_poseidon_full_rounds_state2_cubed_0
                        + global_values.poseidon_poseidon_full_round_key0))
                    * domains[13].field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[107] * value;

                // Constraint: poseidon/poseidon/full_round1.
                let value = (column_row!(mask_values, 5, 201)
                    + poseidon_poseidon_full_rounds_state1_cubed_0
                    - (poseidon_poseidon_full_rounds_state0_cubed_0
                        + poseidon_poseidon_full_rounds_state2_cubed_0
                        + global_values.poseidon_poseidon_full_round_key1))
                    * domains[13].field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[108] * value;

                // Constraint: poseidon/poseidon/full_round2.
                let value = (column_row!(mask_values, 5, 169)
                    + poseidon_poseidon_full_rounds_state2_cubed_0
                    + poseidon_poseidon_full_rounds_state2_cubed_0
                    - (poseidon_poseidon_full_rounds_state0_cubed_0
                        + poseidon_poseidon_full_rounds_state1_cubed_0
                        + global_values.poseidon_poseidon_full_round_key2))
                    * domains[13].field_div(&felt_nonzero!(domains[8]));
                total_sum = total_sum + constraint_coefficients[109] * value;

                // Constraint: poseidon/poseidon/last_full_round0.
                let value = (column_row!(mask_values, 1, 779)
                    - (poseidon_poseidon_full_rounds_state0_cubed_7
                        + poseidon_poseidon_full_rounds_state0_cubed_7
                        + poseidon_poseidon_full_rounds_state0_cubed_7
                        + poseidon_poseidon_full_rounds_state1_cubed_7
                        + poseidon_poseidon_full_rounds_state2_cubed_7))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[110] * value;

                // Constraint: poseidon/poseidon/last_full_round1.
                let value = (column_row!(mask_values, 1, 715)
                    + poseidon_poseidon_full_rounds_state1_cubed_7
                    - (poseidon_poseidon_full_rounds_state0_cubed_7
                        + poseidon_poseidon_full_rounds_state2_cubed_7))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[111] * value;

                // Constraint: poseidon/poseidon/last_full_round2.
                let value = (column_row!(mask_values, 1, 971)
                    + poseidon_poseidon_full_rounds_state2_cubed_7
                    + poseidon_poseidon_full_rounds_state2_cubed_7
                    - (poseidon_poseidon_full_rounds_state0_cubed_7
                        + poseidon_poseidon_full_rounds_state1_cubed_7))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[112] * value;

                // Constraint: poseidon/poseidon/copy_partial_rounds0_i0.
                let value = (column_row!(mask_values, 5, 982) - column_row!(mask_values, 5, 1))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[113] * value;

                // Constraint: poseidon/poseidon/copy_partial_rounds0_i1.
                let value = (column_row!(mask_values, 5, 998) - column_row!(mask_values, 5, 33))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[114] * value;

                // Constraint: poseidon/poseidon/copy_partial_rounds0_i2.
                let value = (column_row!(mask_values, 5, 1014) - column_row!(mask_values, 5, 65))
                    .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[115] * value;

                // Constraint: poseidon/poseidon/margin_full_to_partial0.
                let value = (column_row!(mask_values, 5, 6)
                        + poseidon_poseidon_full_rounds_state2_cubed_3
                        + poseidon_poseidon_full_rounds_state2_cubed_3
                        - (poseidon_poseidon_full_rounds_state0_cubed_3
                            + poseidon_poseidon_full_rounds_state1_cubed_3
                            + FELT_2121140748740143694053732746913428481442990369183417228688865837805149503386))
                .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[116] * value;

                // Constraint: poseidon/poseidon/margin_full_to_partial1.
                let value = (column_row!(mask_values, 5, 22)
                        - (FELT_3618502788666131213697322783095070105623107215331596699973092056135872020477
                            * poseidon_poseidon_full_rounds_state1_cubed_3
                            + FELT_10 * poseidon_poseidon_full_rounds_state2_cubed_3
                    + FELT_4 * column_row!(mask_values, 5, 6)
                            + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479
                                * poseidon_poseidon_partial_rounds_state0_cubed_0
                            + FELT_2006642341318481906727563724340978325665491359415674592697055778067937914672))
                .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[117] * value;

                // Constraint: poseidon/poseidon/margin_full_to_partial2.
                let value = (column_row!(mask_values, 5, 38)
                        - (FELT_8 * poseidon_poseidon_full_rounds_state2_cubed_3
                    + FELT_4 * column_row!(mask_values, 5, 6)
                            + FELT_6 * poseidon_poseidon_partial_rounds_state0_cubed_0
                    + column_row!(mask_values, 5, 22)
                    + column_row!(mask_values, 5, 22)
                            + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479
                                * poseidon_poseidon_partial_rounds_state0_cubed_1
                            + FELT_427751140904099001132521606468025610873158555767197326325930641757709538586))
                .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[118] * value;

                // Constraint: poseidon/poseidon/partial_round0.
                let value = (column_row!(mask_values, 5, 54)
                        - (FELT_8 * poseidon_poseidon_partial_rounds_state0_cubed_0
                    + FELT_4 * column_row!(mask_values, 5, 22)
                            + FELT_6 * poseidon_poseidon_partial_rounds_state0_cubed_1
                    + column_row!(mask_values, 5, 38)
                    + column_row!(mask_values, 5, 38)
                            + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479
                                * poseidon_poseidon_partial_rounds_state0_cubed_2
                            + global_values.poseidon_poseidon_partial_round_key0))
                * domains[17].field_div(&felt_nonzero!(domains[5]));
                total_sum = total_sum + constraint_coefficients[119] * value;

                // Constraint: poseidon/poseidon/partial_round1.
                let value = (column_row!(mask_values, 5, 97)
                        - (FELT_8 * poseidon_poseidon_partial_rounds_state1_cubed_0
                    + FELT_4 * column_row!(mask_values, 5, 33)
                            + FELT_6 * poseidon_poseidon_partial_rounds_state1_cubed_1
                    + column_row!(mask_values, 5, 65)
                    + column_row!(mask_values, 5, 65)
                            + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479
                                * poseidon_poseidon_partial_rounds_state1_cubed_2
                            + global_values.poseidon_poseidon_partial_round_key1))
                * domains[18].field_div(&felt_nonzero!(domains[6]));
                total_sum = total_sum + constraint_coefficients[120] * value;

                // Constraint: poseidon/poseidon/margin_partial_to_full0.
                let value = (column_row!(mask_values, 5, 521)
                        - (FELT_16 * poseidon_poseidon_partial_rounds_state1_cubed_19
                    + FELT_8 * column_row!(mask_values, 5, 641)
                            + FELT_16 * poseidon_poseidon_partial_rounds_state1_cubed_20
                    + FELT_6 * column_row!(mask_values, 5, 673)
                            + poseidon_poseidon_partial_rounds_state1_cubed_21
                            + FELT_560279373700919169769089400651532183647886248799764942664266404650165812023))
                .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[121] * value;

                // Constraint: poseidon/poseidon/margin_partial_to_full1.
                let value = (column_row!(mask_values, 5, 585)
                        - (FELT_4 * poseidon_poseidon_partial_rounds_state1_cubed_20
                    + column_row!(mask_values, 5, 673)
                    + column_row!(mask_values, 5, 673)
                            + poseidon_poseidon_partial_rounds_state1_cubed_21
                            + FELT_1401754474293352309994371631695783042590401941592571735921592823982231996415))
                .field_div(&felt_nonzero!(domains[16]));
                total_sum = total_sum + constraint_coefficients[122] * value;

                // Constraint: poseidon/poseidon/margin_partial_to_full2.
                let value = (column_row!(mask_values, 5, 553)
                        - (FELT_8 * poseidon_poseidon_partial_rounds_state1_cubed_19
                    + FELT_4 * column_row!(mask_values, 5, 641)
                            + FELT_6 * poseidon_poseidon_partial_rounds_state1_cubed_20
                    + column_row!(mask_values, 5, 673)
                    + column_row!(mask_values, 5, 673)
                            + FELT_3618502788666131213697322783095070105623107215331596699973092056135872020479
                                * poseidon_poseidon_partial_rounds_state1_cubed_21
                            + FELT_1246177936547655338400308396717835700699368047388302793172818304164989556526))
                .field_div(&felt_nonzero!(domains[16]));

                total_sum = total_sum + constraint_coefficients[123] * value;

                stack.push_front(&total_sum.to_bytes_be()).unwrap();
                self.total_sum = total_sum;

                self.phase = EvalCompositionPolynomialInnerPhase::Done;

                vec![]
            }

            EvalCompositionPolynomialInnerPhase::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == EvalCompositionPolynomialInnerPhase::Done
    }
}
