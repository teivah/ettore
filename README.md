# Majorana

[Majorana](https://en.wikipedia.org/wiki/Ettore_Majorana) is a RISC-V virtual machine, written in Rust.

## Majorana Virtual Machine (MVM)

### MVM-1

MVM-1 is the first version of a RISC-V virtual machine.
It does not implement any of the known CPU optimizations like pipelining, out-of-order execution, multiple execution units, etc.

Here is the architecture, divided in 4 classic stages:
* Fetch: fetch an instruction from the main memory
* Decode: decode the instruction
* Execute: execute the RISC-V instruction
* Write: write-back the result to a register or the main memory

```
   +-------+
   | Fetch |
   +---+---+
       |
       |
   +---v----+
   | Decode |
   +---+----+
       |
       |
+------v--------+
|     ALU       |
|  +---------+  |
|  | Execute |  |
|  +---------+  |
+------+--------+
       |
       |
   +---v---+
   | Write |
   +-------+
```

## MVM-2

Compared to MVM-1, we add a cache for instructions called L1I (Level 1 Instructions) with a size of 64 KB. The caching policy is straightforward: as soon as we meet an instruction that is not present in L1I, we fetch a cache line of 64 KB instructions from the main memory, and we cache it into LI1.

```
+-----+     +-------+
| L1I <-----+ Fetch |
+-----+     +---+---+
                |
                |
            +---v----+
            | Decode |
            +---+----+
                |
                |
         +------v--------+
         |     ALU       |
         |  +---------+  |
         |  | Execute |  |
         |  +---------+  |
         +---------------+
                |
                |
            +---v---+
            | Write |
            +-------+

```

## MVM-3

MVM-3 keeps the same architecture than MVM-2 with 4 stages and L1I. Yet, this version implements [pipelining](https://en.wikipedia.org/wiki/Instruction_pipelining). 

In a nutshell, pipelining allows to keep every stage as busy as possible. For example, as soon as the fetch unit has fetched an instruction, it will not wait for the instruction to be decoded, executed and written. It will fetch another instruction straight away during the next cycle(s).

This way, the first instruction can be executed in 4 cycles (assuming the fetch is done from L1I), whereas the next instructions will be executed in only 1 cycle.

One of the complexity with pipelining is to handle conditional branches. What if we fetch a [bge](https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html#bge) instruction for example? The next instruction fetched will not be necessarily the one we should have fetched/decoded/executed/written. As a solution, we implemented a first version of branch prediction handled by the Branch Unit. 

The Branch Unit takes the hypothesis that a condition branch will **not** be taken. Hence, after having fetched an instruction, regardless if it's a conditional branch, we will fetch the next instruction after it. If the prediction was wrong, we need to flush the pipeline, revert the program counter to the destination marked by the conditional branch instruction, and continue the execution.

Of course, pipeline flushing has an immediate performance impact. Modern CPUs have a branch prediction mechanism that is move evolved than MVM-3.

```
+-----+     +-------+
| L1I <-----+ Fetch +------------+
+-----+     +---+---+            |
                |         +------v------+
                |         | Branch Unit |
                |         +------^------+
            +---v----+           |
            | Decode <-----------+
            +---+----+
                |
                |
         +------v--------+
         |     ALU       |
         |  +---------+  |
         |  | Execute |  |
         |  +---------+  |
         +---------------+
                |
                |
            +---v---+
            | Write |
            +-------+
```

## Benchmarks

All the benchmarks are executed at a fixed CPU clock frequency: 2.3 GHz.

As a reference, we have also executed a benchmark on an Intel i5-7360U (2.3 GHz as well).

### Is Prime Number

RISC source: [prime-number.asm](res/risc/prime-number.asm)

|Machine|n=1109|
|:--------:|:-------------:|
|i5-7360U|253 ns|
|MVM-1|64100 ns, ~253 times slower|
|MVM-2|4939 ns, ~19 times slower|
|MVM-3|1777 ns, ~7 times slower|