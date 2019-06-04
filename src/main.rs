const NUM_LV1: u128 = 25;
const NUM_LV2: u128 = 25;
const NUM_LV3: u128 = 25;
const NUM_DOMES: u128 = 21;

const BOARD_DIM: u128 = 5;

fn nCr(n: u128, r: u128) -> u128 {
    assert!(n >= r);
    if r == 0 {return 1};
    return (n * nCr(n-1, r-1)) / r;
}

// fn main() {
//     println!("nCr 1C1: {}", nCr(1, 1));
//     println!("nCr 2C1: {}", nCr(2, 1));
//     println!("nCr 2C2: {}", nCr(2, 2));
//     println!("nCr 3C1: {}", nCr(3, 1));
//     println!("nCr 3C2: {}", nCr(3, 2));
//     println!("nCr 3C3: {}", nCr(3, 3));
//     println!("nCr 5C1: {}", nCr(5, 1));
//     println!("nCr 5C2: {}", nCr(5, 2));
//     println!("nCr 5C3: {}", nCr(5, 3));
//     println!("nCr 5C4: {}", nCr(5, 4));
//     println!("nCr 5C5: {}", nCr(5, 5));
// }
// // nCr 1C1: 1
// // nCr 2C1: 2
// // nCr 2C2: 1
// // nCr 3C1: 3
// // nCr 3C2: 3
// // nCr 3C3: 1
// // nCr 5C1: 5
// // nCr 5C2: 10
// // nCr 5C3: 10
// // nCr 5C4: 5
// // nCr 5C5: 1



// this is just a stupid way to do (levels+1) ^ n when n == r
fn count_recursive_combos(n: u128, r: u128, levels: u128) -> u128{
    if levels == 0 {return 1};

    let mut combos: u128 = 0;
    for r in 0..=n {
        combos += nCr(n, r) * count_recursive_combos(r, r, levels - 1);
    }
    return combos;
}

fn count_base_combos() -> u128 {
    let mut base_combos: u128 = 0;
    for r in 1..=NUM_LV1 {
        base_combos += nCr(BOARD_DIM * BOARD_DIM, r);
    }
    return base_combos;
}

fn main() {
    println!("Base Combinations: {}", count_base_combos());

    println!("RC math: {}", count_recursive_combos(25, 25, 4));
    println!("4^21 RC: {}", count_recursive_combos(21, 21, 4));
    println!("4^21 Pow: {}", (5 as u128).pow(21));
    println!("JC math: {}", nCr(25, 4) * nCr(4, 2)  * (3 as u128).pow(4) * (5 as u128).pow(21));
    println!("nCr(25, 4): {}", nCr(25, 4));

    println!("3^2 RC: {}", count_recursive_combos(2, 2, 2));
    println!("3^2 Pow: {}", (3 as u128).pow(2));

}