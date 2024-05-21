use sp1_sdk::{ProverClient, SP1Stdin, utils};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

pub const G1_A: &str = "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266";
pub const G1_B: &str = "07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7";
pub const G1_C: &str = "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204";
pub const FR_D1: &str = "00000000000000000000000000000000000000000000000011138ce750fa15c2";
pub const G1_A_SUM_B: &str = "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915";
pub const G1_C_MUL_D: &str = "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc";

pub const G2_A: &str = "1ecfd2dff2aad18798b64bdb0c2b50c9d73e6c05619e04cbf5b448fd987268800e16c8d96362720af0916592be1b839a26f5e6b710f3ede0d8840d9a70eaf97f2aa778acda9e7d4925c60ad84c12fb3b4f2b9539d5699934b0e6fdd10cc2c0e11e8f2c1f441fed039bb46d6bfb91236cf7ba240c75080cedbe40e049c46b26be";
pub const G2_B: &str = "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c212c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b";
pub const G2_C: &str = "1ecfd2dff2aad18798b64bdb0c2b50c9d73e6c05619e04cbf5b448fd987268800e16c8d96362720af0916592be1b839a26f5e6b710f3ede0d8840d9a70eaf97f2aa778acda9e7d4925c60ad84c12fb3b4f2b9539d5699934b0e6fdd10cc2c0e11e8f2c1f441fed039bb46d6bfb91236cf7ba240c75080cedbe40e049c46b26be";
pub const FR_D2: &str = "03ccf842d4f12075ac9acbfc7d467e30736b9bd5adc2ac0aa21a9708b9cbdb8e";
pub const G2_A_SUM_B: &str = "028ed67dc28dc89a3af1a0dc9d212f71349244df651c68c619885b3e30a392e315f7967dae56ea5434d6918f1bdf261e7f76241baf6d90f1c7aa6345defdd5740a10114ead33e3d235f03e90003f5dd30f87ce1874ec45b06fa3d02cffc83c541bafecee364a3c2b6b1d6cb7eae1c6886f667e3d7e905d6a14a1fe6ee070f63c";
pub const G2_C_MUL_D: &str = "17093252104cabf33e3807a6da5825f8c3c21a41b6aef9004576b4a11eda6bdc09c7727ad2e8cec21dab67c6420d47aab06c5580b736a5200c43e33b35b159ee2d446e0f6750e69b9c869daeb261c8d9051c0b7e7c29263c9f50b3a8cc7f036a2d5f047d94c986a41aac2b39c2863e52e2ed84ba6c9aef0b9b31054d07fdde08";

fn main() {
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();
    stdin.write(&G1_A);
    stdin.write(&G1_B);
    stdin.write(&G1_C);
    stdin.write(&FR_D1);

    stdin.write(&G2_A);
    stdin.write(&G2_B);
    stdin.write(&G2_C);
    stdin.write(&FR_D2);

    let client = ProverClient::new();
    let (pk, _) = client.setup(ELF);
    let mut proof = client.prove_compressed(&pk, stdin).expect("proving failed");

    let g1_a_sum_b = proof.public_values.read::<String>();
    let g1_c_mul_d = proof.public_values.read::<String>();
    let g2_a_sum_b = proof.public_values.read::<String>();
    let g2_c_mul_d = proof.public_values.read::<String>();
  
    assert_eq!(&G1_A_SUM_B, &g1_a_sum_b);
    assert_eq!(&G1_C_MUL_D, &g1_c_mul_d);
    assert_eq!(&G2_A_SUM_B, &g2_a_sum_b);
    assert_eq!(&G2_C_MUL_D, &g2_c_mul_d);
}

#[cfg(test)]
mod tests {
    use bn::{AffineG2, Fq, Fq2, Fr, Group, G2};

    fn serialize_g2(g2: G2) -> String {
        let g2 = AffineG2::from_jacobian(g2).unwrap();
        let mut output = [0u8; 128];
        g2.x().real().to_big_endian(&mut output[..32]).unwrap();
        g2.x().imaginary().to_big_endian(&mut output[32..64]).unwrap();
        g2.y().real().to_big_endian(&mut output[64..96]).unwrap();
        g2.y().imaginary().to_big_endian(&mut output[96..]).unwrap();
        hex::encode(output)
    }    

    #[ignore]
    #[test]
    fn g2_test_vectors() {
        let a = G2::one()
            * Fr::from_str(
                "20390255904278144451778773028944684152769293537511418234311120800877067946",
            ).unwrap();
        println!("A: {}", serialize_g2(a));

        let b: G2 = AffineG2::new(
            Fq2::new(
                Fq::from_str("10857046999023057135944570762232829481370756359578518086990519993285655852781")
                    .expect("a-coeff of g2 x generator is of the right order"),
                Fq::from_str("11559732032986387107991004021392285783925812861821192530917403151452391805634")
                    .expect("b-coeff of g2 x generator is of the right order"),
            ),
            Fq2::new(
                Fq::from_str("8495653923123431417604973247489272438418190587263600148770280649306958101930")
                    .expect("a-coeff of g2 y generator is of the right order"),
                Fq::from_str("4082367875863433681332203403145435568316851327593401208105741076214120093531")
                    .expect("b-coeff of g2 y generator is of the right order"),
            ),
        ).unwrap().into();
        println!("B: {}", serialize_g2(b));
        println!("A + B: {}", serialize_g2(a + b));

        let c = G2::one()
            * Fr::from_str(
                "18097487326282793650237947474982649264364522469319914492172746413872781676",
            ).unwrap();
        println!("C: {}", serialize_g2(a));

        let d = Fr::from_str("20390255904278144451778773028944684152769293537511418234311120800877067946")
            .unwrap();
        
        let mut d_bytes = [0u8; 32];
        d.to_big_endian(&mut d_bytes).unwrap();
        println!("D: {}", hex::encode(d_bytes));
        println!("C * D: {}", serialize_g2(c * d));
    }
}
