package com.polyglot.benchmarks;

import static com.polyglot.benchmarks.BenchmarkUtil.*;

import com.polyglot.LRUCache;
import java.io.File;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.List;

public class LRUCacheBenchmark {

  public static void main(String[] args) {
    try {
      String outDir = getResultsDir();
      new File(outDir).mkdirs();
      runMain(outDir);
    } catch (Throwable t) {
      t.printStackTrace();
      System.exit(1);
    }
  }

  private static void runMain(String outDir) throws Exception {
    String csvPath = outDir + "/java_lru_cache.csv";
    try (PrintWriter pw = new PrintWriter(csvPath)) {
      pw.println(
          "N,put_miss_mean_ms,put_miss_std_ms,put_hit_mean_ms,put_hit_std_ms,get_hit_mean_ms,get_hit_std_ms,get_miss_mean_ms,get_miss_std_ms,eviction_mean_ms,eviction_std_ms,memory_mb");
      for (int n : SCALES) {
        List<Integer> keys = new ArrayList<>(n);
        for (int i = 0; i < n; i++) {
          keys.add(i);
        }
        int capacity = Math.max(16, n);

        // Warm-up: build and use cache once at this scale
        LRUCache warm = new LRUCache(capacity);
        for (int k : keys) {
          warm.put(k, k);
        }
        for (int k : keys) {
          warm.get(k);
        }

        double[] putMissMs = new double[NUM_RUNS];
        double[] putHitMs = new double[NUM_RUNS];
        double[] getHitMs = new double[NUM_RUNS];
        double[] getMissMs = new double[NUM_RUNS];
        double[] evictionMs = new double[NUM_RUNS];

        for (int run = 0; run < NUM_RUNS; run++) {
          // put_miss: cache empty, then (capacity-1) puts of new keys (no eviction)
          LRUCache cache = new LRUCache(capacity);
          long start = System.nanoTime();
          for (int i = 0; i < capacity - 1; i++) {
            cache.put(i, i);
          }
          putMissMs[run] = (System.nanoTime() - start) / 1_000_000.0;

          // put_hit: full cache, N updates of existing keys
          cache = new LRUCache(capacity);
          for (int k : keys) {
            cache.put(k, k);
          }
          start = System.nanoTime();
          for (int i = 0; i < n; i++) {
            cache.put(i % capacity, i);
          }
          putHitMs[run] = (System.nanoTime() - start) / 1_000_000.0;

          // get_hit: full cache, N lookups of existing keys
          cache = new LRUCache(capacity);
          for (int k : keys) {
            cache.put(k, k);
          }
          start = System.nanoTime();
          for (int i = 0; i < n; i++) {
            cache.get(i % capacity);
          }
          getHitMs[run] = (System.nanoTime() - start) / 1_000_000.0;

          // get_miss: full cache, N lookups of a key not in cache
          cache = new LRUCache(capacity);
          for (int k : keys) {
            cache.put(k, k);
          }
          int missing = n; // not in 0..n
          start = System.nanoTime();
          for (int i = 0; i < n; i++) {
            cache.get(missing);
          }
          getMissMs[run] = (System.nanoTime() - start) / 1_000_000.0;

          // eviction: full cache, N puts of new keys so each put evicts LRU
          cache = new LRUCache(capacity);
          for (int k : keys) {
            cache.put(k, k);
          }
          start = System.nanoTime();
          for (int i = n; i < 2 * n; i++) {
            cache.put(i, i);
          }
          evictionMs[run] = (System.nanoTime() - start) / 1_000_000.0;
        }

        double mem = memoryMb();
        double pmMean = mean(putMissMs);
        double pmStd = std(putMissMs, pmMean);
        double phMean = mean(putHitMs);
        double phStd = std(putHitMs, phMean);
        double ghMean = mean(getHitMs);
        double ghStd = std(getHitMs, ghMean);
        double gmMean = mean(getMissMs);
        double gmStd = std(getMissMs, gmMean);
        double evMean = mean(evictionMs);
        double evStd = std(evictionMs, evMean);
        pw.printf(
            "%d,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.4f%n",
            n, pmMean, pmStd, phMean, phStd, ghMean, ghStd, gmMean, gmStd, evMean, evStd, mem);
        System.out.printf(
            "N=%d: put_miss %.6f±%.6f ms, put_hit %.6f±%.6f ms, get_hit %.6f±%.6f ms, get_miss %.6f±%.6f ms, eviction %.6f±%.6f ms, memory=%.4f MB%n",
            n, pmMean, pmStd, phMean, phStd, ghMean, ghStd, gmMean, gmStd, evMean, evStd, mem);
      }
      System.out.println("Wrote " + csvPath);
    }
  }
}
