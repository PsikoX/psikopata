# PSIKOPATA — System Design

## 1. Decisão de stack (pesquisada e justificada)

### O que existe no ecossistema

| Opção | Prós | Contras | Decisão |
|---|---|---|---|
| **Zola** (SSG Rust, binário único) | Maduro, Sass, <1s por site | Conhecido/dogmático, templates Tera impõem estrutura, output genérico | ❌ — não é "experimental" nem dá controle total do output byte-a-byte |
| Next/Astro/11ty | Ecossistema rico | Node, JS inevitável, hydratação | ❌ — viola 0-JS |
| **Gerador Rust próprio (escolhido)** | Output controlado ao byte, 0 deps, auditável | Temos de escrever tudo | ✅ — alinhado com o pedido "experimentos, sem frameworks" |

### Rust: onde e porquê

- **Build-time apenas.** O gerador é um binário Rust (`std` puro, zero crates) que transforma `content/` + `css/` em HTML estático.
- **Porquê `std` puro:** zero supply-chain, build reproduzível, o binário final é o site inteiro em si.

### WASM: NÃO — decisão explícita

1. WASM no browser **requer JavaScript para instanciar** (`WebAssembly.instantiate`) → viola a restrição 0-JS.
2. Num site estático não há computação em runtime que justifique WASM.
3. WASM atrasaria LCP (descarregar + compilar módulo antes de conteúdo).

O Rust entra no projeto **via build tooling**, não via browser. Se um dia houver necessidade de algo interativo pesado (ex.: visualizador áudio), reavalia-se.

### JavaScript: 0 bytes, para sempre

Toda a interatividade usa web platform nativa:

| Necessidade | Solução nativa | Suporte |
|---|---|---|
| Reveals ao scroll | `animation-timeline: view()` | Chrome/Edge 115+, Safari 26+, Firefox (em TP) — com fallback |
| Progresso de leitura | `animation-timeline: scroll()` | idem |
| Carrossel | `scroll-snap` + scroll buttons CSS (`::scroll-button`, progressivo) | Chrome estável; fallback scroll-snap puro |
| Lightbox/expansão | `dialog` / checkbox hack / `details` | universal |
| Player áudio | `<audio controls preload="none">` | universal |
| Tema/cor | CSS custom properties + `light-dark()` | moderno, fallback escuro |

**Fallback:** em browsers sem `animation-timeline`, as animações simplesmente não correm e o conteúdo aparece estático (nunca invisível) — testado por construção (`animation` sem `animation-timeline` suportado = duration 0s).

## 2. Arquitetura

```
content/*.html ──┐
css/site.css ────┤→ [ gerador Rust (std-only) ] → dist/ (HTML+assets)
img/ (otimizadas)┘         │                             │
                           ├─ minifica CSS               ↓
                           ├─ inline CSS crítico    GitHub Actions
                           ├─ sitemap.xml                │ build+deploy
                           ├─ robots.txt                 ↓
                           └─ OG tags/canonicals    GitHub Pages
                                                    (psikox.github.io/psikopata
                                                     → CNAME psikopata.com)
```

### Pipeline de imagens — 100% Rust (decidido)

- **Imagens todas novas** (nenhuma reutilizada do site atual) e com
  **metadados limpos**: re-encode via crate `image` remove EXIF/ICC/thumbnails
  automaticamente — nunca sai metadata de origem (ferramentas, locais, modelos AI).
- Binário separado `psikopata-img` (crate `image` + `webp`) → WebP/AVIF + strip.
  O gerador de páginas continua `std`-puro.
- Metas: hero ≤ 80 KB, galeria ≤ 60 KB cada, `width/height` sempre definidos (CLS 0).
- LCP image (hero): `fetchpriority="high"`, **nunca** `loading="lazy"` (regra web.dev).
- Restantes: `loading="lazy" decoding="async"`.

### Fontes (web.dev font best practices)

- 2 fontes no máximo, **self-hosted**, subsets latin+pt, formatos woff2 só.
- `<link rel="preload" as="font" crossorigin>` + `font-display: swap`
  + fallback com `size-adjust` para CLS ≈ 0.
- Alternativa a considerar: **só fontes do sistema** (0 requests, LCP imbatível,
  menos personalidade). Decisão aberta (ver §6).

## 3. Web Vitals — orçamentos (budgets)

| Métrica | Orçamento | Como garantimos |
|---|---|---|
| **LCP** | < 1,2 s | HTML 1 request (~12 KB), CSS inline, hero texto ou imagem <80 KB com fetchpriority |
| **CLS** | 0,00 | dimensões explícitas em img/fontes/áudio |
| **INP** | 0 ms | sem JS: não há handlers |
| **TBT** | 0 ms | sem JS: não há parse/execute |
| **Requests página inicial** | ≤ 8 | 1 HTML + fontes + hero |
| Peso total página inicial | < 250 KB | vs 22 MB do site atual |

Verificação: Lighthouse CI no GitHub Actions (falha o build se budgets quebrados).

## 4. SEO

- HTML completo pré-renderizado (crawler vê tudo sem executar nada).
- `lang="pt"`, canonical por página, OG/Twitter pt_PT, imagem OG local (não bucket externo).
- `sitemap.xml` + `robots.txt` gerados automaticamente (já implementado).
- **JSON-LD** (Schema.org `MusicGroup`/`Person` + `sameAs` das plataformas) — falta gerar.
- URLs limpas: `/manifesto/`, `/producoes/` (já implementado).

## 5. Acessibilidade (inerente ao 0-JS)

- Landmarks semânticos, skip-link, `aria-current` no nav (já implementado).
- `prefers-reduced-motion` desliga tudo (já implementado).
- Contraste ≥ 4.5:1; foco visível; controlos nativos (`<audio>`, links reais).

## 6. Decisões abertas (preciso da tua resposta)

1. ~~Imagens~~ → decidido: **todas novas, metadados limpos**, pipeline Rust (`image` crate).
2. **Direção visual das imagens:** quem as produz (fotografia, ilustração, AI
   com pós-produção)? Preciso de saber para definir o estilo.
3. **Fontes:** self-hosted com personalidade vs só sistema (0 requests)?
4. **Áudio:** ainda por decidir (prévias 30 s vs faixas completas vs só links).
5. **Domínio:** queres apontar psikopata.com (Cloudflare) via CNAME para
   GitHub Pages, ou manter o site antigo até este estar pronto?
6. **Scope `workflow`:** falta autorizares o código `E8B4-5021` em
   github.com/login/device para eu conseguir fazer push do CI.

## 7. Roadmap

| Fase | Entrega | Estado |
|---|---|---|
| 0 | Gerador std-only + 5 páginas + CI Pages | ✅ feito |
| 1 | Pipeline imagens + fonts + JSON-LD + Lighthouse CI | ⏳ |
| 2 | Design final (hero, paleta, tipografia, conteúdo real) | ⏳ |
| 3 | Áudio + página produções completa | ⏳ |
| 4 | CNAME psikopata.com + teste Web Vitals em produção | ⏳ |
