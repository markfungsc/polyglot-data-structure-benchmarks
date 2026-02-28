# Minimal image to run make bench (Python, Java, C++, Rust).
FROM ubuntu:22.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 python3-pip pytest \
    openjdk-17-jdk maven \
    build-essential cmake \
    curl \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable \
    && . /root/.cargo/env && rustc --version \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /workspace
CMD ["make", "bench"]
