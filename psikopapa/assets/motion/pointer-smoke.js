// The only browser script: move the existing CSS smoke to the real pointer position.
(() => {
  const stage = document.querySelector('.cinematic-universe');
  const smoke = stage?.querySelector('.mouse-smoke');
  if (!stage || !smoke) return;

  const allowed = matchMedia('(min-width: 900px) and (prefers-reduced-motion: no-preference)');
  let x = 0;
  let y = 0;
  let frame = 0;

  const hide = () => {
    if (frame) cancelAnimationFrame(frame);
    frame = 0;
    smoke.classList.remove('is-active');
  };

  stage.addEventListener('pointermove', (event) => {
    if (!allowed.matches || event.pointerType !== 'mouse') {
      hide();
      return;
    }
    x = event.clientX;
    y = event.clientY;
    if (frame) return;
    frame = requestAnimationFrame(() => {
      frame = 0;
      smoke.style.setProperty('--pointer-x', `${x}px`);
      smoke.style.setProperty('--pointer-y', `${y}px`);
      smoke.classList.add('is-active');
    });
  }, { passive: true });

  stage.addEventListener('pointerleave', hide);
  window.addEventListener('scroll', hide, { passive: true });
  allowed.addEventListener('change', hide);
  document.getElementById('pause-motion')?.addEventListener('change', hide);
})();
