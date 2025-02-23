//! This is mainly an example to test linking solely against the SFML system module

use {
    sfml::{
        SfError,
        graphics::*,
        window::*,
    },
};

fn main() -> Result<(), SfError> {
    let mut window = RenderWindow::new( (800, 600),
        "Mouse events",
        Style::CLOSE,
        &Default::default(),
    )?;

    window.set_mouse_cursor_visible(false);
    window.set_vertical_sync_enabled(true);

    let mut circle = CircleShape::new(4., 30);
    let mut texts: Vec<Text> = Vec::new();
    let mut cursor_visible = false;

    'mainloop: loop {
        // window.clear(Color::BLACK);
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => break 'mainloop,
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => break 'mainloop,
                        _ => (),
                    }
                },
                _ => ()
            }
        }

        let mp = window.mouse_position();
        circle.set_position((mp.x as f32, mp.y as f32));
        window.draw(&circle);
        window.display();
    }
    window.close();
    Ok(())
}
