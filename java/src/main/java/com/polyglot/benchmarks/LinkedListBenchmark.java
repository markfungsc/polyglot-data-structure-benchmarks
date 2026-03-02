package com.polyglot.benchmarks;

import static com.polyglot.benchmarks.BenchmarkUtil.*;

import com.polyglot.LinkedList;
import java.io.File;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class LinkedListBenchmark {

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
        String csvPath = outDir + "/java_linked_list.csv";
        try (PrintWriter pw = new PrintWriter(csvPath)) {
            pw.println("N,insert_mean_ms,insert_std_ms,get_mean_ms,get_std_ms,memory_mb");
            for (int n : SCALES) {
                List<Integer> keys = new ArrayList<>(n);
                for (int i = 0; i < n; i++) keys.add(i);
                Collections.shuffle(keys);

                LinkedList warm = new LinkedList();
                for (int k : keys) warm.push(k);
                for (int i = 0; i < n; i++) warm.get(i);

                double[] insertMs = new double[NUM_RUNS];
                double[] getMs = new double[NUM_RUNS];
                for (int run = 0; run < NUM_RUNS; run++) {
                    Collections.shuffle(keys);
                    LinkedList list = new LinkedList();
                    long start = System.nanoTime();
                    for (int k : keys) list.push(k);
                    insertMs[run] = (System.nanoTime() - start) / 1_000_000.0;
                    start = System.nanoTime();
                    for (int i = 0; i < n; i++) list.get(i);
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
            System.out.println("Wrote " + csvPath);
        }
    }
}
