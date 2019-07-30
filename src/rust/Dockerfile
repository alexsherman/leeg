FROM ubuntu:latest AS builder

# install prereqs
RUN apt-get update && apt-get install -y curl libpq-dev build-essential libssl-dev pkg-config

# install nightly
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2019-05-07

# add cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

# build banana
RUN cd banana && cargo build --release

FROM ubuntu:latest

# Install Redis. https://github.com/dockerfile/redis/blob/master/Dockerfile
RUN \
 apt-get update && \
  apt-get install -y wget build-essential tar sed && \
  cd /tmp && \
  wget http://download.redis.io/redis-stable.tar.gz && \
  tar xvzf redis-stable.tar.gz && \
  cd redis-stable && \
  make && \
  make install && \
  cp -f src/redis-sentinel /usr/local/bin && \
  mkdir -p /etc/redis && \
  cp -f *.conf /etc/redis && \
  rm -rf /tmp/redis-stable* && \
  sed -i 's/^\(bind .*\)$/# \1/' /etc/redis/redis.conf && \
  sed -i 's/^\(daemonize .*\)$/# \1/' /etc/redis/redis.conf && \
  sed -i 's/^\(dir .*\)$/# \1\ndir \/data/' /etc/redis/redis.conf && \
  sed -i 's/^\(logfile .*\)$/# \1/' /etc/redis/redis.conf
# Define mountable directories.
VOLUME ["/data"]

# Define working directory.
WORKDIR /data

RUN redis-server /etc/redis/redis.conf &

RUN apt-get update && apt-get install -y libpq-dev

RUN ls

COPY --from=builder \
  banana/target/release/banana \
  /usr/local/bin/

WORKDIR /root
ENV ROCKET_ENV="production"

CMD ROCKET_PORT=$PORT /usr/local/bin/banana