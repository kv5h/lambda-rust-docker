# lambda-rust-docker

A sample repository for building Docker image for AWS Lambda for Rust lang.

## How-to

### Write codes

For details, see a sample code: [./src/main.rs](./src/main.rs)

### Test

As usual, run:

```bash
cargo test
```

### Write Dockerfile

The goal of this repository is to create Docker image for AWS Lambda.

The image definition is based on [./Dockerfile](./Dockerfile), edit it as you
need.

### Build the image

```bash
docker compose build
```

> [!TIPS]
>
> If your machine and the Lambda's architecture are different, adding for
> example `--platform=linux/amd64` will help. You can configure it in
> [./compose.yaml](./compose.yaml). Note that the flag is commented out by
> default.

Now a Docker image is created as `lambda-rust-docker:latest`, follow
[Pushing a Docker image to an Amazon ECR private repository](https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-push-ecr-image.html)
to push your image to ECR.

## Reference

- [github.com/awslabs/aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
