use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// Reference values from https://github.com/bluealloy/revm/blob/d185018d33fd73a880eaa54bdcd6e463f8a6d11a/crates/precompile/src/bn128.rs

pub const G1_A: &str = "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266";
pub const G1_B: &str = "07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7";
pub const G1_C: &str = "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204";
pub const FR_D1: &str = "00000000000000000000000000000000000000000000000011138ce750fa15c2";
pub const G1_A_SUM_B: &str = "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915";
pub const G1_C_MUL_D: &str = "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc";

pub const G2_A: &str = "1ecfd2dff2aad18798b64bdb0c2b50c9d73e6c05619e04cbf5b448fd987268800e16c8d96362720af0916592be1b839a26f5e6b710f3ede0d8840d9a70eaf97f2aa778acda9e7d4925c60ad84c12fb3b4f2b9539d5699934b0e6fdd10cc2c0e11e8f2c1f441fed039bb46d6bfb91236cf7ba240c75080cedbe40e049c46b26be";
pub const G2_B: &str = "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c212c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b";
pub const G2_C: &str = "1b456e8f0f931657ef38681c6ba9503861c2a43161098bf6ed0d4a1c9dad8d8912f3ad32a637feee230eb77b3794645e81694129775c2ba5742550e02a26b86925192143313355e82635421b3cca6546a18d8de4cd273e88169a7239dd32c6572b8bff0df12a623a20f884776e3c9bc912e4bd148d838c7d85f2217feecd219e";
pub const FR_D2: &str = "000b8a5ca04ade1fcdecc669db1861eba5ba2f3e336bdbcba1d5b3c11c2b3eaa";
pub const G2_A_SUM_B: &str = "028ed67dc28dc89a3af1a0dc9d212f71349244df651c68c619885b3e30a392e315f7967dae56ea5434d6918f1bdf261e7f76241baf6d90f1c7aa6345defdd5740a10114ead33e3d235f03e90003f5dd30f87ce1874ec45b06fa3d02cffc83c541bafecee364a3c2b6b1d6cb7eae1c6886f667e3d7e905d6a14a1fe6ee070f63c";
pub const G2_C_MUL_D: &str = "17093252104cabf33e3807a6da5825f8c3c21a41b6aef9004576b4a11eda6bdc09c7727ad2e8cec21dab67c6420d47aab06c5580b736a5200c43e33b35b159ee2d446e0f6750e69b9c869daeb261c8d9051c0b7e7c29263c9f50b3a8cc7f036a2d5f047d94c986a41aac2b39c2863e52e2ed84ba6c9aef0b9b31054d07fdde08";

pub const ADD_IN: &str = "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7";
pub const MUL_IN: &str = "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb20400000000000000000000000000000000000000000000000011138ce750fa15c2";
pub const PAIR_IN: &str = "\
    1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
    3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
    209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
    04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
    2bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d\
    120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550\
    111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c\
    2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411\
    198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
    1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
    090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
    12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";

pub const ADD_OUT: &str = "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915";
pub const MUL_OUT: &str = "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc";
pub const PAIR_OUT: &str = "0000000000000000000000000000000000000000000000000000000000000001";

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

    stdin.write(&ADD_IN);
    stdin.write(&MUL_IN);
    stdin.write(&PAIR_IN);

    let client = ProverClient::new();
    let mut public_values = client.execute(ELF, stdin).expect("proving failed");

    let g1_a_sum_b = public_values.read::<String>();
    let g1_c_mul_d = public_values.read::<String>();
    let g2_a_sum_b = public_values.read::<String>();
    let g2_c_mul_d = public_values.read::<String>();

    let add_out = public_values.read::<String>();
    let mul_out = public_values.read::<String>();
    let pair_out = public_values.read::<String>();

    assert_eq!(&G1_A_SUM_B, &g1_a_sum_b);
    assert_eq!(&G1_C_MUL_D, &g1_c_mul_d);
    assert_eq!(&G2_A_SUM_B, &g2_a_sum_b);
    assert_eq!(&G2_C_MUL_D, &g2_c_mul_d);

    assert_eq!(&ADD_OUT, &add_out);
    assert_eq!(&MUL_OUT, &mul_out);
    assert_eq!(&PAIR_OUT, &pair_out);
}

#[cfg(test)]
mod tests {
    use bn::{AffineG1, AffineG2, Fq, Fq2, Fr, Group, G1, G2};

    use crate::{
        FR_D1, FR_D2, G1_A, G1_A_SUM_B, G1_B, G1_C, G1_C_MUL_D, G2_A, G2_A_SUM_B, G2_B, G2_C,
        G2_C_MUL_D,
    };

    fn serialize_g2(g2: G2) -> String {
        let g2 = AffineG2::from_jacobian(g2).unwrap();
        let mut output = [0u8; 128];
        g2.x().real().to_big_endian(&mut output[..32]).unwrap();
        g2.x()
            .imaginary()
            .to_big_endian(&mut output[32..64])
            .unwrap();
        g2.y().real().to_big_endian(&mut output[64..96]).unwrap();
        g2.y().imaginary().to_big_endian(&mut output[96..]).unwrap();
        hex::encode(output)
    }

    fn read_g1(data: &str) -> G1 {
        let data = hex::decode(data).unwrap();
        AffineG1::new(
            Fq::from_slice(&data[..32]).unwrap(),
            Fq::from_slice(&data[32..]).unwrap(),
        )
        .unwrap()
        .into()
    }

    fn read_g2(data: &str) -> G2 {
        let data = hex::decode(data).unwrap();
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

    fn read_scalar(data: &str) -> Fr {
        let data = hex::decode(data).unwrap();
        Fr::from_slice(&data).unwrap()
    }

    #[ignore]
    #[test]
    fn g1_test_vectors() {
        let a = read_g1(G1_A);
        let b = read_g1(G1_B);
        let a_sum_b = read_g1(G1_A_SUM_B);
        let res = a + b;
        assert_eq!(a_sum_b, res);

        let c = read_g1(G1_C);
        let d = read_scalar(FR_D1);
        let c_mul_d = read_g1(G1_C_MUL_D);
        let res = c * d;
        assert_eq!(c_mul_d, res);
    }

    #[ignore]
    #[test]
    fn g2_test_vectors() {
        let a = read_g2(G2_A);
        let b = read_g2(G2_B);
        let a_sum_b = read_g2(G2_A_SUM_B);
        let res = a + b;
        assert_eq!(a_sum_b, res);

        let c = read_g2(G2_C);
        let d = read_scalar(FR_D2);
        let c_mul_d = read_g2(G2_C_MUL_D);
        let res = c * d;
        assert_eq!(c_mul_d, res);
    }
}
