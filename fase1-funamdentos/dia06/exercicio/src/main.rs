use std::io;

// Struct representando uma conta bancária
struct ContaBancaria {
    titular: String,
    numero: u32,
    saldo: f64,
    ativa: bool,
}

impl ContaBancaria {
    // Construtor (função associada)
    fn new(titular: String, numero: u32, saldo_inicial: f64) -> ContaBancaria {
        ContaBancaria {
            titular,
            numero,
            saldo: saldo_inicial,
            ativa: true,
        }
    }
    
    // Método para depositar (&mut self - modifica saldo)
    fn depositar(&mut self, valor: f64) -> Result<(), String> {
        if !self.ativa {
            return Err(String::from("Conta inativa!"));
        }
        
        if valor <= 0.0 {
            return Err(String::from("Valor deve ser positivo!"));
        }
        
        self.saldo += valor;
        Ok(())
    }
    
    // Método para sacar (&mut self - modifica saldo)
    fn sacar(&mut self, valor: f64) -> Result<(), String> {
        if !self.ativa {
            return Err(String::from("Conta inativa!"));
        }
        
        if valor <= 0.0 {
            return Err(String::from("Valor deve ser positivo!"));
        }
        
        if valor > self.saldo {
            return Err(String::from("Saldo insuficiente!"));
        }
        
        self.saldo -= valor;
        Ok(())
    }
    
    // Método para transferir (&mut self e &mut outra)
    fn transferir(&mut self, destino: &mut ContaBancaria, valor: f64) -> Result<(), String> {
        // Tenta sacar da conta origem
        self.sacar(valor)?;
        
        // Se sacar deu certo, deposita na destino
        match destino.depositar(valor) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Se falhar, devolve o valor para conta origem
                self.saldo += valor;
                Err(e)
            }
        }
    }
    
    // Método para exibir dados (&self - só leitura)
    fn exibir_dados(&self) {
        println!("\n========== DADOS DA CONTA ==========");
        println!("Titular: {}", self.titular);
        println!("Número: {}", self.numero);
        println!("Saldo: R$ {:.2}", self.saldo);
        println!("Status: {}", if self.ativa { "Ativa" } else { "Inativa" });
        println!("====================================\n");
    }
    
    // Método para obter saldo (&self - só leitura)
    fn obter_saldo(&self) -> f64 {
        self.saldo
    }
    
    // Método para desativar conta (&mut self)
    fn desativar(&mut self) -> Result<(), String> {
        if self.saldo > 0.0 {
            return Err(String::from("Não pode desativar conta com saldo positivo!"));
        }
        
        self.ativa = false;
        Ok(())
    }
    
    // Método para verificar se está ativa (&self)
    fn esta_ativa(&self) -> bool {
        self.ativa
    }
}

fn main() {
    println!("=== SISTEMA DE CONTAS BANCÁRIAS ===\n");
    
    // Criar contas usando construtor
    let mut conta_bianeck = ContaBancaria::new(
        String::from("Bianeck"),
        1001,
        1000.0
    );
    
    let mut conta_clara = ContaBancaria::new(
        String::from("Clara"),
        1002,
        500.0
    );
    
    let mut conta_lunna = ContaBancaria::new(
        String::from("Lunna"),
        1003,
        300.0
    );
    
    // Exibir dados iniciais
    println!("--- SITUAÇÃO INICIAL ---");
    conta_bianeck.exibir_dados();
    conta_clara.exibir_dados();
    conta_lunna.exibir_dados();
    
    // Operações de depósito
    println!("--- OPERAÇÃO: DEPÓSITO ---");
    match conta_bianeck.depositar(500.0) {
        Ok(_) => println!("✓ Depósito de R$ 500.00 realizado com sucesso!"),
        Err(e) => println!("✗ Erro: {}", e),
    }
    println!("Novo saldo Bianeck: R$ {:.2}", conta_bianeck.obter_saldo());
    
    // Operações de saque
    println!("\n--- OPERAÇÃO: SAQUE ---");
    match conta_clara.sacar(200.0) {
        Ok(_) => println!("✓ Saque de R$ 200.00 realizado com sucesso!"),
        Err(e) => println!("✗ Erro: {}", e),
    }
    println!("Novo saldo Clara: R$ {:.2}", conta_clara.obter_saldo());
    
    // Tentativa de saque com saldo insuficiente
    println!("\n--- TENTATIVA: SAQUE INVÁLIDO ---");
    match conta_lunna.sacar(500.0) {
        Ok(_) => println!("✓ Saque realizado"),
        Err(e) => println!("✗ Erro esperado: {}", e),
    }
    
    // Transferência
    println!("\n--- OPERAÇÃO: TRANSFERÊNCIA ---");
    println!("Transferindo R$ 300.00 de Bianeck para Clara...");
    match conta_bianeck.transferir(&mut conta_clara, 300.0) {
        Ok(_) => {
            println!("✓ Transferência realizada com sucesso!");
            println!("  Saldo Bianeck: R$ {:.2}", conta_bianeck.obter_saldo());
            println!("  Saldo Clara: R$ {:.2}", conta_clara.obter_saldo());
        },
        Err(e) => println!("✗ Erro: {}", e),
    }
    
    // Múltiplas operações
    println!("\n--- MÚLTIPLAS OPERAÇÕES ---");
    let _ = conta_bianeck.depositar(200.0);
    let _ = conta_clara.sacar(100.0);
    let _ = conta_lunna.depositar(150.0);
    
    // Exibir situação final
    println!("\n--- SITUAÇÃO FINAL ---");
    conta_bianeck.exibir_dados();
    conta_clara.exibir_dados();
    conta_lunna.exibir_dados();
    
    // Teste de desativação
    println!("--- TESTE: DESATIVAR CONTA ---");
    
    // Zerar saldo da conta Lunna
    let _ = conta_lunna.sacar(conta_lunna.obter_saldo());
    
    match conta_lunna.desativar() {
        Ok(_) => println!("✓ Conta Lunna desativada com sucesso!"),
        Err(e) => println!("✗ Erro: {}", e),
    }
    
    // Tentar operar em conta inativa
    println!("\n--- TENTATIVA: OPERAR CONTA INATIVA ---");
    match conta_lunna.depositar(100.0) {
        Ok(_) => println!("✓ Depósito realizado"),
        Err(e) => println!("✗ Erro esperado: {}", e),
    }
}