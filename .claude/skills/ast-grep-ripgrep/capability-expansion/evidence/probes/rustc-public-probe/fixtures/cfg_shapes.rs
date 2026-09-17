// Fixture for PB15. Three deliberately different control-flow shapes.
//
// The shapes differ so the measurement can FAIL. If both arms of the probe reported the same
// block count for every function, that agreement would be worthless when the count is a
// constant -- two broken readers both saying "1" agree perfectly. These three functions must
// produce three DIFFERENT counts, in the same order, in both arms.

// Straight line: no branch, so the only blocks are the ones arithmetic checking introduces.
pub fn straight_line(a: i32) -> i32 {
    a + 1
}

// One branch: a `switchInt` with two arms that rejoin.
pub fn branching(a: i32) -> i32 {
    if a > 0 { a } else { 0 - a }
}

// A loop: the point of this one is the BACK EDGE. A `goto` to a lower-numbered block is the
// thing `program.cfg_edge` exists to hold, and a reader that silently flattened the CFG into a
// statement list would still get the block count right while losing it.
pub fn looping(n: u32) -> u32 {
    let mut total = 0u32;
    let mut i = 0u32;
    while i < n {
        total += i;
        i += 1;
    }
    total
}
