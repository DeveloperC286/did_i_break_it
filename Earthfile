VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


COPY_METADATA:
    COMMAND
    DO +COPY_CI_DATA
    COPY --dir ".git/" "./"


rust-base:
    FROM rust:1.78.0-alpine3.20@sha256:214477ec837f9bedd80be4b087fec09e3f270831979412840f3f6c38e5a0d9c1
    # renovate: datasource=repology depName=alpine_3_20/bash versioning=loose
    ENV BASH_VERSION="5.2.26-r0"
    RUN apk add --no-cache bash=$BASH_VERSION
    # renovate: datasource=repology depName=alpine_3_20/musl-dev versioning=loose
    ENV MUSL_VERSION="1.2.5-r0"
    # renovate: datasource=repology depName=alpine_3_20/openssl-dev versioning=loose
    ENV OPENSSL_VERSION="3.3.2-r1"
    RUN apk add --no-cache bash=$BASH_VERSION musl-dev=$MUSL_VERSION openssl-dev=$OPENSSL_VERSION openssl-libs-static=$OPENSSL_VERSION
    RUN rustup component add rustfmt clippy
    WORKDIR "/did_i_break_it"


check-clean-git-history:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/clean_git_history
    ENV CLEAN_GIT_HISTORY_VERSION="v0.2.0"
    RUN wget -O - "https://github.com/DeveloperC286/clean_git_history/releases/download/${CLEAN_GIT_HISTORY_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d > /usr/bin/clean_git_history && chmod 755 /usr/bin/clean_git_history
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-clean-git-history.sh --from-reference "${from_reference}"


check-conventional-commits-linting:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
    ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.14.3"
    RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d > /usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-conventional-commits-linting.sh --from-reference "${from_reference}"


COPY_SOURCECODE:
    COMMAND
    DO +COPY_CI_DATA
    COPY --if-exists "Cargo.lock" "./"
    COPY --dir "Cargo.toml" "src/" "./"


sourcecode-base:
    FROM +rust-base
    DO +COPY_SOURCECODE


check-rust-formatting:
    FROM +sourcecode-base
    RUN ./ci/check-rust-formatting.sh


golang-base:
    FROM golang:1.23.5@sha256:8c10f21bec412f08f73aa7b97ca5ac5f28a39d8a88030ad8a339fd0a781d72b4
    WORKDIR "/did_i_break_it"


shell-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=mvdan/sh
    ENV SHFMT_VERSION="v3.10.0"
    RUN go install mvdan.cc/sh/v3/cmd/shfmt@$SHFMT_VERSION
    DO +COPY_CI_DATA


check-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/check-shell-formatting.sh


yaml-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=google/yamlfmt
    ENV YAMLFMT_VERSION="v0.15.0"
    RUN go install github.com/google/yamlfmt/cmd/yamlfmt@$YAMLFMT_VERSION
    COPY ".yamlfmt" "./"
    DO +COPY_CI_DATA


check-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/check-yaml-formatting.sh


check-formatting:
    BUILD +check-rust-formatting
    BUILD +check-shell-formatting
    BUILD +check-yaml-formatting


fix-rust-formatting:
    FROM +sourcecode-base
    RUN ./ci/fix-rust-formatting.sh
    SAVE ARTIFACT "src/" AS LOCAL "./"


fix-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/fix-shell-formatting.sh
    SAVE ARTIFACT "ci/" AS LOCAL "./"


fix-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/fix-yaml-formatting.sh
    SAVE ARTIFACT ".github/" AS LOCAL "./"


fix-formatting:
    BUILD +fix-rust-formatting
    BUILD +fix-shell-formatting
    BUILD +fix-yaml-formatting


check-rust-linting:
    FROM +sourcecode-base
    RUN ./ci/check-rust-linting.sh


check-shell-linting:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/shellcheck versioning=loose
    ENV SHELLCHECK_VERSION="0.10.0-r1"
    RUN apk add --no-cache shellcheck=$SHELLCHECK_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-shell-linting.sh


check-github-actions-workflows-linting:
    FROM +golang-base
    # renovate: datasource=github-releases depName=rhysd/actionlint
    ENV ACTIONLINT_VERSION="v1.7.7"
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@$ACTIONLINT_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-rust-linting
    BUILD +check-shell-linting
    BUILD +check-github-actions-workflows-linting


compile:
    FROM +sourcecode-base
    RUN ./ci/compile.sh
    SAVE ARTIFACT "target/" AS LOCAL "./"
    SAVE ARTIFACT "Cargo.lock" AS LOCAL "./"


unit-test:
    FROM +sourcecode-base
    RUN ./ci/unit-test.sh


static-binary-test:
    FROM ubuntu:24.04@sha256:80dd3c3b9c6cecb9f1667e9290b3bc61b78c2678c02cbdae5f0fea92cc6734ab
    COPY "+compile/target/" "target/"
    RUN ./target/debug/did_i_break_it --help


publish-binary:
    FROM +rust-base
    # renovate: datasource=repology depName=alpine_3_20/github-cli versioning=loose
    ENV GITHUB_CLI_VERSION="2.47.0-r4"
    RUN apk add --no-cache github-cli=$GITHUB_CLI_VERSION
    DO +COPY_METADATA
    DO +COPY_SOURCECODE
    ARG release
    RUN --secret GH_TOKEN ./ci/publish-binary.sh --release "${release}"


publish-crate:
    FROM +sourcecode-base
    COPY "README.md" "./"
    RUN --secret CARGO_REGISTRY_TOKEN ./ci/publish-crate.sh
