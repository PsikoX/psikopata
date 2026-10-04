# PSIKOPAPA

Website oficial de PSIKOPAPA. **FER — Fetiche, Educação e Religião** aparece na entrada e tem uma secção própria logo depois do hero; as restantes áreas desenvolvem o universo da agência.

A aplicação é construída em Rust. Dioxus compõe as páginas e `dioxus-ssr` gera HTML estático durante o build. Há uma exceção autorizada: um pequeno script local desenha um fio de fumo vermelho a partir da ponta do rato ou do dedo no ecrã. Todo o conteúdo e a navegação funcionam sem esse script. Não há runtime WASM, hidratação ou serviços externos de frontend. A preview local também é Rust, através de Axum.

A rota `/fer/` apresenta **FER — Fetiche, Educação e Religião: o Tridente da Prosperidade**. A narrativa começa com três bandas entrelaçadas, apresenta as forças independentes e reúne três medalhões num selo triangular na convergência, antes da expansão, do circuito circular, da mulher no centro e do manifesto final. As duas peças completam um ciclo em seis segundos. O F rubi, o E dourado e o R prateado identificam as três forças ao longo da experiência. A prosperidade é descrita como metáfora filosófica de abundância criada, sem prometer resultados. FER não é apresentado como uma nova religião. Os textos editáveis da experiência estão em `src/content/fer.rs`; os componentes Rust em `src/pages/fer.rs` e `src/components/fer_artifact.rs`; o desenho e o movimento em `src/styles/fer.css` e `src/styles/fer_artifacts.css`. As fontes históricas e a natureza artística das imagens constam de `docs/fer-artifact-sources.md`. A imagem da mulher usa a capa autorizada de Karen, enquadrada por CSS sem alterar o ficheiro original. O movimento funciona sem JavaScript, respeita o controlo de pausa e a preferência de movimento reduzido.

## Executar

Instalar Rust stable com rustup. A toolchain está fixada em `rust-toolchain.toml`; Cargo utiliza o lockfile incluído.

```sh
cargo run --locked --bin build-site
cargo run --locked --features preview --bin preview
```

A preview abre em `http://127.0.0.1:8080`. `PREVIEW_ADDR` permite alterar o endereço. `/health` responde `ok`; uma rota inexistente devolve HTTP 404 com a página da marca.

Nesta sessão, a toolchain foi instalada em `/workspace/.rust`. Para utilizar essa instalação:

```sh
export RUSTUP_HOME=/workspace/.rust/rustup
export CARGO_HOME=/workspace/.rust/cargo
export PATH="/workspace/.rust/cargo/bin:$PATH"
```

## Produção

```sh
cargo fmt --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo run --release --locked --bin build-site
```

O resultado fica em `dist/`, pronto para hosting estático. O servidor de preview e os binários Rust não fazem parte do artefacto publicado. `.openai/hosting.json` identifica o Site privado e a pasta pública `dist`.

O domínio configurado está em `site-origin.txt`. É possível substituí-lo apenas para um build:

```sh
SITE_ORIGIN=https://dominio-oficial.example cargo run --release --locked --bin build-site
```

## GitHub Pages

