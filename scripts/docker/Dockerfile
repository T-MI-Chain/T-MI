FROM debian:buster-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="parity/tmi" \
	io.parity.image.description="tmi: a platform for web3" \
	io.parity.image.source="https://github.com/tmi/tmi/blob/${VCS_REF}/scripts/docker/Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/tmi/tmi/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
	libssl1.1 \
	ca-certificates \
	curl && \
	# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
	# add user
	useradd -m -u 1000 -U -s /bin/sh -d /tmi tmi

# add tmi binary to docker image
COPY ./tmi /usr/local/bin

USER tmi

# check if executable works in this container
RUN /usr/local/bin/tmi --version

EXPOSE 30333 9933 9944
VOLUME ["/tmi"]

ENTRYPOINT ["/usr/local/bin/tmi"]

