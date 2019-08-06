#Steps below are commented out because google app engine seems to 
#not have these layers cached and it takes 20 min to compile all libs
#while testing, just take the binary from your local

#FROM ubuntu:latest AS builder

# install prereqs
#RUN apt-get update && apt-get install -y curl libpq-dev libssl-dev pkg-config

# install nightly
#RUN curl https://sh.rustup.rs/ -sSf | \
#  sh -s -- -y --default-toolchain nightly-2019-05-07

# add cargo to path
#ENV PATH="/root/.cargo/bin:${PATH}"
# build banana
#RUN cd banana && cargo build --release

FROM ubuntu:latest

ADD . ./

RUN apt-get update && apt-get install -y libpq-dev

#requires u to have already compiled banana
COPY banana/target/release/banana /usr/local/bin/

WORKDIR /root

COPY banana/Db_config.toml .

COPY banana/Rocket.toml .

CMD ROCKET_ENV="production" RUST_LOG="debug" ROCKET_PORT=$PORT /usr/local/bin/banana