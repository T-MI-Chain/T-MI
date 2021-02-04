FROM tmi/ci-linux:production as builder
LABEL description="This is the build stage for tmi. Here we create the binary."

ARG PROFILE=release
WORKDIR /tmi

COPY . /tmi

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the tmi binary."
ARG PROFILE=release
COPY --from=builder /tmi/target/$PROFILE/tmi /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /tmi tmi && \
	mkdir -p /tmi/.local/share && \
	mkdir /data && \
	chown -R tmi:tmi /data && \
	ln -s /data /tmi/.local/share/tmi && \
	rm -rf /usr/bin /usr/sbin

USER tmi
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/tmi"]
