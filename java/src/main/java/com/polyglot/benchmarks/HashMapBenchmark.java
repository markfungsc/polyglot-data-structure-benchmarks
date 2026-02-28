package com.polyglot.benchmarks;

import com.polyglot.HashMapCustom;
import java.io.File;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class HashMapBenchmark {
    private static final int[] SCALES = { 1_000, 10_000, 100_000, 1_000_000 };
    private static final int NUM_RUNS = 5;
    private static final int LOW_ENTROPY_CAPACITY = 64;  // low-entropy / near-collision: few buckets
    private static final int LOAD_FACTOR_N = 100_000;
    private static final double[] LOAD_FACTORS = { 0.25, 0.5, 0.75, 1.0 };

    public static void main(String[] args) {
        try {
            String outDir = System.getenv("RESULTS_DIR");
            if (outDir == null) outDir = System.getProperty("results.dir", "../results/raw");
            new File(outDir).mkdirs();
            runMain(outDir);
            runLowEntropy(outDir);
            runLoadFactor(outDir);
        } catch (Throwable t) {
            t.printStackTrace();
            System.exit(1);
        }
    }

    private static double mean(double[] a) {
        double s = 0;
        for (double x : a) s += x;
        return s / a.length;
    }

    private static double std(double[] a, double mean) {
        if (a.length < 2) return 0;
        double s = 0;
        for (double x : a) s += (x - mean) * (x - mean);
        return Math.sqrt(s / (a.length - 1));
    }

    private static double memoryMb() {
        Runtime rt = Runtime.getRuntime();
        return (rt.totalMemory() - rt.freeMemory()) / 1_000_000.0;
    }

    private static void runMain(String outDir) throws Exception {
        String csvPath = outDir + "/java_hashmap.csv";
        try (PrintWriter pw = new PrintWriter(csvPath)) {
            pw.println("N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb");
            for (int n : SCALES) {
                List<Integer> keys = new ArrayList<>(n);
                for (int i = 0; i < n; i++) keys.add(i);
                Collections.shuffle(keys);
                HashMapCustom<Integer, Integer> warm = new HashMapCustom<>(n);
                for (int k : keys) warm.insert(k, k);
                for (int k : keys) warm.get(k);

                double[] insertMs = new double[NUM_RUNS];
                double[] getMs = new double[NUM_RUNS];
                for (int run = 0; run < NUM_RUNS; run++) {
                    Collections.shuffle(keys);
                    HashMapCustom<Integer, Integer> map = new HashMapCustom<>(n);
                    long start = System.nanoTime();
                    for (int k : keys) map.insert(k, k);
                    insertMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                    start = System.nanoTime();
                    for (int k : keys) map.get(k);
                    getMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                }
                double mem = memoryMb();
                double iMean = mean(insertMs);
                double iStd = std(insertMs, iMean);
                double gMean = mean(getMs);
                double gStd = std(getMs, gMean);
                pw.printf("%d,%.6f,%.6f,%.6f,%.6f,%.4f%n", n, iMean, iStd, gMean, gStd, mem);
                System.out.printf("N=%d: Insert %.6f ± %.6f ms, Get %.6f ± %.6f ms, memory=%.4f MB%n", n, iMean, iStd, gMean, gStd, mem);
            }
        }
        System.out.println("Wrote " + csvPath);
    }

    private static void runLowEntropy(String outDir) throws Exception {
        String csvPath = outDir + "/java_hashmap_low_entropy.csv";
        try (PrintWriter pw = new PrintWriter(csvPath)) {
            pw.println("N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms");
            for (int n : SCALES) {
                List<Integer> keys = new ArrayList<>(n);
                for (int i = 0; i < n; i++) keys.add(i);
                Collections.shuffle(keys);
                HashMapCustom<Integer, Integer> warm = new HashMapCustom<>(LOW_ENTROPY_CAPACITY);
                for (int k : keys) warm.insert(k, k);
                for (int k : keys) warm.get(k);

                double[] insertMs = new double[NUM_RUNS];
                double[] getMs = new double[NUM_RUNS];
                for (int run = 0; run < NUM_RUNS; run++) {
                    Collections.shuffle(keys);
                    HashMapCustom<Integer, Integer> map = new HashMapCustom<>(LOW_ENTROPY_CAPACITY);
                    long start = System.nanoTime();
                    for (int k : keys) map.insert(k, k);
                    insertMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                    start = System.nanoTime();
                    for (int k : keys) map.get(k);
                    getMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                }
                double iMean = mean(insertMs);
                double iStd = std(insertMs, iMean);
                double gMean = mean(getMs);
                double gStd = std(getMs, gMean);
                pw.printf("%d,%.6f,%.6f,%.6f,%.6f%n", n, iMean, iStd, gMean, gStd);
                System.out.printf("Low-entropy N=%d: Insert %.6f ± %.6f ms, Get %.6f ± %.6f ms%n", n, iMean, iStd, gMean, gStd);
            }
        }
        System.out.println("Wrote " + csvPath);
    }

    private static void runLoadFactor(String outDir) throws Exception {
        String csvPath = outDir + "/java_hashmap_loadfactor.csv";
        int n = LOAD_FACTOR_N;
        try (PrintWriter pw = new PrintWriter(csvPath)) {
            pw.println("load_factor,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms");
            for (double lf : LOAD_FACTORS) {
                int capacity = Math.max(16, (int) (n / lf));
                List<Integer> keys = new ArrayList<>(n);
                for (int i = 0; i < n; i++) keys.add(i);
                Collections.shuffle(keys);
                HashMapCustom<Integer, Integer> warm = new HashMapCustom<>(capacity);
                for (int k : keys) warm.insert(k, k);
                for (int k : keys) warm.get(k);

                double[] insertMs = new double[NUM_RUNS];
                double[] getMs = new double[NUM_RUNS];
                for (int run = 0; run < NUM_RUNS; run++) {
                    Collections.shuffle(keys);
                    HashMapCustom<Integer, Integer> map = new HashMapCustom<>(capacity);
                    long start = System.nanoTime();
                    for (int k : keys) map.insert(k, k);
                    insertMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                    start = System.nanoTime();
                    for (int k : keys) map.get(k);
                    getMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                }
                double iMean = mean(insertMs);
                double iStd = std(insertMs, iMean);
                double gMean = mean(getMs);
                double gStd = std(getMs, gMean);
                pw.printf("%.2f,%.6f,%.6f,%.6f,%.6f%n", lf, iMean, iStd, gMean, gStd);
                System.out.printf("LoadFactor=%.2f: Insert %.6f ± %.6f ms, Get %.6f ± %.6f ms%n", lf, iMean, iStd, gMean, gStd);
            }
        }
        System.out.println("Wrote " + csvPath);
    }
}
