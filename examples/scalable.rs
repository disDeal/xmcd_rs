use nannou::prelude::*;
use xmcd_rs;
use xmcd_rs::xas::Xas;

mod predata;
use predata::Data;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    data: predata::Data,
    paths: Vec<(Point2, Srgba<f32>)>,
    bounds: (f32, f32),
}

impl Model {
    fn draw_data(&self, draw: &app::Draw, rect: geom::Rect, mx: f32) {
        let (left, right) = (rect.left(), rect.right());
        let (top, bottom) = (rect.top(), rect.bottom());
        let size = self.data.buffer.len() - 1;
        let mut x = Vec::with_capacity(size);
        let mut y = Vec::with_capacity(size);

        let range = (self.data.buffer[0].0, self.data.buffer[size].0);
        for i in 0..size {
            x.push(map_range(
                self.data.buffer[i].0,
                range.0,
                range.1,
                left,
                right - 10.,
            ));
            y.push(map_range(
                self.data.buffer[i].1,
                self.bounds.0,
                self.bounds.1,
                0.,
                top,
            ));
        }

        let tris = (0..size - 1)
            .flat_map(|i| {
                let (p1, p2) = (pt2(x[i], y[i]), pt2(x[i + 1], y[i + 1]));
                let zero_line = (pt2(x[i], 0. - 100.), pt2(x[i + 1], 0. - 100.));
                geom::Quad([p1, p2, zero_line.1, zero_line.0]).triangles_iter()
            })
            .map(|tri| {
                tri.map_vertices(|v| {
                    let y_fract = map_range(v.y, 0. - 100., top, 0.0, 1.0);
                    let x_fract = map_range(mx, left, right, 0.0, 1.0);
                    let color = rgba(0.5, 1., y_fract, y_fract.powf(0.1));
                    geom::vertex::Srgba(v, color)
                })
            });
        // println!("{:?}", &tris);
        draw.mesh().tris(tris);
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(1224, 800)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let data = Data::new();
    let bounds = (
        data.buffer
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .1,
        data.buffer
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .1,
    );
    // println!("{:?}", &bounds);
    let bounds = (1., 2.);

    Model {
        _window,
        paths: Vec::new(),
        data,
        bounds,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse;
    let win = (app.window_rect().left(), app.window_rect().right());
    let t = map_range(mouse.x, win.0, win.1, 0., 1.);

    model.data.lerp(t);
}

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

    model.draw_data(&draw, win, app.mouse.x);

    draw.to_frame(app, &frame).unwrap();
}
