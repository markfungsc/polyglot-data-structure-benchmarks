package com.polyglot.benchmarks;

import static com.polyglot.benchmarks.BenchmarkUtil.*;

import java.io.File;
import java.io.PrintWriter;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;

public class ConcurrencyBenchmark {

  private static final int TOTAL_ITEMS = 100_000;
  private static final int QUEUE_CAPACITY = 4096;
  private static final int[][] CONFIGS = {{1, 1}, {2, 2}, {4, 4}, {8, 8}, {4, 1}, {1, 4}};

  /** Sentinel: after producers finish, one per consumer so each consumer can exit. */
  private static final Integer POISON = -1;

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
    String csvPath = outDir + "/java_concurrency.csv";
    try (PrintWriter pw = new PrintWriter(csvPath)) {
      pw.println(
          "num_producers,num_consumers,capacity,total_items,elapsed_mean_ms,elapsed_std_ms,"
              + "throughput_per_sec_mean,memory_mb");
      for (int[] config : CONFIGS) {
        int P = config[0];
        int C = config[1];
        double[] elapsedMs = new double[NUM_RUNS];
        for (int run = 0; run < NUM_RUNS; run++) {
          elapsedMs[run] = runOne(P, C, TOTAL_ITEMS, QUEUE_CAPACITY);
        }
        double eMean = mean(elapsedMs);
        double eStd = std(elapsedMs, eMean);
        double throughput = TOTAL_ITEMS / (eMean / 1000.0);
        double mem = memoryMb();
        pw.printf(
            "%d,%d,%d,%d,%.6f,%.6f,%.6f,%.4f%n",
            P, C, QUEUE_CAPACITY, TOTAL_ITEMS, eMean, eStd, throughput, mem);
        System.out.printf(
            "P=%d C=%d: elapsed %.6f ± %.6f ms, throughput %.0f/s, memory %.4f MB%n",
            P, C, eMean, eStd, throughput, mem);
      }
      System.out.println("Wrote " + csvPath);
    }
  }

  private static double runOne(int numProducers, int numConsumers, int totalItems, int capacity)
      throws InterruptedException {
    BlockingQueue<Integer> queue = new ArrayBlockingQueue<>(capacity);

    long start = System.nanoTime();

    int perProducer = totalItems / numProducers;
    Thread[] producers = new Thread[numProducers];
    for (int p = 0; p < numProducers; p++) {
      int begin = p * perProducer;
      int end = (p == numProducers - 1) ? totalItems : (p + 1) * perProducer;
      producers[p] =
          new Thread(
              () -> {
                try {
                  for (int i = begin; i < end; i++) {
                    queue.put(i);
                  }
                } catch (InterruptedException e) {
                  Thread.currentThread().interrupt();
                  throw new RuntimeException(e);
                }
              });
      producers[p].start();
    }

    Thread[] consumers = new Thread[numConsumers];
    for (int c = 0; c < numConsumers; c++) {
      consumers[c] =
          new Thread(
              () -> {
                try {
                  while (true) {
                    Integer v = queue.take();
                    if (v == POISON) {
                      break;
                    }
                  }
                } catch (InterruptedException e) {
                  Thread.currentThread().interrupt();
                  throw new RuntimeException(e);
                }
              });
      consumers[c].start();
    }

    for (Thread t : producers) {
      t.join();
    }
    for (int i = 0; i < numConsumers; i++) {
      queue.put(POISON);
    }
    for (Thread t : consumers) {
      t.join();
    }

    return (System.nanoTime() - start) / 1_000_000.0;
  }
}
