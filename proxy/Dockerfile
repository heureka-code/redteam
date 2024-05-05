FROM nginx:latest

EXPOSE 80
EXPOSE 443

RUN apt update -y && apt upgrade -y && mkdir /app && mkdir /app/https
WORKDIR /app
COPY . .

RUN chmod +x setup_nginx
RUN /app/setup_nginx
