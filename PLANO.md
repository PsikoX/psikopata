# PSIKOPATA — Plano de Redesenho

**Versão 1.0 · 2026**
Redesenho completo de psikopata.com: site artístico, provocador e tecnologicamente radical — 100% Rust no build, 0 bytes de JavaScript no browser.

---

## 1. Visão

> Um site que se sente antes de se ler. Sensual, quente, luxuoso — como a música que anuncia.

**Missão:** divulgar **dois álbuns** — um para **Cabo Verde**, outro para **Moçambique** — unindo os dois países num único gesto artístico. As cores vêm das duas bandeiras.

**Prioridade máxima (definida pelo artista):** qualidade percebida, **sensualidade e atração feminina pelo site** (ver §3.E — é a secção mais importante deste plano).

O site atual (auditoria) pesa ~22 MB, depende de 972 KB de JS, e o conteúdo é invisível para motores de busca. O novo site será o oposto: **leve como texto, quente como pele**.

### Paleta — fusão Cabo Verde × Moçambique

| Cor | Hex | Origem | Uso |
|---|---|---|---|
| Noite quente | `#0d0a09` | neutro | fundo (nunca preto puro — preto frio mata sensualidade) |
| Azul CV | `#0a3d91` | bandeira CV | álbum I, luz de fundo |
| Verde MZ | `#007168` | bandeira MZ | álbum II, luz de fundo |
| Ouro | `#e8b84b` | presente em ambas as bandeiras | acento principal, tipografia especial, CTAs |
| Rubro | `#d8323c` | presente em ambas as bandeiras | provocação pontual (Von Restorff: pouco e raro) |
| Pele | `#f3e2cf` | neutro quente | texto (nunca branco puro — branco é clínico) |

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

### 3.E — Sensualidade e atração feminina (prioridade máxima)

O que a investigação e a prática de design de marcas de luxo/perfumaria dizem:

| Princípio | Evidência / referência | Como aplicamos |
|---|---|---|
| **Calor cromático** | Tons quentes (âmbar, ouro, pele, rubro) ativam perceção de intimidade e proximidade; o preto frio e o branco clínico afastam | Fundo "noite quente" `#0d0a09`, texto cor-de-pele, acentos ouro/rubro. Nada de cinzas frios |
| **Efeito vermelho** | Elliot & Niesta (2007): o vermelho aumenta a atratividade percebida e a sinalização de status/intenção | Rubro usado com parcimónia em lábios-visuais: CTA, sublinhados, o "b" das frases — nunca em superfícies grandes |
| **Curvas > ângulos** | Formas arredondadas são percecionadas como femininas, orgânicas e seguras; ângulos agudos como agressivos | `border-radius` generoso (140px nos altares dos álbuns), silhuetas em arco, nada de cantos vivos |
| **Tipografia serif itálica** | Serifas + itálico = voz sussurrada, literatura, pele; sans-serif = técnico | Cormorant Garamond itálico para tudo o que é emocional; mono só para metadados frios (datas, coords) |
| **Movimento lento** | Velocidade baixa + easing suave = sensualidade; movimento rápido = ansiedade | Animações 0,6–1,4 s, easing `cubic-bezier(.4,0,.2,1)`, blobs de luz em drift lento (14 s) |
| **Espaço negativo** | Luxo = espaço. Marcas de perfume usam 70%+ de vazio | Margens enormes, uma ideia por ecrã |
| **Luz, não objetos** | Sensualidade sugere-se, não se mostra: gradientes de luz, silhuetas, véus | Glows radiais azul CV / verde MZ como "dois sóis", silhuetas em arco, véus translúcidos |
| **Voz que sussurra** | Copywriting em segunda pessoa, imperativo suave, frases curtas | "Aproxima-te." "Feito para ser sentido devagar." — nunca gritar |
| **Revelação progressiva** | O que se descobre devagar prende mais (curiosity gap) | Conteúdo revela-se ao scroll, álbuns como altares que se aproximam |

**O que NUNCA fazer** (destrói a atração feminina e a qualidade percebida):
- ❌ Branco puro / preto puro como cores de texto-fundo
- ❌ Ângulos agudos, bordas finas cinzentas, estética "dashboard"
- ❌ Movimento rápido, brusco ou saltitante
- ❌ Sans-serif geométricas em títulos emocionais
- ❌ Conteúdo denso sem ar; ❌ mais de um CTA a competir por ecrã

---

## 4. Direções visuais — mockups em `mockups/`

| Direção | Ficheiro | Personalidade | Prós | Riscos |
|---|---|---|---|---|
| **A. Manifesto Brutal** | `a-manifesto-brutal.png` | Preto/vermelho, tipografia gigante, grão, marquee | Máxima provocação, identidade forte | Agressivo; frio; oposto da sensualidade pedida |
| **B. Vidro Noir** | `b-vidro-noir.png` | Glassmorphism, gradientes cyan/violeta, cartões 3D | Tecnologia nova, futurista | Paleta fria (cyan/violeta) não serve a missão CV×MZ |
| **C. Ouro PALOP** | `c-ouro-palop.png` | Editorial escuro + dourado, serif, vinil | Luxo, arte, herança lusófona | Só uma cor de herança; falta Moçambique e Cabo Verde |
| **D. Morna & Marrabenta** ⭐ | `d-morna-marrabenta.png` | Noite quente, azul CV + verde MZ como dois sóis, ouro, serif itálica, dois altares de álbuns | Cumpre TODA a missão: paleta das duas bandeiras, 2 álbuns visíveis, sensualidade editorial, atração | Requer fotografia/imagens novas de qualidade à altura |

**Recomendação: D** — é a única que cumpre a missão completa (cores CV+MZ, dois álbuns, sensualidade). Elementos de B (glass) podem entrar em momentos especiais; A fica para posters/merch, não para o site.

---

## 5. Conteúdo e informação

| Página | Conteúdo | Estado |
|---|---|---|
| `/` | Hero sensual + os dois álbuns como altares (CV × MZ) | skeleton feito; mockup D aprovado = reconstruir |
| `/sodade-azul` | Álbum I — Cabo Verde: faixas, história, cores CV | por criar |
| `/marrabenta-de-ouro` | Álbum II — Moçambique: faixas, história, cores MZ | por criar |
| `/manifesto` | As 4 fases | feito (revestir com paleta nova) |
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

1. **Direção visual**: confirmas a **D (Morna & Marrabenta)**?
2. **Nomes dos álbuns**: "Sodade Azul" (CV) e "Marrabenta de Ouro" (MZ) são placeholders — quais os nomes reais?
3. **Produção das novas imagens**: quem as faz e qual a direção (fotografia tua, ilustração, AI com pós-produção)?
4. **Áudio**: prévias 30 s / faixas completas / só links para plataformas?
5. **Autorização GitHub**: o código `E8B4-5021` em github.com/login/device continua por usar — sem ele não consigo fazer push do CI.
