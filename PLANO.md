# PSIKOPATA — Plano de Redesenho

**Versão 1.0 · 2026**
Redesenho completo de psikopata.com: site artístico, provocador e tecnologicamente radical — 100% Rust no build, 0 bytes de JavaScript no browser.

---

## 1. Visão

> Um site que se sente antes de se ler. Intensidade criativa: bondade, angústia e raiva da ignorância — transformadas em arte digital.

O site atual (auditoria) pesa ~22 MB, depende de 972 KB de JS, e o conteúdo é invisível para motores de busca. O novo site será o oposto: **leve como texto, denso como um álbum**.

### Princípios inegociáveis

1. **0 bytes de JavaScript** — hoje e para sempre. Toda a interatividade via web platform nativa.
2. **100% Rust no build** — gerador std-puro + pipeline de imagens em Rust. Zero dependências no núcleo.
3. **Web Vitals de elite** — LCP < 1,2 s · CLS 0 · INP 0 · TBT 0 · página inicial < 250 KB.
4. **Português** — comunidade lusófona primeiro.
5. **Zero analytics, zero cookies, zero tracking.** Privacidade como declaração artística.

---

## 2. Arquitetura técnica (resumo — detalhe em PLAN.md)

```
content/ + css/ + img/ ─→ [psikopata: gerador Rust std-only] ─→ dist/ (HTML estático)
img/originais ──────────→ [psikopata-img: Rust crate image] ──→ img/ (WebP, EXIF limpo)
dist/ ─→ GitHub Actions (build + Lighthouse CI) ─→ GitHub Pages ─→ CNAME psikopata.com
```

- Sem WASM: exige JS para instanciar e não há computação em runtime. O Rust vive no build.
- Interatividade: `animation-timeline: view()/scroll()`, `backdrop-filter`, `perspective`, `scroll-snap`, `<dialog>`, `<audio controls>`.
- Fallback gracioso: browsers antigos veem o site estático; nunca conteúdo invisível.

---

## 3. Qualidade percebida — estudo e táticas

A decisão científica central: **a estética não é decoração; é parte da função.**

### O que diz a investigação

| Fonte | Descoberta | Implicação |
|---|---|---|
| Kurosu & Kashimura (1995, Hitachi) — **Efeito Estética-Usabilidade** | Utilizadores julgam designs bonitos como *mais usáveis*, mesmo quando não são | O visual eleva a perceção de competência do artista |
| Fogg et al. (Stanford Web Credibility Project) | ~75% dos juízos de credibilidade de um site baseiam-se na estética/apresentação visual | Um site pobre descredibiliza a música antes do primeiro play |
| **Von Restorff / Efeito de Isolamento** | O elemento que difere é o que fica na memória | Um único acento provocador por ecrã (nunca vários a competir) |
| **Peak-End Rule** (Kahneman) | As pessoas lembram-se do pico e do fim, não da média | Investir no hero (pico) e no footer/CTA final (fim) |
| **Jakob's Law** | Utilizadores esperam que o site funcione como os que já conhecem | Navegação e padrões familiares; só a estética é que rompe |

### Táticas concretas para o Psikopata

**A. Autoridade instantânea (primeiros 3 segundos)**
1. Tipografia display em escala extrema — tamanho comunica importância.
2. Uma paleta curta e consistente (máx. 3 cores + neutros). Restrição = sofisticação.
3. Grid assimétrico intencional com muito espaço negativo — espaço vazio é percecionado como luxo.
4. Textura subtil (grão de filme) — superfície tátil vs. plástico digital genérico.

**B. Sensação de vida (perceived quality dinâmica)**
5. Micro-interações em tudo o que é clicável (hover/focus com movimento + som visual).
6. Revelações por scroll — o site "responde" ao utilizador sem JS.
7. Elementos em movimento lento e contínuo (marquee, blobs) — vida sem ruído.
8. Profundidade: camadas, vidro, sombra, 3D — hierarquia física percebida.

