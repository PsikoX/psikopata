# PSIKOPAPA

Website oficial de PSIKOPAPA, com uma narrativa em três atos: desejo, curiosidade e descoberta de **FER — Fetiche, Educação e Religião**.

A aplicação é escrita em Rust. Dioxus compõe as páginas e `dioxus-ssr` gera HTML estático durante o build. O artefacto publicado contém HTML, CSS, imagens, fontes e vídeo local; não contém JavaScript, runtime WASM, hidratação ou serviços externos de frontend. A preview local também é Rust, através de Axum.

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

`SITE_BASE_PATH` ajusta a navegação, as imagens responsivas, os vídeos, as fontes, o canonical e o sitemap ao subdiretório da página. Depois do build, sincronizar `dist/` com `psikopapa/` numa cópia da branch `gh-pages` e fazer push dessa branch. O build normal do Site privado continua a usar a raiz do seu domínio. GitHub Pages serve os ficheiros gerados; o website publicado não carrega JavaScript.

O gerador valida o domínio antes de gerar canonical, Open Graph e sitemap. Um build sem origem utiliza `robots.txt` com `Disallow: /`. Os headers propostos em `dist/_headers` proíbem execução de scripts e restringem recursos à própria origem, com exceção de áudio HTTPS. O hosting deve aplicar esses headers; a preview aplica CSP e `nosniff` diretamente.

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
| `tests` | Contratos de navegação, conteúdo, SEO, assets e zero JavaScript |
| `tools/browser-audit` | Auditoria independente em Rust através do protocolo nativo do Chromium |

As rotas são `/`, `/fer/` e `404.html`. A navegação entre secções utiliza âncoras nativas. Os disclosures de navegação, música e contacto usam `details`/`summary`; o estado pertence ao navegador e não necessita de código de cliente. As animações usam CSS, com alternativas estáticas e suporte a movimento reduzido. `src/styles/motion.css` concentra a atmosfera, as máscaras editoriais, o parallax e as transições entre páginas.

O início prolonga o fumo das capas num fundo partilhado por todas as secções. Uma textura gerada com as referências originais conserva os filamentos e o vermelho luminoso; duas camadas reutilizam a imagem com movimento CSS. Um único vídeo local adiciona curls reais sobre as margens da fotografia, abaixo dos textos. O navegador escolhe o WebP e o MP4 adequados ao viewport através de fontes nativas. O checkbox “PAUSAR EFECTOS” oculta o vídeo, congela as texturas no ponto atual e desativa as restantes animações CSS. Movimento reduzido mantém a textura estática sem solicitar MP4. O foco permanece no viewport, e o footer reserva espaço para o controlo. A intensidade diminui na chegada ao FER nos browsers com suporte às timelines CSS usadas. Os conteúdos continuam visíveis sem esse suporte. O URL do CSS inclui uma impressão do seu conteúdo para carregar o estilo atualizado após cada publicação.

## Conteúdo e fontes reais

Editar `src/content/copy.rs` para textos e headings. Editar `src/content/mod.rs` para capas, muses, músicas, plataformas e contacto. Os nomes Karen e Zoe são rótulos editoriais derivados das capas fornecidas; não foram acrescentadas biografias, interesses ou testemunhos.

Os áudios e URLs oficiais não foram fornecidos. Os campos correspondentes são `None`; a interface comunica essa ausência e não apresenta players falsos ou links presumidos. Ao receber dados confirmados:

- Acrescentar o ficheiro de áudio em `assets/media/` e definir `Track.audio`, por exemplo `Some("/assets/media/faixa.mp3")`. O player nativo aparecerá automaticamente, sem autoplay e com `preload="none"`.
- Preencher `spotify`, `apple_music` e `youtube` nos lançamentos apropriados.
- Preencher `SOCIAL_LINKS`, `CONTACT_EMAIL` ou `CONTACT_WHATSAPP` com destinos oficiais.
- Regenerar e publicar o Site.

LAS 7 apresenta as sete posições conceptuais do programa. Não afirma inscrições, vagas restantes ou participantes confirmadas. O AI Lab apresenta as áreas criativas do projeto; não simula uma ferramenta de geração ou pedidos enviados.

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

Não existem dependências Node, gestores de pacotes JavaScript ou base de dados. Bibliotecas específicas de WASM que constem da resolução transitiva de Dioxus não são compiladas para a página nem publicadas no output estático.

## Auditoria no navegador

Com a preview em execução, iniciar uma instância dedicada de Chromium:

```sh
chromium --headless --no-sandbox --disable-gpu --disable-background-networking --no-first-run --remote-debugging-port=9222 --user-data-dir=/tmp/psikopapa-browser about:blank
```

Executar os checks nativos, sem avaliar JavaScript no navegador:

```sh
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml -- --motion
cargo run --locked --manifest-path tools/browser-audit/Cargo.toml -- --performance
```

Os relatórios e screenshots ficam em `docs/qa/`. A auditoria verifica dimensões, nomes acessíveis, recursos, ausência de requests JavaScript, disclosures por teclado, texto a 200% e movimento reduzido. `--motion` verifica reprodução nativa e seleção responsiva do vídeo e das texturas, pausa e reativação por teclado, estabilidade dos pixels pausados e textura estática com movimento reduzido. Uma captura adicional congela as texturas CSS para provar que os curls do vídeo mudam de forma no primeiro ecrã. Precisa de ImageMagick para ler os pixels das screenshots; a comparação exclui a scrollbar animada do browser. A medição de performance é de laboratório, na preview local, com rede e CPU limitadas; começa numa página vazia e com cache limpo. Não representa dados de utilização real nem certificação WCAG.

Os resultados executados estão descritos em `docs/QA.md`.
