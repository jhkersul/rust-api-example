FROM docker/whalesay:latest
LABEL Name=rust_api_example
RUN apt-get -y update && apt-get install -y fortunes
CMD ["sh", "-c", "/usr/games/fortune -a | cowsay"]
