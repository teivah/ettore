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

## Benchmarks

### Is Prime Number

RISC source: [prime-number.asm](res/risc/prime-number.asm)

|Machine|n=1109|
|:--------:|:-------------:|
|i5-7360U|253 ns|
|MVM-1|64033 ns|