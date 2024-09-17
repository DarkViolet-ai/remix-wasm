use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use std::cell::RefCell;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use rand::Rng;

#[wasm_bindgen]
#[derive(Clone)] // Add this line to derive Clone
pub struct ColorScheme {
    background: RGBColor,
    plot_background: RGBColor,
    axis: RGBColor,
    text: RGBColor,
    line: RGBColor,
    mesh: RGBColor,
}

#[wasm_bindgen]
impl ColorScheme {
    pub fn light() -> Self {
        ColorScheme {
            background: RGBColor(255, 255, 255),
            plot_background: RGBColor(240, 240, 240),
            axis: RGBColor(0, 0, 0),
            text: RGBColor(0, 0, 0),
            line: RGBColor(0, 0, 255),
            mesh: RGBColor(200, 200, 200),
        }
    }

    pub fn dark() -> Self {
        ColorScheme {
            background: RGBColor(30, 30, 30),
            plot_background: RGBColor(50, 50, 50),
            axis: RGBColor(200, 200, 200),
            text: RGBColor(200, 200, 200),
            line: RGBColor(0, 255, 0),
            mesh: RGBColor(100, 100, 100),
        }
    }
}

#[wasm_bindgen]
pub struct AudioAnalyzer {
    context: web_sys::AudioContext,
    analyser: web_sys::AnalyserNode,
    canvas: HtmlCanvasElement,
    canvas_ctx: CanvasRenderingContext2d,
    color_scheme: ColorScheme,
    sample_rate: f32,
}

#[wasm_bindgen]
impl AudioAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new(color_scheme: Option<ColorScheme>) -> Result<AudioAnalyzer, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("analyzer-canvas").unwrap().dyn_into::<HtmlCanvasElement>()?;
        let canvas_ctx = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;

        let context = web_sys::AudioContext::new()?;
        let sample_rate = context.sample_rate();
        let analyser = context.create_analyser()?;
        analyser.set_fft_size(2048);

        Ok(AudioAnalyzer {
            context,
            analyser,
            canvas,
            canvas_ctx,
            color_scheme: color_scheme.unwrap_or_else(ColorScheme::light),
            sample_rate,
        })
    }

    pub fn start(&self) -> Result<(), JsValue> {
        let navigator = web_sys::window().unwrap().navigator();
        let media_devices = navigator.media_devices()?;
        let constraints = web_sys::MediaStreamConstraints::new();
        constraints.set_audio(&JsValue::TRUE);

        let promise = media_devices.get_user_media_with_constraints(&constraints)?;
        let future = wasm_bindgen_futures::JsFuture::from(promise);

        let analyser = self.analyser.clone();
        let context = self.context.clone();
        let canvas = self.canvas.clone();
        let color_scheme = self.color_scheme.clone();
        let sample_rate = self.sample_rate;

        spawn_local(async move {
            let stream = future.await.unwrap();
            let media_stream = stream.dyn_into::<web_sys::MediaStream>().unwrap();
            let source = context.create_media_stream_source(&media_stream).unwrap();
            source.connect_with_audio_node(&analyser).unwrap();
            
            start_drawing(analyser, canvas, color_scheme, sample_rate);
        });

        Ok(())
    }
}

fn start_drawing(analyser: web_sys::AnalyserNode, canvas: HtmlCanvasElement, color_scheme: ColorScheme, sample_rate: f32) {
    fn draw(analyser: &web_sys::AnalyserNode, canvas: &HtmlCanvasElement, color_scheme: &ColorScheme, sample_rate: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = vec![0u8; analyser.frequency_bin_count() as usize];
        analyser.get_byte_frequency_data(&mut data);

        let backend = CanvasBackend::with_canvas_object(canvas.clone()).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&color_scheme.background)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Audio Spectrum", ("sans-serif", 30).into_font().color(&color_scheme.text))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..data.len(), 0..255)?;

        chart
            .configure_mesh()
            .x_desc("Frequency (Hz)")
            .y_desc("Magnitude")
            .axis_desc_style(("sans-serif", 15).into_font().color(&color_scheme.text))
            .x_labels(5)
            .y_labels(5)
            .x_label_formatter(&|x| format!("{:.0}", *x as f32 * sample_rate / (2.0 * data.len() as f32)))
            .y_label_formatter(&|y| format!("{}", y))
            .axis_style(&color_scheme.axis)
            .label_style(("sans-serif", 12).into_font().color(&color_scheme.text))
            .light_line_style(&color_scheme.mesh)
            .draw()?;

        chart.draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, &v)| (i, v as i32)),
            &color_scheme.line,
        ))?;

        root.present()?;
        Ok(())
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw(&analyser, &canvas, &color_scheme, sample_rate).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let mut rng = rand::thread_rng();
        let cells = (0..width * height)
            .map(|_| rng.gen_bool(0.5))
            .collect();
        Universe { width, height, cells }
    }

    pub fn tick(&mut self, birth_threshold: u8, survival_threshold_min: u8, survival_threshold_max: u8) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next[idx] = match (cell, live_neighbors) {
                    (true, x) if x < survival_threshold_min => false,
                    (true, x) if x > survival_threshold_max => false,
                    (false, x) if x == birth_threshold => true,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const bool {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx] = !self.cells[idx];
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}