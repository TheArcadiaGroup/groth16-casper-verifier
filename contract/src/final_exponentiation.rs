use ark_bn254::Fq12Parameters;
use ark_ff::{Field, Fp12, Fp12ParamsWrapper, FromBytes, QuadExtField};
use arrayref::array_ref;

use crate::pvk::get_alpha_g1_beta_g2;
use crate::utils::{get_account_data, put_account_data, BN254_DATA_LEN};

const NAF: [i64; 63] = [
    1, 0, 0, 0, 1, 0, 1, 0, 0, -1, 0, 1, 0, 1, 0, -1, 0, 0, 1, 0, 1, 0, -1, 0, -1, 0, -1, 0, 1, 0,
    0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, -1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, -1, 0, 0,
    0, 1,
];

pub fn final_exponentiation_handler(t: usize, j: usize, input: &[u8], keys: &[String]) {
    match t {
        2 => prepare_final_data(keys, input),
        // Easy part: result = elt^((q^6-1)*(q^2+1)).
        // Follows, e.g., Beuchat et al page 9, by computing result as follows:
        //   elt^((q^6-1)*(q^2+1)) = (conj(elt) * elt^(-1))^(q^2+1)
        3 => easy_part1(keys),
        4 => easy_part2(keys),
        // Hard part follows Laura Fuentes-Castaneda et al. "Faster hashing to G2"
        // by computing:
        //
        // result = elt^(q^3 * (12*z^3 + 6z^2 + 4z - 1) +
        //               q^2 * (12*z^3 + 6z^2 + 6z) +
        //               q   * (12*z^3 + 6z^2 + 4z) +
        //               1   * (12*z^3 + 12z^2 + 6z + 1))
        // which equals
        //
        // result = elt^( 2z * ( 6z^2 + 3z + 1 ) * (q^4 - q^2 + 1)/r ).
        5 => hard_part_y0(keys, j),
        6 => hard_part_y1(keys),
        7 => hard_part_y3(keys),
        8 => hard_part_y4(keys, j),
        9 => hard_part_y6(keys, j),
        10 => hard_part_y8(keys),
        11 => hard_part_y9(keys),
        12 => hard_part_y11(keys),
        13 => hard_part_y13(keys),
        14 => hard_part_y14(keys),
        15 => hard_part_y15(keys),
        16 => hard_part_y16(keys),
        _ => {}
    }
}

fn prepare_final_data(keys: &[String], input: &[u8]) {
    let gamma_account = keys[0].clone();
    let delta_account = keys[1].clone();
    let final_account = keys[2].clone();

    let qap = array_ref![input, 0, BN254_DATA_LEN];
    let mut qap = Fp12::<Fq12Parameters>::read(&mut qap.as_ref()).unwrap();
    qap *= get_account_data(gamma_account, 1);
    qap *= get_account_data(delta_account, 1);

    put_account_data(final_account, &qap);
}

fn easy_part1(keys: &[String]) {
    let final_account = keys[0].clone();
    let f = get_account_data(final_account.clone(), 1);

    // f1 = r.conjugate() = f^(p^6)
    let mut f1 = f;
    f1.conjugate();
    let f2 = f.inverse().unwrap();
    let f = f1 * &f2;
    put_account_data(final_account, &f);
}

fn easy_part2(keys: &[String]) {
    let final_account = keys[0].clone();
    let mut r = get_account_data(final_account.clone(), 1);

    // f2 = f^(p^6 - 1)
    // r = f^((p^6 - 1)(p^2))
    // r = f^((p^6 - 1)(p^2) + (p^6 - 1))
    // r = f^((p^6 - 1)(p^2 + 1))
    let f2 = r;
    r.frobenius_map(2);
    r *= &f2;
    put_account_data(final_account, &r);
}

fn cal_y0(
    f: &Fp12<Fq12Parameters>,
    res: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    j: usize,
) -> Fp12<Fq12Parameters> {
    // y0
    let mut inverse = f.clone();
    inverse.conjugate();
    let y0 = exp_by_neg_x(&f, &inverse, res, j);
    y0
}

fn hard_part_y0(keys: &[String], j: usize) {
    let final_account = keys[0].clone();
    let y0_account = keys[1].clone();
    let r = get_account_data(final_account, 1);
    let y0 = get_account_data(y0_account.clone(), j);
    let mut y0 = cal_y0(&r, y0, j);
    if j == 62 {
        y0.conjugate();
    }
    put_account_data(y0_account, &y0);
}

fn hard_part_y1(keys: &[String]) {
    let y0_account = keys[0].clone();
    let y1_account = keys[1].clone();

    let y0 = get_account_data(y0_account, 1);
    let y1 = y0.cyclotomic_square();
    put_account_data(y1_account, &y1);
}

fn cal_y3(f: &Fp12<Fq12Parameters>) -> Fp12<Fq12Parameters> {
    // y1 y2 y3
    let y1 = f.cyclotomic_square();
    let y2 = y1.cyclotomic_square();
    let y3 = y2 * &y1;
    y3
}

