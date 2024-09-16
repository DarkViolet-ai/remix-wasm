import { useEffect, useRef } from "react";
import {
  initAudioAnalyzer,
  AudioAnalyzer,
  ColorScheme,
} from "~/utils/loadWasm";

export default function AudioAnalyzerRoute() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    async function setupAnalyzer() {
      await initAudioAnalyzer();
      const canvas = canvasRef.current;
      if (canvas) {
        const colorScheme = ColorScheme.dark();
        const analyzer = new AudioAnalyzer(colorScheme);
        analyzer.start();
      }
    }

    setupAnalyzer();
  }, []);

  return (
    <div>
      <h1>Audio Analyzer</h1>
      <canvas
        id="analyzer-canvas"
        ref={canvasRef}
        width="800"
        height="400"
      ></canvas>
    </div>
  );
}
