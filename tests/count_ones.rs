use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use swar::*;

#[test]
fn sum_weight() {
    let mut rng = SmallRng::from_seed([5; 16]);
    let numbers = rng
        .sample_iter(&rand::distributions::Standard)
        .take(100_000)
        .collect::<Vec<u128>>();
    for number in numbers {
        assert_eq!(
            number.count_ones(),
            Bits1(number)
                .sum_weight2()
                .sum_weight2()
                .sum_weight2()
                .sum_weight2()
                .sum_weight2()
                .sum_weight2()
                .sum_weight2()
                .0 as u32
        );
    }
}
