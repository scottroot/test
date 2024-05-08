if "%~1"=="stop" (
    docker stop rust-builder && docker remove rust-builder && docker image rm rust-builder:latest
) else (
    docker build . -t rust-builder:latest && docker run -d --name rust-builder -v "%cd%":"/src" rust-builder:latest && docker exec -it rust-builder /bin/bash
)
