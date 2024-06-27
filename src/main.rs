mod obj;
mod screen;

fn main() {
    let mut screen = screen::Screen::new((70, 35), 90.0, "src/ex/cube.obj");
    screen.draw_shape('0');
    screen.print_frame();
}
