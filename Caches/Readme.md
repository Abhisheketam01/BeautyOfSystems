# Why Caches Matter – Proof of Work

## Problem
CPUs are extremely fast, but memory and disk are slow.
This gap forces CPUs to wait, wasting performance.

## Idea
Modern systems use cache memories to keep frequently used data
close to the processor by exploiting locality.

## Experiment
I wrote a C program that:
- Writes to a large array sequentially (cache-friendly)
- Writes to the same array randomly (cache-hostile)

## Observation
Sequential access is significantly faster than random access,
even though both access the same memory.

## Conclusion
Caches dramatically improve performance.
Access patterns matter as much as algorithms.

## Key Lesson
Performance is not only about computation —
it is about data movement.
