#!/bin/bash
# Performance comparison script

PATTERN="^[Xx][Yy]"  # Pattern complex enough to take a few seconds

echo "Running with all available threads..."
START=$(date +%s)
cargo run --release -- "$PATTERN"
END=$(date +%s)
ALL_THREADS_TIME=$((END - START))
echo "Time with all threads: $ALL_THREADS_TIME seconds"

echo ""
echo "Running with 1 thread only..."
START=$(date +%s)
cargo run --release -- "$PATTERN" --threads 1
END=$(date +%s)
SINGLE_THREAD_TIME=$((END - START))
echo "Time with 1 thread: $SINGLE_THREAD_TIME seconds"

SPEEDUP=$(echo "scale=2; $SINGLE_THREAD_TIME / $ALL_THREADS_TIME" | bc)
echo ""
echo "Performance Summary:"
echo "--------------------"
echo "All threads: $ALL_THREADS_TIME seconds"
echo "Single thread: $SINGLE_THREAD_TIME seconds"
echo "Speedup factor: ${SPEEDUP}x"