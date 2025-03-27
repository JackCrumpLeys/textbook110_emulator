use criterion::{criterion_group, criterion_main, Criterion};
use tools_for_210::turing::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut tm = TuringMachine::new();
    tm.rules.insert(
        (0, 0b00),
        Rule {
            write: 0b11,
            move_right: true,
            next_state: 0,
        },
    );
    tm.rules.insert(
        (0, 0b11),
        Rule {
            write: 0b00,
            move_right: true,
            next_state: 0,
        },
    );
    tm.rules.insert(
        (0, 0b10),
        Rule {
            write: 0b10,
            move_right: false,
            next_state: 1,
        },
    );
    tm.rules.insert(
        (1, 0b00),
        Rule {
            write: 0b11,
            move_right: false,
            next_state: 1,
        },
    );
    tm.rules.insert(
        (1, 0b11),
        Rule {
            write: 0b00,
            move_right: false,
            next_state: 1,
        },
    );
    tm.rules.insert(
        (1, 0b10),
        Rule {
            write: 0b10,
            move_right: true,
            next_state: 0,
        },
    );

    tm.tape = vec![3, 3, 0xAA];

    // while !tm.step() {
    //     for i in tm.tape.iter() {
    //         for j in 0..4 {
    //             match (i >> (6 - j * 2)) & 0b11 {
    //                 0b00 => print!("0"),
    //                 0b11 => print!("1"),
    //                 0b10 => print!("b"),
    //                 0b01 => print!("E"),
    //                 _ => panic!(),
    //             }
    //         }
    //         print!(" ");
    //     }
    //     println!();
    // }

    c.bench_function("forever machine", |b| b.iter(|| tm.step()));
    assert!(!tm.step())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
