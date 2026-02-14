use rand::{Rng, SeedableRng};

fn main() {
    // シードを固定した乱数生成は、rand_chachaを使用して行う。
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    //乱数の生成
    // 0から100までの乱数を生成
    let random_number = rng.random_range(0..=100);
    println!("Random number between 0 and 100: {}", random_number);
}
