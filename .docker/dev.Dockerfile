FROM "node:slim" AS node_dev
ARG UID=1000
ARG GID=1000

USER ${UID}:${GID}

FROM "rust:slim" AS rust_dev

ARG UID=1000
ARG GID=1000

RUN mkdir /cargo_home \
    && chown -R ${UID}:${GID} /cargo_home

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install --no-install-recommends -y \
        build-essential \
        ca-certificates \
        cmake \
        git \
        libusb-1.0.0-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/rtlsdrblog/rtl-sdr-blog.git \
    && cd rtl-sdr-blog \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make \
    && make install \
    && cd / \
    && rm -rf /rtl-sdr-blog \
    && ldconfig

RUN rustup component add rustfmt

USER ${UID}:${GID}
