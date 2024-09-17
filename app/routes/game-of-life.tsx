import { useEffect, useRef, useState } from "react";
import initWasm, { Universe } from "../../rust-wasm-lib/pkg/rust_wasm_lib";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const BOARD_FILL_FACTOR = 0.6;

export default function GameOfLife() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [universe, setUniverse] = useState<Universe | null>(null);
  const [birthThreshold, setBirthThreshold] = useState(3);
  const [survivalThresholdMin, setSurvivalThresholdMin] = useState(2);
  const [survivalThresholdMax, setSurvivalThresholdMax] = useState(3);

  useEffect(() => {
    const runGame = async () => {
      const wasm = await initWasm();
      const windowWidth = window.innerWidth;
      const windowHeight = window.innerHeight;
      const width = Math.floor((windowWidth * BOARD_FILL_FACTOR) / CELL_SIZE);
      const height = Math.floor((windowHeight * BOARD_FILL_FACTOR) / CELL_SIZE);
      const universe = Universe.new(width, height);
      const canvas = canvasRef.current;
      if (!canvas) return;
      canvas.width = (CELL_SIZE + 1) * width + 1;
      canvas.height = (CELL_SIZE + 1) * height + 1;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      setUniverse(universe);

      const renderLoop = () => {
        universe.tick(
          birthThreshold,
          survivalThresholdMin,
          survivalThresholdMax
        );
        drawGrid(ctx, width, height);
        drawCells(ctx, universe, width, height, wasm.memory);
        requestAnimationFrame(renderLoop);
      };

      drawGrid(ctx, width, height);
      drawCells(ctx, universe, width, height, wasm.memory);
      requestAnimationFrame(renderLoop);
    };

    runGame();
  }, [birthThreshold, survivalThresholdMin, survivalThresholdMax]);

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

  return (
    <div className="flex flex-col items-center">
      <canvas ref={canvasRef} onClick={handleCanvasClick} />
      <div className="mt-4 space-y-2">
        <div>
          <label htmlFor="birth-threshold">
            Birth Threshold: {birthThreshold}
          </label>
          <input
            id="birth-threshold"
            type="range"
            min="1"
            max="8"
            value={birthThreshold}
            onChange={(e) => setBirthThreshold(parseInt(e.target.value))}
          />
        </div>
        <div>
          <label htmlFor="survival-threshold-min">
            Survival Threshold Min: {survivalThresholdMin}
          </label>
          <input
            id="survival-threshold-min"
            type="range"
            min="1"
            max="8"
            value={survivalThresholdMin}
            onChange={(e) => setSurvivalThresholdMin(parseInt(e.target.value))}
          />
        </div>
        <div>
          <label htmlFor="survival-threshold-max">
            Survival Threshold Max: {survivalThresholdMax}
          </label>
          <input
            id="survival-threshold-max"
            type="range"
            min="1"
            max="8"
            value={survivalThresholdMax}
            onChange={(e) => setSurvivalThresholdMax(parseInt(e.target.value))}
          />
        </div>
      </div>
    </div>
  );
}
