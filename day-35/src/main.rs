use num_bigint::BigInt;
use std::collections::HashMap;

fn ways_to_make(cache: &mut HashMap<BigInt, BigInt>, this_num: BigInt) -> BigInt {
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
        .map(|offset| {
            let next: BigInt = &this_num - offset;

            match cache.get(&next) {
                Some(result) => result.clone(),
                None => {
                    let result = ways_to_make(cache, next.clone());
                    cache.insert(next.clone(), result.clone());
                    // println!("returning {result}");
                    result
                }
            }
        })
        .iter()
        .sum()
}

// answer is 361595632332583638761407421958897298379960745882500550853575978681928496636233758054533916390012124244431806190608039087381666468880612638124124662565287224989590899000769252066051
// need like... huge int?
fn main() {
    let mut cache: HashMap<BigInt, BigInt> = HashMap::new();
    let result = ways_to_make(&mut cache, BigInt::from(856));
    println!("{result}");
}
