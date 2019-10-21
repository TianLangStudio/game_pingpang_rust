//extern crate piston_window;
use piston_window::*;
use game_pingpang::tianlangstudio::*;
fn draw(ctx: Context, gph: &mut impl Graphics) {
            clear([1.0; 4], gph);
            let racket = Racket::new();
            racket.draw(ctx, gph);
}
fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Game Pingpang", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            draw(context, graphics);
        });
    }
}
