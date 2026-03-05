package com.polyglot.benchmarks;

/** Shared constants and helpers for benchmark classes (same schema and methodology). */
public final class BenchmarkUtil {
  public static final int[] SCALES = {1_000, 10_000, 100_000, 1_000_000};
  public static final int NUM_RUNS = 5;

  private BenchmarkUtil() {}

  public static String getResultsDir() {
    String outDir = System.getenv("RESULTS_DIR");
    if (outDir == null) {
      outDir = System.getProperty("results.dir", "../results/raw");
    }
    return outDir;
  }

  public static double mean(double[] a) {
    double s = 0;
    for (double x : a) {
      s += x;
    }
    return s / a.length;
  }

  public static double std(double[] a, double mean) {
    if (a.length < 2) {
      return 0;
    }
    double s = 0;
    for (double x : a) {
      s += (x - mean) * (x - mean);
    }
    return Math.sqrt(s / (a.length - 1));
  }

  public static double memoryMb() {
    Runtime rt = Runtime.getRuntime();
    return (rt.totalMemory() - rt.freeMemory()) / 1_000_000.0;
  }
}
