# FER — revisão editorial, 2026-10-04

Os três mockups foram criados antes das alterações da página. Depois da aprovação do painel, as três cenas finais seguem a mesma direção: entrelaçado na entrada, selo na união e rosa no Tridente. Cada imagem aparece uma vez. O manifesto abre a experiência; as definições são curtas, e o contexto adicional fica em disclosures nativos. **Prosperidad** está no centro da rosa, com F/E/R nos vértices que convergem para ela. As imagens anteriores permanecem como estudos arquivados, sem carregamento pela página FER.

Verificações executadas em Chromium com viewports emulados:

- 320, 390, 768 e 1440 px: sem overflow horizontal, pedidos de recursos falhados ou ações sem nome acessível.
- Página FER: zero pedidos de JavaScript.
- Texto a 200%, viewport de 390 px: largura do conteúdo de 390 px.
- Primeira pergunta abre com a tecla Enter.
- Os traços, as luzes, a textura de fumo e a câmara das peças mudam de posição ao longo do tempo.
- A pausa oculta as luzes e o fumo sobre as peças e retira a animação da câmara; movimento reduzido mostra as cenas completas sem overlays.
- O reflexo do aro muda de transformação; fica oculto com pausa e movimento reduzido.
- Imagem da rosa: versão de 600 px em mobile com DPR 1, 1200 px em desktop. Entrada mobile com DPR 2: imagem de 1200 px, compatível com o preload responsivo.
- Fontes efetivamente renderizadas verificadas via Chromium: Bodoni Moda regular e italic nos títulos e Cormorant Garamond na explicação curta. Cores próprias das três letras sobre os medallhões confirmadas.
- Enquadramentos da entrada, forças, ciclo e retrato revistos por screenshots.

Rust: `cargo fmt`, `cargo check --locked --all-targets --all-features`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, nove testes de contrato e build de produção executados com sucesso. Os contratos verificam também que há três imagens distintas e que a prosperidade está no centro da composição da rosa. A auditoria de browser usa `tools/browser-audit` e um script de captura local; os scripts de QA não integram a aplicação.

As capturas são guardadas em `docs/qa/fer-editorial-*.png` e `docs/qa/fer-review-*.png`, ignoradas pelo Git. Viewports emulados verificam layout e interações em Chromium; não substituem ensaios em dispositivos físicos.
