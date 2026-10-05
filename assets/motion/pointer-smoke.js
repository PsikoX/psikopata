// The only browser script: draw a short red smoke filament from a mouse or finger.
(() => {
  const canvas = document.querySelector('.mouse-smoke');
  const context = canvas?.getContext('2d', { alpha: true });
  if (!context) return;

  const allowed = matchMedia('(prefers-reduced-motion: no-preference)');
  const pause = document.getElementById('pause-motion');
  const points = [];
  const lifetime = 560;
  const maximumLength = 115;
  let frame = 0;
  let activeTouchId = null;
  let lastTouchAt = -Infinity;

  const clear = () => {
    if (frame) cancelAnimationFrame(frame);
    frame = 0;
    points.length = 0;
    context.clearRect(0, 0, innerWidth, innerHeight);
  };

  const resize = () => {
    clear();
    if (!allowed.matches) {
      canvas.width = canvas.height = 0;
      return;
    }
    const ratio = Math.min(devicePixelRatio || 1, 1.5);
    canvas.width = Math.round(innerWidth * ratio);
    canvas.height = Math.round(innerHeight * ratio);
    context.setTransform(ratio, 0, 0, ratio, 0, 0);
    context.lineCap = 'round';
    context.lineJoin = 'round';
  };

  const trim = (now) => {
    while (points.length && now - points[0].time > lifetime) points.shift();
    let distance = 0;
    for (let index = points.length - 1; index > 0; index--) {
      distance += Math.hypot(points[index].x - points[index - 1].x, points[index].y - points[index - 1].y);
      if (distance > maximumLength) {
        points.splice(0, index);
        break;
      }
    }
  };

  const draw = (now) => {
    frame = 0;
    context.clearRect(0, 0, innerWidth, innerHeight);
    if (!allowed.matches || pause?.checked) return clear();
    trim(now);

    for (let index = 1; index < points.length; index++) {
      const from = points[index - 1];
      const to = points[index];
      const oldness = Math.min(1, (now - to.time) / lifetime);
      const strength = (1 - oldness) ** 2 * (0.2 + 0.8 * index / (points.length - 1));
      if (strength <= 0) continue;
      const curl = (point) => {
        const age = Math.min(1, (now - point.time) / lifetime);
        return [point.x + Math.sin(now * 0.008 + point.x * 0.025) * age * 3, point.y - age * 9];
      };
      const [x1, y1] = curl(from);
      const [x2, y2] = curl(to);
      context.beginPath();
      context.moveTo(x1, y1);
      context.lineTo(x2, y2);
      context.shadowColor = '#ee173e';
      context.shadowBlur = 7;
      context.lineWidth = 4.2;
      context.strokeStyle = `rgba(232, 17, 49, ${0.3 * strength})`;
      context.stroke();
      context.shadowBlur = 0;
      context.lineWidth = 1.4;
      context.strokeStyle = `rgba(255, 58, 82, ${0.9 * strength})`;
      context.stroke();
    }

    if (points.length) frame = requestAnimationFrame(draw);
  };

  const addPoint = (x, y, now) => {
    const last = points[points.length - 1];
    const distance = last ? Math.hypot(x - last.x, y - last.y) : 0;
    if (distance > 180) points.length = 0;
    const previous = points[points.length - 1];
    if (previous && distance > 1 && distance <= 180) {
      const steps = Math.ceil(distance / 6);
      for (let step = 1; step <= steps; step++) {
        const fraction = step / steps;
        points.push({
          x: previous.x + (x - previous.x) * fraction,
          y: previous.y + (y - previous.y) * fraction,
          time: now,
        });
      }
    } else if (!previous) {
      points.push({ x, y, time: now });
    }
    trim(now);
    if (!frame) frame = requestAnimationFrame(draw);
  };

  document.addEventListener('pointermove', (event) => {
    if (event.pointerType !== 'mouse' || activeTouchId !== null) return;
    if (!allowed.matches || pause?.checked) return clear();
    addPoint(event.clientX, event.clientY, performance.now());
  }, { passive: true });

  document.addEventListener('touchstart', (event) => {
    if (!allowed.matches || pause?.checked || activeTouchId !== null) return;
    const touch = event.changedTouches[0];
    if (!touch) return;
    clear();
    activeTouchId = touch.identifier;
    lastTouchAt = performance.now();
    // A tiny exposed wisp remains visible beside the finger after a quick tap.
    points.push({ x: touch.clientX - 9, y: touch.clientY + 19, time: lastTouchAt });
    points.push({ x: touch.clientX - 4, y: touch.clientY + 11, time: lastTouchAt });
    points.push({ x: touch.clientX, y: touch.clientY + 3, time: lastTouchAt });
    frame = requestAnimationFrame(draw);
  }, { passive: true });

  window.addEventListener('touchmove', (event) => {
    if (activeTouchId === null || !allowed.matches || pause?.checked) return;
    const touch = Array.from(event.touches).find(item => item.identifier === activeTouchId);
    if (!touch) return;
    lastTouchAt = performance.now();
    addPoint(touch.clientX, touch.clientY, lastTouchAt);
  }, { passive: true });

  const finishTouch = (event) => {
    if (Array.from(event.changedTouches).some(item => item.identifier === activeTouchId)) {
      activeTouchId = null;
      lastTouchAt = performance.now();
    }
  };
  window.addEventListener('touchend', finishTouch, { passive: true });
  window.addEventListener('touchcancel', finishTouch, { passive: true });
  document.documentElement.addEventListener('pointerleave', (event) => {
    if (event.pointerType === 'mouse') clear();
  });
  window.addEventListener('scroll', () => {
    if (performance.now() - lastTouchAt > lifetime) clear();
  }, { passive: true });
  window.addEventListener('resize', resize);
  allowed.addEventListener('change', resize);
  pause?.addEventListener('change', clear);
  document.addEventListener('visibilitychange', () => { if (document.hidden) clear(); });
  resize();
})();
