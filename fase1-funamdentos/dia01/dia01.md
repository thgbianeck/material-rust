# 🦀 Rust - Dia 1: Setup Completo e Hello World

**Bem-vindo à sua jornada em Rust, Bianeck!** Hoje você vai configurar todo o ambiente e criar seu primeiro projeto. Vamos comparar bastante com Java para facilitar sua transição.

---

## 📑 Índice

**[Parte 1: Instalação e Configuração](#parte-1-instalação-e-configuração)**
- [1.1 Instalando o Rust](#11-instalando-o-rust)
- [1.2 Configurando o VSCode](#12-configurando-o-vscode)

**[Parte 2: Primeiro Projeto](#parte-2-primeiro-projeto)**
- [2.1 Criando com Cargo](#21-criando-com-cargo)
- [2.2 Anatomia do Projeto](#22-anatomia-do-projeto)
- [2.3 Comandos Essenciais](#23-comandos-essenciais)

**[Parte 3: Hello World Explicado](#parte-3-hello-world-explicado)**
- [3.1 Anatomia do Código](#31-anatomia-do-código)
- [3.2 Comparações com Java](#32-comparações-com-java)

**[Parte 4: Exercícios Práticos](#parte-4-exercícios-práticos)**
- [Exercício 1: Hello Personalizado](#exercício-1-hello-personalizado-fácil)
- [Exercício 2: Calculadora Básica](#exercício-2-calculadora-básica-médio)
- [Exercício 3: Sistema de Boas-Vindas](#exercício-3-sistema-de-boas-vindas-desafiador)

**[Parte 5: Troubleshooting](#parte-5-troubleshooting)**

**[Checkpoint Final](#checkpoint-final)**

---

## Parte 1: Instalação e Configuração

### 1.1 Instalando o Rust

Rust é distribuído através do **rustup**, que gerencia versões do compilador, ferramentas e toolchains. Pense no rustup como o SDKMAN para Java, mas oficial e integrado.

#### 🐧 **Linux (Ubuntu/Debian/Fedora)**

**Passo 1:** Abra o terminal e execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Passo 2:** Durante a instalação, você verá opções:

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

**Recomendação:** Pressione `1` (Enter) para instalação padrão.

**Passo 3:** Adicione o Rust ao PATH (o instalador mostra isso):

```bash
source "$HOME/.cargo/env"
```

**Passo 4:** Verifique a instalação:

```bash
rustc --version  # Mostra a versão do compilador
cargo --version  # Mostra a versão do gerenciador de projetos
rustfmt --version  # Formatador de código
```

**Saída esperada:**
```
rustc 1.75.0 (ou superior)
cargo 1.75.0
rustfmt 1.7.0-stable
```

#### 🪟 **Windows**

**Passo 1:** Baixe o instalador:
- Acesse: https://rustup.rs/
- Baixe `rustup-init.exe`

**Passo 2:** Execute o instalador
- Ele detectará se você precisa do Visual Studio Build Tools
- **IMPORTANTE:** Você precisa do C++ Build Tools instalado

**Se não tiver o Build Tools:**
1. O instalador pedirá para baixar
2. Ou acesse: https://visualstudio.microsoft.com/visual-cpp-build-tools/
3. Instale "Desktop development with C++"

**Passo 3:** Após instalar o Build Tools, reabra o `rustup-init.exe`
- Escolha opção `1` (instalação padrão)

**Passo 4:** Verifique no PowerShell ou CMD:

```powershell
rustc --version
cargo --version
```

> **💡 Analogia:** Rustup é como ter um gerenciador de JDKs, Maven/Gradle, e formatter (como o Google Java Format) tudo em uma ferramenta oficial.

---

### 1.2 Configurando o VSCode

#### **Passo 1: Instalar Extensões Essenciais**

Abra o VSCode e instale estas extensões (Ctrl+Shift+X):

**1. rust-analyzer** (obrigatória)
- ID: `rust-lang.rust-analyzer`
- Autocomplete inteligente, análise de erros em tempo real
- **Analogia:** É como o IntelliSense do Java, mas turbinado

**2. CodeLLDB** (para debugging)
- ID: `vadimcn.vscode-lldb`
- Permite debug com breakpoints

**3. Even Better TOML** (opcional, mas útil)
- ID: `tamasfe.even-better-toml`
- Syntax highlighting para Cargo.toml

**4. Error Lens** (opcional)
- ID: `usernamehw.errorlens`
- Mostra erros inline no código

#### **Passo 2: Configurar settings.json**

Pressione `Ctrl+Shift+P` → Digite "Preferences: Open Settings (JSON)"

Adicione estas configurações:

```json
{
    // Formata automaticamente ao salvar
    "editor.formatOnSave": true,
    
    // Rust-analyzer configurações
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    },
    
    // Habilita hints inline (tipo inferência)
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    
    // Auto-import de módulos
    "rust-analyzer.completion.autoimport.enable": true,
    
    // Mostra documentação ao hover
    "rust-analyzer.hover.documentation.enable": true,
    
    // Check on save (compila ao salvar)
    "rust-analyzer.checkOnSave.enable": true,
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

> **💡 Dica:** `clippy` é o linter do Rust (como o Checkstyle/PMD para Java). Ele dá sugestões idiomáticas.

#### **Passo 3: Verificar funcionamento**

Crie uma pasta teste:

```bash
mkdir ~/rust-test
cd ~/rust-test
code .
```

O rust-analyzer deve iniciar automaticamente quando você abrir um projeto Rust.

**[⬆️ Voltar ao Índice](#📑-índice)**

---

## Parte 2: Primeiro Projeto

### 2.1 Criando com Cargo

**Cargo** é o gerenciador de projetos e build tool oficial do Rust. Pense nele como Maven + Gradle combinados, mas muito mais rápido e simples.

#### **Criando um novo projeto:**

```bash
cargo new hello_rust
cd hello_rust
```

**O que aconteceu:**
- ✅ Criou diretório `hello_rust/`
- ✅ Inicializou repositório Git automaticamente
- ✅ Criou `Cargo.toml` (arquivo de configuração)
- ✅ Criou `src/main.rs` (ponto de entrada)
- ✅ Criou `.gitignore` pré-configurado

**Comparação com Java:**

| Java | Rust |
|------|------|
| `mvn archetype:generate` | `cargo new` |
| `pom.xml` | `Cargo.toml` |
| `src/main/java/Main.java` | `src/main.rs` |
| `mvn compile`, `mvn package` | `cargo build` |
| `java -jar target/app.jar` | `cargo run` |

---

### 2.2 Anatomia do Projeto

Estrutura criada:

```
hello_rust/
│
├── Cargo.toml          ← Configuração do projeto (como pom.xml)
├── Cargo.lock          ← Lock de dependências (como package-lock.json)
├── .git/               ← Git já inicializado
├── .gitignore          ← Ignora target/ automaticamente
│
└── src/
    └── main.rs         ← Ponto de entrada do programa
```

#### **📄 Cargo.toml - O "pom.xml" do Rust**

Abra o arquivo `Cargo.toml`:

```toml
[package]
name = "hello_rust"        # Nome do executável
version = "0.1.0"          # Versionamento semântico
edition = "2021"           # Edição do Rust (2015, 2018, 2021)

[dependencies]             # Dependências externas (como <dependencies> no Maven)
# Exemplo: serde = "1.0"
```

**Seções importantes:**

- **`[package]`**: Metadados do projeto
  - `edition`: Rust tem "edições" que atualizam a linguagem (sempre use a mais recente)
  
- **`[dependencies]`**: Crates (bibliotecas) externas
  - Exemplo: `serde = "1.0"` baixa a versão 1.x mais recente
  - Comparação: Isso é como `<dependency>` no Maven, mas mais simples

> **💡 Termo importante:** Em Rust, bibliotecas são chamadas de **crates** (caixas). Repositório oficial: [crates.io](https://crates.io)

#### **📄 src/main.rs - O ponto de entrada**

```rust
fn main() {
    println!("Hello, world!");
}
```

**Comparação Java vs Rust:**

```java
// ☕ JAVA
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

```rust
// 🦀 RUST
fn main() {
    println!("Hello, world!");
}
```

**Diferenças fundamentais:**

| Aspecto | Java | Rust |
|---------|------|------|
| **Modificadores** | `public static` obrigatório | Sem modificadores (implícito) |
| **Classes** | Tudo dentro de uma classe | Sem classes obrigatórias |
| **Argumentos** | `String[] args` | Opcional (use `std::env::args()`) |
| **Print** | `System.out.println()` | `println!()` é uma macro |
| **Ponto e vírgula** | Obrigatório | Obrigatório para statements |
| **Compilação** | Bytecode (JVM) | Nativo (executável direto) |

> **💡 Por que `println!` tem exclamação?** O `!` indica que é uma **macro**, não uma função. Macros são expandidas em tempo de compilação (como templates C++).

**[⬆️ Voltar ao Índice](#📑-índice)**

---

### 2.3 Comandos Essenciais

#### **🔨 `cargo build` - Compila o projeto**

```bash
cargo build
```

**O que faz:**
- ✅ Compila o código em modo debug
- ✅ Cria executável em `target/debug/hello_rust`
- ✅ Gera `Cargo.lock` (se não existir)

**Saída:**
```
   Compiling hello_rust v0.1.0 (/home/user/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
```

**Executar manualmente:**
```bash
./target/debug/hello_rust  # Linux/Mac
.\target\debug\hello_rust.exe  # Windows
```

**Build de produção (otimizado):**
```bash
cargo build --release
```
- ✅ Otimizações máximas (muito mais rápido)
- ✅ Executável em `target/release/`
- ⏱️ Demora mais para compilar

> **Analogia Java:** `cargo build` = `mvn compile`, `--release` = configurar otimizações no javac

---

#### **🚀 `cargo run` - Compila e executa**

```bash
cargo run
```

**O que faz:**
1. Detecta mudanças no código
2. Recompila se necessário
3. Executa automaticamente

**Saída:**
```
   Compiling hello_rust v0.1.0
    Finished dev target(s) in 0.5s
     Running `target/debug/hello_rust`
Hello, world!
```

**Com argumentos:**
```bash
cargo run -- arg1 arg2  # Passa argumentos para o programa
```

> **💡 Dica:** Use `cargo run` para desenvolvimento. É o equivalente a "Run" na IDE do Java.

---

#### **✅ `cargo check` - Verifica sem compilar**

```bash
cargo check
```

**O que faz:**
- ✅ Verifica se o código compila
- ✅ Mostra erros de tipos e sintaxe
- ❌ **NÃO** gera executável
- ⚡ **MUITO mais rápido** que `cargo build`

**Por que usar:**
- Durante desenvolvimento, para feedback rápido
- Rust-analyzer usa isso em background

> **Analogia:** É como se o Java verificasse todos os erros sem gerar .class files. Muito útil!

---

#### **🎨 `cargo fmt` - Formata o código**

```bash
cargo fmt
```

**O que faz:**
- Formata todo código seguindo o estilo oficial
- Equivalente ao Google Java Format

**Verificar sem modificar:**
```bash
cargo fmt -- --check
```

---

#### **📋 `cargo clippy` - Linter inteligente**

```bash
cargo clippy
```

**O que faz:**
- Analisa código e sugere melhorias idiomáticas
- Detecta code smells
- Mais rigoroso que o compilador

**Exemplo de sugestão:**
```rust
// Seu código
let x = String::from("hello");
let y = x.clone();

// Clippy sugere: use .to_owned() ou empreste com &x
```

---

#### **📊 Resumo dos Comandos**

| Comando | O que faz | Quando usar |
|---------|-----------|-------------|
| `cargo new <nome>` | Cria projeto novo | Início de projeto |
| `cargo build` | Compila (debug) | Gerar executável debug |
| `cargo build --release` | Compila otimizado | Deploy/produção |
| `cargo run` | Compila + executa | Desenvolvimento diário |
| `cargo check` | Só verifica erros | Feedback rápido |
| `cargo fmt` | Formata código | Antes de commit |
| `cargo clippy` | Linter avançado | Code review |
| `cargo test` | Executa testes | TDD (veremos depois) |
| `cargo clean` | Remove `target/` | Liberar espaço |

**[⬆️ Voltar ao Índice](#📑-índice)**

---

## Parte 3: Hello World Explicado

### 3.1 Anatomia do Código

Vamos dissecar o Hello World linha por linha:

```rust
fn main() {
    println!("Hello, world!");
}
```

#### **Linha 1: `fn main()`**

```rust
fn main() {
    // corpo da função
}
```

**Breakdown:**

- **`fn`**: Palavra-chave para declarar função (function)
- **`main`**: Nome da função (ponto de entrada obrigatório)
- **`()`**: Parâmetros (vazio neste caso)
- **`{}`**: Bloco de código

**Detalhes importantes:**

1. **Sem `public`:** Em Rust, `fn main()` não precisa de modificadores
   - É **automaticamente** o ponto de entrada
   - Apenas um `fn main()` por binário

2. **Sem tipo de retorno explícito:**
   - Ausência de `-> Type` significa que retorna `()` (unit type)
   - `()` é como `void` em Java, mas mais preciso

3. **Sem `args`:**
   - Argumentos são opcionais
   - Use `std::env::args()` quando necessário

**Comparação completa:**

```java
// ☕ JAVA - Verboso
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello");
    }
}
```

```rust
// 🦀 RUST - Conciso
fn main() {
    println!("Hello");
}
```

---

#### **Linha 2: `println!("Hello, world!")`**

```rust
println!("Hello, world!");
```

**Breakdown:**

- **`println!`**: Macro de impressão com quebra de linha
- **`()`**: Argumentos da macro
- **`"Hello, world!"`**: String literal
- **`;`**: Termina o statement (obrigatório)

**Por que é uma macro (`!`)?**

Macros em Rust são expandidas em tempo de compilação. Veja a diferença:

```rust
// 🦀 Macro - aceita número variável de argumentos
println!("Nome: {}, Idade: {}", "Bianeck", 40);
println!("Só texto");
println!("Três valores: {}, {}, {}", 1, 2, 3);

// Se fosse função normal, precisaria de versões diferentes:
// println1(msg)
// println2(msg, arg1, arg2)
// println3(msg, arg1, arg2, arg3)
```

**Comparação com Java:**

```java
// ☕ JAVA
System.out.println("Hello");
System.out.printf("Nome: %s, Idade: %d\n", "Bianeck", 40);

// 🦀 RUST
println!("Hello");
println!("Nome: {}, Idade: {}", "Bianeck", 40);
```

**Variantes de print:**

```rust
print!("Sem quebra de linha");
println!("Com quebra de linha");
eprintln!("Imprime no stderr");  // Como System.err em Java
```

---

### 3.2 Comparações com Java

#### **Estrutura de Programa**

```java
// ☕ JAVA - Orientado a objetos obrigatório
package com.example;

public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

```rust
// 🦀 RUST - Procedural simples, OOP opcional
fn main() {
    println!("Hello, world!");
}
```

**Diferenças filosóficas:**

| Aspecto | Java | Rust |
|---------|------|------|
| **Paradigma default** | OOP obrigatório | Multi-paradigma (procedural, funcional, OOP) |
| **Classes** | Tudo em classes | Structs + traits (mais flexível) |
| **Herança** | Sim (single) | Não (composição + traits) |
| **Null** | `null` (NPE comum) | Não existe! Use `Option<T>` |
| **Garbage Collector** | Sim (automático) | Não (ownership system) |
| **Exceções** | Try-catch | `Result<T, E>` (erros são valores) |

---

#### **Compilação e Execução**

**☕ JAVA:**
```bash
# Compila para bytecode
javac HelloWorld.java  → HelloWorld.class

# Executa na JVM
java HelloWorld
```

**🦀 RUST:**
```bash
# Compila para binário nativo
cargo build  → target/debug/hello_rust

# Executa diretamente (sem VM)
./target/debug/hello_rust
```

**Implicações:**

- ✅ Rust é **mais rápido** (sem overhead de JVM)
- ✅ Rust usa **menos memória** (sem GC)
- ✅ Rust gera **binário único** (sem dependência de JRE)
- ⚠️ Rust compila **mais devagar** (otimizações agressivas)

---

#### **Formatação de Strings**

```java
// ☕ JAVA
String name = "Bianeck";
int age = 40;

// Printf style
System.out.printf("Nome: %s, Idade: %d%n", name, age);

// String.format
String msg = String.format("Nome: %s, Idade: %d", name, age);

// Concatenação
System.out.println("Nome: " + name + ", Idade: " + age);
```

```rust
// 🦀 RUST
let name = "Bianeck";
let age = 40;

// Placeholder {} - inferência automática de tipo
println!("Nome: {}, Idade: {}", name, age);

// Posicional
println!("2º: {1}, 1º: {0}", name, age);  // "2º: 40, 1º: Bianeck"

// Nomeado
println!("Nome: {n}, Idade: {i}", n=name, i=age);

// Debug (funciona com qualquer tipo)
println!("Debug: {:?}", vec![1, 2, 3]);
```

**Rust é mais simples:** Não precisa lembrar `%s`, `%d`, `%f` - usa `{}` para tudo!

**[⬆️ Voltar ao Índice](#📑-índice)**

---

## Parte 4: Exercícios Práticos

### Exercício 1: Hello Personalizado (Fácil)

**🎯 Objetivo:** Modificar o Hello World para imprimir informações personalizadas.

**📝 Tarefa:**
Crie um programa que imprima:
- Seu nome
- Sua idade
- Sua profissão
- Uma mensagem motivacional

**💡 Conceitos praticados:**
- Variáveis com `let`
- `println!` com múltiplos argumentos
- String formatting

#### **Solução Completa:**

```rust
// 📁 src/main.rs

fn main() {
    // Declaração de variáveis com 'let'
    // Em Rust, variáveis são IMUTÁVEIS por padrão (veremos isso melhor no Dia 2)
    let nome = "Thiago Bianeck";
    let idade = 40;
    let profissao = "Engenheiro de Software Sênior";
    
    // println! com placeholders {}
    // Rust infere automaticamente o tipo e formata corretamente
    println!("===================================");
    println!("       APRESENTAÇÃO PESSOAL        ");
    println!("===================================");
    println!("Nome: {}", nome);
    println!("Idade: {} anos", idade);
    println!("Profissão: {}", profissao);
    println!("===================================");
    
    // Podemos usar múltiplas variáveis em uma linha
    println!("Olá! Sou {}, tenho {} anos e trabalho como {}.", 
             nome, idade, profissao);
    
    // String literal multilinha com indentação
    println!("\n💪 Mensagem motivacional:");
    println!("\"Aprender Rust vai expandir sua mente e");
    println!(" tornar você um desenvolvedor melhor!\"");
    
    // Usando escape de caracteres
    println!("\n\t→ Vamos com tudo! 🚀");
}
```

**🏃 Como executar:**

```bash
# Crie o projeto
cargo new hello_personalizado
cd hello_personalizado

# Cole o código acima em src/main.rs

# Execute
cargo run
```

**📤 Saída esperada:**

```
===================================
       APRESENTAÇÃO PESSOAL        
===================================
Nome: Thiago Bianeck
Idade: 40 anos
Profissão: Engenheiro de Software Sênior
===================================
Olá! Sou Thiago Bianeck, tenho 40 anos e trabalho como Engenheiro de Software Sênior.

💪 Mensagem motivacional:
"Aprender Rust vai expandir sua mente e
 tornar você um desenvolvedor melhor!"

	→ Vamos com tudo! 🚀
```

**🎓 Aprendizados:**

- ✅ Variáveis em Rust usam `let` (sem tipo explícito - inferência)
- ✅ Strings literais usam aspas duplas `""`
- ✅ `println!` formata automaticamente qualquer tipo
- ✅ `\n` = nova linha, `\t` = tab (como em Java)
- ✅ Emojis funcionam nativamente! 🎉

---

### Exercício 2: Calculadora Básica (Médio)

**🎯 Objetivo:** Criar uma calculadora que realiza operações matemáticas básicas.

**📝 Tarefa:**
Crie um programa que:
- Declare duas variáveis numéricas
- Calcule: soma, subtração, multiplicação, divisão, resto
- Formate a saída de forma clara
- Use diferentes tipos numéricos (inteiros e float)

**💡 Conceitos praticados:**
- Tipos numéricos (`i32`, `f64`)
- Operadores aritméticos
- Type casting
- Formatação de floats

#### **Solução Completa:**

```rust
// 📁 src/main.rs

fn main() {
    // ========================================
    // PARTE 1: Operações com Inteiros
    // ========================================
    
    // Em Rust, precisamos especificar o tipo se não for óbvio
    // i32 = inteiro de 32 bits (padrão do Rust)
    // Comparação: int em Java também é 32 bits
    let numero1: i32 = 42;
    let numero2: i32 = 10;
    
    println!("╔══════════════════════════════════╗");
    println!("║  CALCULADORA RUST - INTEIROS     ║");
    println!("╚══════════════════════════════════╝");
    println!("Números: {} e {}\n", numero1, numero2);
    
    // Operações básicas
    let soma = numero1 + numero2;
    let subtracao = numero1 - numero2;
    let multiplicacao = numero1 * numero2;
    let divisao = numero1 / numero2;  // Divisão inteira (trunca)
    let resto = numero1 % numero2;     // Módulo (resto da divisão)
    
    println!("➕ Soma:          {} + {} = {}", numero1, numero2, soma);
    println!("➖ Subtração:     {} - {} = {}", numero1, numero2, subtracao);
    println!("✖️  Multiplicação: {} × {} = {}", numero1, numero2, multiplicacao);
    println!("➗ Divisão:       {} ÷ {} = {}", numero1, numero2, divisao);
    println!("📐 Resto:         {} % {} = {}", numero1, numero2, resto);
    
    // ========================================
    // PARTE 2: Operações com Floats
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  CALCULADORA RUST - DECIMAIS     ║");
    println!("╚══════════════════════════════════╝");
    
    // f64 = float de 64 bits (padrão do Rust, como double em Java)
    let preco_produto: f64 = 127.50;
    let desconto_percentual: f64 = 15.0;
    
    // Cálculo de desconto
    let valor_desconto = preco_produto * (desconto_percentual / 100.0);
    let preco_final = preco_produto - valor_desconto;
    
    // Formatação de floats: {:.2} = 2 casas decimais
    println!("💰 Preço original: R$ {:.2}", preco_produto);
    println!("🏷️  Desconto:       {}%", desconto_percentual);
    println!("💸 Valor desconto: R$ {:.2}", valor_desconto);
    println!("✅ Preço final:    R$ {:.2}", preco_final);
    
    // ========================================
    // PARTE 3: Type Casting (Conversão)
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  CONVERSÃO DE TIPOS              ║");
    println!("╚══════════════════════════════════╝");
    
    let inteiro = 42;
    let float = 10.5;
    
    // Em Rust, conversão explícita é obrigatória (sem coerção automática)
    // Use 'as' para casting (similar ao cast em Java)
    let inteiro_como_float = inteiro as f64;
    let float_como_inteiro = float as i32;  // Trunca (não arredonda)
    
    println!("Inteiro {} como float: {:.1}", inteiro, inteiro_como_float);
    println!("Float {} como inteiro: {} (truncado)", float, float_como_inteiro);
    
    // Operação mista (precisa converter)
    let resultado_misto = inteiro_como_float + float;
    println!("Operação mista: {} + {} = {:.1}", inteiro, float, resultado_misto);
    
    // ========================================
    // PARTE 4: Operações Matemáticas Avançadas
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  FUNÇÕES MATEMÁTICAS             ║");
    println!("╚══════════════════════════════════╝");
    
    let numero: f64 = 16.0;
    
    // Funções matemáticas são métodos do tipo float
    let raiz_quadrada = numero.sqrt();      // Square root
    let potencia = numero.powf(2.0);        // Power (potência)
    let arredondado = 3.7_f64.round();      // Arredondamento
    let piso = 3.7_f64.floor();             // Piso (floor)
    let teto = 3.2_f64.ceil();              // Teto (ceil)
    
    println!("Raiz quadrada de {}: {}", numero, raiz_quadrada);
    println!("Potência {}²: {}", numero, potencia);
    println!("Arredondar 3.7: {}", arredondado);
    println!("Piso de 3.7: {}", piso);
    println!("Teto de 3.2: {}", teto);
    
    // ========================================
    // COMPARAÇÃO: Rust vs Java
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  🦀 RUST vs ☕ JAVA              ║");
    println!("╚══════════════════════════════════╝");
    println!("RUST:  let x: i32 = 42;    (tipo explícito)");
    println!("JAVA:  int x = 42;         (similar)");
    println!();
    println!("RUST:  let y = 42;         (inferência automática)");
    println!("JAVA:  var y = 42;         (Java 10+)");
    println!();
    println!("RUST:  42_i32, 3.14_f64    (sufixos de tipo)");
    println!("JAVA:  42, 3.14D           (literal D para double)");
}
```

**🏃 Como executar:**

```bash
cargo new calculadora_basica
cd calculadora_basica
# Cole o código em src/main.rs
cargo run
```

**📤 Saída esperada:**

```
╔══════════════════════════════════╗
║  CALCULADORA RUST - INTEIROS     ║
╚══════════════════════════════════╝
Números: 42 e 10

➕ Soma:          42 + 10 = 52
➖ Subtração:     42 - 10 = 32
✖️  Multiplicação: 42 × 10 = 420
➗ Divisão:       42 ÷ 10 = 4
📐 Resto:         42 % 10 = 2

╔══════════════════════════════════╗
║  CALCULADORA RUST - DECIMAIS     ║
╚══════════════════════════════════╝
💰 Preço original: R$ 127.50
🏷️  Desconto:       15%
💸 Valor desconto: R$ 19.12
✅ Preço final:    R$ 108.38

╔══════════════════════════════════╗
║  CONVERSÃO DE TIPOS              ║
╚══════════════════════════════════╝
Inteiro 42 como float: 42.0
Float 10.5 como inteiro: 10 (truncado)
Operação mista: 42 + 10.5 = 52.5

╔══════════════════════════════════╗
║  FUNÇÕES MATEMÁTICAS             ║
╚══════════════════════════════════╝
Raiz quadrada de 16: 4
Potência 16²: 256
Arredondar 3.7: 4
Piso de 3.7: 3
Teto de 3.2: 4

╔══════════════════════════════════╗
║  🦀 RUST vs ☕ JAVA              ║
╚══════════════════════════════════╝
RUST:  let x: i32 = 42;    (tipo explícito)
JAVA:  int x = 42;         (similar)

RUST:  let y = 42;         (inferência automática)
JAVA:  var y = 42;         (Java 10+)

RUST:  42_i32, 3.14_f64    (sufixos de tipo)
JAVA:  42, 3.14D           (literal D para double)
```

**🎓 Aprendizados:**

- ✅ Tipos numéricos: `i32` (int), `f64` (double)
- ✅ Operadores: `+`, `-`, `*`, `/`, `%` (iguais ao Java)
- ✅ Type casting: `as` (similar ao cast Java)
- ✅ Formatação: `{:.2}` para 2 casas decimais
- ✅ Métodos matemáticos: `.sqrt()`, `.powf()`, `.round()`
- ⚠️ **Importante:** Rust NÃO faz conversão automática entre tipos!

---

### Exercício 3: Sistema de Boas-Vindas (Desafiador)

**🎯 Objetivo:** Criar um sistema que exibe informações formatadas de forma elaborada.

**📝 Tarefa:**
Crie um programa que:
- Simule um sistema de login
- Exiba um banner ASCII art
- Mostre estatísticas do usuário
- Calcule e exiba progresso percentual
- Use formatação avançada

**💡 Conceitos praticados:**
- Múltiplas variáveis
- Cálculos complexos
- Formatação elaborada
- String multilinha
- Operações com tipos diferentes

#### **Solução Completa:**

```rust
// 📁 src/main.rs

fn main() {
    // ========================================
    // DADOS DO USUÁRIO
    // ========================================
    
    let usuario_nome = "Thiago Bianeck";
    let usuario_login = "bianeck";
    let usuario_nivel = "Sênior";
    let linguagens_dominadas = vec!["Java", "JavaScript", "SQL"];  // Vec = Vector (lista)
    let linguagem_aprendendo = "Rust";
    
    // Estatísticas de estudo
    let dias_total_curso = 60;
    let dias_completos = 1;
    let horas_hoje = 3.5;
    let exercicios_concluidos = 3;
    let exercicios_totais = 180;  // 3 exercícios × 60 dias
    
    // ========================================
    // CÁLCULOS
    // ========================================
    
    // Conversão para float para divisão exata
    let progresso_dias = (dias_completos as f64 / dias_total_curso as f64) * 100.0;
    let progresso_exercicios = (exercicios_concluidos as f64 / exercicios_totais as f64) * 100.0;
    
    // Dias restantes
    let dias_restantes = dias_total_curso - dias_completos;
    
    // Estimativa de horas totais (assumindo 3h/dia)
    let horas_estimadas_totais = dias_total_curso as f64 * 3.0;
    let horas_investidas = dias_completos as f64 * 3.0 + horas_hoje;
    let progresso_horas = (horas_investidas / horas_estimadas_totais) * 100.0;
    
    // ========================================
    // BANNER ASCII ART
    // ========================================
    
    println!("\n");
    println!("██████╗ ██╗   ██╗███████╗████████╗");
    println!("██╔══██╗██║   ██║██╔════╝╚══██╔══╝");
    println!("██████╔╝██║   ██║███████╗   ██║   ");
    println!("██╔══██╗██║   ██║╚════██║   ██║   ");
    println!("██║  ██║╚██████╔╝███████║   ██║   ");
    println!("╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ");
    println!("    🦀 SISTEMA DE APRENDIZADO 🦀");
    println!("\n═══════════════════════════════════════════════════════\n");
    
    // ========================================
    // INFORMAÇÕES DO USUÁRIO
    // ========================================
    
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  👤 PERFIL DO DESENVOLVEDOR                         │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Nome:       {:<40}│", usuario_nome);
    println!("│  Login:      {:<40}│", usuario_login);
    println!("│  Nível:      {:<40}│", usuario_nivel);
    
    // Formatação de lista (convertendo Vec para string)
    // join() junta elementos com um separador
    let langs_str = linguagens_dominadas.join(", ");
    println!("│  Domínio:    {:<40}│", langs_str);
    println!("│  Aprendendo: {:<40}│", linguagem_aprendendo);
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // ESTATÍSTICAS DE PROGRESSO
    // ========================================
    
    println!("\n┌─────────────────────────────────────────────────────┐");
    println!("│  📊 ESTATÍSTICAS DO CURSO                           │");
    println!("├─────────────────────────────────────────────────────┤");
    
    // Usando formatação com alinhamento
    // {:<30} = alinhado à esquerda com 30 caracteres
    // {:>6} = alinhado à direita com 6 caracteres
    println!("│  {:<35} {:>3}/{:<3} dias     │", 
             "Dias completos:", dias_completos, dias_total_curso);
    println!("│  {:<35} {: >15} dias │", 
             "Dias restantes:", dias_restantes);
    println!("│  {:<35} {:>18.1}h │", 
             "Horas hoje:", horas_hoje);
    println!("│  {:<35} {:>18.1}h │", 
             "Total de horas investidas:", horas_investidas);
    
    println!("│                                                     │");
    
    println!("│  {:<35} {:>3}/{:<3} ex.     │", 
             "Exercícios concluídos:", exercicios_concluidos, exercicios_totais);
    
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // BARRAS DE PROGRESSO
    // ========================================
    
    println!("\n┌─────────────────────────────────────────────────────┐");
    println!("│  🎯 PROGRESSO GERAL                                 │");
    println!("├─────────────────────────────────────────────────────┤");
    
    // Função auxiliar para criar barra de progresso
    // Vamos simular isso com strings repetidas
    let criar_barra = |percentual: f64| -> String {
        let largura_total = 40;
        let blocos_cheios = ((percentual / 100.0) * largura_total as f64) as usize;
        let blocos_vazios = largura_total - blocos_cheios;
        
        let mut barra = String::from("│  ");
        barra.push_str(&"█".repeat(blocos_cheios));
        barra.push_str(&"░".repeat(blocos_vazios));
        barra.push_str(&format!(" {:>5.1}% │", percentual));
        barra
    };
    
    println!("│  📅 Dias do curso:                                  │");
    println!("{}", criar_barra(progresso_dias));
    println!("│                                                     │");
    
    println!("│  ⏰ Horas investidas:                               │");
    println!("{}", criar_barra(progresso_horas));
    println!("│                                                     │");
    
    println!("│  ✅ Exercícios resolvidos:                          │");
    println!("{}", criar_barra(progresso_exercicios));
    
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // MENSAGEM MOTIVACIONAL
    // ========================================
    
    println!("\n╔═════════════════════════════════════════════════════╗");
    println!("║  💡 MENSAGEM DO DIA                                 ║");
    println!("╠═════════════════════════════════════════════════════╣");
    println!("║                                                     ║");
    
    // Mensagem condicional baseada no progresso
    if progresso_dias < 10.0 {
        println!("║  🚀 Você está começando sua jornada!                ║");
        println!("║  Todo grande programador começou do zero.          ║");
        println!("║  Continue firme, um dia de cada vez! 💪            ║");
    } else if progresso_dias < 50.0 {
        println!("║  🔥 Você está no caminho certo!                     ║");
        println!("║  Cada linha de código te torna mais forte.        ║");
        println!("║  Mantenha o ritmo! 🎯                              ║");
    } else {
        println!("║  🏆 Você está dominando Rust!                       ║");
        println!("║  A reta final está chegando.                      ║");
        println!("║  Continue assim, campeão! 🥇                       ║");
    }
    
    println!("║                                                     ║");
    println!("╚═════════════════════════════════════════════════════╝");
    
    // ========================================
    // RODAPÉ COM TIMESTAMP SIMULADO
    // ========================================
    
    println!("\n─────────────────────────────────────────────────────────");
    println!("  📅 Dia {} de {}  |  ⏱️  {:.1}h hoje  |  🎯 {} ex. concluídos",
             dias_completos, dias_total_curso, horas_hoje, exercicios_concluidos);
    println!("─────────────────────────────────────────────────────────\n");
}
```

**🏃 Como executar:**

```bash
cargo new sistema_boas_vindas
cd sistema_boas_vindas
# Cole o código em src/main.rs
cargo run
```

**📤 Saída esperada:**

```


██████╗ ██╗   ██╗███████╗████████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝
██████╔╝██║   ██║███████╗   ██║   
██╔══██╗██║   ██║╚════██║   ██║   
██║  ██║╚██████╔╝███████║   ██║   
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
    🦀 SISTEMA DE APRENDIZADO 🦀

═══════════════════════════════════════════════════════

┌─────────────────────────────────────────────────────┐
│  👤 PERFIL DO DESENVOLVEDOR                         │
├─────────────────────────────────────────────────────┤
│  Nome:       Thiago Bianeck                         │
│  Login:      bianeck                                │
│  Nível:      Sênior                                 │
│  Domínio:    Java, JavaScript, SQL                  │
│  Aprendendo: Rust                                   │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│  📊 ESTATÍSTICAS DO CURSO                           │
├─────────────────────────────────────────────────────┤
│  Dias completos:                      1/60  dias    │
│  Dias restantes:                              59 dias│
│  Horas hoje:                                   3.5h │
│  Total de horas investidas:                    3.5h │
│                                                     │
│  Exercícios concluídos:                 3/180 ex.   │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│  🎯 PROGRESSO GERAL                                 │
├─────────────────────────────────────────────────────┤
│  📅 Dias do curso:                                  │
│  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   1.7% │
│                                                     │
│  ⏰ Horas investidas:                               │
│  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   1.9% │
│                                                     │
│  ✅ Exercícios resolvidos:                          │
│  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   1.7% │
└─────────────────────────────────────────────────────┘

╔═════════════════════════════════════════════════════╗
║  💡 MENSAGEM DO DIA                                 ║
╠═════════════════════════════════════════════════════╣
║                                                     ║
║  🚀 Você está começando sua jornada!                ║
║  Todo grande programador começou do zero.          ║
║  Continue firme, um dia de cada vez! 💪            ║
║                                                     ║
╚═════════════════════════════════════════════════════╝

─────────────────────────────────────────────────────────
  📅 Dia 1 de 60  |  ⏱️  3.5h hoje  |  🎯 3 ex. concluídos
─────────────────────────────────────────────────────────

```

**🎓 Aprendizados Avançados:**

**1. Vec (Vector):**
```rust
let lista = vec!["Java", "JavaScript", "SQL"];
let texto_junto = lista.join(", ");  // "Java, JavaScript, SQL"
```

**2. Closures (funções anônimas):**
```rust
let criar_barra = |percentual: f64| -> String {
    // corpo da função
};
```
- Similar a lambdas em Java: `(percentual) -> { ... }`

**3. String manipulation:**
```rust
let mut texto = String::from("Olá");  // String mutável
texto.push_str(" mundo");             // Adiciona texto
let repetido = "=".repeat(50);        // Repete caractere
```

**4. Formatação avançada:**
- `{:<40}`: Alinhamento à esquerda, 40 caracteres
- `{:>6}`: Alinhamento à direita, 6 caracteres
- `{:.1}`: Float com 1 casa decimal
- `{:>5.1}`: Direita, 5 chars, 1 decimal

**5. Conversão de tipos:**
```rust
let inteiro = 42;
let float = inteiro as f64;  // Conversão explícita
```

**6. Condicional if/else:**
```rust
if progresso < 10.0 {
    println!("Iniciante");
} else if progresso < 50.0 {
    println!("Intermediário");
} else {
    println!("Avançado");
}
```

**[⬆️ Voltar ao Índice](#📑-índice)**

---

## Parte 5: Troubleshooting

### 🔧 Problemas Comuns e Soluções

#### **1. "cargo: command not found" (Linux/Mac)**

**Problema:** O PATH não foi configurado após instalação.

**Solução:**
```bash
# Adicione ao PATH manualmente
source "$HOME/.cargo/env"

# Ou adicione ao ~/.bashrc (permanente)
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
source ~/.bashrc

# Para zsh (.zshrc)
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc
source ~/.zshrc
```

**Verificar:**
```bash
echo $PATH | grep cargo
# Deve mostrar: /home/seu-usuario/.cargo/bin
```

---

#### **2. "error: linker 'cc' not found" (Linux)**

**Problema:** Ferramentas de build C/C++ não instaladas.

**Solução Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential
```

**Solução Fedora:**
```bash
sudo dnf install gcc
```

**Solução Arch:**
```bash
sudo pacman -S base-devel
```

---

#### **3. "error: toolchain 'stable-x86_64-pc-windows-msvc' is not installed" (Windows)**

**Problema:** Visual Studio Build Tools não instalado.

**Solução:**
1. Baixe: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Execute o instalador
3. Selecione: "Desktop development with C++"
4. Instale (pode demorar 15-30 min)
5. Reinicie o terminal
6. Execute `rustup default stable`

**Alternativa (usar toolchain GNU):**
```powershell
rustup toolchain install stable-gnu
rustup default stable-gnu
```

---

#### **4. rust-analyzer não funciona no VSCode**

**Problema:** Extensão não detecta o projeto.

**Checklist:**
1. ✅ Você abriu a pasta do projeto (não a pasta `src`)?
2. ✅ O arquivo `Cargo.toml` está na raiz?
3. ✅ A extensão "rust-analyzer" está instalada (não "Rust")?

**Solução:**
```bash
# 1. Feche o VSCode
# 2. No terminal, na pasta do projeto:
code .

# 3. Se ainda não funcionar, recarregue a janela:
# Ctrl+Shift+P → "Developer: Reload Window"
```

**Forçar reload do rust-analyzer:**
- Ctrl+Shift+P → "rust-analyzer: Restart server"

---

#### **5. "error: could not compile `hello_rust`"**

**Problema:** Erros de sintaxe no código.

**Como ler a mensagem:**

```rust
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:20
  |
3 |     println!("{}", x);
  |                    ^ not found in this scope
```

**Anatomia do erro:**
- `E0425`: Código do erro (pesquisável)
- `src/main.rs:3:20`: Arquivo, linha 3, coluna 20
- `^`: Indicador visual do problema
- Mensagem: "cannot find value `x`" (variável não existe)

**Dica:** Rust tem as **melhores mensagens de erro** entre todas as linguagens!

---

#### **6. "warning: unused variable: `x`"**

**Problema:** Variável declarada mas não usada.

**Solução 1 - Use a variável:**
```rust
let x = 42;
println!("{}", x);  // Agora está sendo usada
```

**Solução 2 - Prefixe com underscore:**
```rust
let _x = 42;  // Underscore informa: "sei que não vou usar"
```

**Por que isso importa:**
- Rust avisa sobre código morto
- Ajuda a limpar o código
- Em Java, isso seria apenas um "warning" ignorável

---

#### **7. "error: format argument must be a string literal"**

**Problema:**
```rust
let msg = "Hello";
println!(msg);  // ❌ ERRADO
```

**Solução:**
```rust
let msg = "Hello";
println!("{}", msg);  // ✅ CORRETO

// Ou, se for literal:
println!("Hello");  // ✅ CORRETO
```

**Explicação:**
- `println!` é uma macro, não função
- Primeiro argumento **deve** ser string literal
- Use `{}` para variáveis

---

#### **8. Compilação lenta**

**Problema:** `cargo build` demora muito.

**Soluções:**

**1. Use `cargo check` durante desenvolvimento:**
```bash
cargo check  # 3-5x mais rápido que build
```

**2. Instale `sccache` (cache de compilação):**
```bash
cargo install sccache
```

**Adicione ao `~/.cargo/config.toml`:**
```toml
[build]
rustc-wrapper = "/home/seu-usuario/.cargo/bin/sccache"
```

**3. Use build incremental (já é padrão):**
```toml
# Cargo.toml
[profile.dev]
incremental = true
```

**4. Compile em release apenas quando necessário:**
```bash
cargo build        # Debug: rápido, binário lento
cargo build --release  # Release: lento, binário otimizado
```

---

#### **9. VSCode mostra erros fantasma**

**Problema:** Código compila mas VSCode mostra erro vermelho.

**Solução:**
```bash
# 1. Limpe o cache do Cargo
cargo clean

# 2. Recompile
cargo check

# 3. Restart do rust-analyzer
# Ctrl+Shift+P → "rust-analyzer: Restart server"
```

---

#### **10. "thread 'main' panicked at 'already borrowed: BorrowMutError'"**

**Problema:** Violação das regras de borrowing (veremos no Dia 15-17).

**Por enquanto:**
- Esse erro só aparece em código mais avançado
- Se vir isso no Dia 1, provavelmente copiou código complexo
- Foque nos exercícios básicos primeiro

---

### 📚 Recursos de Ajuda

**Documentação Oficial:**
- https://doc.rust-lang.org/book/ (The Rust Book - imprescindível)
- https://doc.rust-lang.org/std/ (Biblioteca padrão)

**Pesquisar erros:**
```bash
# Copie o código do erro (ex: E0425) e pesquise
https://doc.rust-lang.org/error-index.html#E0425
```

**Comunidade:**
- https://users.rust-lang.org/ (Fórum oficial)
- Discord oficial do Rust
- Reddit: r/rust, r/learnrust

**Compilador interativo:**
- https://play.rust-lang.org/ (testar código online)

**[⬆️ Voltar ao Índice](#📑-índice)**

---

## Checkpoint Final

### ✅ O que você deve saber agora:

Marque o que você consegue fazer sem consultar o material:

**Instalação e Setup:**
- [ ] Instalar Rust via rustup
- [ ] Verificar versão com `rustc --version`
- [ ] Configurar VSCode com rust-analyzer
- [ ] Entender o papel do rustup, rustc, cargo

**Cargo Basics:**
- [ ] Criar projeto: `cargo new nome`
- [ ] Compilar: `cargo build`
- [ ] Executar: `cargo run`
- [ ] Verificar: `cargo check`
- [ ] Formatar: `cargo fmt`

**Estrutura de Projeto:**
- [ ] Entender `Cargo.toml`
- [ ] Saber onde fica o código (`src/main.rs`)
- [ ] Entender `target/` (binários)
- [ ] Saber que `.gitignore` é criado automaticamente

**Código Rust Básico:**
- [ ] Escrever `fn main()`
- [ ] Usar `println!()` com e sem placeholders
- [ ] Declarar variáveis com `let`
- [ ] Usar tipos básicos: `i32`, `f64`, `&str`
- [ ] Fazer operações aritméticas básicas
- [ ] Converter tipos com `as`

**Comparações Java ↔ Rust:**
- [ ] `fn main()` vs `public static void main()`
- [ ] `println!()` vs `System.out.println()`
- [ ] `let` vs `var/int/double`
- [ ] Compilação nativa vs JVM

**Troubleshooting:**
- [ ] Ler mensagens de erro do compilador
- [ ] Resolver problemas de PATH
- [ ] Reiniciar rust-analyzer quando necessário
- [ ] Usar `cargo clean` quando compilação trava

---

### 🎯 Desafio Extra (Opcional)

Se você completou tudo e quer mais, tente:

**1. Combine os 3 exercícios:**
Crie um programa que:
- Mostra banner de boas-vindas
- Solicita dois números (use valores fixos por enquanto)
- Calcula todas operações matemáticas
- Mostra resultados formatados como o Exercício 3

**2. Personalize o banner:**
- Crie seu próprio ASCII art
- Use seu nome no banner
- Adicione cores (pesquise: "colored terminal rust")

**3. Explore a documentação:**
```bash
# Abra a documentação offline
rustup doc
```
- Leia o capítulo 1 e 2 do Rust Book
- Explore a documentação da `std` (biblioteca padrão)

---

### 📊 Autoavaliação

**Nível de conforto (1-5):**

| Tópico | ⭐ 1-5 |
|--------|--------|
| Instalação do Rust | __/5 |
| Uso do Cargo | __/5 |
| Sintaxe básica de Rust | __/5 |
| Exercícios práticos | __/5 |
| Comparações com Java | __/5 |

**Meta:** Todas acima de 3 antes de prosseguir para o Dia 2.

Se algum tópico está abaixo de 3:
1. Revise a seção específica
2. Refaça os exercícios
3. Crie variações dos exercícios
4. Pesquise na documentação oficial

---

### 🚀 Próximos Passos

**Dia 2: Variáveis e Tipos**
- Imutabilidade por padrão (`let` vs `let mut`)
- Shadowing (redeclaração)
- Tipos primitivos completos
- Conversões e casting avançado
- Constantes (`const` vs `let`)

**Preparação:**
- Garanta que todos comandos `cargo` funcionam
- VSCode configurado corretamente
- Todos 3 exercícios resolvidos
- Troubleshooting lido

---

### 💬 Feedback e Reflexão

**Anote suas impressões sobre o Dia 1:**

**O que foi mais fácil?**
_________________________________

**O que foi mais desafiador?**
_________________________________

**Comparado ao Java, o que te surpreendeu?**
_________________________________

**Dúvidas que ficaram:**
_________________________________

**Tempo gasto hoje:** _____ horas

---

## 🎉 Parabéns, Bianeck!

Você completou o **Dia 1** do seu plano de 60 dias em Rust! 

**Hoje você:**
- ✅ Instalou todo o ambiente Rust
- ✅ Configurou um editor profissional
- ✅ Criou e executou seus primeiros programas
- ✅ Aprendeu comandos essenciais do Cargo
- ✅ Resolveu 3 exercícios progressivos
- ✅ Começou a comparar Rust com Java

**Próximo encontro:** Dia 2 - Variáveis e Tipos
**Lembretes:**
- Continue praticando os exercícios
- Experimente modificar os códigos
- Não tenha medo dos erros do compilador (ele é seu amigo!)
- Use `cargo check` com frequência

**Frase motivacional:**
> *"A jornada de mil milhas começa com um único passo."*  
> Você deu o primeiro passo hoje. Continue assim! 🦀💪

---

**[⬆️ Voltar ao Índice](#📑-índice)**