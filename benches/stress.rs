use criterion::{
    criterion_group, criterion_main, BatchSize, Criterion, Throughput,
};
use piku::assembler::Assembler;
use std::fmt::Write;
use std::hint::black_box;
use std::time::Duration;

const LINES: usize = 1_000_000;

fn push_block(s: &mut String, i: usize) {
    writeln!(
        s,
        "func_{i}:
    LD r0, 0
    LD r1, 100
loop_{i}:
    INC r0
    ADD r2, r0
    CMP r0, r1
    JLT loop_{i}
    ST [r3], r2
    JMP end_{i}
end_{i}:"
    )
    .unwrap();
}

fn realistic_program(lines: usize) -> String {
    let blocks = (lines / 10).max(1);
    let mut s = String::with_capacity(lines * 32);
    for i in 0..blocks {
        push_block(&mut s, i);
    }
    s.push_str("    HLT\n");
    s
}

fn stress(c: &mut Criterion) {
    let src = realistic_program(LINES);
    Assembler::new(src.clone())
        .process()
        .unwrap_or_else(|e| panic!("stress input failed to assemble: {e}"));

    eprintln!(
        "stress input: {} lines, {:.1} MB",
        src.lines().count(),
        src.len() as f64 / 1_000_000.0
    );

    let mut group = c.benchmark_group("stress");
    group.sample_size(10); // minimum Criterion allows
    group.measurement_time(Duration::from_secs(10));
    group.throughput(Throughput::Bytes(src.len() as u64));

    group.bench_function("process_1M_lines", |b| {
        b.iter_batched(
            || src.clone(),
            |s| Assembler::new(black_box(s)).process(),
            BatchSize::PerIteration,
        )
    });

    group.finish();
}

criterion_group!(benches, stress);
criterion_main!(benches);
