//! Benchmarking BN254 operations

#![no_main]
sp1_zkvm::entrypoint!(main);

use bn::{miller_loop_batch, pairing, AffineG1, AffineG2, Fq, Fq2, Fr, Group, Gt, G1, G2};
use revm_precompile::bn128;

fn read_g1() -> G1 {
    decode_g1(&read_hex())
}

fn read_g2() -> G2 {
    decode_g2(&read_hex())
}

fn read_fr() -> Fr {
    Fr::from_slice(&read_hex()).unwrap()
}

fn read_hex() -> Vec<u8> {
    let hex_data = sp1_zkvm::io::read::<String>();
    hex::decode(&hex_data).unwrap()
}

fn write_g1(g1: G1) {
    write_hex(&encode_g1(g1))
}

fn write_g2(g2: G2) {
    write_hex(&encode_g2(g2))
}

fn write_hex(data: &[u8]) {
    sp1_zkvm::io::commit(&hex::encode(data));
}

#[sp1_derive::cycle_tracker]
fn decode_g1(data: &[u8]) -> G1 {
    AffineG1::new(
        Fq::from_slice(&data[..32]).unwrap(),
        Fq::from_slice(&data[32..]).unwrap(),
    )
    .unwrap()
    .into()
}

#[sp1_derive::cycle_tracker]
fn decode_g2(data: &[u8]) -> G2 {
    AffineG2::new(
        Fq2::new(
            Fq::from_slice(&data[..32]).unwrap(),
            Fq::from_slice(&data[32..64]).unwrap(),
        ),
        Fq2::new(
            Fq::from_slice(&data[64..96]).unwrap(),
            Fq::from_slice(&data[96..]).unwrap(),
        ),
    )
    .unwrap()
    .into()
}

#[sp1_derive::cycle_tracker]
fn encode_g1(g1: G1) -> Vec<u8> {
    let g1 = AffineG1::from_jacobian(g1).unwrap();
    let mut output = [0u8; 64];
    g1.x().to_big_endian(&mut output[..32]).unwrap();
    g1.y().to_big_endian(&mut output[32..]).unwrap();
    output.to_vec()
}

#[sp1_derive::cycle_tracker]
fn encode_g2(g2: G2) -> Vec<u8> {
    let g2 = AffineG2::from_jacobian(g2).unwrap();
    let mut output = [0u8; 128];
    g2.x().real().to_big_endian(&mut output[..32]).unwrap();
    g2.x()
        .imaginary()
        .to_big_endian(&mut output[32..64])
        .unwrap();
    g2.y().real().to_big_endian(&mut output[64..96]).unwrap();
    g2.y().imaginary().to_big_endian(&mut output[96..]).unwrap();
    output.to_vec()
}

#[sp1_derive::cycle_tracker]
fn sum_g1(a: G1, b: G1) -> G1 {
    a + b
}

#[sp1_derive::cycle_tracker]
fn sum_g2(a: G2, b: G2) -> G2 {
    a + b
}

#[sp1_derive::cycle_tracker]
fn mul_g1(point: G1, scalar: Fr) -> G1 {
    point * scalar
}

#[sp1_derive::cycle_tracker]
fn mul_g2(point: G2, scalar: Fr) -> G2 {
    point * scalar
}

#[sp1_derive::cycle_tracker]
fn miller_loop(g1: G1, g2: G2) -> Gt {
    miller_loop_batch(&[(g2, g1)]).unwrap()
}

#[sp1_derive::cycle_tracker]
fn final_exp(gt: Gt) -> Gt {
    gt.final_exponentiation().unwrap()
}

#[sp1_derive::cycle_tracker]
fn revm_alt_bn128_add(input: &[u8]) -> Vec<u8> {
    let (_, res) = bn128::run_add(input, 500, 500).unwrap();
    res.to_vec()
}

#[sp1_derive::cycle_tracker]
fn revm_alt_bn128_mul(input: &[u8]) -> Vec<u8> {
    let (_, res) = bn128::run_mul(input, 40_000, 40_000).unwrap();
    res.to_vec()
}

#[sp1_derive::cycle_tracker]
fn revm_alt_bn128_pair(input: &[u8]) -> Vec<u8> {
    let (_, res) = bn128::run_pair(input, 80_000, 100_000, 260_000).unwrap();
    res.to_vec()
}

pub fn main() {
    let g1_a = read_g1();
    let g1_b = read_g1();
    let g1_c = read_g1();
    let fr_d1 = read_fr();

    let g2_a = read_g2();
    let g2_b = read_g2();
    let g2_c = read_g2();
    let fr_d2 = read_fr();

    let add_in = read_hex();
    let mul_in = read_hex();
    let pair_in = read_hex();

    let g1_sum = sum_g1(g1_a, g1_b);
    let g1_mul = mul_g1(g1_c, fr_d1);
    let g2_sum = sum_g2(g2_a, g2_b);
    let g2_mul = mul_g2(g2_c, fr_d2);

    // No outputs
    let fqt_ml = miller_loop(g1_a, g2_b);
    let fqt_fe = final_exp(fqt_ml.clone());

    let add_out = revm_alt_bn128_add(&add_in);
    let mul_out = revm_alt_bn128_mul(&mul_in);
    let pair_out = revm_alt_bn128_pair(&pair_in);

    write_g1(g1_sum);
    write_g1(g1_mul);
    write_g2(g2_sum);
    write_g2(g2_mul);

    write_hex(&add_out);
    write_hex(&mul_out);
    write_hex(&pair_out);
}
