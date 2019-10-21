pub mod tianlangstudio {
   use piston_window::*;
   pub struct Racket {
      x: f64,
      y: f64,
      width: f64,
      height: f64
   }

   impl Racket {
     pub fn new() -> Self{
        Self {
           x: 270.0,
           y: 460.0,
           width: 100.0,
           height: 20.0
        }
     }
     pub fn draw(&self, ctx: Context, gph: &mut impl Graphics) {
        rectangle([0.0, 1.0, 0.0, 1.0], // green
                      [self.x, self.y, self.width, self.height],
                      ctx.transform,                                                                
                      gph);

     }
   }

}
