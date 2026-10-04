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

O selo triangular e a escultura entrelaçada foram gerados como propostas artísticas. As três bandas da escultura foram isoladas com `image_gen` a partir do estudo visual e convertidas para WebP com transparência. O selo completo tem 398 KiB; a escultura completa, 257 KiB; as três bandas comprimidas somam cerca de 580 KiB. As bandas da peça de abertura carregam de imediato para que a animação comece sem atraso; o selo da convergência usa carregamento diferido. A animação desloca e roda camadas com CSS; a imagem completa permanece disponível como estado estático para movimento reduzido e para o controlo de pausa. Nenhuma das imagens é apresentada como fotografia de um objeto histórico. As referências consultadas estão em `docs/fer-artifact-sources.md`.

## Tipografia e transições

Cormorant Garamond normal e italic, pesos variáveis 400–600, são servidos localmente em WOFF2 latin. Origem: Google Fonts / Christian Thalmann. A licença SIL OFL encontra-se em `assets/fonts/Cormorant-Garamond-OFL.txt`. Manrope continua a ser a família dos textos de leitura e navegação. Apenas a Cormorant normal é pré-carregada; o browser solicita o itálico quando encontra conteúdo que o usa.

As revelações, o parallax e as transições continuam a ser melhorias progressivas CSS: os conteúdos permanecem visíveis quando as timelines de scroll não são suportadas. O URL do stylesheet inclui um identificador baseado no seu conteúdo para evitar utilizar CSS anterior após uma publicação.
