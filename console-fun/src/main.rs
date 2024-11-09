use std::{io::{self}, time::{Duration, Instant}, u16};
use ratatui::{
    buffer::{Buffer, Cell}, crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Alignment,
    Constraint, Layout, Margin, Rect}, prelude::Style, style::{Color, Stylize},
    symbols::{border::EMPTY, Marker}, widgets::{ block::{Position, Title}, canvas::{Canvas, Circle,
    Context, Line, Painter, Rectangle}, Block, BorderType, Borders, Padding, Paragraph, Wrap
    }, DefaultTerminal, Frame
};

struct Point {
    x: f64,
    y: f64,
}

struct SandGrain {
    position: Point,
}

impl SandGrain {
    fn set_pos(&mut self, _x: f64, _y: f64) {
        self.position.x = _x;
        self.position.y = _y;
    }
}

struct SandSim {
    sand_vec: Vec<SandGrain>,
    next_step: bool,
    generate: bool,
}

impl SandSim {
    pub fn new() -> Self {
        let vec: Vec<SandGrain> = Vec::new();

        // DO WE REALLY NEED TO USE THE PARAMETER NAMES???
        // for i in 1..10 {
        //     vec.push(SandGrain{position: Point{x:36.0, y:25.0-2.0*f64::from(i)}});
        // }

        Self {sand_vec: vec, next_step: false, generate: false}
    }

    fn reset(&mut self) {
        self.sand_vec.clear();
    }

    fn step(&mut self, frame: &mut Frame) {
        // use context to get specific grid point to check in buffer 
        // if it is already filled or not
        
        let buffer = frame.buffer_mut();

        let gen_points = vec![(33.0,8.0), (36.0,8.0), (39.0,8.0), (42.0,8.0), (45.0,8.0)];

        if self.generate {
            for gp in gen_points.iter() {
                if buffer[(gp.0 as u16, gp.1 as u16)] == Cell::EMPTY {
                    self.sand_vec.push(SandGrain{position: Point{x:gp.0, y:gp.1}});
                }
            }
        }

        let mut test_point;

        for s in self.sand_vec.iter_mut() {
            for i in 0..3 {
                test_point = match i {
                    0 => (s.position.x,     s.position.y+1.0),
                    1 => (s.position.x-1.0, s.position.y+1.0),
                    2 => (s.position.x+1.0, s.position.y+1.0),
                    _ => (-1.0,-1.0),
                };

                if buffer[(test_point.0 as u16, test_point.1 as u16)] == Cell::EMPTY {
                    buffer[(test_point.0 as u16, test_point.1 as u16)] = buffer[(s.position.x as u16, s.position.y as u16)].clone();
                    buffer[(s.position.x as u16, s.position.y as u16)] = Cell::EMPTY;

                    s.set_pos(test_point.0, test_point.1);
                    break;
                }
            }
        }
        self.next_step = false;
    }

    fn draw_particles(&self, frame: &mut Frame) {
        let buffer = frame.buffer_mut();
        for s in self.sand_vec.iter() {
            buffer[(s.position.x as u16, s.position.y as u16)].set_symbol("•").set_fg(Color::Yellow);
        }
    }
}

struct App {
    tick_rate_value: u64, 
    sim: SandSim,
}

impl App {
    pub fn new(tick_rate: u64) -> Self {
        Self {tick_rate_value: tick_rate, sim: SandSim::new()}
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(self.tick_rate_value);
        let mut last_tick = Instant::now();

        loop {
            // really calling mmember functions just like this???
            terminal.draw(|frame| Self::on_draw(self, frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('g') => self.sim.generate = !self.sim.generate,
                            KeyCode::Char('r') => self.sim.reset(),
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.sim.next_step = true;
                last_tick = Instant::now();
            }
        }
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Ratatui test fun")
            .dark_gray()
            .alignment(Alignment::Center),
            area,
        );
    }

    fn draw_canvas(&self, frame: &mut Frame, area: Rect) {
        let bottom:f64 = 5.0;
        let top:f64 = 55.0;

        let left:f64 = 10.0;
        let right:f64 = 30.0;

        let width = right-left;
        let height = top-bottom;

        let side_length:f64 = (height)/4.0;

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Title::from("Main Window").position(Position::Top).alignment(Alignment::Left))
            .title(Title::from("Press q to exit").position(Position::Bottom).alignment(Alignment::Right))
            .title_style(Style::new().gray().bold());

        let canvas = Canvas::default()
            .marker(Marker::Block)
            .block(block)
            .x_bounds([0.0,40.0])
            .y_bounds([0.0,60.0])
            .paint(|context| {
                context.draw(&Rectangle {
                    x: left,
                    y: bottom,
                    width: width,
                    height: height,
                    color: Color::DarkGray,
                });

                for i in 0..2 {
                    context.draw(&Line {
                        x1: left,
                        y1: match i { 0 => bottom, _ => top},
                        x2: right,
                        y2: match i { 0 => bottom, _ => top},
                        color: Color::White
                    });
                }

                for i in 0..5 {
                    context.draw(&Line {
                        x1: match i { 0 | 1 => left, _ => right },
                        y1: match i { 0 | 2 => bottom, _ => top},
                        x2: match i { 0 | 1 => left, _ => right },
                        y2: match i { 0 | 2 => bottom + side_length, _ => top - side_length},
                        color: Color::Yellow
                    });
                }

                for i in 0..5 {
                    context.draw(&Line {
                        x1: match i { 0 | 1 => left, _ => right },
                        y1: match i { 0 | 2 => bottom + width*0.48 , _ => top - width*0.48},
                        x2: match i { 0 | 1 => left + width*0.48, _ => right-width*0.48},
                        y2: match i { 0 | 2 => bottom + height/2.0, _ => top - height/2.0},
                        color: Color::Red});

                    context.draw(&Line {
                        x1: match i { 0 | 1 => left, _ => right },
                        y1: match i { 0 | 2 => bottom -1.0 + width*0.48, _ => top + 1.0 - width*0.48},
                        x2: match i { 0 | 1 => left + width*0.48, _ => right-width*0.48},
                        y2: match i { 0 | 2 => bottom -1.0 + height/2.0, _ => top + 1.0 - height/2.0},
                        color: Color::Red
                    });
                }
            });

        frame.render_widget(canvas, area);
    }


    fn on_draw(&mut self, frame: &mut Frame) {
        let new_area = frame.area().inner(Margin{vertical:2,horizontal:8});
        let (title_area, main_area) = get_layout(new_area);

        Self::render_title(self, frame, title_area);

        Self::draw_canvas(self, frame, main_area);

        self.sim.draw_particles(frame);

        if self.sim.next_step {
            self.sim.step(frame);
        }
    }
} 

fn get_layout(area: Rect) -> (Rect, Rect) {
    let main_layout = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]);
    let [title_area, main_area] = main_layout.areas(area);

    (title_area, main_area)
}



fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::new(20);
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
