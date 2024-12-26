FROM nginx:1.26.0

EXPOSE 80
EXPOSE 443

RUN apt update -y && apt upgrade -y && mkdir /app && mkdir /app/https && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

RUN chmod +x setup_nginx
RUN /app/setup_nginx
