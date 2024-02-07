use vetor::Vetor;

fn main() {
    println!("Hello, world!");
}

struct Pendulo {
    origem: Vetor,
    posicao: Vetor,

    angulo: f32,
    velocidade_angular: f32,
    aceleracao_angular: f32,

    raio: f32,
    massa: f32,
    gravidade: f32,
}

impl Pendulo {
    fn new(x: f32, y: f32, raio: f32) -> Pendulo {
        Pendulo {
            origem: Vetor::new(x, y),
            posicao: Vetor::new(0.0, 0.0),
            angulo: 1.0,
            velocidade_angular: 0.0,
            aceleracao_angular: 0.0,
            raio: raio,
            massa: 1.0,
            gravidade: 1.5,
        }
    }
    fn update(&mut self) {
        // Calcular a aceleração angular com a equação do pêndulo
        self.aceleracao_angular = -1.0 * self.gravidade * self.angulo.sin() / self.raio;

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
    fn draw() {}
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

        pub fn add(&mut self, outro: Vetor) -> &Vetor {
            self.x += outro.x;
            self.y += outro.y;
            return self;
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vetor {
            self.x = x;
            self.x = y;
            return self;
        }
    }
}