fn hard_part_y3(keys: &[String]) {
    let y0_account = keys[0].clone();
    let y3_account = keys[1].clone();
    let y0 = get_account_data(y0_account, 1);
    let y3 = cal_y3(&y0);
    put_account_data(y3_account, &y3);
}

fn cal_y4(
    f: &Fp12<Fq12Parameters>,
    res: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    j: usize,
) -> Fp12<Fq12Parameters> {
    let mut inverse = f.clone();
    inverse.conjugate();
    let y4 = exp_by_neg_x(&f, &inverse, res, j);
    y4
}

fn hard_part_y4(keys: &[String], j: usize) {
    let y3_account = keys[0].clone();
    let y4_account = keys[1].clone();
    let y3 = get_account_data(y3_account, 1);
    let y4 = get_account_data(y4_account.clone(), j);
    let mut y4 = cal_y4(&y3, y4, j);
    if j == 62 {
        y4.conjugate();
    }
    put_account_data(y4_account, &y4);
}

fn cal_y6(
    f: &Fp12<Fq12Parameters>,
    res: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    j: usize,
) -> Fp12<Fq12Parameters> {
    // y6
    let mut inverse = f.clone();
    inverse.conjugate();
    let y6 = exp_by_neg_x(&f, &inverse, res, j);
    y6
}

fn hard_part_y6(keys: &[String], j: usize) {
    let y4_account = keys[0].clone();
    let y6_account = keys[1].clone();

    let y4 = get_account_data(y4_account, 1);
    let y5 = y4.cyclotomic_square();
    let y6 = get_account_data(y6_account.clone(), j);
    let mut y6 = cal_y6(&y5, y6, j);
    if j == 62 {
        y6.conjugate();
    }
    put_account_data(y6_account, &y6);
}

fn hard_part_y8(keys: &[String]) {
    let y3_account = keys[0].clone();
    let y4_account = keys[1].clone();
    let y6_account = keys[2].clone();
    let y8_account = keys[3].clone();

    let mut y3 = get_account_data(y3_account, 1);
    let y4 = get_account_data(y4_account, 1);
    let mut y6 = get_account_data(y6_account, 1);

    y3.conjugate();
    y6.conjugate();
    let y7 = y6 * y4;
    let y8 = y7 * y3;

    put_account_data(y8_account, &y8);
}

fn hard_part_y9(keys: &[String]) {
    let y1_account = keys[0].clone();
    let y8_account = keys[1].clone();
    let y9_account = keys[2].clone();

    let y1 = get_account_data(y1_account, 1);
    let y8 = get_account_data(y8_account, 1);

    let y9 = y8 * y1;

    put_account_data(y9_account, &y9);
}

fn hard_part_y11(keys: &[String]) {
    let y4_account = keys[0].clone();
    let y8_account = keys[1].clone();
    let final_account = keys[2].clone();
    let y11_account = keys[3].clone();

    let y4 = get_account_data(y4_account, 1);
    let y8 = get_account_data(y8_account, 1);
    let r = get_account_data(final_account, 1);

    let y11 = y8 * y4 * r;

    put_account_data(y11_account, &y11);
}

fn hard_part_y13(keys: &[String]) {
    let y9_account = keys[0].clone();
    let y11_account = keys[1].clone();
    let y13_account = keys[2].clone();

    let y9 = get_account_data(y9_account, 1);
    let y11 = get_account_data(y11_account, 1);

    let mut y12 = y9;
    y12.frobenius_map(1);
    let y13 = y12 * y11;

    put_account_data(y13_account, &y13);
}

fn hard_part_y14(keys: &[String]) {
    let y8_account = keys[0].clone();
    let y13_account = keys[1].clone();
    let y14_account = keys[2].clone();

    let mut y8 = get_account_data(y8_account, 1);
    let y13 = get_account_data(y13_account, 1);

    y8.frobenius_map(2);
    let y14 = y8 * y13;

    put_account_data(y14_account, &y14);
}

fn hard_part_y15(keys: &[String]) {
    let y9_account = keys[0].clone();
    let final_account = keys[1].clone();
    let y15_account = keys[2].clone();

    let mut r = get_account_data(final_account, 1);
    let y9 = get_account_data(y9_account, 1);

    r.conjugate();
    let mut y15 = r * y9;
    y15.frobenius_map(3);

    put_account_data(y15_account, &y15);
}

fn hard_part_y16(keys: &[String]) {
    let y14_account = keys[0].clone();
    let y15_account = keys[1].clone();

    let y14 = get_account_data(y14_account, 1);
    let y15 = get_account_data(y15_account, 1);

    let y16 = y15 * &y14;
    let alpha_g1_beta_g2 = get_alpha_g1_beta_g2();
    assert!(y16 == alpha_g1_beta_g2);
}

fn exp_by_neg_x(
    fe: &QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    self_inverse: &QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    mut res: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    j: usize,
) -> Fp12<Fq12Parameters> {
    let value = NAF[j];
    if j > 0 {
        res.square_in_place();
    }

    if value != 0 {
        if value > 0 {
            res *= fe;
        } else {
            res *= self_inverse;
        }
    }
    res
}
