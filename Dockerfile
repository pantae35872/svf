FROM archlinux:latest 

RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm \
    curl \
    vim \
    base-devel \
    && rm -rf /var/cache/pacman/pkg/*

# Update dynamic linker to use the new glibc
ENV LD_LIBRARY_PATH=/opt/glibc-2.38/lib:$LD_LIBRARY_PATH

COPY svf-server/target/release/svf-server /usr/local/bin/server
COPY fullchain.pem /app/certs/cert.pem
COPY privkey.pem /app/certs/priv.pem

ENV PROD=1
ENV CERT_PATH=/app/certs/cert.pem
ENV KEY_PATH=/app/certs/priv.pem

RUN chmod +x /usr/local/bin/server

EXPOSE 80 443

WORKDIR /app

CMD ["/usr/local/bin/server"]
