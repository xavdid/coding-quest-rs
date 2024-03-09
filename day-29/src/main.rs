use std::ops::Range;

// 45000377
// 01234567

fn convert_index(i: usize) -> usize {
    (i - 1) * 2 // 1 index and double (since each byte is 2 chars)
}

/// This handles conversion between 1-indexed "give me the 3rd bit" and
/// the actual `Range` in the slice. Takes the 1-indexed first byte
/// to fetch and optionally, the last byte to fetch
fn build_range(first: usize, last: Option<usize>) -> Range<usize> {
    let start = convert_index(first);
    let stop = match last {
        None => start,
        Some(stop) => convert_index(stop),
    } + 2;

    let r = start..stop;
    assert!(
        r.len() % 2 == 0,
        "should only ask for an even-number of chars, got {:?} (len: {})",
        r,
        r.len()
    );
    r
}

fn get_num_from_bytes(line: &str, start: usize, stop: Option<usize>) -> i32 {
    let r = build_range(start, stop);
    i32::from_str_radix(&line[r], 16).unwrap()
}

fn get_packet_size(line: &str) -> i32 {
    get_num_from_bytes(line, 3, Some(4))
}

fn get_source_pair(line: &str) -> (i32, i32) {
    (
        get_num_from_bytes(line, 13, None),
        get_num_from_bytes(line, 14, None),
    )
}
fn get_dest_pair(line: &str) -> (i32, i32) {
    (
        get_num_from_bytes(line, 17, None),
        get_num_from_bytes(line, 18, None),
    )
}

fn main() {
    let mut internal = 0;
    let mut external = 0;
    for line in include_str!("input.txt").lines() {
        match get_source_pair(line) {
            (10, 0) => external += get_packet_size(line),
            (192, 168) => internal += get_packet_size(line),
            _ => match get_dest_pair(line) {
                (10, 0) => external += get_packet_size(line),
                (192, 168) => internal += get_packet_size(line),
                _ => {}
            },
        }
    }
    let answer = format!("{}/{}", internal, external);
    // assert_eq!(answer, "258956/256237");
    println!("{answer}");
}
