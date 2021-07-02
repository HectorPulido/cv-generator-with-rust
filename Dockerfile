FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y

# Only for pdfs
RUN apt-get install xfonts-75dpi wget -y
RUN wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.bionic_amd64.deb
RUN apt-get install ./wkhtmltox_0.12.6-1.bionic_amd64.deb -y
RUN wkhtmltopdf --version

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./source/ /app
RUN cargo build --release


FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y

# Only for pdfs
RUN apt-get install xfonts-75dpi wget -y
RUN wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.bionic_amd64.deb
RUN apt-get install ./wkhtmltox_0.12.6-1.bionic_amd64.deb -y
RUN wkhtmltopdf --version

WORKDIR /app

COPY --from=0 /app/.env /app
COPY --from=0 /app/target/release/ /app

# COPY /source/templates/ /app/templates
# COPY /source/static/ /app/static

CMD ./cv_generator