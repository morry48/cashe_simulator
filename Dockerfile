FROM rust:latest
RUN apt-get update 
RUN apt-get install build-essential
RUN yes | apt-get install valgrind