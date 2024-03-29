use common::intcode2::{Computer, WhatsUp};
use std::collections::VecDeque;

fn main() {
    let mut nodes: Vec<_> = (0..50).map(Node::new).collect();
    'part1: loop {
        for i in 0..nodes.len() {
            if let Some((dst, msg)) = nodes[i].dispatch() {
                if dst == 255 {
                    println!("Part 1: {}", msg.1);
                    break 'part1;
                }
                nodes[dst as usize].inque.push_back(msg);
            }
        }
    }

    let mut nodes: Vec<_> = (0..50).map(Node::new).collect();
    let mut nat = (0, 0);
    let mut last_y = 0;
    'part2: loop {
        let mut n_sent = 0;
        for i in 0..nodes.len() {
            if let Some((dst, msg)) = nodes[i].dispatch() {
                n_sent += 1;
                if dst == 255 {
                    nat = msg;
                } else {
                    nodes[dst as usize].inque.push_back(msg);
                }
            }
        }
        if n_sent == 0 {
            let n_queued: usize = nodes.iter().map(|n| n.inque.len()).sum();
            if n_queued == 0 {
                nodes[0].inque.push_back(nat);
                if last_y == nat.1 {
                    println!("Part 2: {}", last_y);
                    break 'part2;
                }
                last_y = nat.1;
            }
        }
    }
}

struct Node {
    vm: Computer,
    inque: VecDeque<(i64, i64)>,
}

impl Node {
    fn new(addr: i64) -> Self {
        let mut vm = Computer::new(&INPUT);
        vm.run(Some(addr)).unwrap();
        Node {
            vm,
            inque: VecDeque::new(),
        }
    }

    fn dispatch(&mut self) -> Option<(i64, (i64, i64))> {
        let dst = self.process_input()?;
        let x = self.process_output();
        let y = self.process_output();
        Some((dst, (x, y)))
    }

    fn process_input(&mut self) -> Option<i64> {
        let r = match self.inque.pop_front() {
            None => self.vm.run(Some(-1)).expect("VM Error"),
            Some((x, y)) => {
                self.vm.run(Some(x)).expect("VM Error");
                self.vm.run(Some(y)).expect("VM Error")
            }
        };
        match r {
            WhatsUp::Halt => panic!("HALT"),
            WhatsUp::NeedInput => None,
            WhatsUp::Output(addr) => Some(addr),
        }
    }

    fn process_output(&mut self) -> i64 {
        match self.vm.run(None).expect("VM Error") {
            WhatsUp::Halt => panic!("HALT"),
            WhatsUp::NeedInput => unreachable!(),
            WhatsUp::Output(val) => val,
        }
    }
}

