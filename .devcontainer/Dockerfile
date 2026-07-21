FROM mcr.microsoft.com/devcontainers/universal:linux

RUN apt-get update && apt-get install -y lua5.4 && rm -rf /var/lib/apt/lists/* \
    && npm install -g opencode-ai
