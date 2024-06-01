# lambda-rust-docker

A sample repository for building a Rust Docker image for AWS Lambda.

## How-to

### Write codes

For details, see a sample code: [`src/main.rs`](./src/main.rs)

### Test

As usual, run:

```bash
cargo test
```

### Write Dockerfile

The goal of this repository is to create Docker image for AWS Lambda.

The image definition is based on [`Dockerfile`](./Dockerfile), edit it as you
need.

### Build the image

```bash
docker compose build
```

> [!TIP]
>
> If your machine and the Lambda's architecture are different, adding for
> example `--platform=linux/amd64` will help. You can configure it in
> [`compose.yaml`](./compose.yaml). Note that the flag is commented out by
> default.

Now a Docker image is created as `lambda-rust-docker:latest`, follow
[Pushing a Docker image to an Amazon ECR private repository](https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-push-ecr-image.html)
to push your image to ECR.

## Appendix

### Local dev server with Cargo Lambda

Here are some samples of using
[`cargo lambda watch`](https://www.cargo-lambda.info/commands/watch.html) and
[`cargo lambda invoke`](https://www.cargo-lambda.info/commands/invoke.html).

Start watching:

```bash
>>> cargo lambda watch -p 19999 -a 127.0.0.1
 INFO invoke server waiting for requests socket_addr=127.0.0.1:19999
 INFO starting lambda function function="_" manifest="Cargo.toml" cmd=Exec
 { prog: "cargo", args: ["run", "--manifest-path", "Cargo.toml", "--color", "auto"] }
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/lambda-rust-docker`
```

And then, invoking:

```bash
>>> cargo lambda invoke lambda-rust-docker \
    --data-ascii '{ "key": "xxxx" }' -p 19999 -a 127.0.0.1
{"key":"xxxx"}
```

## Reference

- [github.com/awslabs/aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
