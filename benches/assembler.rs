use criterion::{
    criterion_group, criterion_main, Bencher, BatchSize, BenchmarkId, Criterion, Throughput,
};
use piku::assembler::Assembler;
use std::fmt::Write;
use std::hint::black_box;

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
    let mut s = String::new();
    for i in 0..blocks {
        push_block(&mut s, i);
    }
    s.push_str("    HLT\n");
    s
}

fn whitespace_program(lines: usize) -> String {
    let base = realistic_program(lines);
    let mut s = String::with_capacity(base.len() * 3);
    for line in base.lines() {
        writeln!(s).unwrap();
        writeln!(s, "        ").unwrap();
        writeln!(s, "\t\t{}   ", line.trim().replace(", ", ",     ")).unwrap();
    }
    s
}

fn forward_refs(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        writeln!(s, "    JMP label_{i}").unwrap();
    }
    for i in 0..n {
        writeln!(s, "label_{i}:\n    INC r0").unwrap();
    }
    s.push_str("    HLT\n");
    s
}

fn backward_refs(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        writeln!(s, "label_{i}:\n    INC r0").unwrap();
    }
    for i in 0..n {
        writeln!(s, "    JMP label_{i}").unwrap();
    }
    s.push_str("    HLT\n");
    s
}

fn operand_forms(lines: usize) -> String {
    const FORMS: &[&str] = &[
        "    LD r0, r1",
        "    LD r0, 1234",
        "    LD r0, [r1]",
        "    LD r0, [1234]",
        "    ST [r0], r1",
        "    ADD r0, r1",
        "    ADD r0, 7",
        "    CMP r0, r1",
        "    CMP r0, 7",
        "    PUSH r0",
        "    PUSH 7",
        "    POP r0",
        "    LDS r0, 2",
        "    STS 2, r0",
        "    OUT r0",
        "    OUT 65",
        "    NOT r0",
        "    JMP top",
        "    JEQ top",
        "    CALL top",
        "    RET",
    ];
    let mut s = String::from("top:\n");
    for i in 0..lines {
        s.push_str(FORMS[i % FORMS.len()]);
        s.push('\n');
    }
    s.push_str("    HLT\n");
    s
}

fn run(b: &mut Bencher, src: &str) {
    b.iter_batched(
        || src.to_string(),
        |s| Assembler::new(black_box(s)).process(),
        BatchSize::LargeInput,
    );
}

fn sanity_check(name: &str, src: &str) {
    Assembler::new(src.to_string())
        .process()
        .unwrap_or_else(|e| panic!("bench input `{name}` failed to assemble: {e}"));
}

fn full_assemble_realistic(c: &mut Criterion) {
    let src = realistic_program(3_000);
    sanity_check("full_assemble_realistic", &src);

    let mut group = c.benchmark_group("assemble");
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function("full_realistic", |b| {
        run(b, &src)
    });
    group.finish();
}

fn scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    for lines in [1_000usize, 2_000, 4_000, 8_000] {
        let src = realistic_program(lines);
        sanity_check("scaling", &src);
        group.throughput(Throughput::Bytes(src.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(lines), &src, |b, src| {
            run(b, src)
        });
    }
    group.finish();
}

fn labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("labels");
    for n in [500usize, 1_000, 2_000, 4_000] {
        let fwd = forward_refs(n);
        let bwd = backward_refs(n);
        sanity_check("labels_forward", &fwd);
        sanity_check("labels_backward", &bwd);

        group.bench_with_input(BenchmarkId::new("forward", n), &fwd, |b, src| {
            run(b, src)
        });
        group.bench_with_input(BenchmarkId::new("backward", n), &bwd, |b, src| {
            run(b, src)
        });
    }
    group.finish();
}

fn operand_forms_bench(c: &mut Criterion) {
    let src = operand_forms(5_000);
    sanity_check("operand_forms", &src);

    let mut group = c.benchmark_group("assemble");
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function("operand_forms", |b| {
        run(b, &src)
    });
    group.finish();
}

fn whitespace_heavy(c: &mut Criterion) {
    let src = whitespace_program(3_000);
    sanity_check("whitespace_heavy", &src);

    let mut group = c.benchmark_group("assemble");
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function("whitespace_heavy", |b| {
        run(b, &src)
    });
    group.finish();
}

criterion_group!(
    assembler,
    full_assemble_realistic,
    scaling,
    labels,
    operand_forms_bench,
    whitespace_heavy
);
criterion_main!(assembler);
