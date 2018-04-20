#FROM archlinux/base

#RUN pacman -Syy
#RUN pacman -S --noconfirm base-devel
#RUN pacman -S --noconfirm rustup

#RUN rustup override set nightly

FROM rustlang/rust:nightly

WORKDIR /root

COPY . .

RUN cargo build --release

FROM archlinux/base

RUN pacman -Syy
RUN pacman --noconfirm -S bzip2 expat fontconfig freetype2 gcc-libs glib2 glibc graphite harfbuzz libjpeg-turbo libx11 libxau libxdmcp libxext libxrender pcre zlib gsfonts tar xz

WORKDIR /tmp
RUN curl -o wkhtmltox.tar.xz -L "https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz"
RUN tar -xf wkhtmltox.tar.xz
WORKDIR /tmp/wkhtmltox
RUN cp -R * /usr

WORKDIR /app

COPY --from=0 /root/target/release/pdfgen .
COPY Rocket.toml .
RUN mkdir -p /app/out
COPY templates templates/
EXPOSE 8000

ENTRYPOINT ["/app/pdfgen"]