**C. Confiança e detalhe (o que separa amador de profissional)**
9. Imagens com direção única — todas do mesmo universo visual, metadados limpos.
10. Favicon, seleção de texto personalizada, scrollbar estilizada — detalhes que 99% dos sites ignoram.
11. Página 404 com personalidade.
12. Velocidade visível — o site carrega instantaneamente; a própria velocidade é percebida como qualidade.
13. Consistência total: mesmas cores/tipografia em site, OG image, Substack, capas.

**D. Pico e fim (memória)**
14. O hero é o pico: uma frase que provoca ("Não é música. É um movimento.").
15. O fim é convite: CTA quente e humano no final de cada página.

---

## 4. Direções visuais — mockups em `mockups/`

| Direção | Ficheiro | Personalidade | Prós | Riscos |
|---|---|---|---|---|
| **A. Manifesto Brutal** | `a-manifesto-brutal.png` | Preto/vermelho, tipografia gigante, grão, marquee | Máxima provocação, identidade forte, rápido | Pode parecer "agressivo" para parceiros institucionais |
| **B. Vidro Noir** | `b-vidro-noir.png` | Glassmorphism, gradientes cyan/violeta, cartões 3D | Tecnologia nova, futurista, contemporâneo | Estética "tech" pode diluir a alma musical |
| **C. Ouro PALOP** | `c-ouro-palop.png` | Editorial escuro + dourado, serif, vinil | Luxo, arte, herança lusófona/africana | Menos "provocador", mais clássico |

**Recomendação: híbrido A+C** — a escala brutal e a provocação de A com o dourado e a elegância serif de C (vermelho como acento de rutura, dourado como acento de herança). O glassmorphism de B fica reservado para momentos especiais (página LAB, lançamentos). Decisão final tua.

---

## 5. Conteúdo e informação

| Página | Conteúdo | Estado |
|---|---|---|
| `/` | Hero provocador + manifesto curto + faixa destaque | skeleton feito |
| `/manifesto` | As 4 fases | feito |
| `/producoes` | Faixas (áudio TBD) + links plataformas | feito, sem áudio |
| `/universo` | Galeria das personagens (imagens novas) | feito, sem imagens |
| `/lab` | Demonstração técnica (glass + 3D) | feito |
| `/entrar` | Comunidade: WhatsApp, Substack, TikTok | feito |

Sem formulários (decidido). Sem analytics (decidido).

---

## 6. Roadmap e entregáveis

| # | Entrega | Critério de aceitação | Estado |
|---|---|---|---|
| F0 | Gerador + CI + repositório | Build 0-JS, push automático para Pages | ✅ |
| F1 | **Decisão visual** (tu escolhes A/B/C/híbrido) | Mockup aprovado | ⏳ este documento |
| F2 | Sistema de imagens Rust (`psikopata-img`: WebP + EXIF strip) | Todas as imagens novas < 80 KB, metadados limpos | ⏳ |
| F3 | Design system final (fontes self-hosted, paleta, componentes) | 1 HTML ~12 KB, fontes preload, CLS 0 | ⏳ |
| F4 | Conteúdo real + imagens novas + áudio | Todas as páginas finais | ⏳ |
| F5 | SEO completo (JSON-LD MusicGroup, OG image nova) | Rich results válidos | ⏳ |
| F6 | Lighthouse CI com budgets no GitHub Actions | Score 100/100/100/100 | ⏳ |
| F7 | CNAME psikopata.com + verificação produção | Web Vitals reais "Good" | ⏳ |

---

## 7. Decisões pendentes (preciso de ti)

1. **Direção visual**: A, B, C ou híbrido A+C?
2. **Produção das novas imagens**: quem as faz e qual a direção (fotografia tua, ilustração, AI com pós-produção)?
3. **Áudio**: prévias 30 s / faixas completas / só links para plataformas?
4. **Autorização GitHub**: o código `E8B4-5021` em github.com/login/device continua por usar — sem ele não consigo fazer push do CI.
