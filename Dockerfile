FROM ubuntu:xenial

RUN apt-get update \
 && apt-get install -y -q \
    curl \
    gcc \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m -r -G sudo rustacean && passwd -d rustacean

USER rustacean

WORKDIR /home/rustacean

RUN curl https://sh.rustup.rs -sSf > rustup-init \
 && chmod +x rustup-init \
 && ./rustup-init -y

ENV PATH=$PATH:/home/rustacean/.cargo/bin \
    USER=rustacean

CMD bash
