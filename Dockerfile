#FROM archlinux/base

#RUN pacman -Syy
#RUN pacman -S --noconfirm base-devel
#RUN pacman -S --noconfirm rustup

#RUN rustup override set nightly

FROM rustlang/rust:nightly

WORKDIR /root

COPY . .

COPY webproxy.crt /usr/local/share/ca-certificates/
RUN update-ca-certificates

RUN cargo build
#RUN cargo build --release

FROM archlinux/base
ENV RUST_BACKTRACE=1
ENV LC_ALL="no_NB.UTF-8"
ENV LANG="no_NB.UTF-8"
ENV TZ="Europe/Oslo"

RUN pacman -Syy
RUN pacman --noconfirm -S bzip2 expat fontconfig freetype2 gcc-libs glib2 glibc graphite harfbuzz libjpeg-turbo libx11 libxau libxdmcp libxext libxrender pcre zlib gsfonts tar xz

WORKDIR /tmp
RUN curl -o wkhtmltox.tar.xz -L "https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz"
RUN tar -xf wkhtmltox.tar.xz
WORKDIR /tmp/wkhtmltox
RUN cp -R * /usr

WORKDIR /app

COPY --from=0 /root/target/debug/pdfgen .
#COPY --from=0 /root/target/release/pdfgen .
COPY Rocket.toml .
RUN mkdir -p /app/out
COPY templates templates/
EXPOSE 8000

ENV RUST_BACKTRACE=1
ENTRYPOINT ["/app/pdfgen"]
