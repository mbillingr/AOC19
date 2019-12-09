use std::fs::File;
use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let mut input = String::new();
    File::open("data/day08-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let data: Vec<_> = input.bytes().filter(u8::is_ascii_digit).collect();

    let depth = data.len() / (WIDTH * HEIGHT);
    assert_eq!(data.len(), depth * WIDTH * HEIGHT);

    let mut layer_counts = vec![];
    for layer in data.chunks(WIDTH * HEIGHT) {
        let mut counts = [0; 10];
        for digit in layer {
            counts[(digit - b'0') as usize] += 1;
        }
        layer_counts.push(counts);
    }

    let part1 = layer_counts
        .iter()
        //.inspect(|x|println!("{:?}", x))
        .min()
        .map(|c| c[1] * c[2])
        .unwrap();
    println!("Part 1: {}", part1);

    let mut image = vec![b'8'; WIDTH * HEIGHT];
    for layer in data.chunks(WIDTH * HEIGHT).rev() {
        for (src, dst) in layer.iter().zip(&mut image) {
            match *src {
                b'2' => {}
                s => *dst = s,
            }
        }
    }

    println!("\nPart 2:");
    for row in image.chunks(WIDTH) {
        for col in row {
            print!("{}", if *col == b'1' { '#' } else { ' ' });
        }
        println!()
    }
}
