# Docker



```bash
docker run -itd rust:1.87.0-bookworm --name sustcsc-rs \
    -v challenge:/root/challenge
docker exec -it sustcsc-rs bash

# Inside container
cd /root/challenge
cargo build -p 4 --release
```