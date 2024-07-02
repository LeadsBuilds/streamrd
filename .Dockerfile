from rust:1.79

RUN groupadd -r -g 2200 streamrd && useradd -rM -g streamrd -u 2200 streamrd
USER $USER:2200

WORKDIR /home/rust/app

COPY --chown=2200 . .

EXPOSE 8000
RUN cargo install cargo-watch
RUN cargo build