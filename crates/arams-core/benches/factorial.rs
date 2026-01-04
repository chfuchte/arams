use arams_core::{compile, execute};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_factorial_execution(c: &mut Criterion) {
    const FACTORIAL_PROGRAM: &str = "
    load 1
    jzero return_one
    sub #1
    jzero return_one
    load 1
    store 2
    loop: load 1
    sub #1
    jzero break
    store 1
    mul 2
    store 2
    goto loop
    return_one: load #1
    end
    break: load 2
    end";

    let program = compile(FACTORIAL_PROGRAM);
    assert!(program.is_ok());
    let program = program.unwrap();

    // calculate all factorials from 0! to 12!
    c.bench_function("factorial 0! to 12!", |b| {
        b.iter(|| {
            for n in 0..=12 {
                let regs_map = std::collections::HashMap::from([(1, n)]);
                let execution_result = execute(program.clone(), Some(regs_map));
                assert!(execution_result.is_ok());
                let machine = execution_result.unwrap();
                let result = machine.get_accumulator();
                assert!(match n {
                    0 | 1 => result == 1,
                    2 => result == 2,
                    3 => result == 6,
                    4 => result == 24,
                    5 => result == 120,
                    6 => result == 720,
                    7 => result == 5040,
                    8 => result == 40320,
                    9 => result == 362880,
                    10 => result == 3628800,
                    11 => result == 39916800,
                    12 => result == 479001600,
                    _ => false,
                });
                black_box(result);
            }
        })
    });
}

criterion_group!(benches, bench_factorial_execution);
criterion_main!(benches);
