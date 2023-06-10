export RUST_LOG := 'info'

default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

deploy branch='master' domain='45.33.94.40':
  ssh root@{{domain}} "mkdir -p deploy \
    && apt-get update --yes \
    && apt-get upgrade --yes \
    && apt-get install --yes git rsync"
  rsync -avz deploy/checkout root@{{domain}}:deploy/checkout
  ssh root@{{domain}} 'cd deploy && ./checkout {{branch}} {{domain}}'

fmt:
  cargo fmt

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

run *args:
  cargo run -- {{args}}

test:
  cargo test

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
