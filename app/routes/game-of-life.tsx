import { useEffect, useRef, useState } from "react";
import initWasm, { Universe } from "../../rust-wasm-lib/pkg/rust_wasm_lib";

const CELL_SIZE = 5; // Smaller cells
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

export default function GameOfLife() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [universe, setUniverse] = useState<Universe | null>(null);

  useEffect(() => {
    const runGame = async () => {
      const wasm = await initWasm();
      const universe = Universe.new();
      const width = universe.width();
      const height = universe.height();
      const canvas = canvasRef.current;
      if (!canvas) return;
      canvas.width = (CELL_SIZE + 1) * width + 1;
      canvas.height = (CELL_SIZE + 1) * height + 1;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      setUniverse(universe);

      const renderLoop = () => {
        universe.tick();
        drawGrid(ctx, width, height);
        drawCells(ctx, universe, width, height, wasm.memory);
        requestAnimationFrame(renderLoop);
      };

      drawGrid(ctx, width, height);
      drawCells(ctx, universe, width, height, wasm.memory);
      requestAnimationFrame(renderLoop);
    };

    runGame();
  }, []);

  const drawGrid = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number
  ) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const drawCells = (
    ctx: CanvasRenderingContext2D,
    universe: Universe,
    width: number,
    height: number,
    memory: WebAssembly.Memory
  ) => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = row * width + col;

        ctx.fillStyle = cells[idx] === 1 ? ALIVE_COLOR : DEAD_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  };

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!universe) return;
    const canvas = canvasRef.current;
    if (!canvas) return;

    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(
      Math.floor(canvasTop / (CELL_SIZE + 1)),
      universe.height() - 1
    );
    const col = Math.min(
      Math.floor(canvasLeft / (CELL_SIZE + 1)),
      universe.width() - 1
    );

    universe.toggle_cell(row, col);
  };

  return <canvas ref={canvasRef} onClick={handleCanvasClick} />;
}
