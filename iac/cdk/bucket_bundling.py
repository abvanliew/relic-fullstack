from aws_cdk import (
  App,
  Stack,
  aws_s3 as s3,
  aws_s3_deployment as s3deploy,
  BundlingOptions,
  DockerImage,
)


def bundler():
  BundlingOptions(
    image=DockerImage.from_registry("rust:1.98-bookworm"),
    command=[
      "bash",
      "-c",
      """
        cargo build --release &&
        cp target/release/my-binary /asset-output/
        """,
    ],
  )
