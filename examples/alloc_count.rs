#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let src = "func:
    LD r0, 0
    LD r1, 100
loop:
    INC r0
    ADD r2, r0
    CMP r0, r1
    JLT loop
    ST [r3], r2
    JMP end
end:".to_string();
    let _profiler = dhat::Profiler::new_heap();

    let before = dhat::HeapStats::get();
    let _ = piku::assembler::Assembler::new(src).process();
    let after = dhat::HeapStats::get();

    println!("allocations: {}", after.total_blocks - before.total_blocks);
}
