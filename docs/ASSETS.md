# Fumo das capas e tipografia

A atmosfera deriva visualmente das referências `9253.jpg` (Karen mi amor) e `9254.jpg` (PSIKOPAPA). A textura de continuação foi gerada com `image_gen`, usando ambas as imagens como referências, para manter os filamentos finos, o vermelho luminoso saturado e os intervalos negros das capas. O asset não contém pessoas, rostos, texto, logos ou outros objetos. Os retratos originais permanecem intactos.

Foi criada uma única imagem, sem variantes: PNG RGB de 1536 × 1024 px, SHA-256 `2bbb062e239b41b1e0f88e7c8357dfe3901ca98dc5131bad27a5f56b95e52cb1`. O original de trabalho fica fora do checkout; apenas as versões comprimidas são publicadas. A compressão foi executada com FFmpeg, sem alteração de cor: WebP desktop de 1536 × 1024 px (185 600 bytes) e mobile de 960 × 640 px (88 624 bytes).

```sh
ffmpeg -i smoke-continuation.png -vf scale=960:640 -c:v libwebp -quality 87 -compression_level 6 -frames:v 1 assets/atmosphere/cover-smoke-mobile.webp
ffmpeg -i smoke-continuation.png -c:v libwebp -quality 87 -compression_level 6 -frames:v 1 assets/atmosphere/cover-smoke-desktop.webp
```

O HTML usa `picture` com seleção nativa da versão mobile até 899 px. Duas camadas reutilizam o mesmo ficheiro, com movimentos CSS independentes de deslocamento, escala e rotação. Em mobile, cada camada enquadra a respetiva margem da textura com `object-fit: cover`, preservando as curvas do fumo num ecrã vertical. As máscaras fundem as margens da fotografia com o fundo. Todas as secções partilham essa atmosfera.

## Movimento real do fumo

