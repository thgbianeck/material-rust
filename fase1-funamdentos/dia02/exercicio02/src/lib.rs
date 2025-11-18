use std::fmt;

/// Enum para representar escalas de temperatura
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EscalaTemperatura {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Implementa Display para exibir nomes bonitos
impl fmt::Display for EscalaTemperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EscalaTemperatura::Celsius => write!(f, "Celsius (°C)"),
            EscalaTemperatura::Fahrenheit => write!(f, "Fahrenheit (°F)"),
            EscalaTemperatura::Kelvin => write!(f, "Kelvin (K)"),
        }
    }
}

/// Struct para representar uma temperatura
#[derive(Debug, Clone, Copy)]
pub struct Temperatura {
    pub valor: f64,
    pub escala: EscalaTemperatura,
}

impl Temperatura {
    /// Cria nova temperatura
    pub fn new(valor: f64, escala: EscalaTemperatura) -> Self {
        Temperatura { valor, escala }
    }

    /// Converte para Celsius
    pub fn para_celsius(&self) -> f64 {
        match self.escala {
            EscalaTemperatura::Celsius => self.valor,
            EscalaTemperatura::Fahrenheit => (self.valor - 32.0) * 5.0 / 9.0,
            EscalaTemperatura::Kelvin => self.valor - 273.15,
        }
    }

    /// Converte para Fahrenheit
    pub fn para_fahrenheit(&self) -> f64 {
        match self.escala {
            EscalaTemperatura::Celsius => self.valor * 9.0 / 5.0 + 32.0,
            EscalaTemperatura::Fahrenheit => self.valor,
            EscalaTemperatura::Kelvin => (self.valor - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    /// Converte para Kelvin
    pub fn para_kelvin(&self) -> f64 {
        match self.escala {
            EscalaTemperatura::Celsius => self.valor + 273.15,
            EscalaTemperatura::Fahrenheit => (self.valor - 32.0) * 5.0 / 9.0 + 273.15,
            EscalaTemperatura::Kelvin => self.valor,
        }
    }

    /// Converte para outra escala
    pub fn converter_para(&self, escala_destino: EscalaTemperatura) -> Temperatura {
        let novo_valor = match escala_destino {
            EscalaTemperatura::Celsius => self.para_celsius(),
            EscalaTemperatura::Fahrenheit => self.para_fahrenheit(),
            EscalaTemperatura::Kelvin => self.para_kelvin(),
        };

        Temperatura::new(novo_valor, escala_destino)
    }
}

/// Implementa Display para Temperatura
impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let simbolo = match self.escala {
            EscalaTemperatura::Celsius => "°C",
            EscalaTemperatura::Fahrenheit => "°F",
            EscalaTemperatura::Kelvin => "K",
        };
        write!(f, "{:.2}{}", self.valor, simbolo)
    }
}
