import aws_cdk as cdk

from aws_cdk import App


def main():
  app = App()
  # stack goes here
  _ = app.synth()


if __name__ == "main":
  main()
