use std::str::FromStr;

use cached::proc_macro::cached;
use num_bigint::BigInt;

#[cached]
fn ways_to_make(this_num: BigInt) -> BigInt {
    let zero = BigInt::from(0);
    if this_num < zero {
        // overshot, bad path
        return zero;
    }
    if this_num == zero {
        // reached the end of a valid path!
        return BigInt::from(1);
    }

    [40, 12, 2, 1]
        .map(|offset| ways_to_make(&this_num - offset))
        .iter()
        .sum()
}

fn main() {
    let result = ways_to_make(BigInt::from(856));
    assert_eq!(result, BigInt::from_str("361595632332583638761407421958897298379960745882500550853575978681928496636233758054533916390012124244431806190608039087381666468880612638124124662565287224989590899000769252066051").unwrap());
    println!("{result}");
}
