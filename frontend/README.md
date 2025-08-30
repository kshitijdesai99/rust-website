# Frontend

Rust-based frontend using Yew framework.

## Structure

```
frontend/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── styles.css
├── src/
│   ├── lib.rs
│   ├── config.rs
│   ├── routes.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   ├── button.rs
│   │   └── form.rs
│   └── services/
│       ├── mod.rs
│       ├── api.rs
│       └── storage.rs
```

## Development

```bash
trunk serve --open
```
