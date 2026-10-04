# FER — revisão editorial, 2026-10-04

Os três mockups foram criados antes das alterações da página. O manifesto enviado pelo utilizador abre a experiência; as definições são curtas, e o contexto adicional fica em disclosures nativos. O entrelaçado e o selo selecionados permanecem; o circuito utiliza uma nova peça material com aro de latão e rosa.

Verificações executadas em Chromium com viewports emulados:

- 320, 390, 768 e 1440 px: sem overflow horizontal, pedidos de recursos falhados ou ações sem nome acessível.
- Página FER: zero pedidos de JavaScript.
- Texto a 200%, viewport de 390 px: largura do conteúdo de 390 px.
- Primeira pergunta abre com a tecla Enter.
- O traço e a luz das peças mudam de posição ao longo do tempo.
- O reflexo do aro muda de transformação; fica oculto com pausa e movimento reduzido.
- Imagem do ciclo: versão de 600 px em mobile, 1200 px em desktop.
- Enquadramentos da entrada, forças, ciclo e retrato revistos por screenshots.

Rust: `cargo fmt -- --check`, `cargo check --locked --all-targets --all-features`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, nove testes de contrato e build de produção executados com sucesso. A auditoria de browser usa `tools/browser-audit` e um script de captura local; os scripts de QA não integram a aplicação.

As capturas são guardadas em `docs/qa/fer-editorial-*.png` e `docs/qa/fer-review-*.png`, ignoradas pelo Git. Viewports emulados verificam layout e interações em Chromium; não substituem ensaios em dispositivos físicos.
