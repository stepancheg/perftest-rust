extern mod std;
extern mod extra;

use std::comm;
use std::io;

use extra::time;

fn to_us(ts: &time::Timespec) -> u64 {
    ts.sec * 1000000 as u64 + ((ts.nsec / 1000) as u64)
}

fn now_us() -> u64 {
    to_us(&time::get_time())
}

// execute as RUST_THREADS=1 ./switch
// result is about 2us per switch
fn main() {

    let (port1, chan1): (Port<int>, Chan<int>) = comm::stream();
    let (port2, chan2): (Port<int>, Chan<int>) = comm::stream();

    do spawn || {
        loop {
            chan2.send(port1.recv());
        }
    };

    loop {

        let start = now_us();

        let mut steps = 0;
        while (now_us() - start < 1000000) {
            let iter = 100000;
            let mut i = 0;
            while i < iter {
                chan1.send(i as int);
                port2.recv();
                i += 1;
            }
            steps += iter;
        }

        let d = now_us() - start;
        let per_step = d * 1000 / steps / 2;
        io::println(per_step.to_str() + "ns");
    }

}
