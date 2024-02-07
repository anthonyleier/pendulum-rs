use speedy2d::{
    color::Color,
    window::{WindowHandler, WindowHelper},
    Graphics2D, Window,
};
use vetor::Vetor;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();
    let my_window = MyWindowHandler {
        pendulo1: Pendulo::new(400.0, 0.0, 200.0, Color::from_int_rgb(25, 125, 224)),
        pendulo2: Pendulo::new(400.0, 0.0, 400.0, Color::from_int_rgb(6, 33, 59)),
    };
    window.run_loop(my_window);
}

struct MyWindowHandler {
    pendulo1: Pendulo,
    pendulo2: Pendulo,
}
impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let cor = Color::from_int_rgb(193, 209, 224);
        graphics.clear_screen(cor);

        self.pendulo1.update();
        self.pendulo1.draw(graphics);

        self.pendulo2.update();
        self.pendulo2.draw(graphics);

        helper.request_redraw();
    }
}

struct Pendulo {
    origem: Vetor,
    posicao: Vetor,
    cor: Color,

    angulo: f32,
    velocidade_angular: f32,
    aceleracao_angular: f32,

    raio: f32,
    massa: f32,
    gravidade: f32,
}

impl Pendulo {
    fn new(x: f32, y: f32, raio: f32, cor: Color) -> Pendulo {
        Pendulo {
            origem: Vetor::new(x, y),
            posicao: Vetor::new(0.0, 0.0),
            angulo: 1.0,
            velocidade_angular: 0.0,
            aceleracao_angular: 0.0,
            raio: raio,
            massa: 1.0,
            gravidade: 1.5,
            cor: cor,
        }
    }
    fn update(&mut self) {
        // Calcular a aceleração angular com a equação do pêndulo
        self.aceleracao_angular =
            -1.0 * (1.0 / self.massa) * self.gravidade * self.angulo.sin() / self.raio;

        // A velocidade angular é a velocidade angular mais a aceleração angular
        self.velocidade_angular += self.aceleracao_angular;

        // O angulo é o angulo mais a velocidade angular
        self.angulo += self.velocidade_angular;

        // A posição são as coordenadas polares convertidas em coordenadas cartesianas
        self.posicao
            .set(self.raio * self.angulo.sin(), self.raio * self.angulo.cos());

        // A posição final da bola na tela é a origem do pêndulo mais o vetor de posição
        self.posicao.add(&self.origem);
    }
    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origem.x, self.origem.y),
            (self.posicao.x, self.posicao.y),
            3.0,
            self.cor,
        );
        graphics.draw_circle((self.posicao.x, self.posicao.y), 30.0, self.cor);
    }
}

mod vetor {
    pub struct Vetor {
        pub x: f32,
        pub y: f32,
    }
    impl Vetor {
        pub fn new(x: f32, y: f32) -> Vetor {
            Vetor { x, y }
        }

        pub fn add(&mut self, outro: &Vetor) -> &Vetor {
            self.x += outro.x;
            self.y += outro.y;
            return self;
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vetor {
            self.x = x;
            self.y = y;
            return self;
        }
    }
}