Um único vídeo local e silencioso adiciona espirais de fumo que mudam de forma sobre as margens da fotografia e abaixo dos textos. Origem: [Smoke effect over black background](https://mixkit.co/free-stock-video/smoke-effect-over-black-background-1967/), item 1967 da Mixkit, identificado na página como **Stock Video Free License**. Essa licença permite projetos comerciais e modificações sem exigir atribuição. A origem e o texto consultado ficam em `docs/licenses/mixkit-smoke-flow.txt`.

O original de 1280 × 720 px não é publicado. O vídeo recebe uma transformação RGB para vermelho luminoso, conservando os intervalos negros. O loop de 12 segundos dissolve os últimos dois segundos nos primeiros dois. O ficheiro desktop tem 960 × 540 px, 24 fps e 789 665 bytes. A versão mobile tem 360 × 640 px, 24 fps e 442 285 bytes; combina duas orientações da mesma sequência, com uma diferença temporal de dois segundos. A composição ocorre em RGB para conservar a tonalidade vermelha.

```sh
ffmpeg -i smoke-source.mp4 -filter_complex "[0:v]split[a][b];[a]trim=start=2:end=14,setpts=PTS-STARTPTS,fps=24,settb=AVTB[body];[b]trim=start=0:end=2,setpts=PTS-STARTPTS,fps=24,settb=AVTB[head];[body][head]xfade=transition=fade:duration=2:offset=10,scale=960:540,format=gbrp,lutrgb=r='clip((val-8)*2.8,0,255)':g='clip((val-8)*0.16,0,22)':b='clip((val-8)*0.22,0,30)',format=yuv420p[out]" -map '[out]' -an -c:v libx264 -preset slow -crf 27 -maxrate 550k -bufsize 1100k -movflags +faststart assets/media/cover-smoke-flow-desktop.mp4
ffmpeg -i assets/media/cover-smoke-flow-desktop.mp4 -filter_complex '[0:v]scale=640:360,transpose=clock,fps=24,format=gbrp,split[l][r];[r]hflip,split[r0][r1];[r0]trim=start=2:end=12,setpts=PTS-STARTPTS[t];[r1]trim=start=0:end=2,setpts=PTS-STARTPTS[h];[t][h]concat=n=2:v=1:a=0[phase];[l][phase]blend=all_mode=screen,format=yuv420p[out]' -map '[out]' -an -c:v libx264 -preset slow -crf 29 -maxrate 280k -bufsize 560k -movflags +faststart assets/media/cover-smoke-flow-mobile.mp4
```

O processamento com FFmpeg ocorre apenas na preparação dos assets. A aplicação continua exclusivamente em Rust. As fontes `video/source` usam condições nativas de viewport e movimento reduzido. O vídeo usa autoplay silencioso, `playsinline`, `loop` e `preload="none"`; os browsers podem impedir autoplay por preferência ou poupança de energia. A textura das capas permanece visível se o vídeo não reproduzir.

O controlo nativo “PAUSAR EFECTOS” oculta o vídeo, congela as texturas e desativa as restantes animações CSS. Ocultar o vídeo não garante suspender a descodificação em todos os browsers. Movimento reduzido mantém a textura estática e não seleciona fontes MP4. O FER torna a atmosfera mais discreta, preservando a continuidade visual. Nenhum rosto recebe animação ou transformação de identidade.

## Selo e entrelaçado do FER

O selo triangular e a escultura entrelaçada originais permanecem como estudos arquivados. As bandas isoladas também estão arquivadas: não encaixavam exatamente nas imagens completas. A página carrega exclusivamente as novas cenas `fer-editorial-*`, preparadas a partir da direção aprovada dos mockups. Nenhuma imagem é apresentada como fotografia de um objeto histórico. As referências consultadas estão em `docs/fer-artifact-sources.md`.

## Revisão editorial do FER

O painel de três mockups foi gerado antes da alteração da página e está em `docs/mockups/fer/page-direction.webp` (321 KiB). Usa a captura do manifesto fornecida pelo utilizador e as peças A/C como referências. A galeria de mockups é separada da página pública; o painel não carrega na experiência principal.

As três cenas finais foram geradas com `image_gen`, seguindo os materiais, cores e composição dos mockups aprovados. Os PNGs de trabalho ficam em `/workspace/generated_images/`; a preparação WebP utiliza apenas redimensionamento e compressão FFmpeg. Cada imagem aparece uma vez na página, em variantes quadradas de 600 e 1200 px selecionadas pelo browser segundo viewport e densidade de pixels:

| Cena | WebP 600 px | WebP 1200 px |
|---|---:|---:|
| Entrelaçado, `fer-editorial-interlace` | 78 174 bytes | 240 458 bytes |
| Selo, `fer-editorial-seal` | 84 748 bytes | 254 076 bytes |
| Rosa e Tridente, `fer-editorial-rose` | 111 462 bytes | 346 342 bytes |

Na terceira cena, F/E/R circundam a rosa num Tridente triangular; **Prosperidad** é texto HTML no centro da flor. O circuito abaixo mostra desejo, criação, liberdade e novo desejo. O entrelaçado carrega de imediato e tem preload responsivo; o selo e a rosa têm carregamento diferido. Câmara, textura de fumo vermelho, linhas e luzes partilham as coordenadas de cada cena. A convergência e o brilho repetem-se em cinco segundos; a câmara respira em oito segundos e o reflexo do aro em seis. Os textos permanecem imóveis e legíveis. Pausa e movimento reduzido mantêm os objetos completos e ocultam os overlays animados. Não se adicionou runtime ou dependência de aplicação.

## Tipografia e transições

Cormorant Garamond normal e italic, pesos variáveis 400–600, são servidos localmente em WOFF2 latin. Origem: Google Fonts / Christian Thalmann. A licença SIL OFL encontra-se em `assets/fonts/Cormorant-Garamond-OFL.txt`. Manrope permanece na navegação. A página FER usa Bodoni Moda nos títulos e nas letras, com Cormorant nos textos editoriais curtos. Bodoni normal já existia; foi acrescentada a versão italic latina de 16 552 bytes, obtida de [Google Fonts](https://fonts.google.com/specimen/Bodoni+Moda), coberta por `assets/fonts/Bodoni-Moda-OFL.txt`. Cormorant normal tem preload global e Bodoni italic tem preload apenas na rota FER.

As revelações, o parallax e as transições continuam a ser melhorias progressivas CSS: os conteúdos permanecem visíveis quando as timelines de scroll não são suportadas. O URL do stylesheet inclui um identificador baseado no seu conteúdo para evitar utilizar CSS anterior após uma publicação.
