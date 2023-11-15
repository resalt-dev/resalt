# ⚠️ RAPID DEVELOPMENT
## THIS PROJECT IS UNDER RAPID DEVELOPMENT, NO SUPPORT IS GIVEN YET
## THIS PROJECT IS UNDER RAPID DEVELOPMENT, NO SUPPORT IS GIVEN YET
## THIS PROJECT IS UNDER RAPID DEVELOPMENT, NO SUPPORT IS GIVEN YET
## THIS PROJECT IS UNDER RAPID DEVELOPMENT, NO SUPPORT IS GIVEN YET
## THIS PROJECT IS UNDER RAPID DEVELOPMENT, NO SUPPORT IS GIVEN YET

<p align="center"><a href="https://github.com/resalt-dev/resalt" target="_blank" rel="noopener noreferrer"><img src="docs/images/logo.png?raw=true" alt="re-frame logo"></a></p>

[![License](https://img.shields.io/github/license/resalt-dev/resalt?style=for-the-badge)](https://github.com/resalt-dev/resalt/blob/main/LICENSE)

Resalt is a free, open-source, self-hosted, web-based administration and monitoring panel for SaltStack.

> *Resalt is a word play from "salt" and "results", which is the goal of this project.*

## 🔧 Set-up

Easiest is to use the Docker container image [UNDER CONSTRUCTION].

Additional needed config in `/etc/salt/master`:

```
external_auth:
  rest:
    ^url: http://resalt:8000/api/token
keep_acl_in_token: True
netapi_enable_clients:
  - local
  - local_async
  - local_batch
  - runner
  - runner_async
  - wheel
  - wheel_async
```

## 💖 Contributing

Setup development system:
```
docker compose up -d
```

Access it at [http://localhost:1234](http://localhost:1234).

Both the frontend (Svelte) and backend (Rust) will reload automatically.