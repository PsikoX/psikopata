# Verificação executada — 2026-10-03

## Rust e output

- `cargo fmt`, `cargo check --all-targets --all-features` e Clippy com `-D warnings`: passaram.
- `cargo test --all-features`: nove testes de integração passaram, incluindo vídeo nativo, texturas decorativas, fontes condicionais, controlo de movimento e URLs no subdiretório do GitHub Pages.
- `cargo run --release --locked --bin build-site`: build de produção concluído.
- HTML: um H1 por página, linguagem `es-VE`, títulos distintos, descriptions, Open Graph, Twitter/X e canonical configurados.
- Todos os destinos internos e assets referenciados existem; IDs e títulos acessíveis são consistentes.
- FER preserva FETICHE, EDUCACIÓN e RELIGIÓN, com destaque no hero e secção própria imediatamente a seguir.
- Nenhum script, iframe, atributo de evento ou URL `javascript:` foi gerado.

## Chromium

A auditoria em Rust utiliza comandos DOM, Accessibility, Input, Emulation, Network, Media e PerformanceTimeline. O teste específico `--pointer` usa `Runtime.evaluate` apenas para medir os píxeis do canvas; o restante não precisa dessa avaliação.

Foram verificados o início a 320, 360, 390, 430, 768, 1024, 1440 e 1920 px e a página FER a 390 e 1440 px, com JavaScript desativado e movimento reduzido:

- Nenhum overflow horizontal, request de script ou recurso com resposta HTTP de erro.
- Nenhum link ou botão sem nome na árvore acessível consultada.
- Menu mobile, história da música e contacto abriram e fecharam com Enter.
- Texto ampliado a 200% em mobile manteve o viewport a 390 px.
- `scroll-behavior` tornou-se `auto` com movimento reduzido.
- O stylesheet com identificador de conteúdo carregou corretamente.
- Os cartões de Karen e Zoe mostram botões de pesquisa Spotify identificados; nenhum botão afirma abrir uma faixa direta sem URL confirmada.

Relatório: `qa/browser-report.json`. Os PNG são artefactos locais excluídos do Git e da publicação.

## Fumo com movimento real

Executado `tools/browser-audit --motion`, com JavaScript desativado, a 390×844 e 1440×1000:

- Mobile selecionou apenas `cover-smoke-flow-mobile.mp4` e desktop apenas `cover-smoke-flow-desktop.mp4`. As texturas WebP também corresponderam ao viewport.
- Os eventos Media confirmaram reprodução nativa de vídeo H.264 sem áudio.
- Uma captura adicional congelou as duas texturas CSS para isolar o movimento real do vídeo. Após dois segundos, cerca de **20%** dos pixels amostrados nas margens mudaram no mobile e **10%** no desktop. A contagem usa uma diferença RGB somada de pelo menos 18 e exclui o header e a scrollbar.
- Space no checkbox ocultou o vídeo, congelou as texturas e desativou as restantes animações CSS. Os pixels do conteúdo ficaram estáveis após a pausa. A reativação por teclado voltou a mostrar o vídeo e a mover as texturas.
- Com movimento reduzido, não ocorreram requests MP4; o fumo ficou estático e o controlo de pausa oculto.
- Cormorant Garamond LightItalic confirmou o itálico real do hero. A revelação das imagens abriu; a atmosfera permanece intensa no FER inicial e fica mais discreta perto do contacto.
- O foco por teclado aproximou o cartão da muse em mobile e desktop; no desktop, o hover ampliou-o mais. Movimento reduzido mantém os cartões estáticos.
- Nenhum overflow horizontal ou request de JavaScript nas duas larguras.
- Foram inspecionadas as capturas do hero, FER inicial, Muses e Educação em mobile e desktop para verificar tonalidade, continuidade e leitura.

Relatório: `qa/motion-report.json`. O vídeo mobile tem 442 285 bytes; o desktop, 789 665 bytes. Há apenas um vídeo para toda a página. A preparação e licença constam de `ASSETS.md`.

