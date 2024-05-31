# Ref: https://github.com/aws/aws-lambda-base-images/blob/provided.al2023/README.md
FROM public.ecr.aws/lambda/provided:al2023 as builder

RUN dnf install -y \
    gcc \
    gcc-c++ \
    openssl-devel
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY Cargo.lock Cargo.toml ./
COPY src /app/src

# Build an executable
RUN rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl

# Using multi stage build to minimize the image size
FROM public.ecr.aws/lambda/provided:al2023

ARG BIN_NAME

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${BIN_NAME} /var/runtime/bootstrap
CMD [ "bootstrap" ]
