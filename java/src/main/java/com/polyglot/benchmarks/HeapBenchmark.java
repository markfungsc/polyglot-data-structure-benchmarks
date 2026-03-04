package com.polyglot.benchmarks;

import static com.polyglot.benchmarks.BenchmarkUtil.*;

import com.polyglot.MinHeap;
import java.io.File;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class HeapBenchmark {

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
        String csvPath = outDir + "/java_heap.csv";
        try (PrintWriter pw = new PrintWriter(csvPath)) {
            pw.println("N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb");
            for (int n : SCALES) {
                List<Integer> keys = new ArrayList<>(n);
                for (int i = 0; i < n; i++) keys.add(i);
                Collections.shuffle(keys);

                MinHeap warm = new MinHeap(n);
                for (int k : keys) warm.insert(k);
                for (int i = 0; i < n; i++) { warm.pop(); }

                double[] insertMs = new double[NUM_RUNS];
                double[] popMs = new double[NUM_RUNS];
                for (int run = 0; run < NUM_RUNS; run++) {
                    Collections.shuffle(keys);
                    MinHeap h = new MinHeap(n);
                    long start = System.nanoTime();
                    for (int k : keys) h.insert(k);
                    insertMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                    start = System.nanoTime();
                    for (int i = 0; i < n; i++) h.pop();
                    popMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                }
                double mem = memoryMb();
                double iMean = mean(insertMs);
                double iStd = std(insertMs, iMean);
                double pMean = mean(popMs);
                double pStd = std(popMs, pMean);
                pw.printf("%d,%.6f,%.6f,%.6f,%.6f,%.4f%n", n, iMean, iStd, pMean, pStd, mem);
                System.out.printf("N=%d: Insert %.6f ± %.6f ms, Pop %.6f ± %.6f ms, memory=%.4f MB%n", n, iMean, iStd, pMean, pStd, mem);
            }
            System.out.println("Wrote " + csvPath);
        }
    }
}
