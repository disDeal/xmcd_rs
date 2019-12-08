use nannou::prelude::*;
use xmcd_rs;
use xmcd_rs::xas::Xas;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    xas: Xas,
    paths: Vec<(Point2, Srgba<f32>)>,
    bounds: (f64, f64),
}

impl Model {
    fn draw_xas(&self, draw: &app::Draw, rect: geom::Rect) {
        let (left, right) = (rect.left(), rect.right());
        let (top, _bottom) = (rect.top(), rect.bottom());

        let size = self.xas.energy.len() - 1;
        let range = (self.xas.energy[0], self.xas.energy[size]);
        let x = self
            .xas
            .energy
            .iter()
            .map(|&x| map_range(x, range.0, range.1, left, right) as f32)
            .collect::<Vec<f32>>();

        let y = self
            .xas
            .mui
            .iter()
            .map(|&x| map_range(x, self.bounds.0, self.bounds.1, 0., top) as f32)
            .collect::<Vec<f32>>();

        // let points = x.zip(y).map(|p| (pt2(p.0, p.1), BLUE));
        // draw.polyline().weight(4.).colored_points(points);

        let tris = (0..size)
            // .step_by(2)
            .flat_map(|i| {
                let (p1, p2) = (pt2(x[i], y[i]), pt2(x[i + 1], y[i + 1]));
                let zero_line = (pt2(x[i], 0f32), pt2(x[i + 1], 0f32));
                geom::Quad([p1, p2, zero_line.1, zero_line.0]).triangles_iter()
            })
            .map(|tri| {
                tri.map_vertices(|v| {
                    let y_fract = map_range(v.y.abs(), 0.0, top, 0.0, 1.0);
                    let color = rgba(1., 0.5, y_fract, y_fract.sqrt());
                    geom::vertex::Srgba(v, color)
                })
            });
        draw.mesh().tris(tris);
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let file = std::fs::File::open("data/Fe-1_20161207215237.txt").unwrap();
    let buffer = std::io::BufReader::new(file);
    let xas = Xas::new(buffer).unwrap();
    let bounds = (
        *xas.mui
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        *xas.mui
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );

    Model {
        _window,
        paths: Vec::new(),
        xas,
        bounds,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(SLATEGRAY);
    let weight = 4.0;

    let win = app.window_rect();

    model.draw_xas(&draw, win);

    draw.polyline()
        .weight(weight)
        .colored_points(model.paths.clone());
    draw.ellipse();

    draw.to_frame(app, &frame).unwrap();
}
