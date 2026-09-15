const audio = document.querySelector('#audio');
const player = document.querySelector('#player');
const playButton = document.querySelector('#playButton');
const restartButton = document.querySelector('#restartButton');
const muteButton = document.querySelector('#muteButton');
const seek = document.querySelector('#seek');
const volume = document.querySelector('#volume');
const currentTime = document.querySelector('#currentTime');
const duration = document.querySelector('#duration');
const status = document.querySelector('#playerStatus');

const formatTime = (seconds) => {
  if (!Number.isFinite(seconds)) return '0:00';
  const minutes = Math.floor(seconds / 60);
  const remainder = String(Math.floor(seconds % 60)).padStart(2, '0');
  return `${minutes}:${remainder}`;
};

const updateTimeline = () => {
  currentTime.textContent = formatTime(audio.currentTime);
  currentTime.dateTime = `PT${Math.round(audio.currentTime)}S`;

  if (Number.isFinite(audio.duration) && audio.duration > 0) {
    seek.value = (audio.currentTime / audio.duration) * 100;
  }
};

const showPlayingState = (isPlaying) => {
  player.classList.toggle('is-playing', isPlaying);
  playButton.setAttribute('aria-pressed', String(isPlaying));
  playButton.setAttribute('aria-label', isPlaying ? 'Pausar Psicopapa' : 'Reproduzir Psicopapa');
};

playButton.addEventListener('click', async () => {
  if (audio.paused) {
    try {
      await audio.play();
      status.textContent = '';
    } catch {
      status.textContent = 'Não foi possível iniciar a música. Tenta novamente.';
    }
  } else {
    audio.pause();
  }
});

restartButton.addEventListener('click', async () => {
  audio.currentTime = 0;
  updateTimeline();
  try {
    await audio.play();
  } catch {
    status.textContent = 'Carrega no botão central para ouvir.';
  }
});

muteButton.addEventListener('click', () => {
  audio.muted = !audio.muted;
  muteButton.setAttribute('aria-pressed', String(audio.muted));
  muteButton.setAttribute('aria-label', audio.muted ? 'Ativar som' : 'Silenciar música');
});

seek.addEventListener('input', () => {
  if (Number.isFinite(audio.duration)) {
    audio.currentTime = (Number(seek.value) / 100) * audio.duration;
  }
});

volume.addEventListener('input', () => {
  audio.volume = Number(volume.value);
  audio.muted = false;
  muteButton.setAttribute('aria-pressed', 'false');
  muteButton.setAttribute('aria-label', 'Silenciar música');
});

audio.addEventListener('loadedmetadata', () => {
  duration.textContent = formatTime(audio.duration);
  duration.dateTime = `PT${Math.round(audio.duration)}S`;
});
audio.addEventListener('timeupdate', updateTimeline);
audio.addEventListener('play', () => showPlayingState(true));
audio.addEventListener('pause', () => showPlayingState(false));
audio.addEventListener('ended', () => {
  audio.currentTime = 0;
  updateTimeline();
});
audio.addEventListener('error', () => {
  status.textContent = 'A música não ficou disponível. Atualiza a página e tenta novamente.';
});

audio.volume = Number(volume.value);
