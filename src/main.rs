//extern crate piston_window;
use piston_window::*;
use game_pingpang::tianlangstudio::*;
fn draw(ctx: Context, gph: &mut impl Graphics, racket: &Racket) {
            clear([1.0; 4], gph);
            racket.draw(ctx, gph);
}
fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Game Pingpang", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut racket = Racket::new();
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Key::Left = key {
               racket.mv_left();
            }
            if let Key::Right = key {
               racket.mv_right(640.0);
            }
        }
        window.draw_2d(&event, |context, graphics, _device| {
            draw(context, graphics, &racket);
        });
    }
}
