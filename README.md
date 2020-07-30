# Majorana

[Majorana](https://en.wikipedia.org/wiki/Ettore_Majorana) is a set of open-source RISC-V cores written in Rust.

## Majorana Virtual Machine (MVM)

### MVM-1

MVM-1 is the first virtual machine implementation.
It does not implement any of the known CPU optimizations like pipelining, out-of-order execution, multiple execution units, etc.

Here is the architecture, divided in 4 classic stages:

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

MVM-1 is the starting point to build more advanced virtual machines.

## MVM-2

Compared to MVM-1, we add a first level of instructions caching called L1I (Level 1 Instructions): 

```
   +-------+     +-----+
   | Fetch +-----> L1I |
   +---+---+     +-----+
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

The cache size is 64 bytes and is composed of a single cache line of 64 bytes.

## Benchmarks

All the benchmarks are executed at a fixed CPU clock frequency: 2.3 GHz.

### Is Prime Number

RISC source: [prime-number.asm](res/risc/prime-number.asm)

|Machine|n=1109|
|:--------:|:-------------:|
|i5-7360U|253 ns|
|MVM-1|64033 ns|
|MVM-2|4914 ns|