Esta versão temporária está em `https://psikox.github.io/psikopata/psikopapa/`. O código está na branch [`psikopapa-source`](https://github.com/PsikoX/psikopata/tree/psikopapa-source) do repositório público `PsikoX/psikopata`. Os ficheiros estáticos gerados ficam no diretório `psikopapa/` da branch `gh-pages`, preservando a página que já ocupava a raiz desse repositório.

Para gerar os ficheiros dessa página:

```sh
SITE_ORIGIN=https://psikox.github.io SITE_BASE_PATH=/psikopata/psikopapa cargo run --release --locked --bin build-site
```

`SITE_BASE_PATH` ajusta a navegação, as imagens responsivas, os vídeos, as fontes, o script local, o canonical e o sitemap ao subdiretório da página. Depois do build, sincronizar `dist/` com `psikopapa/` numa cópia da branch `gh-pages` e fazer push dessa branch. O build normal do Site privado continua a usar a raiz do seu domínio. GitHub Pages serve os ficheiros gerados.

O gerador valida o domínio antes de gerar canonical, Open Graph e sitemap. Um build sem origem utiliza `robots.txt` com `Disallow: /`. Os headers propostos em `dist/_headers` permitem apenas scripts locais e restringem os restantes recursos à própria origem, com exceção de áudio HTTPS. O hosting deve aplicar esses headers; a preview aplica CSP e `nosniff` diretamente.

## Estrutura

| Pasta | Responsabilidade |
|---|---|
| `src/app` | Rotas, configuração de origem, documento e renderização |
| `src/models` | Imagens, músicas, muses, capítulos e estruturas editoriais |
| `src/content` | Dados e textos editáveis |
| `src/components` | Imagens responsivas, headings, marca e links reutilizáveis |
| `src/layouts` | Navegação, contacto e footer |
| `src/pages` | Início, FER e 404 |
| `src/sections` | Secções da narrativa |
| `src/styles` | Tokens, layouts, responsive e movimento |
| `src/bin` | Gerador estático e preview opcional |
| `assets` | Referências originais, WebP responsivo, fontes locais e favicon |
| `tests` | Contratos de navegação, conteúdo, SEO, assets e script externo limitado à página inicial |
| `tools/browser-audit` | Auditoria independente em Rust através do protocolo nativo do Chromium |

As rotas são `/`, `/fer/` e `404.html`. A navegação entre secções utiliza âncoras nativas. Os disclosures de navegação, música e contacto usam `details`/`summary`; o estado pertence ao navegador e não necessita de código de cliente. As animações usam CSS, com alternativas estáticas e suporte a movimento reduzido. `src/styles/motion.css` concentra a atmosfera, as máscaras editoriais, o parallax e as transições entre páginas.

O início prolonga o fumo das capas num fundo partilhado por todas as secções. Uma textura gerada com as referências originais conserva os filamentos e o vermelho luminoso; duas camadas reutilizam a imagem com movimento CSS. Um único vídeo local adiciona curls reais sobre as margens da fotografia, abaixo dos textos. O navegador escolhe o WebP e o MP4 adequados ao viewport através de fontes nativas. O script local usa `pointermove` no desktop e eventos de toque passivos no mobile para desenhar num canvas transparente um rasto fino, com no máximo 115 px, desde a ponta da seta ou do dedo. Um toque breve cria um pequeno fio visível ao largar o dedo; um arrasto prolonga-o sem impedir o scroll. O rasto dissipa-se em 560 ms, não captura cliques e não faz pedidos de rede. O checkbox “PAUSAR EFECTOS” oculta o vídeo e o rasto, congela as texturas no ponto atual e desativa as restantes animações CSS. Movimento reduzido mantém a textura estática sem solicitar MP4 nem alocar o canvas. O foco permanece no viewport, e o footer reserva espaço para o controlo. A intensidade diminui perto do contacto nos browsers com suporte às timelines CSS usadas. Os conteúdos continuam visíveis sem esse suporte. Os URLs do CSS e do script incluem impressões do seu conteúdo para carregar os ficheiros atualizados após cada publicação.

## Conteúdo e fontes reais

Editar `src/content/copy.rs` para textos e headings. Editar `src/content/mod.rs` para capas, muses, músicas, plataformas e contacto. Os nomes Karen e Zoe são rótulos editoriais derivados das capas fornecidas; não foram acrescentadas biografias, interesses ou testemunhos.

Os quatro lançamentos atribuídos ao perfil PSIKOPAPA na Apple Music Venezuela foram confirmados no catálogo público: “Karen”, “Zoe (La Cortada)”, “Gata 4x4” e “Flaquita (Casados por Error)”. As duas primeiras faixas acompanham as capas editoriais; as restantes aparecem numa lista de lançamentos. O site liga diretamente às faixas e ao perfil do artista.

Não foram confirmados links diretos Spotify, YouTube Music nem ficheiros de áudio das novas faixas. Os botões com o ícone Spotify abrem uma pesquisa pelo artista e pela faixa e dizem explicitamente “BUSCAR EN SPOTIFY”; não simulam reprodução nem atribuem um perfil não verificado. Os cartões Karen e Zoe aproximam-se durante o scroll; uma luz dourada percorre continuamente as bordas enquanto os cartões estão no ecrã. Hover, foco e toque acrescentam um reflexo breve. O movimento é desativado pelo controlo de pausa ou quando o sistema pede redução de animações. Ao receber dados confirmados:

- Acrescentar o ficheiro de áudio em `assets/media/` e definir `Track.audio`, por exemplo `Some("/assets/media/faixa.mp3")`. O player nativo aparecerá automaticamente, sem autoplay e com `preload="none"`.
- Preencher `spotify` e `youtube` nos lançamentos apropriados.
- Preencher `SOCIAL_LINKS`, `CONTACT_EMAIL` ou `CONTACT_WHATSAPP` com destinos oficiais.
- Definir `COMMUNITY_WHATSAPP` com o convite real da comunidade e substituir a fotografia editorial do cartão pela fotografia autorizada do grupo.

A referência discreta à Seeta aponta para a sua [página na Google Play](https://play.google.com/store/apps/details?id=com.mztech.seeta&hl=es_VE), que descreve conversas por vídeo e salas ao vivo. O site não afirma que PSIKOPAPA possui a aplicação. O cartão de comunidade usa uma imagem editorial já fornecida e o ícone WhatsApp de Simple Icons (CC0). O ícone Spotify vem da mesma coleção; a proveniência de ambos está em `docs/licenses/`.

Depois de alterar o conteúdo, regenerar e publicar o Site.

O AI Lab apresenta as áreas criativas do projeto; não simula uma ferramenta de geração ou pedidos enviados.

As quatro imagens do briefing foram preservadas como JPEG e reencodadas em WebP nas larguras 160, 320, 480, 800 e 1280. Os enquadramentos editoriais são CSS; os rostos não foram retocados nem substituídos. Novas imagens e informações pessoais devem corresponder ao conteúdo autorizado pela participante.

Cormorant Garamond normal e italic e Manrope são servidas localmente em WOFF2. As licenças SIL Open Font License estão em `assets/fonts`; os ficheiros anteriores de Bodoni permanecem como assets de origem, sem carregamento pela página. O preload da imagem principal usa o mesmo srcset e sizes do componente; as imagens secundárias têm prioridade baixa e lazy loading. A proveniência e preparação da atmosfera estão documentadas em [docs/ASSETS.md](docs/ASSETS.md).

## Justificação das dependências

| Dependência | Necessidade técnica |
|---|---|
| `dioxus` | Componentes Rust, macros RSX e tipos HTML; `signals` é exigido pelas macros desta versão. As features de web e launch estão desativadas. |
| `dioxus-ssr` | Renderização dos componentes em HTML no build, sem hidratação. |
| `tracing` / `tracing-subscriber` | Logs estruturados do build e preview. |
| `axum` / `tokio` / `tower-http` | Apenas na feature opcional `preview`: servidor HTTP Rust, assets, headers e respostas 404. |
| `scraper` | Apenas em testes: análise semântica do HTML, sem depender de comparação de strings para navegação e acessibilidade estrutural. |
| `tungstenite` / `serde_json` / `base64` | Apenas na ferramenta independente de auditoria: comunicação WebSocket com Chromium, protocolo JSON e gravação de screenshots. |

Não existem dependências Node, gestores de pacotes JavaScript ou base de dados. Bibliotecas específicas de WASM que constem da resolução transitiva de Dioxus não são compiladas para a página nem publicadas no output estático. `assets/motion/pointer-smoke.js` é a única exceção de JavaScript e limita-se ao movimento do fumo com o rato ou com o toque.

## Auditoria no navegador

Com a preview em execução, iniciar uma instância dedicada de Chromium:

```sh
chromium --headless --no-sandbox --disable-gpu --disable-background-networking --no-first-run --remote-debugging-port=9222 --user-data-dir=/tmp/psikopapa-browser about:blank
```

Executar os checks nativos, sem avaliar JavaScript no navegador:

```sh
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml -- --motion
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml -- --pointer
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml -- --performance
```

Os relatórios e screenshots ficam em `docs/qa/`. A auditoria sem JavaScript verifica dimensões, nomes acessíveis, recursos, disclosures por teclado, texto a 200% e movimento reduzido. `--motion` verifica reprodução nativa e seleção responsiva do vídeo e das texturas, a escala dos cartões durante o scroll e os pixels da borda a mudarem enquanto o brilho percorre os cartões Karen e Zoe, pausa e reativação por teclado, estabilidade dos pixels pausados e textura estática com movimento reduzido. `--pointer` ativa o único script e mede os píxeis desenhados no canvas: a linha termina na posição do cursor, mantém-se estreita, aparece sobre um cartão, desaparece ao parar e fica oculta com pausa ou movimento reduzido. No mobile confirma o fio num toque no menu, a abertura desse menu, o rasto durante o arrasto e o scroll real da página. Uma captura adicional congela as texturas CSS para provar que os curls do vídeo mudam de forma no primeiro ecrã. Precisa de ImageMagick para ler os pixels das screenshots; a comparação exclui a scrollbar animada do browser. A medição de performance é de laboratório, na preview local, com rede e CPU limitadas; começa numa página vazia e com cache limpo. Não representa dados de utilização real nem certificação WCAG.

Os resultados executados estão descritos em `docs/QA.md`.
