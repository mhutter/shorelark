"use strict";

import { Simulation } from "lib-simulation-wasm";

const simulation = new Simulation();

const viewport = document.getElementById("viewport");

const vw = Math.max(
  document.documentElement.clientWidth || 0,
  window.innerWidth || 0,
);
const vh = Math.max(
  document.documentElement.clientHeight || 0,
  window.innerHeight || 0,
);
const viewportWidth = vw - 16;
const viewportHeight = vh - 16;
const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.height = viewportHeight + "px";
viewport.style.width = viewportWidth + "px";

const ctx = viewport.getContext("2d");
ctx.scale(viewportScale, viewportScale);

CanvasRenderingContext2D.prototype.drawTriangle = function(
  x,
  y,
  size,
  rotation,
) {
  this.beginPath();

  this.moveTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5,
  );

  this.lineTo(
    x - Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
  );
  this.lineTo(
    x - Math.sin(rotation + (4.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * size,
  );

  this.lineTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5,
  );

  this.stroke();
  this.fillStyle = "rgb(244, 246, 255)"; // pretty bright white
  this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = function(x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2.0 * Math.PI);
  this.fillStyle = "rgb(243, 198, 35)"; // brigh yellow
  this.fill();
};

let frames = 0;
let tSim = 0.0;
let tRender = 0.0;

// log FPS every second
setInterval(() => {
  const avgSim = tSim / frames;
  const avgRender = tRender / frames;
  const budget = 1000.0 / frames;
  console.debug(
    `FPS: ${frames},`,
    `avg Δt sim: ${avgSim.toFixed(2)}ms,`,
    `avg Δt render: ${avgRender.toFixed(2)}ms,`,
    `budget: ${budget.toFixed(2)}ms`,
  );
  frames = 0;
  tSim = 0.0;
  tRender = 0.0;
}, 1000);

function redraw() {
  const startSim = performance.now();
  simulation.step();
  tSim += performance.now() - startSim;

  const startRender = performance.now();
  const world = simulation.world();

  ctx.clearRect(0, 0, viewportWidth, viewportHeight);

  for (const food of world.foods) {
    ctx.drawCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      (0.01 / 2.0) * viewportWidth,
    );
  }

  for (const animal of world.animals) {
    ctx.drawTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation,
    );
  }

  frames += 1;
  tRender += performance.now() - startRender;
  requestAnimationFrame(redraw);
}

redraw();
