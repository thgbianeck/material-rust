# 🗓️ PLANO DE ESTUDOS RUST: 60 DIAS PARA DOMÍNIO COMPLETO

**👤 Perfil:** Engenheiro de Software Sênior (Java, JavaScript, SQL)  
**⏱️ Dedicação:** 1 hora/dia  
**📅 Início:** 15 de Novembro de 2025  
**🎯 Conclusão:** 14 de Janeiro de 2026  
**🎓 Meta:** Proficiência plena em Rust para mercado de trabalho

---

## 📑 ÍNDICE NAVEGÁVEL

**[FASE 1: Fundamentos](#fase-1)** → Dias 1-14  
**[FASE 2: Ownership & Memory](#fase-2)** → Dias 15-28  
**[FASE 3: Tipos Avançados](#fase-3)** → Dias 29-42  
**[FASE 4: Concorrência & Async](#fase-4)** → Dias 43-52  
**[FASE 5: Projeto Final](#fase-5)** → Dias 53-60

---

<a name="fase-1"></a>
# 🌟 FASE 1: FUNDAMENTOS (Dias 1-14)

**Objetivo:** Dominar sintaxe básica e estruturas de controle

---

## 📅 DIA 1 (15/11/2025) - Setup e Hello World

**📚 Recursos:**
- [Instalação Oficial Rust](https://www.rust-lang.org/tools/install)
- [The Rust Book - Cap 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

**🎯 Tópicos:**
- Instalação do Rust (rustc, cargo, rustfmt)
- Configuração VSCode com rust-analyzer
- Primeiro projeto com Cargo
- Estrutura de um projeto Rust
- Compilação e execução

**💻 Exercício Prático:**
- Criar projeto "hello_rust"
- Programa que exibe informações pessoais
- Usar println! com formatação

**✅ Checkpoint:**
- [ ] Rust instalado e funcionando
- [ ] VSCode configurado
- [ ] Primeiro programa compilado

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Sou Engenheiro de Software Sênior com domínio de Java, JavaScript e SQL. Estou começando a aprender Rust do zero, no meu primeiro dia de estudos (1 hora disponível).

Crie um material de estudo completo e didático sobre:
1. Instalação do Rust (rustc, cargo, rustfmt) em Linux/Windows
2. Configuração do VSCode com rust-analyzer
3. Criação do primeiro projeto com Cargo
4. Estrutura de um projeto Rust (Cargo.toml, src/main.rs)
5. Comandos essenciais: cargo build, cargo run, cargo check

O material deve incluir:
- Passo a passo detalhado da instalação
- Explicação da anatomia do Hello World em Rust
- Comparações com Java quando relevante (ex: fn main() vs public static void main)
- 3 exercícios práticos progressivos (fácil, médio, desafiador)
- Código completo e bem comentado para cada exercício
- Dicas de troubleshooting comum

Formato: markdown estruturado, com exemplos de código, analogias práticas e checkpoint no final.
```

---

## 📅 DIA 2 (16/11/2025) - Variáveis e Tipos de Dados

**📚 Recursos:**
- [The Rust Book - Cap 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Book - Cap 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)

**🎯 Tópicos:**
- Imutabilidade por padrão vs `mut`
- Shadowing
- Tipos escalares (inteiros, float, bool, char)
- Tipos compostos (tuplas, arrays)
- Type casting e conversões

**💻 Exercício Prático:**
- Calculadora de IMC
- Conversor de temperaturas
- Sistema de tipos com validação

**✅ Checkpoint:**
- [ ] Entende diferença entre let e let mut
- [ ] Domina shadowing
- [ ] Conhece todos tipos primitivos

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Sou Engenheiro de Software Sênior (Java expert) aprendendo Rust. Dia 2 de estudos.

Crie material didático detalhado sobre variáveis e tipos de dados em Rust:

TEORIA:
1. Imutabilidade por padrão vs let mut
2. Shadowing (conceito único do Rust)
3. Tipos escalares: i8, i16, i32, i64, i128, u8-u128, f32, f64, bool, char
4. Tipos compostos: tuplas e arrays
5. Type casting com 'as'
6. Parse de strings para números

COMPARAÇÕES COM JAVA:
- Imutabilidade: Rust (let) vs Java (final)
- Tipos primitivos: diferenças e semelhanças
- char: 4 bytes (Rust) vs 2 bytes (Java)

PRÁTICA:
- Exercício 1: Calculadora IMC completa com input do usuário
- Exercício 2: Conversor de temperatura (Celsius/Fahrenheit/Kelvin)
- Exercício 3: Analisador de tipos com casting

Para cada exercício:
- Código completo e comentado
- Explicação linha por linha
- Casos de teste

Adicione analogias práticas e dicas de boas práticas.
```

---

## 📅 DIA 3 (17/11/2025) - Controle de Fluxo

**📚 Recursos:**
- [The Rust Book - Cap 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example - Flow Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)

**🎯 Tópicos:**
- if/else como expressões
- loop, while, for
- Range e iteradores básicos
- match (pattern matching)
- break e continue com labels

**💻 Exercício Prático:**
- Sistema de classificação de notas
- Gerador de sequência Fibonacci
- Jogo de adivinhação básico

**✅ Checkpoint:**
- [ ] Domina if como expressão
- [ ] Usa loops corretamente
- [ ] Entende match básico

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Sou Engenheiro de Software Sênior (Java) estudando Rust. Dia 3, foco em controle de fluxo (1 hora).

Crie material completo sobre estruturas de controle em Rust:

TEORIA:
1. if/else como EXPRESSÕES (não apenas statements)
2. Loops: loop, while, for
3. Ranges (1..10, 1..=10)
4. match - pattern matching poderoso
5. break e continue com labels

COMPARAÇÕES COM JAVA:
- if/else: expressão vs statement
- switch vs match (Rust é muito mais poderoso)
- for tradicional vs for com iteradores

EXERCÍCIOS PRÁTICOS:
1. Sistema de classificação de notas (A-F) com estatísticas
2. Gerador Fibonacci até N termos com loop/while/for
3. Jogo de adivinhação com validação de entrada

Cada exercício deve ter:
- Código completo comentado
- Versões alternativas (loop vs while vs for)
- Tratamento de erros básico
- Testes manuais sugeridos

Foco em diferenças entre Rust e Java, especialmente que if/match retornam valores.
```

---

## 📅 DIA 4 (18/11/2025) - Funções

**📚 Recursos:**
- [The Rust Book - Cap 3.3](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

**🎯 Tópicos:**
- Declaração de funções
- Parâmetros e tipos
- Retorno implícito vs explícito
- Múltiplos retornos com tuplas
- Funções como expressões

**💻 Exercício Prático:**
- Biblioteca de funções matemáticas
- Calculadora modular
- Sistema de validação

**✅ Checkpoint:**
- [ ] Cria funções com parâmetros
- [ ] Entende retorno implícito
- [ ] Usa tuplas para múltiplos retornos

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) estudando Rust. Dia 4: Funções (1 hora).

Material didático sobre funções em Rust:

TEORIA:
1. Sintaxe: fn nome(param: tipo) -> tipo_retorno
2. Parâmetros: por valor vs por referência (introdução)
3. Retorno implícito (sem ;) vs explícito (return)
4. Retorno de tuplas para múltiplos valores
5. Statements vs Expressions

COMPARAÇÕES COM JAVA:
- fn vs public/private methods
- Tipos explícitos obrigatórios vs inferência parcial
- Retorno implícito (não existe em Java)
- Tuplas vs múltiplos retornos com objetos

EXERCÍCIOS PRÁTICOS:
1. Biblioteca matemática: fatorial, fibonacci, primo, mdc, mmc
2. Calculadora modular com operações separadas
3. Sistema de validação: CPF, email, senha (básico)

Para cada exercício:
- Separar em múltiplas funções reutilizáveis
- Usar retorno implícito onde apropriado
- Demonstrar tuplas para retornos múltiplos
- Testes de cada função

Incluir boas práticas de nomenclatura e organização.
```

---

## 📅 DIA 5 (19/11/2025) - Strings e Input/Output

**📚 Recursos:**
- [The Rust Book - Cap 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust by Example - Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)

**🎯 Tópicos:**
- String vs &str
- String mutável e imutável
- Métodos de String
- Leitura de input do usuário
- Formatação com println!

**💻 Exercício Prático:**
- Sistema de cadastro com input
- Manipulador de textos
- Parser de CSV simples

**✅ Checkpoint:**
- [ ] Entende String vs &str
- [ ] Lê input do usuário
- [ ] Manipula strings

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 5: Strings e I/O (1 hora).

Material completo sobre Strings em Rust:

TEORIA:
1. String vs &str (owned vs borrowed)
2. String::from() e .to_string()
3. Concatenação: +, format!, push_str
4. Métodos: len, trim, split, replace, contains
5. Input com std::io::stdin()
6. Formatação: println!, format!, {:?}, {:#?}

COMPARAÇÕES COM JAVA:
- String (heap) vs &str (stack/static)
- Java String vs Rust String ownership
- StringBuilder vs String mutável

EXERCÍCIOS:
1. Sistema de cadastro: nome, email, telefone com validação
2. Manipulador de texto: contar palavras, inverter, capitalizar
3. Parser CSV: ler arquivo, separar campos, validar

Cada exercício:
- Uso prático de String vs &str
- Leitura de input com tratamento de erro
- Manipulação avançada de strings
- Código completo e testado

Foco especial na diferença String/&str que confunde iniciantes.
```

---

## 📅 DIA 6 (20/11/2025) - Structs

**📚 Recursos:**
- [The Rust Book - Cap 5](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

**🎯 Tópicos:**
- Definição de structs
- Instanciação
- Métodos (impl)
- Funções associadas
- Tuple structs

**💻 Exercício Prático:**
- Sistema de usuários
- Geometria (Retângulo, Círculo)
- Mini banco de dados em memória

**✅ Checkpoint:**
- [ ] Cria e usa structs
- [ ] Implementa métodos
- [ ] Usa funções associadas

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java OO) aprendendo Rust. Dia 6: Structs (1 hora).

Material sobre Structs (similar a classes Java):

TEORIA:
1. Definição: struct Nome { campo: tipo }
2. Instanciação e field init shorthand
3. impl: métodos de instância (&self, &mut self, self)
4. Funções associadas (similar a static em Java)
5. Tuple structs e Unit structs
6. Destruturaçãoo

COMPARAÇÕES COM JAVA:
- struct vs class (sem herança!)
- impl vs métodos na classe
- &self vs this
- Funções associadas vs static methods

EXERCÍCIOS:
1. Sistema Usuario: struct com métodos (criar, validar, atualizar)
2. Geometria: Rectangle e Circle com área, perímetro
3. Banco de dados: struct com Vec interno, CRUD básico

Cada exercício:
- Struct bem modelada
- Métodos úteis (&self, &mut self)
- Funções associadas (construtores)
- Exemplo de uso completo

Enfatizar: Rust não tem herança, usa composição e traits (virá depois).
```

---

## 📅 DIA 7 (21/11/2025) - Enums e Pattern Matching

**📚 Recursos:**
- [The Rust Book - Cap 6](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)

**🎯 Tópicos:**
- Definição de enums
- Enums com dados associados
- Option<T>
- Result<T, E>
- Pattern matching avançado

**💻 Exercício Prático:**
- Sistema de estados
- Tratamento de erros com Result
- Menu de aplicação

**✅ Checkpoint:**
- [ ] Cria enums customizados
- [ ] Usa Option e Result
- [ ] Match completo

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) estudando Rust. Dia 7: Enums (1 hora).

Material sobre Enums (MUITO mais poderosos que Java):

TEORIA:
1. Enums básicos: enum Nome { Variante1, Variante2 }
2. Enums com dados: enum Msg { Text(String), Number(i32) }
3. Option<T>: Some(valor) e None (sem null!)
4. Result<T, E>: Ok(valor) e Err(erro)
5. Pattern matching avançado com match
6. if let e while let

COMPARAÇÕES COM JAVA:
- Java enum vs Rust enum (Rust é algebraic data type!)
- Option vs null (Rust não tem null!)
- Result vs Exceptions
- match vs switch (incomparável!)

EXERCÍCIOS:
1. Máquina de estados: Pedido (Pendente, Processando, Enviado, Entregue)
2. Calculadora com Result: divisão por zero retorna Err
3. Menu interativo: usar enum para opções + match

Cada exercício:
- Enum bem modelado com dados associados
- Pattern matching exaustivo
- Tratamento adequado de Option/Result
- Código real, não toy examples

Foco: enums em Rust são revolucionários, não apenas constantes!
```

---

## 📅 DIA 8 (22/11/2025) - Vectors

**📚 Recursos:**
- [The Rust Book - Cap 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust by Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)

**🎯 Tópicos:**
- Criação de Vec<T>
- Push, pop, insert, remove
- Iteração sobre vectors
- Slice de vectors
- Capacidade vs tamanho

**💻 Exercício Prático:**
- Lista de tarefas (Todo list)
- Gerenciador de notas
- Sistema de inventário

**✅ Checkpoint:**
- [ ] Manipula Vec<T>
- [ ] Itera sobre vectors
- [ ] Usa slices

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java ArrayList/List) aprendendo Rust. Dia 8: Vectors (1 hora).

Material sobre Vec<T> (similar a ArrayList):

TEORIA:
1. Criação: Vec::new(), vec![], Vec::with_capacity()
2. Manipulação: push, pop, insert, remove, clear
3. Acesso: get() vs indexação direta
4. Iteração: for, iter(), iter_mut(), into_iter()
5. Slices: &vec[inicio..fim]
6. Capacidade vs comprimento

COMPARAÇÕES COM JAVA:
- Vec<T> vs ArrayList<T>
- Ownership ao iterar (importante!)
- get() retorna Option (seguro) vs array[i] (panic)

EXERCÍCIOS:
1. Todo List: adicionar, remover, marcar completo, filtrar
2. Gerenciador de notas de alunos: CRUD completo, calcular média
3. Inventário: produtos com struct, buscar, ordenar, filtrar

Cada exercício:
- Operações CRUD completas
- Iteração de diferentes formas
- Uso de slices
- Tratamento de índices inválidos

Enfatizar diferenças de ownership ao iterar (emprestar vs consumir).
```

---

## 📅 DIA 9 (23/11/2025) - HashMaps

**📚 Recursos:**
- [The Rust Book - Cap 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust by Example - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)

**🎯 Tópicos:**
- Criação de HashMap<K, V>
- Insert, get, remove
- Entry API
- Iteração sobre chaves/valores
- HashMap de structs

**💻 Exercício Prático:**
- Dicionário de palavras
- Contador de frequência
- Cache simples

**✅ Checkpoint:**
- [ ] Usa HashMap corretamente
- [ ] Entry API
- [ ] Itera sobre maps

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java HashMap) estudando Rust. Dia 9: HashMaps (1 hora).

Material sobre HashMap<K, V>:

TEORIA:
1. Criação: HashMap::new()
2. Operações: insert, get, remove, contains_key
3. Entry API: entry().or_insert(), or_insert_with()
4. Iteração: keys(), values(), iter()
5. Atualização de valores existentes

COMPARAÇÕES COM JAVA:
- HashMap<K,V> vs HashMap<K,V>
- get() retorna Option (não null!)
- Entry API (mais idiomático que Java)

EXERCÍCIOS:
1. Dicionário Português-Inglês: buscar, adicionar, remover
2. Contador de palavras: ler texto, contar frequência, top 10
3. Cache LRU simples: capacidade máxima, eviction

Cada exercício:
- Uso prático de HashMap
- Entry API em ação
- Iteração sobre pares chave-valor
- Casos de uso reais

Foco na Entry API que é idiomática e eficiente em Rust.
```

---

## 📅 DIA 10 (24/11/2025) - Error Handling

**📚 Recursos:**
- [The Rust Book - Cap 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)

**🎯 Tópicos:**
- panic! vs Result
- unwrap, expect
- Operador ?
- Propagação de erros
- Erros customizados

**💻 Exercício Prático:**
- Leitor de arquivos robusto
- Validador com erros detalhados
- Parser com Result

**✅ Checkpoint:**
- [ ] Usa Result corretamente
- [ ] Operador ?
- [ ] Cria erros customizados

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java Exceptions) estudando Rust. Dia 10: Error Handling (1 hora).

Material sobre tratamento de erros em Rust (SEM exceptions!):

TEORIA:
1. panic! vs Result<T, E> (irrecuperável vs recuperável)
2. unwrap(), expect() - quando usar
3. Operador ?: propagação automática
4. match vs if let para Result
5. Criar tipos de erro customizados
6. From trait para conversão de erros

COMPARAÇÕES COM JAVA:
- Result vs try/catch (explícito vs implícito)
- ? vs throws/rethrow
- Sem checked exceptions
- Sem stack unwinding por padrão

EXERCÍCIOS:
1. Leitor de arquivo: ler, parse JSON, tratar erros específicos
2. Validador: CPF/email/senha com erros detalhados
3. Calculadora: divisão por zero, overflow, parse errors

Cada exercício:
- Uso adequado de Result
- Operador ? para propagação
- Enum de erro customizado
- Mensagens de erro úteis
- Código sem unwrap() desnecessário

Enfatizar: erros em Rust são valores, não exceções!
```

---

## 📅 DIA 11 (25/11/2025) - Módulos e Organização

**📚 Recursos:**
- [The Rust Book - Cap 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)

**🎯 Tópicos:**
- Módulos (mod)
- Visibilidade (pub)
- use e paths
- Arquivos separados
- Organização de projeto

**💻 Exercício Prático:**
- Refatorar projeto em módulos
- Biblioteca reutilizável
- Estrutura de projeto limpa

**✅ Checkpoint:**
- [ ] Organiza código em módulos
- [ ] Usa pub corretamente
- [ ] Estrutura multi-arquivo

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java packages) estudando Rust. Dia 11: Módulos (1 hora).

Material sobre organização de código em Rust:

TEORIA:
1. Módulos: mod nome { }
2. Visibilidade: pub vs privado (padrão)
3. use para importar
4. Paths: crate, super, self
5. Arquivos separados: mod.rs vs nome.rs
6. Estrutura src/lib.rs vs src/main.rs

COMPARAÇÕES COM JAVA:
- mod vs package
- pub vs public/private
- use vs import
- Estrutura de arquivos

EXERCÍCIOS:
1. Refatorar calculadora em módulos: operations, validators, utils
2. Biblioteca matemática: separar geometria, estatística, álgebra
3. App completo: models, services, utils em arquivos separados

Cada exercício:
- Estrutura de diretórios clara
- Visibilidade apropriada
- Re-exports quando necessário
- Testes em cada módulo

Incluir exemplo completo de projeto bem estruturado.
```

---

## 📅 DIA 12 (26/11/2025) - Testes

**📚 Recursos:**
- [The Rust Book - Cap 11](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)

**🎯 Tópicos:**
- #[test] e #[cfg(test)]
- assert!, assert_eq!, assert_ne!
- should_panic
- Integration tests
- cargo test

**💻 Exercício Prático:**
- Suite de testes unitários
- Testes de integração
- TDD simples

**✅ Checkpoint:**
- [ ] Escreve testes unitários
- [ ] Usa assertions
- [ ] Roda testes com cargo

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (JUnit) estudando Rust. Dia 12: Testes (1 hora).

Material sobre testing em Rust:

TEORIA:
1. Testes unitários: #[test]
2. Módulo de testes: #[cfg(test)]
3. Assertions: assert!, assert_eq!, assert_ne!
4. #[should_panic] para testes de panic
5. Testes de integração: tests/
6. cargo test: rodar, filtrar, mostrar output

COMPARAÇÕES COM JAVA:
- #[test] vs @Test (JUnit)
- Macros de assert vs JUnit assertions
- cargo test vs Maven/Gradle test

EXERCÍCIOS:
1. Testar biblioteca matemática: 20+ testes para fatorial, fibonacci, primo
2. TDD: criar validador de senha com testes primeiro
3. Testes de integração: testar módulo completo

Cada exercício:
- Testes unitários completos
- Casos de sucesso e falha
- Edge cases
- Testes de panic quando apropriado
- Organização em módulo tests

Incluir boas práticas: nomenclatura, organização, cobertura.
```

---

## 📅 DIA 13 (27/11/2025) - Iteradores Básicos

**📚 Recursos:**
- [The Rust Book - Cap 13.2](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

**🎯 Tópicos:**
- iter(), iter_mut(), into_iter()
- Métodos: map, filter, fold
- collect()
- Iteradores lazy
- Chains

**💻 Exercício Prático:**
- Processamento de coleções
- Pipeline de transformações
- Análise de dados simples

**✅ Checkpoint:**
- [ ] Usa iteradores funcionalmente
- [ ] map, filter, fold
- [ ] collect()

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java Streams) estudando Rust. Dia 13: Iteradores (1 hora).

Material sobre iteradores em Rust (similar a Streams):

TEORIA:
1. iter() vs iter_mut() vs into_iter()
2. Métodos adaptadores: map, filter, take, skip
3. Métodos consumidores: collect, fold, for_each
4. Lazy evaluation
5. Chains: filter().map().collect()

COMPARAÇÕES COM JAVA:
- Iteradores Rust vs Java Streams
- collect() vs Collectors
- Ownership em into_iter()

EXERCÍCIOS:
1. Processar lista de números: filtrar pares, dobrar, somar
2. Análise de texto: contar palavras longas, capitalizar, filtrar
3. Pipeline complexo: ler dados, transformar, agrupar, estatísticas

Cada exercício:
- Uso de múltiplos adaptadores
- Comparação: imperativo vs funcional
- Performance considerations
- Exemplos práticos reais

Enfatizar: iteradores em Rust são zero-cost abstractions!
```

---

## 📅 DIA 14 (28/11/2025) - PROJETO: CLI App

**📚 Recursos:**
- [The Rust Book - Cap 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [clap - CLI parser](https://docs.rs/clap/latest/clap/)

**🎯 Tópicos:**
- Aplicação CLI completa
- Leitura de argumentos
- Organização de projeto
- Integração de tudo aprendido

**💻 Projeto Final Fase 1:**
- Gerenciador de Tarefas CLI
- CRUD completo
- Persistência em arquivo
- Testes

**✅ Checkpoint FASE 1:**
- [ ] Projeto funcionando
- [ ] Código organizado
- [ ] Testes passando
- [ ] Pronto para Ownership!

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) finalizando Fase 1 de Rust. Dia 14: Projeto CLI (1 hora).

Especificação completa para projeto CLI - Gerenciador de Tarefas:

REQUISITOS:
1. Comandos: add, list, complete, remove, clear
2. Persistência: salvar em JSON
3. Estrutura: models, services, utils
4. Testes: unitários e integração

FEATURES:
- todo add "descrição" [prioridade]
- todo complete <id>
- todo remove <id>
- todo clear

ARQUITETURA:
\```
src/
├── main.rs (CLI parsing)
├── lib.rs
├── models/
│   └── task.rs (struct Task)
├── services/
│   └── task_service.rs (CRUD)
└── storage/
    └── file_storage.rs (JSON)
\```

ENTREGÁVEL:
- Código completo comentado
- README com instruções
- Testes em cada módulo
- Exemplos de uso

Guia passo a passo para implementação, começando pela struct Task até CLI completo.
```

---

<a name="fase-2"></a>
# 🔥 FASE 2: OWNERSHIP & MEMORY SAFETY (Dias 15-28)

**Objetivo:** Dominar o coração do Rust - Sistema de Ownership

---

## 📅 DIA 15 (29/11/2025) - Conceitos de Memória

**📚 Recursos:**
- [The Rust Book - Cap 4 Intro](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Visualizing Memory Layout](https://www.youtube.com/watch?v=rAl-9HwD858)

**🎯 Tópicos:**
- Stack vs Heap
- Ponteiros e referências
- Como Java gerencia memória (GC)
- Como Rust gerencia memória (Ownership)
- Copy vs Move semantics

**💻 Exercício Prático:**
- Visualizar alocações
- Comparar com Java
- Exemplos de move

**✅ Checkpoint:**
- [ ] Entende Stack vs Heap
- [ ] Conceito de ownership
- [ ] Move semantics básico

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java com GC) começando o核心 do Rust. Dia 15: Memória (1 hora).

Material FUNDAMENTAL sobre gerenciamento de memória:

TEORIA:
1. Stack: LIFO, tamanho fixo, rápido
2. Heap: dinâmico, alocação lenta
3. Ponteiros e endereços
4. Como funciona GC em Java
5. Como funciona Ownership em Rust
6. Copy types vs Move types

COMPARAÇÕES CRÍTICAS:
Java:
\```
String s1 = new String("hello");
String s2 = s1; // ambos apontam para o mesmo objeto
System.out.println(s1); // funciona
\```

Rust:
\```
let s1 = String::from("hello");
let s2 = s1; // s1 foi MOVIDO
// println!("{}", s1); // ERRO! s1 não existe mais
\```

EXERCÍCIOS:
1. Comparar alocações: tipos stack vs heap
2. Demonstrar move semantics com vários tipos
3. Identificar quando ocorre copy vs move

Cada exercício:
- Diagramas de memória
- Código que compila e código que não compila
- Explicação do porquê
- Analogias do mundo real

Este é o dia mais importante! Base para tudo em Rust.
```

---

## 📅 DIA 16 (30/11/2025) - Ownership Rules

**📚 Recursos:**
- [The Rust Book - Cap 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Ownership Visualizer](https://play.rust-lang.org/)

**🎯 Tópicos:**
- Regras de ownership
- Transferência de ownership
- Funções e ownership
- Return values e ownership
- Clone

**💻 Exercício Prático:**
- Rastrear ownership
- Fix compilation errors
- Refatorar código

**✅ Checkpoint:**
- [ ] Conhece as 3 regras
- [ ] Rastreia ownership
- [ ] Usa clone apropriadamente

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 16: Ownership Rules (1 hora CRÍTICA).

Material sobre as 3 REGRAS SAGRADAS do Ownership:

REGRAS:
1. Cada valor tem um owner
2. Só pode haver um owner por vez
3. Quando o owner sai de escopo, o valor é dropped

TEORIA:
1. Transferência de ownership em atribuições
2. Ownership em funções (passar parâmetros)
3. Ownership em retornos
4. Clone trait para cópia profunda
5. Drop trait (destrutor automático)

EXERCÍCIOS:
1. Fix the errors: 10 exemplos de código quebrado para consertar
2. Ownership tracker: seguir ownership através de chamadas
3. Refatorar: código com clones desnecessários

Cada exercício:
- Código inicial (não compila)
- Análise do problema
- 2-3 soluções possíveis
- Trade-offs de cada solução
- Código final comentado

CRÍTICO: Muitos exemplos de código quebrado para entender os erros do compilador.
```

---

## 📅 DIA 17 (01/12/2025) - References e Borrowing

**📚 Recursos:**
- [The Rust Book - Cap 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

**🎯 Tópicos:**
- Referências imutáveis (&T)
- Referências mutáveis (&mut T)
- Regras de borrowing
- Dangling references
- Múltiplas referências

**💻 Exercício Prático:**
- Usar referências corretamente
- Fix borrow checker errors
- Estruturas com referências

**✅ Checkpoint:**
- [ ] Usa & e &mut
- [ ] Entende regras de borrowing
- [ ] Resolve erros do borrow checker

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 17: Borrowing (1 hora).

Material sobre empréstimos (CHAVE para produtividade em Rust):

REGRAS DE BORROWING:
1. Múltiplas referências imutáveis OU uma referência mutável
2. Referências devem ser sempre válidas
3. Não pode haver dangling references

TEORIA:
1. & (referência imutável): empresta sem ownership
2. &mut (referência mutável): empresta com permissão de modificar
3. Por que não pode ter &mut + & ao mesmo tempo
4. Lifetimes implícitos
5. Borrow checker: o "compilador chato"

COMPARAÇÕES JAVA:
Em Java: tudo é referência (exceto primitivos)
Em Rust: ownership é padrão, referências são explícitas

EXERCÍCIOS:
1. Fix 15 borrow checker errors (progressivamente mais complexos)
2. Refatorar funções para usar referências em vez de ownership
3. Implementar métodos &self vs &mut self vs self

Cada exercício:
- Erro do compilador completo
- Explicação em português do erro
- Como o compilador está te ajudando
- Solução com explicação

O borrow checker é seu amigo! Enfatizar isso.
```

---

## 📅 DIA 18 (02/12/2025) - Slices

**📚 Recursos:**
- [The Rust Book - Cap 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)

**🎯 Tópicos:**
- String slices (&str)
- Array slices (&[T])
- Criação de slices
- Ranges
- Slices como parâmetros

**💻 Exercício Prático:**
- Manipulação de strings
- Parsing com slices
- Funções com slices

**✅ Checkpoint:**
- [ ] Usa &str vs String
- [ ] Cria slices de arrays
- [ ] Ranges corretamente

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 18: Slices (1 hora).

Material sobre Slices (view em uma sequência):

TEORIA:
1. &str: slice de String
2. &[T]: slice de Vec<T> ou array
3. Ranges: ..  , ..=, a.., ..b, a..b
4. Slices não possuem ownership
5. Métodos úteis de slices

COMPARAÇÕES JAVA:
String.substring() copia em Java
&str é apenas uma view em Rust (zero-cost)

EXERCÍCIOS:
1. Parser de CSV: usar slices para dividir linha
2. Análise de texto: first_word, split_sentences com slices
3. Otimizar: refatorar código que usa String para usar &str

Cada exercício:
- Demonstrar vantagem de slices
- Performance: copy vs view
- Quando usar String vs &str
- Funções flexíveis aceitando &str

Enfatizar: slices são views eficientes, não cópias!
```

---

## 📅 DIA 19 (03/12/2025) - Lifetimes Básicos

**📚 Recursos:**
- [The Rust Book - Cap 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

**🎯 Tópicos:**
- Lifetime annotations
- 'a sintaxe
- Lifetimes em structs
- Lifetime elision
- 'static lifetime

**💻 Exercício Prático:**
- Structs com referências
- Funções com múltiplas refs
- Fix lifetime errors

**✅ Checkpoint:**
- [ ] Entende 'a sintaxe
- [ ] Usa lifetimes em structs
- [ ] Resolve erros de lifetime

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 19: Lifetimes (1 hora - conceito avançado).

Material sobre Lifetimes (garante referências válidas):

TEORIA:
1. Por que lifetimes existem
2. Sintaxe: 'a, 'b, 'static
3. Lifetimes em parâmetros de função
4. Lifetimes em structs
5. Lifetime elision rules (quando não precisa anotar)
6. 'static: vive por todo o programa

ANALOGIA:
Lifetimes são como "prazo de validade" de referências.

EXERCÍCIOS:
1. Função que retorna a maior de duas &str com lifetime
2. Struct que guarda referências (Parser, por exemplo)
3. Fix 10 lifetime errors

Cada exercício:
- Código sem lifetimes (não compila)
- Por que o compilador reclama
- Adicionar annotations corretas
- Explicar o que cada 'a significa

IMPORTANTE: Começar simples! Lifetimes confundem no início.
Focar em casos práticos, não teoria excessiva.
```

---

## 📅 DIA 20 (04/12/2025) - Smart Pointers: Box

**📚 Recursos:**
- [The Rust Book - Cap 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)

**🎯 Tópicos:**
- Box<T> básico
- Heap allocation
- Recursive types
- Deref trait
- Drop trait

**💻 Exercício Prático:**
- Linked List
- Binary Tree
- Estruturas recursivas

**✅ Checkpoint:**
- [ ] Usa Box<T>
- [ ] Cria tipos recursivos
- [ ] Entende Deref

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 20: Box<T> (1 hora).

Material sobre Box (heap allocation explícita):

TEORIA:
1. O que é Box<T>: ponteiro único para heap
2. Quando usar: tamanho desconhecido, ownership transfer, trait objects
3. Deref coercion: Box se comporta como T
4. Drop automático
5. Tipos recursivos (impossíveis sem Box)

COMPARAÇÕES JAVA:
Em Java: tudo (objetos) já está no heap
Em Rust: stack por padrão, heap com Box

EXERCÍCIOS:
1. Implementar Linked List com Box<Node>
2. Implementar Binary Tree com Box para filhos
3. Parser recursivo (expressões matemáticas)

Cada exercício:
- Por que precisa de Box
- Implementação completa
- Métodos úteis (insert, search, traverse)
- Testes

Enfatizar: Box é o smart pointer mais simples, começar por ele.
```

---

## 📅 DIA 21 (05/12/2025) - Smart Pointers: Rc e Arc

**📚 Recursos:**
- [The Rust Book - Cap 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html)

**🎯 Tópicos:**
- Rc<T> (reference counting)
- Arc<T> (atomic reference counting)
- Múltiplos owners
- Weak<T>
- Ciclos e memory leaks

**💻 Exercício Prático:**
- Graph com Rc
- Compartilhamento de dados
- Cache compartilhado

**✅ Checkpoint:**
- [ ] Usa Rc<T>
- [ ] Entende Arc<T>
- [ ] Evita ciclos

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 21: Rc/Arc (1 hora).

Material sobre shared ownership:

TEORIA:
1. Rc<T>: reference counting (single-threaded)
2. Arc<T>: atomic reference counting (thread-safe)
3. Rc::clone() vs .clone()
4. Weak<T>: evitar ciclos
5. Quando usar vs Box vs &

COMPARAÇÕES JAVA:
Java: garbage collector cuida de tudo
Rust: reference counting manual com Rc/Arc

EXERCÍCIOS:
1. Graph: nós com múltiplos parents (Rc)
2. Cache compartilhado: Arc para threads
3. Prevenir leak: usar Weak para ciclos

Cada exercício:
- Demonstrar múltiplos owners
- strong_count() para debugging
- Problema sem Weak
- Solução com Weak

Importante: Rc/Arc tem custo runtime! Usar apenas quando necessário.
```

---

## 📅 DIA 22 (06/12/2025) - RefCell e Interior Mutability

**📚 Recursos:**
- [The Rust Book - Cap 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

**🎯 Tópicos:**
- Interior mutability pattern
- RefCell<T>
- borrow() e borrow_mut()
- Runtime borrow checking
- Rc<RefCell<T>>

**💻 Exercício Prático:**
- Mock objects
- Cache mutável compartilhado
- Estruturas com mutabilidade interna

**✅ Checkpoint:**
- [ ] Usa RefCell<T>
- [ ] Rc<RefCell<T>>
- [ ] Runtime checking

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 22: Interior Mutability (1 hora avançada).

Material sobre mutação através de referências imutáveis:

TEORIA:
1. Interior mutability: mutar através de &T
2. RefCell<T>: borrow checking em runtime
3. borrow() retorna Ref<T>
4. borrow_mut() retorna RefMut<T>
5. Panic em runtime se regras violadas
6. Rc<RefCell<T>>: compartilhar + mutar

QUANDO USAR:
- Mocks em testes
- Cache interno
- Estruturas com métodos &self que precisam mutar

EXERCÍCIOS:
1. Mock logger: contar chamadas sem &mut self
2. Cache: HashMap interno mutável
3. Graph: Rc<RefCell<Node>> para modificação compartilhada

Cada exercício:
- Por que RefCell é necessário
- Demonstrar borrow() e borrow_mut()
- Causar panic propositalmente (aprendizado)
- Solução correta

AVISO: Interior mutability é escape hatch! Usar com cuidado.
```

---

## 📅 DIA 23 (07/12/2025) - Clone vs Copy

**📚 Recursos:**
- [The Rust Book - Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [The Rust Book - Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)

**🎯 Tópicos:**
- Copy trait
- Clone trait
- Diferenças fundamentais
- Implementação manual
- Performance implications

**💻 Exercício Prático:**
- Tipos Copy customizados
- Clone para tipos complexos
- Benchmarks

**✅ Checkpoint:**
- [ ] Entende Copy vs Clone
- [ ] Implementa ambos
- [ ] Sabe quando usar cada um

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 23: Copy vs Clone (1 hora).

Material sobre cópia em Rust:

TEORIA:
1. Copy: cópia implícita (bitwise copy)
   - Apenas tipos simples (i32, f64, bool, char, tuples de Copy, arrays de Copy)
   - Implementado automaticamente para tipos elegíveis
2. Clone: cópia explícita (pode ser cara)
   - Para tipos complexos (String, Vec, HashMap)
   - .clone() método explícito
3. Por que não pode ter Copy + Drop

COMPARAÇÕES JAVA:
Java: tudo é referência (exceto primitivos)
Rust: Copy para tipos baratos, Clone para caros

EXERCÍCIOS:
1. Implementar Point2D: #[derive(Copy, Clone)]
2. Implementar Person: #[derive(Clone)] (tem String)
3. Benchmark: medir overhead de clone em loop

Cada exercício:
- Quando derive automaticamente
- Quando implementar manualmente
- Medir performance
- Trade-offs

Enfatizar: Copy é barato, Clone pode ser caro!
```

---

## 📅 DIA 24 (08/12/2025) - Debugging Ownership

**📚 Recursos:**
- [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [Common Errors](https://doc.rust-lang.org/book/appendix-02-operators.html)

**🎯 Tópicos:**
- Erros comuns do borrow checker
- Mensagens do compilador
- Estratégias para resolver
- Ferramentas (rust-analyzer)
- Patterns de refatoração

**💻 Exercício Prático:**
- Fix 20 erros de ownership
- Refatorar código problemático
- Code review de exemplos ruins

**✅ Checkpoint:**
- [ ] Lê mensagens do compilador
- [ ] Estratégias de debug
- [ ] Refatora corretamente

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 24: Debugging Ownership (1 hora PRÁTICA).

Guia de troubleshooting para erros de ownership:

ERROS COMUNS:
1. "value borrowed after move"
2. "cannot borrow as mutable"
3. "lifetime may not live long enough"
4. "returns value referencing data owned by current function"
5. "cannot move out of borrowed content"

ESTRATÉGIAS:
1. Ler a mensagem completa do compilador
2. Usar .clone() temporariamente para isolar problema
3. Refatorar para usar referências
4. Dividir em funções menores
5. Desenhar ownership flow

EXERCÍCIOS:
20 exemplos de código quebrado:
- 5 erros de move
- 5 erros de borrow
- 5 erros de lifetime
- 5 erros mistos

Para cada um:
- Código original (erro)
- Mensagem do compilador
- Explicação do problema
- 2-3 soluções possíveis
- Solução recomendada

Este dia é CRÍTICO para ganhar fluência!
```

---

## 📅 DIA 25 (09/12/2025) - Patterns Avançados

**📚 Recursos:**
- [The Rust Book - Cap 18](https://doc.rust-lang.org/book/ch18-00-patterns.html)

**🎯 Tópicos:**
- Pattern matching avançado
- Destructuring
- @ bindings
- Guards
- Ranges em patterns

**💻 Exercício Prático:**
- Parser complexo
- State machine
- Validação avançada

**✅ Checkpoint:**
- [ ] Patterns avançados
- [ ] Destructuring complexo
- [ ] Guards

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 25: Pattern Matching Avançado (1 hora).

Material sobre patterns além do básico:

TEORIA:
1. Destructuring: tuplas, structs, enums aninhados
2. @ binding: capturar e testar
3. Guards: if dentro de match arm
4. Ranges: 1..=5
5. _ para ignorar partes
6. | para múltiplos patterns

EXERCÍCIOS:
1. Parser de comando: match complexo para CLI
2. State machine: Game state transitions
3. Validator: pattern matching para regras de negócio

Cada exercício:
- Patterns complexos aninhados
- Uso de @ e guards
- Refatorar if/else para match
- Código mais expressivo

Mostrar como patterns deixam código mais declarativo.
```

---

## 📅 DIA 26 (10/12/2025) - Move Semantics Avançado

**📚 Recursos:**
- [Rust Nomicon - Ownership](https://doc.rust-lang.org/nomicon/ownership.html)

**🎯 Tópicos:**
- Partial moves
- Move closures
- Move em loops
- Consumindo iteradores
- Drop order

**💻 Exercício Prático:**
- Builder pattern
- Closure ownership
- Iterator consumers

**✅ Checkpoint:**
- [ ] Partial moves
- [ ] Move closures
- [ ] Iterator ownership

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java) em Rust. Dia 26: Move Semantics Avançado (1 hora).

Material sobre aspectos avançados de ownership:

TEORIA:
1. Partial moves: mover campos individuais de struct
2. Move closures: move keyword
3. Move em loops: problema comum
4. into_iter() vs iter()
5. Drop order e RAII

EXERCÍCIOS:
1. Builder pattern: mover self em cada método
2. Thread spawn: move closure
3. Pipeline: consumir iteradores corretamente

Cada exercício:
- Demonstrar problema
- Solução idiomática
- Trade-offs
- Testes

Foco em patterns reais de código Rust idiomático.
```

---

## 📅 DIA 27 (11/12/2025) - Memory Layout e Unsafe

**📚 Recursos:**
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/)
- [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

**🎯 Tópicos:**
- Memory layout
- Unsafe básico
- Raw pointers
- Quando usar unsafe
- Abstrações seguras

**💻 Exercício Prático:**
- FFI básico
- Otimização com unsafe
- Wrapper seguro

**✅ Checkpoint:**
- [ ] Entende unsafe
- [ ] Raw pointers
- [ ] Quando NÃO usar

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java/C background) em Rust. Dia 27: Unsafe (1 hora).

Material sobre unsafe Rust (CUIDADO!):

TEORIA:
1. Por que unsafe existe
2. 5 superpoderes de unsafe
3. Raw pointers: *const T e *mut T
4. Dereferencing raw pointers
5. Unsafe functions
6. Unsafe traits

QUANDO USAR:
- FFI (chamar C)
- Otimizações críticas
- Estruturas de dados exóticas
- NUNCA por conveniência!

EXERCÍCIOS:
1. FFI: chamar função C simples
2. Otimização: remover bounds checking provadamente seguro
3. Wrapper: criar abstração segura sobre unsafe

Cada exercício:
- Justificar uso de unsafe
- Garantir invariantes
- Documentar assumptions
- Testes extensivos

AVISO: unsafe é escape hatch! 99% do código não precisa.
```

---

## 📅 DIA 28 (12/12/2025) - PROJETO: Data Structure Library

**📚 Recursos:**
- [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

**🎯 Tópicos:**
- Aplicar todo ownership aprendido
- Estruturas de dados complexas
- API segura e ergonômica

**💻 Projeto Final Fase 2:**
- Biblioteca de estruturas de dados
- Stack, Queue, LinkedList, BST
- Testes completos
- Documentação

**✅ Checkpoint FASE 2:**
- [ ] Domina ownership completamente
- [ ] Structs com lifetimes
- [ ] Smart pointers
- [ ] Pronto para tipos avançados!

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior finalizando Fase 2 de Rust (Ownership). Dia 28: Projeto Library (1 hora).

Especificação para biblioteca de estruturas de dados:

IMPLEMENTAR:
1. Stack<T>: push, pop, peek, is_empty
2. Queue<T>: enqueue, dequeue, front
3. LinkedList<T>: push_front, push_back, pop, iter
4. BinarySearchTree<T>: insert, search, remove, traverse

REQUISITOS:
- Usar Box, Rc, Option apropriadamente
- Implementar iteradores
- Testes unitários completos
- Documentação com exemplos

ESTRUTURA:
\```
src/
├── lib.rs
├── stack.rs
├── queue.rs
├── linked_list.rs
└── bst.rs
tests/
└── integration_tests.rs
\```

GUIA:
- Começar por Stack (mais simples)
- LinkedList (desafiador - ownership)
- BST (mais complexo - recursive)

Incluir código completo comentado de pelo menos Stack e Queue.
Para LinkedList e BST, dar estrutura e guiar implementação.
```

---

<a name="fase-3"></a>
# 💎 FASE 3: TIPOS AVANÇADOS & PATTERNS (Dias 29-42)

**Objetivo:** Traits, Genéricos, Lifetimes Avançados

---

## 📅 DIA 29 (13/12/2025) - Traits Básicos

**📚 Recursos:**
- [The Rust Book - Cap 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html)

**🎯 Tópicos:**
- Definição de traits
- Implementação de traits
- Default implementations
- Trait bounds
- where clauses

**💻 Exercício Prático:**
- Trait Drawable
- Trait Summary
- Trait Animal

**✅ Checkpoint:**
- [ ] Define traits
- [ ] Implementa para tipos
- [ ] Usa trait bounds

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java interfaces) em Rust. Dia 29: Traits (1 hora).

Material sobre Traits (interfaces on steroids):

TEORIA:
1. Definição: trait Nome { }
2. Implementação: impl Trait for Type
3. Default methods
4. Trait como parâmetro: impl Trait
5. Trait bounds: T: Trait
6. where clauses para limpar assinaturas

COMPARAÇÕES JAVA:
Interface Java vs Trait Rust:
- Traits podem ter default methods (como Java 8+)
- Traits podem ser implementados para tipos existentes
- Multiple trait bounds (como multiple interfaces)

EXERCÍCIOS:
1. Trait Drawable: draw(&self) - implementar para Circle, Rectangle
2. Trait Summary: summarize() com default - News, Tweet
3. Trait Comparable: compare() - ordenação customizada

Cada exercício:
- Definir trait com e sem default
- Implementar para múltiplos tipos
- Função genérica usando trait bound
- Testes

Enfatizar: traits são fundamentais para polimorfismo em Rust!
```

---

## 📅 DIA 30 (14/12/2025) - Genéricos

**📚 Recursos:**
- [The Rust Book - Cap 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html)

**🎯 Tópicos:**
- Funções genéricas
- Structs genéricos
- Enums genéricos
- Métodos genéricos
- Monomorphization

**💻 Exercício Prático:**
- Generic Stack
- Generic Result/Option custom
- Generic algorithms

**✅ Checkpoint:**
- [ ] Funções genéricas
- [ ] Structs genéricos
- [ ] Performance implications

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java Generics) em Rust. Dia 30: Genéricos (1 hora).

Material sobre Generics em Rust:

TEORIA:
1. Sintaxe: <T>
2. Múltiplos parâmetros: <T, U>
3. Constraints: <T: Trait>
4. Lifetime + Generic: <'a, T>
5. Monomorphization: zero-cost abstraction

COMPARAÇÕES JAVA:
Java: type erasure (runtime)
Rust: monomorphization (compile-time) - zero overhead!

EXERCÍCIOS:
1. Generic Stack<T>: push, pop, peek
2. Generic pair: Pair<T, U> com métodos
3. Generic largest<T: PartialOrd>(list: &[T]) -> &T

Cada exercício:
- Implementação genérica completa
- Trait bounds necessários
- Exemplos com diferentes tipos
- Explicar monomorphization

Mostrar que generics em Rust não têm custo runtime!
```

---

## 📅 DIA 31 (15/12/2025) - Associated Types

**📚 Recursos:**
- [The Rust Book - Associated Types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)

**🎯 Tópicos:**
- Associated types vs generics
- Iterator trait
- Quando usar cada um
- Associated constants

**💻 Exercício Prático:**
- Custom iterator
- Graph com associated types
- Builder pattern

**✅ Checkpoint:**
- [ ] Associated types
- [ ] Custom iterator
- [ ] Vs generics

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 31: Associated Types (1 hora avançada).

Material sobre associated types:

TEORIA:
1. trait com type Item
2. Diferença vs generic <T>
3. Iterator trait: type Item + fn next()
4. Quando usar associated type vs generic

QUANDO USAR:
- Associated type: quando há apenas UMA implementação lógica por tipo
- Generic: quando múltiplas implementações fazem sentido

EXERCÍCIOS:
1. Implementar Iterator para tipo customizado
2. Trait Graph com associated types para Node e Edge
3. Builder pattern com associated types

Cada exercício:
- Mostrar por que associated type é melhor
- Comparar com versão genérica
- Implementação completa
- Uso prático

Conceito avançado mas muito útil!
```

---

## 📅 DIA 32 (16/12/2025) - Trait Objects

**📚 Recursos:**
- [The Rust Book - Cap 17.2](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

**🎯 Tópicos:**
- dyn Trait
- Box<dyn Trait>
- Virtual dispatch
- Object safety
- Vs generics (monomorphization)

**💻 Exercício Prático:**
- Plugin system
- GUI components
- Heterogeneous collections

**✅ Checkpoint:**
- [ ] Usa dyn Trait
- [ ] Box<dyn Trait>
- [ ] Trade-offs

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java polimorfismo) em Rust. Dia 32: Trait Objects (1 hora).

Material sobre dynamic dispatch:

TEORIA:
1. dyn Trait: trait object
2. Box<dyn Trait>, &dyn Trait
3. Virtual dispatch (runtime)
4. Object safety: quando trait pode ser objeto
5. Vs generic: compile-time vs runtime

COMPARAÇÕES JAVA:
Java: tudo é virtual dispatch
Rust: escolha entre monomorphization (genérico) e trait object

EXERCÍCIOS:
1. Plugin system: Vec<Box<dyn Plugin>>
2. GUI: componentes heterogêneos (Button, Label, Input)
3. Logger: múltiplas implementações em runtime

Cada exercício:
- Por que trait object é necessário
- Criar coleção heterogênea
- Trade-off performance
- Quando usar vs generic

Mostrar quando trait objects são inevitáveis.
```

---

## 📅 DIA 33 (17/12/2025) - Lifetimes Avançados

**📚 Recursos:**
- [The Rust Book - Advanced Lifetimes](https://doc.rust-lang.org/book/ch19-02-advanced-lifetimes.html)

**🎯 Tópicos:**
- Lifetime bounds
- Higher-rank trait bounds (HRTB)
- 'static em detalhes
- Lifetime subtyping
- Elision rules completas

**💻 Exercício Prático:**
- Parser com estado
- Iterator complexo
- API com lifetimes

**✅ Checkpoint:**
- [ ] Lifetimes complexos
- [ ] HRTB básico
- [ ] 'static

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 33: Lifetimes Avançados (1 hora).

Material sobre lifetimes além do básico:

TEORIA:
1. Lifetime bounds: T: 'a
2. Multiple lifetimes: 'a, 'b com relacionamentos
3. 'static: referências que vivem forever
4. HRTB: for<'a>
5. Elision rules completas

EXERCÍCIOS:
1. Parser que mantém referência ao input
2. Iterator que retorna referências com lifetime
3. Struct com múltiplos lifetimes relacionados

Cada exercício:
- Situação que requer lifetimes complexos
- Análise do problema
- Anotação correta
- Por que é necessário

Este é um dos tópicos mais difíceis! Muitos exemplos.
```

---

## 📅 DIA 34 (18/12/2025) - Operator Overloading

**📚 Recursos:**
- [The Rust Book - Appendix B](https://doc.rust-lang.org/book/appendix-02-operators.html)

**🎯 Tópicos:**
- Traits para operadores
- Add, Sub, Mul, Div
- Index, IndexMut
- Deref, DerefMut
- Display, Debug

**💻 Exercício Prático:**
- Complex number
- Vector math
- Matrix

**✅ Checkpoint:**
- [ ] Operator overloading
- [ ] Display trait
- [ ] Index trait

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 34: Operator Overloading (1 hora).

Material sobre sobrecarga de operadores via traits:

TEORIA:
1. Add trait: a + b
2. Sub, Mul, Div: operações aritméticas
3. Index: vec[i]
4. Display: println!("{}", x)
5. Debug: println!("{:?}", x)

EXERCÍCIOS:
1. Complex: números complexos com +, -, *, /
2. Vec2D: vetor 2D com operações matemáticas
3. Matrix: matriz 2x2 com multiplicação

Cada exercício:
- Implementar múltiplos traits de operador
- Tornar tipos "naturais" de usar
- Display e Debug bem formatados
- Testes de cada operação

Mostrar como Rust permite criar tipos que parecem built-in!
```

---

## 📅 DIA 35 (19/12/2025) - From, Into, TryFrom

**📚 Recursos:**
- [The Rust Book - From and Into](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Conversions](https://doc.rust-lang.org/rust-by-example/conversion.html)

**🎯 Tópicos:**
- From trait
- Into trait (automático)
- TryFrom, TryInto
- Conversões idiomáticas
- Error conversion

**💻 Exercício Prático:**
- Conversões entre tipos
- Parser com TryFrom
- Error chain

**✅ Checkpoint:**
- [ ] From/Into
- [ ] TryFrom/TryInto
- [ ] Conversões idiomáticas

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 35: Conversion Traits (1 hora).

Material sobre conversões idiomáticas:

TEORIA:
1. From<T>: conversão infalível
2. Into<T>: implementado automaticamente
3. TryFrom<T>: conversão que pode falhar
4. TryInto<T>: automático
5. Usar em APIs: aceitar Into<String>

EXERCÍCIOS:
1. Temperature: From<Celsius> for Fahrenheit
2. User: TryFrom<String> com validação
3. Error types: From<IoError> for AppError

Cada exercício:
- Implementar From e/ou TryFrom
- Mostrar Into automático
- APIs flexíveis com Into
- Conversão de erros com From

Pattern muito comum em código idiomático Rust!
```

---

## 📅 DIA 36 (20/12/2025) - Closures Avançados

**📚 Recursos:**
- [The Rust Book - Cap 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html)

**🎯 Tópicos:**
- Fn, FnMut, FnOnce
- Closure captures
- move closures
- Returning closures
- impl Fn vs Box<dyn Fn>

**💻 Exercício Prático:**
- Callback system
- Custom iterators
- Functional patterns

**✅ Checkpoint:**
- [ ] Fn vs FnMut vs FnOnce
- [ ] Move closures
- [ ] Return closures

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java lambdas) em Rust. Dia 36: Closures Avançados (1 hora).

Material sobre closures além do básico:

TEORIA:
1. Fn: empresta imutavelmente
2. FnMut: empresta mutavelmente
3. FnOnce: consome valores
4. move: forçar ownership
5. Retornar closures: Box<dyn Fn()>

EXERCÍCIOS:
1. Callback system: registrar e executar callbacks
2. Custom map/filter: implementar com closures
3. Factory: função que retorna closure

Cada exercício:
- Escolher Fn/FnMut/FnOnce apropriado
- Usar move quando necessário
- Lifetime de closures
- Testes

Mostrar poder de closures em Rust!
```

---

## 📅 DIA 37 (21/12/2025) - Iteradores Avançados

**📚 Recursos:**
- [The Rust Book - Cap 13.2-13.4](https://doc.rust-lang.org/book/ch13-02-iterators.html)

**🎯 Tópicos:**
- Implementar Iterator trait
- IntoIterator trait
- Iterator adapters
- zip, enumerate, chain
- fold, scan

**💻 Exercício Prático:**
- Custom iterator
- Lazy evaluation
- Iterator combinators

**✅ Checkpoint:**
- [ ] Custom Iterator
- [ ] IntoIterator
- [ ] Adapters avançados

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 37: Iteradores Avançados (1 hora).

Material sobre criar iteradores customizados:

TEORIA:
1. Implementar Iterator: type Item + next()
2. IntoIterator: permitir for loop
3. Adapters: zip, enumerate, chain, flat_map
4. fold: reducer
5. scan: fold com estado

EXERCÍCIOS:
1. Fibonacci iterator: implementar Iterator
2. Range customizado: IntoIterator
3. Pipeline complexo: usar múltiplos adapters

Cada exercício:
- Implementação completa de Iterator
- IntoIterator para for loop
- Uso criativo de adapters
- Comparar performance

Iteradores são abstrações zero-cost essenciais!
```

---

## 📅 DIA 38 (22/12/2025) - Type State Pattern

**📚 Recursos:**
- [Type State Pattern](https://cliffle.com/blog/rust-typestate/)

**🎯 Tópicos:**
- Phantom types
- Type state pattern
- Builder safety
- API impossível de usar errado
- Zero-cost abstractions

**💻 Exercício Prático:**
- State machine em tipos
- Safe builder
- Conexão de banco (estados)

**✅ Checkpoint:**
- [ ] Type state pattern
- [ ] Phantom types
- [ ] API segura por construção

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 38: Type State Pattern (1 hora avançada).

Material sobre segurança em compile-time com tipos:

TEORIA:
1. PhantomData<T>
2. Type state: estados como tipos
3. Transições: consumir e retornar novo tipo
4. Impossível usar incorretamente
5. Zero runtime cost

QUANDO USAR:
- Builder com validação
- State machines
- APIs com ordem de chamadas obrigatória

EXERCÍCIOS:
1. Connection: New -> Connected -> Closed (type state)
2. Builder: campos obrigatórios verificados em compile-time
3. File: Opened -> Read/Write -> Closed

Cada exercício:
- Estados como tipos
- Métodos que consomem e retornam
- Exemplos de código que não compila (bom!)
- API ergonômica e segura

Pattern avançado mas muito poderoso!
```

---

## 📅 DIA 39 (23/12/2025) - Macros Declarativas

**📚 Recursos:**
- [The Rust Book - Cap 19.6](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

**🎯 Tópicos:**
- macro_rules!
- Pattern matching em macros
- Repetições
- Hygiene
- Macros úteis

**💻 Exercício Prático:**
- vec! customizado
- map! macro
- assert variants

**✅ Checkpoint:**
- [ ] macro_rules!
- [ ] Patterns básicos
- [ ] Repetições

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 39: Macros Declarativas (1 hora).

Material sobre macros básicas:

TEORIA:
1. macro_rules! nome
2. Patterns: $name:type
3. Types: expr, ident, ty, pat
4. Repetições: $(...)*
5. Hygiene: escopo automático

EXEMPLOS:
- vec![]: como funciona
- println!(): formatação
- Custom macros

EXERCÍCIOS:
1. hashmap!: criar HashMap facilmente
2. assert_matches!: pattern matching assertion
3. create_struct!: gerar struct

Cada exercício:
- Pattern matching correto
- Repetições quando necessário
- Expandir macro manualmente
- Testes

Macros são meta-programação! Começar simples.
```

---

## 📅 DIA 40 (24/12/2025) - Derive Macros

**📚 Recursos:**
- [Rust Macros Book](https://doc.rust-lang.org/reference/procedural-macros.html)

**🎯 Tópicos:**
- Derive macros comuns
- Debug, Clone, Copy
- PartialEq, Eq
- PartialOrd, Ord
- Default

**💻 Exercício Prático:**
- Structs com derives
- Custom comparisons
- Default implementations

**✅ Checkpoint:**
- [ ] Usa derives
- [ ] Implementa manualmente quando necessário
- [ ] Trade-offs

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 40: Derive Macros (1 hora).

Material sobre derive e traits comuns:

TEORIA:
1. #[derive(Debug)] - {:?}
2. #[derive(Clone, Copy)]
3. #[derive(PartialEq, Eq)] - ==
4. #[derive(PartialOrd, Ord)] - <, >
5. #[derive(Default)]

QUANDO DERIVAR:
- Automático quando possível
- Manual quando lógica customizada

EXERCÍCIOS:
1. Person: derives + custom PartialEq (comparar por ID)
2. Product: derives + custom Ord (ordenar por preço)
3. Config: Default customizado

Cada exercício:
- Usar derives apropriados
- Implementar manualmente quando necessário
- Justificar escolhas
- Testes de comparação/ordenação

Derives economizam muito código boilerplate!
```

---

## 📅 DIA 41 (25/12/2025) - Error Handling Avançado

**📚 Recursos:**
- [thiserror crate](https://docs.rs/thiserror/)
- [anyhow crate](https://docs.rs/anyhow/)

**🎯 Tópicos:**
- Custom error types
- Error chains
- thiserror
- anyhow
- Contexto em erros

**💻 Exercício Prático:**
- Error hierarchy
- Conversão de erros
- Error reporting

**✅ Checkpoint:**
- [ ] Custom errors
- [ ] Error chains
- [ ] thiserror/anyhow

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 41: Error Handling Avançado (1 hora).

Material sobre gerenciamento profissional de erros:

TEORIA:
1. Error enum customizado
2. impl std::error::Error
3. From para conversão automática
4. thiserror: derive Error
5. anyhow: error type flexível

QUANDO USAR:
- thiserror: bibliotecas (tipos específicos)
- anyhow: aplicações (flexibilidade)

EXERCÍCIOS:
1. App errors: enum com variantes para cada erro
2. Error chain: IO -> Parse -> Business logic
3. Refatorar: usar thiserror para eliminar boilerplate

Cada exercício:
- Error enum bem modelado
- Mensagens descritivas
- From implementations
- Contexto preservado

Erros informativos são essenciais para debugging!
```

---

## 📅 DIA 42 (26/12/2025) - PROJETO: Generic Library

**📚 Recursos:**
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

**🎯 Tópicos:**
- Biblioteca com genéricos
- Traits bem definidos
- API ergonômica
- Documentação

**💻 Projeto Final Fase 3:**
- Biblioteca genérica de coleções
- Traits customizados
- Testes extensivos
- Docs

**✅ Checkpoint FASE 3:**
- [ ] Domina traits e genéricos
- [ ] API profissional
- [ ] Pronto para concorrência!

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior finalizando Fase 3 de Rust. Dia 42: Projeto Generic Library (1 hora).

Especificação para biblioteca genérica de validação:

OBJETIVO:
Criar biblioteca de validação reutilizável e extensível.

FEATURES:
1. Trait Validator<T>
2. Validators built-in: range, length, regex, custom
3. Composição: and, or, not
4. Generic sobre tipos validados
5. Error types descritivos

ESTRUTURA:
\```
validator_lib/
├── src/
│   ├── lib.rs
│   ├── validator.rs (trait)
│   ├── validators/
│   │   ├── range.rs
│   │   ├── length.rs
│   │   └── regex.rs
│   ├── combinators.rs
│   └── error.rs
├── examples/
│   └── usage.rs
└── tests/
    └── integration_tests.rs
\```

API EXAMPLE:
\```
let validator = RangeValidator::new(0, 100)
    .and(MultipleOf::new(5));
    
validator.validate(&75)?; // Ok
validator.validate(&73)?; // Err
\```

ENTREGÁVEL:
- Código genérico completo
- Trait Validator bem definido
- Pelo menos 3 validators concretos
- Combinators (and/or/not)
- Testes extensivos
- Documentação com exemplos

Guia passo a passo focando em design com traits e genéricos.
```

---

<a name="fase-4"></a>
# ⚡ FASE 4: CONCORRÊNCIA & ASYNC (Dias 43-52)

**Objetivo:** Programação concorrente e assíncrona

---

## 📅 DIA 43 (27/12/2025) - Threads Básicas

**📚 Recursos:**
- [The Rust Book - Cap 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

**🎯 Tópicos:**
- std::thread::spawn
- JoinHandle
- move closures em threads
- Thread safety
- Send e Sync traits

**💻 Exercício Prático:**
- Processar dados em paralelo
- Worker threads
- Thread pool básico

**✅ Checkpoint:**
- [ ] Cria threads
- [ ] move closures
- [ ] Join threads

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java Threads) em Rust. Dia 43: Threads (1 hora).

Material sobre threads em Rust:

TEORIA:
2. JoinHandle::join()
3. move closures: mover ownership
4. Send trait: pode mover entre threads
5. Sync trait: pode compartilhar referências

COMPARAÇÕES JAVA:
new Thread() vs thread::spawn
Java: tudo é Sync por padrão (perigoso!)
Rust: Send/Sync verificados em compile-time

EXERCÍCIOS:
1. Processar Vec em múltiplas threads
2. Download paralelo: simular N downloads
3. Worker pool: fila de tarefas com threads

Cada exercício:
- Spawn múltiplas threads
- move para ownership
- Join e coletar resultados
- Tratar panics em threads

Mostrar segurança de threads em compile-time!
```

---

## 📅 DIA 44 (28/12/2025) - Channels

**📚 Recursos:**
- [The Rust Book - Cap 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

**🎯 Tópicos:**
- mpsc channels
- Sender e Receiver
- Multiple producers
- Async channels (crossbeam)

**💻 Exercício Prático:**
- Producer-consumer
- Pipeline de processamento
- Message passing

**✅ Checkpoint:**
- [ ] mpsc channels
- [ ] Multiple producers
- [ ] Message passing

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 44: Channels (1 hora).

Material sobre comunicação entre threads:

TEORIA:
1. mpsc::channel(): multiple producer, single consumer
2. Sender::send(), Receiver::recv()
3. Clone Sender para múltiplos produtores
4. Iteração sobre Receiver
5. Fechamento automático de canais

EXERCÍCIOS:
1. Producer-consumer: N produtores, 1 consumidor
2. Pipeline: stage1 -> channel -> stage2 -> channel -> output
3. Worker pool: enviar tasks via channel

Cada exercício:
- Criar channel
- Múltiplos produtores
- Consumir até fechar
- Coordenação entre threads

Channels são idiomáticos em Rust!
```

---

## 📅 DIA 45 (29/12/2025) - Mutexes e Arc

**📚 Recursos:**
- [The Rust Book - Cap 16.3](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

**🎯 Tópicos:**
- Mutex<T>
- Arc<Mutex<T>>
- RwLock
- Deadlocks
- Poison

**💻 Exercício Prático:**
- Contador compartilhado
- Cache thread-safe
- Evitar deadlocks

**✅ Checkpoint:**
- [ ] Mutex<T>
- [ ] Arc para compartilhar
- [ ] Evita deadlocks

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (Java synchronized) em Rust. Dia 45: Mutex (1 hora).

Material sobre estado compartilhado:

TEORIA:
1. Mutex<T>: mutual exclusion
2. lock(): adquire lock, retorna MutexGuard
3. Arc<Mutex<T>>: compartilhar entre threads
4. RwLock: múltiplos leitores, um escritor
5. Poison: se thread panic com lock

COMPARAÇÕES JAVA:
synchronized vs Mutex
Java: implicit locking
Rust: explicit lock/unlock (via RAII)

EXERCÍCIOS:
1. Contador: Arc<Mutex<i32>> incrementado por N threads
2. Cache: HashMap thread-safe
3. Deadlock: causar e resolver

Cada exercício:
- Arc para compartilhar ownership
- Lock para acessar
- Escopo de MutexGuard
- Evitar deadlocks (ordem de locks)

Mutex em Rust é mais seguro que Java!
```

---

## 📅 DIA 46 (30/12/2025) - Async/Await Basics

**📚 Recursos:**
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

**🎯 Tópicos:**
- async fn
- .await
- Future trait
- Executors (Tokio)
- async vs threads

**💻 Exercício Prático:**
- HTTP requests async
- Múltiplas tasks
- tokio::spawn

**✅ Checkpoint:**
- [ ] async/await
- [ ] Tokio runtime
- [ ] Async vs threads

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior (JavaScript async/await) em Rust. Dia 46: Async/Await (1 hora).

Material sobre programação assíncrona:

TEORIA:
1. async fn: retorna Future
2. .await: suspender execução
3. Runtime: Tokio (executor)
4. tokio::spawn: task assíncrona
5. Quando async vs threads

COMPARAÇÕES JAVASCRIPT:
Similar: async/await sintaxe
Diferente: runtime explícito (Tokio)

SETUP:
Cargo.toml: tokio = { version = "1", features = ["full"] }

EXERCÍCIOS:
1. Fetch URLs: múltiplos requests concorrentes
2. Timer: sleep assíncrono
3. Converter sync para async

Cada exercício:
- #[tokio::main]
- async fn
- .await em calls
- tokio::spawn para concorrência

Async para I/O, threads para CPU-bound!
```

---

## 📅 DIA 47 (31/12/2025) - Tokio Avançado

**📚 Recursos:**
- [Tokio Docs](https://docs.rs/tokio/)

**🎯 Tópicos:**
- tokio::select!
- tokio::join!
- tokio::time
- Cancellation
- Async streams

**💻 Exercício Prático:**
- Timeout em operações
- Select em múltiplos futures
- Async pipeline

**✅ Checkpoint:**
- [ ] select! e join!
- [ ] Timeouts
- [ ] Cancellation

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 47: Tokio Avançado (1 hora).

Material sobre padrões assíncronos:

TEORIA:
1. tokio::join!: esperar múltiplos futures
2. tokio::select!: primeiro que completar
3. tokio::time::timeout
4. Cancellation: dropar Future
5. Stream trait

EXERCÍCIOS:
1. Timeout: operação com limite de tempo
2. Race: select! entre múltiplas operações
3. Pipeline async: stream processing

Cada exercício:
- Usar macros Tokio
- Tratar timeouts
- Composição de futures
- Error handling async

Patterns avançados para código assíncrono robusto!
```

---

## 📅 DIA 48 (01/01/2026) - Rayon: Data Parallelism

**📚 Recursos:**
- [Rayon Docs](https://docs.rs/rayon/)

**🎯 Tópicos:**
- par_iter()
- Parallel iterators
- par_sort
- join e scope
- Quando usar Rayon

**💻 Exercício Prático:**
- Processar grande dataset
- Sort paralelo
- Map-reduce

**✅ Checkpoint:**
- [ ] Rayon basics
- [ ] Parallel iterators
- [ ] Performance gains

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 48: Rayon (1 hora).

Material sobre paralelismo de dados:

TEORIA:
1. use rayon::prelude::*
2. .par_iter(): parallel iterator
3. Métodos: map, filter, sum - paralelos!
4. par_sort(): ordenação paralela
5. Rayon vs threads manuais

QUANDO USAR:
- Processar coleções grandes
- CPU-bound
- "Fácil" paralelismo (Rayon cuida dos threads)

EXERCÍCIOS:
1. Processar 1M números: filter/map/reduce
2. Ordenar grande array
3. Image processing: processar pixels em paralelo

Cada exercício:
- Versão sequencial
- Versão paralela (trocar iter por par_iter)
- Benchmark: medir speedup
- Análise de performance

Rayon torna paralelismo trivial!
```

---

## 📅 DIA 49 (02/01/2026) - Atomics

**📚 Recursos:**
- [The Rustonomicon - Atomics](https://doc.rust-lang.org/nomicon/atomics.html)

**🎯 Tópicos:**
- Atomic types
- Ordering
- Compare-and-swap
- Lock-free structures
- Quando usar

**💻 Exercício Prático:**
- Contador lock-free
- Flags atômicos
- Simple spinlock

**✅ Checkpoint:**
- [ ] Atomic types
- [ ] Ordering basics
- [ ] Lock-free

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 49: Atomics (1 hora avançada).

Material sobre operações atômicas:

TEORIA:
1. AtomicUsize, AtomicBool, etc
2. load(), store(), fetch_add()
3. Ordering: Relaxed, Acquire, Release, SeqCst
4. Compare-and-swap
5. Quando atomics vs Mutex

QUANDO USAR:
- Performance crítica
- Lock-free algorithms
- Flags simples

EXERCÍCIOS:
1. Contador: AtomicUsize incrementado por threads
2. Flag: AtomicBool para shutdown
3. Spinlock básico (educacional)

Cada exercício:
- Usar atomic apropriado
- Ordering correto
- Comparar performance vs Mutex
- AVISOS sobre dificuldade

Atomics são avançados! Usar Mutex quando em dúvida.
```

---

## 📅 DIA 50 (03/01/2026) - Sync Primitives

**📚 Recursos:**
- [std::sync docs](https://doc.rust-lang.org/std/sync/)

**🎯 Tópicos:**
- Barrier
- Condvar
- Once
- Patterns de sincronização
- Escolher primitiva certa

**💻 Exercício Prático:**
- Barrier para coordenação
- Condvar producer-consumer
- Singleton com Once

**✅ Checkpoint:**
- [ ] Sync primitives
- [ ] Patterns de coordenação
- [ ] Escolher ferramenta certa

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 50: Sync Primitives (1 hora).

Material sobre primitivas de sincronização:

TEORIA:
1. Barrier: esperar todas threads
2. Condvar: wait/notify
3. Once: executar apenas uma vez
4. Semaphore (tokio)
5. Choosing: qual usar quando

EXERCÍCIOS:
1. Barrier: N threads sincronizadas em checkpoints
2. Producer-Consumer: Condvar + Mutex
3. Singleton: Once para init

Cada exercício:
- Usar primitiva apropriada
- Coordenação correta
- Evitar race conditions
- Comparar com alternativas

Conhecer toolkit completo de concorrência!
```

---

## 📅 DIA 51 (04/01/2026) - Testing Concurrency

**📚 Recursos:**
- [Loom](https://docs.rs/loom/)

**🎯 Tópicos:**
- Testar código concorrente
- Race conditions
- Loom para testes
- Stress testing
- Debugging concorrência

**💻 Exercício Prático:**
- Testes com threads
- Detectar race conditions
- Stress tests

**✅ Checkpoint:**
- [ ] Testa código concorrente
- [ ] Detecta races
- [ ] Usa Loom

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior em Rust. Dia 51: Testing Concurrency (1 hora).

Material sobre testar código concorrente:

TEORIA:
1. Dificuldades: non-determinism
2. Stress tests: rodar muitas vezes
3. Loom: model checker
4. ThreadSanitizer
5. Debugging: print, logs, tracing

EXERCÍCIOS:
1. Testar contador: detectar race sem Mutex
2. Stress test: bounded queue
3. Loom: verificar lock-free structure

Cada exercício:
- Testes que passam "às vezes" (bug!)
- Adicionar sincronização
- Testes determinísticos
- Usar Loom quando possível

Testar concorrência é difícil mas essencial!
```

---

## 📅 DIA 52 (05/01/2026) - PROJETO: Web Scraper Concorrente

**📚 Recursos:**
- [reqwest](https://docs.rs/reqwest/)
- [tokio](https://tokio.rs/)

**🎯 Tópicos:**
- Aplicar async/await
- HTTP requests concorrentes
- Processar resultados
- Rate limiting

**💻 Projeto Final Fase 4:**
- Web scraper
- Múltiplas URLs concorrentes
- Parse HTML
- Salvar resultados

**✅ Checkpoint FASE 4:**
- [ ] Domina concorrência
- [ ] Async/await fluente
- [ ] Pronto para projeto final!

**🤖 PROMPT PARA GERAR CONTEÚDO:**

```
Engenheiro de Software Sênior finalizando Fase 4 de Rust. Dia 52: Web Scraper (1 hora).

Especificação para web scraper concorrente:

FEATURES:
1. Ler lista de URLs
2. Fetch concorrente (max N simultâneos)
3. Parse HTML (scraper crate)
4. Extrair dados específicos
5. Rate limiting
6. Retry em falhas
7. Salvar resultados

ARQUITETURA:
- Async com Tokio
- Semaphore para limitar concorrência
- Channel para resultados
- Error handling robusto

ESTRUTURA:
\```
web_scraper/
├── src/
│   ├── main.rs
│   ├── fetcher.rs (async fetch)
│   ├── parser.rs (HTML parsing)
│   └── storage.rs (save results)
└── urls.txt
\```

DEPENDENCIES:
- tokio
- reqwest
- scraper
- anyhow

EXEMPLO USO:
\```
cargo run -- --urls urls.txt --max-concurrent 10 --output results.json
\```

ENTREGÁVEL:
- Código async completo
- Rate limiting
- Error handling
- Logs (tracing)
- README com instruções

Guia passo a passo do setup até scraper funcional.
```

---

<a name="fase-5"></a>
# 🚀 FASE 5: PROJETO FINAL & PORTFOLIO (Dias 53-60)

**Objetivo:** Aplicação completa e profissional

---

## 📅 DIA 53-60 (06-14/01/2026) - PROJETO FINAL

**📚 Recursos:**
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

**🎯 Projeto: REST API Completa**

**Escolha UMA das opções:**

**OPÇÃO A: Task Manager API**
- CRUD de tarefas
- Usuários e autenticação
- Tags e filtros
- Persistência (SQL)
- Testes completos

**OPÇÃO B: URL Shortener**
- Encurtar URLs
- Estatísticas de acesso
- Rate limiting
- Cache (Redis)
- API REST

**OPÇÃO C: Chat Server**
- WebSockets
- Rooms/channels
- Mensagens persistidas
- Online users
- Concorrência

**💻 Cronograma 8 Dias:**

**Dia 53: Planejamento e Setup**
- Arquitetura
- Dependencies
- Database schema
- Estrutura de código

**Dia 54-55: Core Features**
- Models
- Business logic
- Database layer

**Dia 56-57: API Layer**
- Routes
- Handlers
- Middleware
- Error handling

**Dia 58: Testes**
- Unit tests
- Integration tests
- API tests

**Dia 59: Documentação**
- README
- API docs
- Deployment guide

**Dia 60: Polish e Deploy**
- Refatoração
- Performance
- Docker
- Deploy (opcional)

**✅ Checkpoint FINAL:**
- [ ] Aplicação completa funcionando
- [ ] Testes passando
- [ ] Documentação profissional
- [ ] Código limpo e idiomático
- [ ] Pronto para portfolio!

**🤖 PROMPT PARA CADA DIA:**

**DIA 53:**
```
Sou Engenheiro de Software Sênior finalizando aprendizado de Rust (60 dias). Dia 53/60.

Estou iniciando projeto final: [ESCOLHER OPÇÃO].

Crie especificação DETALHADA incluindo:

ARQUITETURA:
- Camadas: models, services, api, storage
- Tecnologias: Axum/Actix, SQLx/Diesel, etc
- Diagrama de componentes

DATABASE SCHEMA:
- Tabelas necessárias
- Relacionamentos
- Migrations

ESTRUTURA DE CÓDIGO:
\```
project/
├── src/
│   ├── main.rs
│   ├── models/
│   ├── services/
│   ├── api/
│   └── db/
├── migrations/
├── tests/
└── Cargo.toml
\```

DEPENDENCIES (Cargo.toml completo)

FEATURES PRIORITIZADAS:
- MVP (dias 54-55)
- Extras (dia 56-57)

Guia para começar: primeiro arquivo, primeiro model, primeira migration.
```

**DIA 54-60:**
```
[Ajustar prompt conforme necessidade de cada dia - pedir implementação de feature específica, testes, documentação, etc]
```

---

## 🎓 CONCLUSÃO DO PLANO

**Após 60 dias você terá:**

✅ Domínio completo de Rust  
✅ Portfolio com 5+ projetos  
✅ Conhecimento de ownership profundo  
✅ Experiência com async/concorrência  
✅ Projeto final profissional  
✅ Pronto para entrevistas  
✅ Apto para posições Rust no mercado

**Próximos Passos:**
1. Contribuir para projetos open source
2. Participar da comunidade Rust (forum, Discord)
3. Aplicar para vagas Rust
4. Continuar aprendendo: unsafe avançado, proc macros, embedded

**Recursos Contínuos:**
- [This Week in Rust](https://this-week-in-rust.org/)
- [Rust Blog](https://blog.rust-lang.org/)
- [r/rust](https://reddit.com/r/rust)
- [Rust Discord](https://discord.gg/rust-lang)

---

**BOA SORTE NA SUA JORNADA RUST, BIANECK! 🦀🚀**