const INPUT: [i64; 2245] = [
    3,
    62,
    1001,
    62,
    11,
    10,
    109,
    2245,
    105,
    1,
    0,
    1973,
    1674,
    1771,
    2107,
    1812,
    602,
    1474,
    1600,
    1540,
    1738,
    703,
    2014,
    1942,
    2047,
    2183,
    1272,
    1843,
    1707,
    765,
    1301,
    1639,
    839,
    2144,
    1505,
    899,
    2076,
    1148,
    965,
    1239,
    1439,
    1336,
    1569,
    1000,
    734,
    1373,
    1115,
    1208,
    932,
    870,
    1876,
    1402,
    571,
    637,
    1177,
    2214,
    1911,
    670,
    806,
    1031,
    1078,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    3,
    64,
    1008,
    64,
    -1,
    62,
    1006,
    62,
    88,
    1006,
    61,
    170,
    1105,
    1,
    73,
    3,
    65,
    20102,
    1,
    64,
    1,
    21001,
    66,
    0,
    2,
    21102,
    1,
    105,
    0,
    1106,
    0,
    436,
    1201,
    1,
    -1,
    64,
    1007,
    64,
    0,
    62,
    1005,
    62,
    73,
    7,
    64,
    67,
    62,
    1006,
    62,
    73,
    1002,
    64,
    2,
    133,
    1,
    133,
    68,
    133,
    102,
    1,
    0,
    62,
    1001,
    133,
    1,
    140,
    8,
    0,
    65,
    63,
    2,
    63,
    62,
    62,
    1005,
    62,
    73,
    1002,
    64,
    2,
    161,
    1,
    161,
    68,
    161,
    1101,
    1,
    0,
    0,
    1001,
    161,
    1,
    169,
    101,
    0,
    65,
    0,
    1102,
    1,
    1,
    61,
    1101,
    0,
    0,
    63,
    7,
    63,
    67,
    62,
    1006,
    62,
    203,
    1002,
    63,
    2,
    194,
    1,
    68,
    194,
    194,
    1006,
    0,
    73,
    1001,
    63,
    1,
    63,
    1106,
    0,
    178,
    21101,
    210,
    0,
    0,
    105,
    1,
    69,
    2102,
    1,
    1,
    70,
    1102,
    1,
    0,
    63,
    7,
    63,
    71,
    62,
    1006,
    62,
    250,
    1002,
    63,
    2,
    234,
    1,
    72,
    234,
    234,
    4,
    0,
    101,
    1,
    234,
    240,
    4,
    0,
    4,
    70,
    1001,
    63,
    1,
    63,
    1105,
    1,
    218,
    1105,
    1,
    73,
    109,
    4,
    21101,
    0,
    0,
    -3,
    21102,
    1,
    0,
    -2,
    20207,
    -2,
    67,
    -1,
    1206,
    -1,
    293,
    1202,
    -2,
    2,
    283,
    101,
    1,
    283,
    283,
    1,
    68,
    283,
    283,
    22001,
    0,
    -3,
    -3,
    21201,
    -2,
    1,
    -2,
    1106,
    0,
    263,
    22102,
    1,
    -3,
    -3,
    109,
    -4,
    2105,
    1,
    0,
    109,
    4,
    21101,
    0,
    1,
    -3,
    21102,
    0,
    1,
    -2,
    20207,
    -2,
    67,
    -1,
    1206,
    -1,
    342,
    1202,
    -2,
    2,
    332,
    101,
    1,
    332,
    332,
    1,
    68,
    332,
    332,
    22002,
    0,
    -3,
    -3,
    21201,
    -2,
    1,
    -2,
    1106,
    0,
    312,
    21202,
    -3,
    1,
    -3,
    109,
    -4,
    2105,
    1,
    0,
    109,
    1,
    101,
    1,
    68,
    358,
    21002,
    0,
    1,
    1,
    101,
    3,
    68,
    366,
    21002,
    0,
    1,
    2,
    21101,
    0,
    376,
    0,
    1106,
    0,
    436,
    22101,
    0,
    1,
    0,
    109,
    -1,
    2105,
    1,
    0,
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    109,
    8,
    21202,
    -6,
    10,
    -5,
    22207,
    -7,
    -5,
    -5,
    1205,
    -5,
    521,
    21101,
    0,
    0,
    -4,
    21102,
    0,
    1,
    -3,
    21102,
    1,
    51,
    -2,
    21201,
    -2,
    -1,
    -2,
    1201,
    -2,
    385,
    471,
    20102,
    1,
    0,
    -1,
    21202,
    -3,
    2,
    -3,
    22207,
    -7,
    -1,
    -5,
    1205,
    -5,
    496,
    21201,
    -3,
    1,
    -3,
    22102,
    -1,
    -1,
    -5,
    22201,
    -7,
    -5,
    -7,
    22207,
    -3,
    -6,
    -5,
    1205,
    -5,
    515,
    22102,
    -1,
    -6,
    -5,
    22201,
    -3,
    -5,
    -3,
    22201,
    -1,
    -4,
    -4,
    1205,
    -2,
    461,
    1105,
    1,
    547,
    21101,
    -1,
    0,
    -4,
    21202,
    -6,
    -1,
    -6,
    21207,
    -7,
    0,
    -5,
    1205,
    -5,
    547,
    22201,
    -7,
    -6,
    -7,
    21201,
    -4,
    1,
    -4,
    1105,
    1,
    529,
    21201,
    -4,
    0,
    -7,
    109,
    -8,
    2105,
    1,
    0,
    109,
    1,
    101,
    1,
    68,
    563,
    21001,
    0,
    0,
    0,
    109,
    -1,
    2106,
    0,
    0,
    1102,
    1,
    101359,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1,
    598,
    68,
    1101,
    556,
    0,
    69,
    1101,
    0,
    1,
    71,
    1102,
    600,
    1,
    72,
    1105,
    1,
    73,
    1,
    10531,
    16,
    562,
    1102,
    63391,
    1,
    66,
    1102,
    1,
    3,
    67,
    1101,
    0,
    629,
    68,
    1102,
    302,
    1,
    69,
    1102,
    1,
    1,
    71,
    1101,
    635,
    0,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    40,
    104001,
    1101,
    28979,
    0,
    66,
    1101,
    0,
    1,
    67,
    1101,
    0,
    664,
    68,
    1102,
    556,
    1,
    69,
    1101,
    0,
    2,
    71,
    1102,
    1,
    666,
    72,
    1105,
    1,
    73,
    1,
    10,
    3,
    46798,
    18,
    126402,
    1101,
    0,
    3089,
    66,
    1101,
    2,
    0,
    67,
    1102,
    697,
    1,
    68,
    1102,
    1,
    302,
    69,
    1101,
    1,
    0,
    71,
    1102,
    1,
    701,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    40,
    138668,
    1102,
    1,
    70079,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1,
    730,
    68,
    1102,
    556,
    1,
    69,
    1102,
    1,
    1,
    71,
    1101,
    732,
    0,
    72,
    1105,
    1,
    73,
    1,
    13,
    30,
    73924,
    1101,
    0,
    20287,
    66,
    1101,
    1,
    0,
    67,
    1102,
    761,
    1,
    68,
    1101,
    0,
    556,
    69,
    1101,
    0,
    1,
    71,
    1101,
    763,
    0,
    72,
    1106,
    0,
    73,
    1,
    109,
    30,
    18481,
    1101,
    21067,
    0,
    66,
    1102,
    6,
    1,
    67,
    1102,
    1,
    792,
    68,
    1102,
    1,
    302,
    69,
    1102,
    1,
    1,
    71,
    1101,
    804,
    0,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    60778,
    1101,
    8387,
    0,
    66,
    1101,
    1,
    0,
    67,
    1102,
    833,
    1,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    2,
    71,
    1102,
    835,
    1,
    72,
    1106,
    0,
    73,
    1,
    6653,
    2,
    616758,
    20,
    103534,
    1101,
    0,
    54881,
    66,
    1101,
    1,
    0,
    67,
    1101,
    0,
    866,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    1,
    71,
    1101,
    868,
    0,
    72,
    1105,
    1,
    73,
    1,
    4523,
    2,
    411172,
    1101,
    16693,
    0,
    66,
    1101,
    1,
    0,
    67,
    1102,
    897,
    1,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    0,
    71,
    1101,
    899,
    0,
    72,
    1106,
    0,
    73,
    1,
    1433,
    1101,
    95479,
    0,
    66,
    1101,
    0,
    2,
    67,
    1102,
    1,
    926,
    68,
    1102,
    1,
    302,
    69,
    1101,
    0,
    1,
    71,
    1102,
    930,
    1,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    9,
    69302,
    1102,
    49663,
    1,
    66,
    1102,
    2,
    1,
    67,
    1102,
    959,
    1,
    68,
    1102,
    302,
    1,
    69,
    1101,
    0,
    1,
    71,
    1102,
    963,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    46,
    6178,
    1102,
    1,
    2089,
    66,
    1101,
    3,
    0,
    67,
    1102,
    992,
    1,
    68,
    1101,
    302,
    0,
    69,
    1101,
    1,
    0,
    71,
    1102,
    998,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    40,
    34667,
    1101,
    25793,
    0,
    66,
    1102,
    1,
    1,
    67,
    1101,
    1027,
    0,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1029,
    0,
    72,
    1106,
    0,
    73,
    1,
    1193,
    29,
    65551,
    1102,
    1,
    99257,
    66,
    1102,
    1,
    1,
    67,
    1101,
    1058,
    0,
    68,
    1101,
    0,
    556,
    69,
    1102,
    1,
    9,
    71,
    1101,
    1060,
    0,
    72,
    1106,
    0,
    73,
    1,
    1,
    16,
    281,
    24,
    190958,
    9,
    34651,
    11,
    83267,
    49,
    52534,
    29,
    196653,
    28,
    94726,
    35,
    68863,
    39,
    80783,
    1102,
    1,
    26267,
    66,
    1101,
    0,
    4,
    67,
    1101,
    0,
    1105,
    68,
    1101,
    302,
    0,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1113,
    0,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    7,
    173343,
    1102,
    1,
    68863,
    66,
    1102,
    1,
    2,
    67,
    1102,
    1,
    1142,
    68,
    1101,
    0,
    302,
    69,
    1102,
    1,
    1,
    71,
    1102,
    1,
    1146,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    39,
    161566,
    1101,
    0,
    11057,
    66,
    1102,
    1,
    1,
    67,
    1101,
    1175,
    0,
    68,
    1101,
    0,
    556,
    69,
    1101,
    0,
    0,
    71,
    1102,
    1,
    1177,
    72,
    1105,
    1,
    73,
    1,
    1232,
    1102,
    1,
    103687,
    66,
    1102,
    1,
    1,
    67,
    1101,
    0,
    1204,
    68,
    1101,
    0,
    556,
    69,
    1102,
    1,
    1,
    71,
    1102,
    1,
    1206,
    72,
    1106,
    0,
    73,
    1,
    8,
    30,
    55443,
    1101,
    0,
    7759,
    66,
    1102,
    1,
    1,
    67,
    1102,
    1,
    1235,
    68,
    1101,
    0,
    556,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1237,
    0,
    72,
    1106,
    0,
    73,
    1,
    125,
    3,
    70197,
    1101,
    47363,
    0,
    66,
    1102,
    1,
    2,
    67,
    1102,
    1266,
    1,
    68,
    1102,
    302,
    1,
    69,
    1102,
    1,
    1,
    71,
    1102,
    1270,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    35,
    137726,
    1102,
    76579,
    1,
    66,
    1101,
    0,
    1,
    67,
    1101,
    0,
    1299,
    68,
    1102,
    556,
    1,
    69,
    1101,
    0,
    0,
    71,
    1101,
    1301,
    0,
    72,
    1105,
    1,
    73,
    1,
    1866,
    1101,
    0,
    13063,
    66,
    1102,
    1,
    1,
    67,
    1101,
    1328,
    0,
    68,
    1101,
    556,
    0,
    69,
    1101,
    0,
    3,
    71,
    1101,
    1330,
    0,
    72,
    1105,
    1,
    73,
    1,
    2,
    30,
    36962,
    18,
    21067,
    18,
    42134,
    1101,
    18481,
    0,
    66,
    1101,
    0,
    4,
    67,
    1102,
    1,
    1363,
    68,
    1101,
    302,
    0,
    69,
    1101,
    1,
    0,
    71,
    1101,
    0,
    1371,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    7,
    231124,
    1102,
    22171,
    1,
    66,
    1102,
    1,
    1,
    67,
    1102,
    1,
    1400,
    68,
    1101,
    0,
    556,
    69,
    1101,
    0,
    0,
    71,
    1102,
    1,
    1402,
    72,
    1106,
    0,
    73,
    1,
    1253,
    1101,
    0,
    34667,
    66,
    1101,
    4,
    0,
    67,
    1101,
    1429,
    0,
    68,
    1101,
    253,
    0,
    69,
    1101,
    1,
    0,
    71,
    1102,
    1,
    1437,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    30389,
    1101,
    65551,
    0,
    66,
    1102,
    3,
    1,
    67,
    1101,
    0,
    1466,
    68,
    1101,
    0,
    302,
    69,
    1102,
    1,
    1,
    71,
    1101,
    0,
    1472,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    7,
    115562,
    1101,
    0,
    12853,
    66,
    1102,
    1,
    1,
    67,
    1101,
    1501,
    0,
    68,
    1102,
    1,
    556,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1503,
    0,
    72,
    1106,
    0,
    73,
    1,
    6256,
    7,
    57781,
    1101,
    0,
    39157,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1,
    1532,
    68,
    1102,
    556,
    1,
    69,
    1101,
    0,
    3,
    71,
    1101,
    0,
    1534,
    72,
    1106,
    0,
    73,
    1,
    5,
    3,
    23399,
    3,
    93596,
    18,
    63201,
    1102,
    49157,
    1,
    66,
    1102,
    1,
    1,
    67,
    1102,
    1567,
    1,
    68,
    1102,
    556,
    1,
    69,
    1102,
    0,
    1,
    71,
    1101,
    1569,
    0,
    72,
    1106,
    0,
    73,
    1,
    1532,
    1101,
    58997,
    0,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1596,
    1,
    68,
    1102,
    1,
    556,
    69,
    1101,
    1,
    0,
    71,
    1101,
    0,
    1598,
    72,
    1106,
    0,
    73,
    1,
    160,
    18,
    84268,
    1101,
    57781,
    0,
    66,
    1101,
    5,
    0,
    67,
    1102,
    1,
    1627,
    68,
    1101,
    0,
    253,
    69,
    1101,
    1,
    0,
    71,
    1102,
    1,
    1637,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    37,
    99326,
    1102,
    1,
    51767,
    66,
    1101,
    0,
    3,
    67,
    1101,
    1666,
    0,
    68,
    1102,
    1,
    302,
    69,
    1101,
    0,
    1,
    71,
    1102,
    1672,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    5,
    190173,
    1101,
    0,
    30389,
    66,
    1102,
    1,
    2,
    67,
    1102,
    1701,
    1,
    68,
    1101,
    351,
    0,
    69,
    1101,
    1,
    0,
    71,
    1102,
    1705,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    255,
    8191,
    1101,
    0,
    68699,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1,
    1734,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1736,
    0,
    72,
    1106,
    0,
    73,
    1,
    -3,
    20,
    155301,
    1102,
    34651,
    1,
    66,
    1102,
    2,
    1,
    67,
    1101,
    1765,
    0,
    68,
    1101,
    302,
    0,
    69,
    1101,
    0,
    1,
    71,
    1101,
    1769,
    0,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    11,
    166534,
    1101,
    0,
    102793,
    66,
    1102,
    6,
    1,
    67,
    1101,
    1798,
    0,
    68,
    1101,
    0,
    302,
    69,
    1101,
    1,
    0,
    71,
    1101,
    1810,
    0,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    40,
    69334,
    1101,
    67961,
    0,
    66,
    1102,
    1,
    1,
    67,
    1102,
    1,
    1839,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1841,
    0,
    72,
    1105,
    1,
    73,
    1,
    46,
    29,
    131102,
    1101,
    0,
    281,
    66,
    1101,
    0,
    2,
    67,
    1102,
    1870,
    1,
    68,
    1101,
    0,
    302,
    69,
    1102,
    1,
    1,
    71,
    1101,
    1874,
    0,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    24,
    95479,
    1101,
    80783,
    0,
    66,
    1102,
    1,
    3,
    67,
    1102,
    1,
    1903,
    68,
    1101,
    0,
    302,
    69,
    1101,
    0,
    1,
    71,
    1101,
    0,
    1909,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    7,
    288905,
    1101,
    24251,
    0,
    66,
    1101,
    0,
    1,
    67,
    1101,
    1938,
    0,
    68,
    1101,
    0,
    556,
    69,
    1102,
    1,
    1,
    71,
    1102,
    1,
    1940,
    72,
    1106,
    0,
    73,
    1,
    42841,
    28,
    47363,
    1101,
    0,
    49877,
    66,
    1102,
    1,
    1,
    67,
    1102,
    1969,
    1,
    68,
    1102,
    556,
    1,
    69,
    1101,
    1,
    0,
    71,
    1102,
    1,
    1971,
    72,
    1105,
    1,
    73,
    1,
    71,
    49,
    26267,
    1101,
    8191,
    0,
    66,
    1101,
    0,
    1,
    67,
    1102,
    1,
    2000,
    68,
    1101,
    0,
    556,
    69,
    1101,
    0,
    6,
    71,
    1102,
    1,
    2002,
    72,
    1106,
    0,
    73,
    1,
    29846,
    46,
    3089,
    5,
    63391,
    5,
    126782,
    27,
    2089,
    27,
    4178,
    27,
    6267,
    1101,
    0,
    83267,
    66,
    1102,
    1,
    2,
    67,
    1101,
    0,
    2041,
    68,
    1102,
    302,
    1,
    69,
    1102,
    1,
    1,
    71,
    1102,
    2045,
    1,
    72,
    1105,
    1,
    73,
    0,
    0,
    0,
    0,
    37,
    49663,
    1101,
    0,
    13267,
    66,
    1102,
    1,
    1,
    67,
    1101,
    2074,
    0,
    68,
    1101,
    556,
    0,
    69,
    1101,
    0,
    0,
    71,
    1102,
    1,
    2076,
    72,
    1105,
    1,
    73,
    1,
    1103,
    1102,
    167,
    1,
    66,
    1101,
    1,
    0,
    67,
    1102,
    1,
    2103,
    68,
    1102,
    556,
    1,
    69,
    1101,
    0,
    1,
    71,
    1102,
    2105,
    1,
    72,
    1105,
    1,
    73,
    1,
    -11,
    2,
    513965,
    1102,
    23399,
    1,
    66,
    1101,
    0,
    4,
    67,
    1101,
    2134,
    0,
    68,
    1101,
    302,
    0,
    69,
    1101,
    0,
    1,
    71,
    1101,
    2142,
    0,
    72,
    1106,
    0,
    73,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    18,
    105335,
    1101,
    65951,
    0,
    66,
    1101,
    0,
    1,
    67,
    1101,
    0,
    2171,
    68,
    1101,
    556,
    0,
    69,
    1102,
    1,
    5,
    71,
    1101,
    0,
    2173,
    72,
    1105,
    1,
    73,
    1,
    3,
    2,
    102793,
    2,
    205586,
    49,
    78801,
    39,
    242349,
    20,
    51767,
    1101,
    55787,
    0,
    66,
    1102,
    1,
    1,
    67,
    1102,
    2210,
    1,
    68,
    1101,
    0,
    556,
    69,
    1101,
    1,
    0,
    71,
    1101,
    2212,
    0,
    72,
    1106,
    0,
    73,
    1,
    659,
    2,
    308379,
    1101,
    37997,
    0,
    66,
    1101,
    1,
    0,
    67,
    1102,
    1,
    2241,
    68,
    1102,
    1,
    556,
    69,
    1102,
    1,
    1,
    71,
    1102,
    2243,
    1,
    72,
    1106,
    0,
    73,
    1,
    -152,
    49,
    105068,
];
