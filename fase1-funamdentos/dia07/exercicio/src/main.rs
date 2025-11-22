// Enum que representa diferentes tipos de transações
enum Transacao {
    // TODO: Implemente as 4 variantes
    // Deposito -> valor: f64
    // Saque -> valor: f64
    // Transferencia -> valor: f64, conta_destino: String
    // Pagamento -> descricao: String, valor: f64
}

// Struct que representa uma conta bancária
struct ContaBancaria {
    titular: String,
    saldo: f64,
}

impl ContaBancaria {
    // Cria nova conta
    fn nova(titular: String, saldo_inicial: f64) -> Self {
        ContaBancaria {
            titular,
            saldo: saldo_inicial,
        }
    }
    
    // Processa uma transação
    // Retorna Ok com mensagem de sucesso ou Err com mensagem de erro
    fn processar_transacao(&mut self, transacao: Transacao) -> Result<String, String> {
        // TODO: Implemente usando match
        // - Validar valores (devem ser > 0)
        // - Deposito: adicionar ao saldo
        // - Saque: verificar saldo suficiente antes de subtrair
        // - Transferencia: verificar saldo suficiente antes de subtrair
        // - Pagamento: verificar saldo suficiente antes de subtrair
        // Retornar mensagens descritivas
        
        todo!() // Remova esta linha e implemente
    }
    
    // Consulta saldo
    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }
}

// Função que processa múltiplas transações e exibe extrato
fn exibir_extrato(conta: &mut ContaBancaria, transacoes: Vec<Transacao>) {
    // TODO: Implemente
    // - Iterar sobre transacoes
    // - Processar cada uma com processar_transacao
    // - Usar match para Ok/Err
    // - Exibir resultado de cada operação
    // - Ao final, mostrar saldo
}

fn main() {
    println!("=== SISTEMA BANCÁRIO ===\n");
    
    let mut conta = ContaBancaria::nova(
        String::from("Bianeck"),
        1000.0
    );
    
    println!("Titular: {}", conta.titular);
    println!("Saldo inicial: R$ {:.2}\n", conta.consultar_saldo());
    
    // Lista de transações para processar
    let transacoes = vec![
        Transacao::Deposito(500.0),
        Transacao::Saque(200.0),
        Transacao::Transferencia {
            valor: 300.0,
            conta_destino: String::from("12345-6"),
        },
        Transacao::Pagamento {
            descricao: String::from("Conta de Luz"),
            valor: 150.0,
        },
        Transacao::Saque(2000.0), // Vai falhar - saldo insuficiente
    ];
    
    exibir_extrato(&mut conta, transacoes);
}