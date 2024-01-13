# DB

```sh
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal)
docker exec -it -u postgres pg psql


# Start Docker and daemon
sudo systemctl start docker.socket
sudo systemctl enable docker.socket
sudo systemctl start docker
sudo systemctl enable docker

# Disable Docker and socket
sudo systemctl stop docker
sudo systemctl disable docker
sudo systemctl stop docker.socket
sudo systemctl disable docker.socket

# Check Docker status
sudo systemctl status docker
sudo systemctl status docker.socket


## Run the dev tests
cargo watch -q  -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```
