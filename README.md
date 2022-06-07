# Hibike

## Development

### Docker

Setup configuration for system:
```
docker compose up -d
```

### Frontend 

Install dependencies for frontend (new terminal):
```
cd frontend

npm install rollup
```

Start frontend: `npm run dev`

### Backend

Install dependencies for backend (new terminal):
```
cd backend

# Install system libraries
sudo apt install build-essential pkg-config libssl-dev mysql-client libmysqlclient-dev

# Install cargo-watch for hot reload
cargo install cargo-watch

# Install DB maintenance tool
cargo install diesel_cli --no-default-features --features "mysql"
```

Start backend: `cargo watch -x run`