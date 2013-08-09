extern mod std;
extern mod extra;

use std::io;

use extra::time;

fn to_us(ts: &time::Timespec) -> u64 {
    (ts.sec * 1000000) as u64 + ((ts.nsec / 1000) as u64)
}

fn now_us() -> u64 {
    to_us(&time::get_time())
}

// result is 74ns
fn main() {
    loop {
        let start = now_us();
        let mut count = 0;
        while (now_us() - start < 1000000) {
            count += 1;
        }
        let d = now_us() - start;
        let per_step = d * 1000 / count;
        io::println(per_step.to_str() + "ns");
    }
}

