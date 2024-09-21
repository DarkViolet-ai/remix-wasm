import { useEffect, useRef, useState, useCallback } from "react";
import initWasm, { Breakout } from "../../rust-wasm-lib/pkg/rust_wasm_lib";

export default function BreakoutGame() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gameRef = useRef<Breakout | null>(null);
  const [isRunning, setIsRunning] = useState(false);

  const toggleGame = useCallback(() => {
    if (gameRef.current) {
      gameRef.current.toggle_game();
      setIsRunning(gameRef.current.isRunning());
    }
  }, []);

  const handleRestart = useCallback(() => {
    if (gameRef.current) {
      gameRef.current.restart();
      setIsRunning(false);
    }
  }, []);

  useEffect(() => {
    let animationId: number | null = null;

    const runGame = async () => {
      console.log("Initializing WASM module...");
      await initWasm();
      console.log("WASM module initialized");

      const canvas = canvasRef.current;
      if (!canvas) {
        console.error("Canvas not found");
        return;
      }

      canvas.width = 1200;
      canvas.height = 800;

      const newGame = new Breakout("breakout-canvas", 5, 8);
      gameRef.current = newGame;

      const gameLoop = () => {
        if (newGame.isRunning()) {
          newGame.update();
        }
        newGame.draw();
        animationId = requestAnimationFrame(gameLoop);
      };

      gameLoop();
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (gameRef.current) {
        switch (event.key) {
          case "a":
            gameRef.current.start_move("left");
            break;
          case "d":
            gameRef.current.start_move("right");
            break;
          case " ":
            toggleGame();
            break;
        }
      }
    };

    const handleKeyUp = (event: KeyboardEvent) => {
      if (gameRef.current) {
        switch (event.key) {
          case "a":
            gameRef.current.stop_move("left");
            break;
          case "d":
            gameRef.current.stop_move("right");
            break;
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
    runGame();

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
      if (animationId !== null) {
        cancelAnimationFrame(animationId);
      }
    };
  }, [toggleGame]);

  return (
    <div className="flex flex-col items-center">
      <h1 className="text-2xl font-bold mb-4">Breakout Game</h1>
      <canvas
        id="breakout-canvas"
        ref={canvasRef}
        className="border border-gray-300"
      />
      <p className="mt-4">
        Use &apos;A&apos; and &apos;D&apos; keys to move the paddle
      </p>
      <p>Press Space to {isRunning ? "pause" : "start"} the game</p>
      <button
        onClick={toggleGame}
        className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
      >
        {isRunning ? "Pause" : "Start"} Game
      </button>
      <button
        onClick={handleRestart}
        className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
      >
        Restart Game
      </button>
    </div>
  );
}
