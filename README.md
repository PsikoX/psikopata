# PSIKOPATA — site

Site oficial do movimento Psikopata. Começa no som, termina na mente.

## Experimental

- Gerador estático em **Rust puro** (`std` apenas, zero dependências)
- **0 bytes de JavaScript** — toda a interação é CSS moderno
  (`animation-timeline`, `:has()`, scroll-driven reveals)
- HTML semântico + CSS crítico inline → máximo SEO e Web Vitals
- Servidor de preview próprio em `std::net`

## Uso

```sh
cargo run --release          # build → dist/
cargo run --release -- serve # build + preview em http://127.0.0.1:8080
```

## Estrutura

```
content/   páginas (HTML + front matter)
css/       estilo único (minificado pelo gerador)
img/       imagens (otimizadas)
src/       gerador + servidor
```