## Performance de laboratório

Chromium, viewport 390×844, cache vazio, latência de 150 ms, download de 200 000 bytes/s (1,6 Mbps), CPU limitada a 4× e animações normais. JavaScript desativado.

- LCP: aproximadamente **3,04 s**, incluindo o vídeo.
- Soma dos layout shifts observados sem interação recente: aproximadamente **0,000852**.
- Tempo de execução de scripts: **0 s**.
- Hero: preload WebP responsivo de 800 px, prioridade alta; fontes locais e texturas com prioridade baixa.

Os eventos originais estão em `qa/performance-report.json`. O LCP foi calculado do último evento `largest-contentful-paint` até ao início da navegação, alinhando os relógios monotónico e de parede. A soma de shifts não substitui CLS de campo. Não foram executados Lighthouse, testes em dispositivos físicos ou certificação formal de acessibilidade.

## Dados pendentes

As quatro faixas e o perfil Apple Music do artista foram confirmados no catálogo Apple Music Venezuela. Áudios locais, links diretos Spotify/YouTube Music, convite real da comunidade WhatsApp, fotografia do grupo e canal de contacto não foram fornecidos. O cartão de comunidade mostra uma imagem editorial identificada como tal e comunica que o acesso é por convite. Não foram criadas biografias, inscrições, disponibilidade comercial ou confirmações de ações externas.

Após a inclusão do teaser musical mobile, Seeta, lançamentos e cartão de comunidade, repetiu-se a auditoria em Chromium com JavaScript desativado: 320, 360, 390, 430, 768, 1024, 1440 e 1920 px sem overflow ou recursos falhados. A verificação a 200% de texto no viewport de 390 px também passou. Houve uma regressão inicial de overflow no link Seeta; a largura do link foi corrigida antes deste resultado final.

## Atualização — 2026-10-04

- LAS 7 foi removida da página, navegação, copy e estilos. O contacto segue agora a comunidade; o convite na página FER aponta para a comunidade.
- Em desktop, o hover acende uma camada leve do fumo vermelho original junto ao bloco editorial sob o rato. O fumo deslocou-se entre os cartões Karen e Zoe na auditoria `--pointer`; não recebe cliques e desaparece com pausa ou movimento reduzido. A camada fica desativada no mobile.
- A auditoria responsiva passou novamente de 320 a 1920 px, sem overflow, scripts ou recursos falhados. `--motion` passou em 390 e 1440 px com o novo ponto de redução do fumo perto do contacto.

## Correção do fumo do rato — 2026-10-04

- A auditoria anterior media apenas mudanças entre blocos. Não demonstrava movimento dentro do mesmo bloco; a crítica do utilizador estava correta.
- Com a exceção autorizada de um script local, a auditoria `--pointer` passou a testar dois pontos no mesmo bloco do hero. O centro do fumo correspondeu a cada posição do rato e percorreu os mesmos 1104 px. O pedido do único script foi confirmado.
- A camada mantém `pointer-events:none`, desaparece ao fazer scroll, com “PAUSAR EFECTOS” e com movimento reduzido, e não aparece no mobile. A versão sem JavaScript continua legível e navegável.

## Rasto fino — 2026-10-04

- O efeito grande junto ao rato foi substituído por um fio luminoso vermelho, com filamentos suaves, que sai da ponta da seta e dura até 560 ms. O rasto usa no máximo 115 px de percurso.
- No teste do hero, o canvas desenhou uma linha de aproximadamente 85 × 14 px terminada na coordenada do cursor. Após 700 ms, não restou nenhum píxel; sobre Karen voltou a aparecer. Pausa, movimento reduzido e mobile não exibiram o efeito.
- O script e o CSS têm URL versionado pelo conteúdo para evitar que o navegador continue a usar o efeito anterior em cache.
