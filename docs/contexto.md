# 📋 CONTEXTO DIÁRIO - PLANO RUST 60 DIAS

**Guia rápido para geração de conteúdo por dia**

---

## 🌟 FASE 1: FUNDAMENTOS (Dias 1-14)

### DIA 1 (15/11) - Setup e Hello World
**Tópicos:** Instalação, VSCode, Cargo, Compilação
**Exercício:** Projeto "hello_rust"
**Prompt-resumo:** "Setup Rust + primeiro Hello World com comparações Java"

### DIA 2 (16/11) - Variáveis e Tipos
**Tópicos:** let/mut, shadowing, tipos primitivos, casting
**Exercício:** Calculadora IMC, Conversor temperaturas
**Prompt-resumo:** "Variáveis: imutabilidade, tipos, comparar com Java final"

### DIA 3 (17/11) - Controle de Fluxo
**Tópicos:** if/else expressões, loops, ranges, match
**Exercício:** Sistema notas, Fibonacci, Jogo adivinhação
**Prompt-resumo:** "Controle fluxo: if expressão, match vs switch Java"

### DIA 4 (18/11) - Funções
**Tópicos:** fn, parâmetros, retorno implícito, tuplas
**Exercício:** Biblioteca matemática, Calculadora modular
**Prompt-resumo:** "Funções: retorno implícito, comparar métodos Java"

### DIA 5 (19/11) - Strings e I/O
**Tópicos:** String vs &str, métodos, stdin, formatação
**Exercício:** Sistema cadastro, Manipulador textos
**Prompt-resumo:** "String vs &str (owned vs borrowed), Java String"

### DIA 6 (20/11) - Structs
**Tópicos:** struct, impl, métodos (&self, &mut self)
**Exercício:** Sistema usuários, Geometria
**Prompt-resumo:** "Structs: similar classes Java mas sem herança"

### DIA 7 (21/11) - Enums e Pattern Matching
**Tópicos:** enum com dados, Option, Result, match
**Exercício:** Máquina estados, Calculadora Result
**Prompt-resumo:** "Enums (algebraic types) vs Java enum simples"

### DIA 8 (22/11) - Vectors
**Tópicos:** Vec<T>, push/pop, iteração, slices
**Exercício:** Todo list, Gerenciador notas
**Prompt-resumo:** "Vec<T> similar ArrayList, ownership ao iterar"

### DIA 9 (23/11) - HashMaps
**Tópicos:** HashMap<K,V>, Entry API, iteração
**Exercício:** Dicionário, Contador frequência
**Prompt-resumo:** "HashMap: get() Option, Entry API idiomática"

### DIA 10 (24/11) - Error Handling
**Tópicos:** panic vs Result, unwrap/expect, ?, erros custom
**Exercício:** Leitor arquivos, Validador
**Prompt-resumo:** "Result vs try/catch: erros são valores"

### DIA 11 (25/11) - Módulos
**Tópicos:** mod, pub, use, paths, arquivos separados
**Exercício:** Refatorar módulos, Biblioteca
**Prompt-resumo:** "Módulos: similar packages Java, pub"

### DIA 12 (26/11) - Testes
**Tópicos:** #[test], assertions, should_panic, cargo test
**Exercício:** Suite testes, TDD
**Prompt-resumo:** "Testes: #[test] similar @Test JUnit"

### DIA 13 (27/11) - Iteradores Básicos
**Tópicos:** iter/iter_mut/into_iter, map/filter/fold
**Exercício:** Pipeline transformações
**Prompt-resumo:** "Iteradores: similar Streams, zero-cost"

### DIA 14 (28/11) - PROJETO CLI
**Tópicos:** Aplicação completa, organização, testes
**Exercício:** Gerenciador Tarefas CLI completo
**Prompt-resumo:** "Projeto CLI: CRUD, JSON, testes"

---

## 🔥 FASE 2: OWNERSHIP (Dias 15-28)

### DIA 15 (29/11) - Conceitos Memória
**Tópicos:** Stack vs Heap, ponteiros, GC vs Ownership
**Exercício:** Visualizar alocações, move semantics
**Prompt-resumo:** "Stack/Heap, GC Java vs Ownership Rust"

### DIA 16 (30/11) - Ownership Rules
**Tópicos:** 3 regras, transferência, funções, clone
**Exercício:** Fix 10 erros compilação
**Prompt-resumo:** "3 regras ownership, fix erros, exemplos"

### DIA 17 (01/12) - References e Borrowing
**Tópicos:** & e &mut, regras borrowing, borrow checker
**Exercício:** Fix 15 borrow errors
**Prompt-resumo:** "Borrowing: & vs &mut, borrow checker amigo"

### DIA 18 (02/12) - Slices
**Tópicos:** &str, &[T], ranges, slices como parâmetros
**Exercício:** Parser CSV, Análise texto
**Prompt-resumo:** "Slices: views eficientes, String vs &str"

### DIA 19 (03/12) - Lifetimes Básicos
**Tópicos:** 'a sintaxe, lifetimes structs, elision, 'static
**Exercício:** Structs com refs, fix lifetime errors
**Prompt-resumo:** "Lifetimes: 'a sintaxe, prazo validade refs"

### DIA 20 (04/12) - Smart Pointers: Box
**Tópicos:** Box<T>, heap allocation, tipos recursivos
**Exercício:** Linked List, Binary Tree
**Prompt-resumo:** "Box: heap explícita, tipos recursivos"

### DIA 21 (05/12) - Smart Pointers: Rc e Arc
**Tópicos:** Rc<T>, Arc<T>, reference counting, Weak<T>
**Exercício:** Graph com Rc, Cache Arc
**Prompt-resumo:** "Rc/Arc: shared ownership, evitar ciclos"

### DIA 22 (06/12) - RefCell e Interior Mutability
**Tópicos:** RefCell<T>, borrow/borrow_mut, Rc<RefCell<T>>
**Exercício:** Mock objects, Cache mutável
**Prompt-resumo:** "Interior mutability: mutar através &T"

### DIA 23 (07/12) - Clone vs Copy
**Tópicos:** Copy trait, Clone trait, diferenças, performance
**Exercício:** Tipos Copy custom, benchmarks
**Prompt-resumo:** "Copy barato vs Clone explícito e caro"

### DIA 24 (08/12) - Debugging Ownership
**Tópicos:** Erros comuns, mensagens compilador, estratégias
**Exercício:** Fix 20 erros diversos
**Prompt-resumo:** "Troubleshooting: ler compilador, estratégias"

### DIA 25 (09/12) - Patterns Avançados
**Tópicos:** Destructuring, @ bindings, guards, ranges
**Exercício:** Parser complexo, State machine
**Prompt-resumo:** "Pattern matching avançado: @, guards"

### DIA 26 (10/12) - Move Semantics Avançado
**Tópicos:** Partial moves, move closures, iteradores
**Exercício:** Builder pattern, closure ownership
**Prompt-resumo:** "Move avançado: partial, closures, iterators"

### DIA 27 (11/12) - Memory Layout e Unsafe
**Tópicos:** unsafe básico, raw pointers, FFI
**Exercício:** FFI básico, wrapper seguro
**Prompt-resumo:** "Unsafe: quando usar, FFI, abstrações seguras"

### DIA 28 (12/12) - PROJETO: Data Structures
**Tópicos:** Stack, Queue, LinkedList, BST
**Exercício:** Biblioteca estruturas dados completa
**Prompt-resumo:** "Biblioteca: Box, Rc, Option, iteradores"

---

## 💎 FASE 3: TIPOS AVANÇADOS (Dias 29-42)

### DIA 29 (13/12) - Traits Básicos
**Tópicos:** trait, impl Trait for Type, bounds, where
**Exercício:** Drawable, Summary, Comparable
**Prompt-resumo:** "Traits: interfaces on steroids, polimorfismo"

### DIA 30 (14/12) - Genéricos
**Tópicos:** <T>, múltiplos parâmetros, monomorphization
**Exercício:** Generic Stack, largest<T>
**Prompt-resumo:** "Generics: zero-cost, monomorphization"

### DIA 31 (15/12) - Associated Types
**Tópicos:** type Item, Iterator trait, vs generics
**Exercício:** Custom iterator, Graph
**Prompt-resumo:** "Associated types: quando usar vs generics"

### DIA 32 (16/12) - Trait Objects
**Tópicos:** dyn Trait, Box<dyn>, virtual dispatch
**Exercício:** Plugin system, GUI heterogêneo
**Prompt-resumo:** "dyn Trait: dynamic dispatch, trade-offs"

### DIA 33 (17/12) - Lifetimes Avançados
**Tópicos:** Lifetime bounds, HRTB, 'static, subtyping
**Exercício:** Parser estado, Iterator complexo
**Prompt-resumo:** "Lifetimes complexos: múltiplos, HRTB"

### DIA 34 (18/12) - Operator Overloading
**Tópicos:** Add, Sub, Mul, Index, Display, Debug
**Exercício:** Complex numbers, Vec2D, Matrix
**Prompt-resumo:** "Operators: traits para +, -, *, Display"

### DIA 35 (19/12) - From, Into, TryFrom
**Tópicos:** From<T>, Into<T> auto, TryFrom, conversões
**Exercício:** Temperature, User validation
**Prompt-resumo:** "Conversões idiomáticas: From/Into/TryFrom"

### DIA 36 (20/12) - Closures Avançados
**Tópicos:** Fn, FnMut, FnOnce, move, return closures
**Exercício:** Callback system, factory
**Prompt-resumo:** "Closures: Fn/FnMut/FnOnce, captures"

### DIA 37 (21/12) - Iteradores Avançados
**Tópicos:** Implementar Iterator, IntoIterator, adapters
**Exercício:** Fibonacci iterator, pipeline
**Prompt-resumo:** "Custom Iterator: zero-cost abstractions"

### DIA 38 (22/12) - Type State Pattern
**Tópicos:** PhantomData, type state, builder safety
**Exercício:** Connection states, Builder type-safe
**Prompt-resumo:** "Type state: segurança compile-time, API"

### DIA 39 (23/12) - Macros Declarativas
**Tópicos:** macro_rules!, patterns, repetições
**Exercício:** hashmap!, assert_matches!
**Prompt-resumo:** "Macros: macro_rules!, patterns, hygiene"

### DIA 40 (24/12) - Derive Macros
**Tópicos:** Debug, Clone, Copy, PartialEq, Default
**Exercício:** Structs derives, custom impls
**Prompt-resumo:** "Derives: Debug/Clone/PartialEq/Ord/Default"

### DIA 41 (25/12) - Error Handling Avançado
**Tópicos:** Custom errors, thiserror, anyhow, chains
**Exercício:** Error hierarchy, conversão
**Prompt-resumo:** "Errors profissionais: thiserror/anyhow"

### DIA 42 (26/12) - PROJETO: Generic Library
**Tópicos:** Biblioteca validação genérica
**Exercício:** Validator<T>, combinators, extensível
**Prompt-resumo:** "Library: traits, generics, API ergonômica"

---

## ⚡ FASE 4: CONCORRÊNCIA (Dias 43-52)

### DIA 43 (27/12) - Threads Básicas
**Tópicos:** thread::spawn, JoinHandle, move, Send/Sync
**Exercício:** Processar paralelo, worker threads
**Prompt-resumo:** "Threads: spawn, move, Send/Sync traits"

### DIA 44 (28/12) - Channels
**Tópicos:** mpsc, Sender/Receiver, múltiplos produtores
**Exercício:** Producer-consumer, pipeline
**Prompt-resumo:** "Channels: mpsc message passing idiomático"

### DIA 45 (29/12) - Mutexes e Arc
**Tópicos:** Mutex<T>, Arc<Mutex<T>>, RwLock, deadlocks
**Exercício:** Contador compartilhado, cache
**Prompt-resumo:** "Mutex: Arc para compartilhar, evitar deadlock"

### DIA 46 (30/12) - Async/Await Basics
**Tópicos:** async fn, .await, Tokio runtime, vs threads
**Exercício:** HTTP requests async, múltiplas tasks
**Prompt-resumo:** "Async/await: Tokio, Future, I/O bound"

### DIA 47 (31/12) - Tokio Avançado
**Tópicos:** select!, join!, timeout, cancellation
**Exercício:** Timeout operações, race
**Prompt-resumo:** "Tokio: select/join/timeout, cancellation"

### DIA 48 (01/01) - Rayon: Data Parallelism
**Tópicos:** par_iter(), parallel iterators, par_sort
**Exercício:** Dataset grande, sort paralelo
**Prompt-resumo:** "Rayon: paralelismo dados trivial"

### DIA 49 (02/01) - Atomics
**Tópicos:** Atomic types, Ordering, compare-and-swap
**Exercício:** Contador lock-free, flags
**Prompt-resumo:** "Atomics: lock-free, performance crítica"

### DIA 50 (03/01) - Sync Primitives
**Tópicos:** Barrier, Condvar, Once, patterns
**Exercício:** Barrier coordenação, Condvar
**Prompt-resumo:** "Primitivas: Barrier/Condvar/Once, quando usar"

### DIA 51 (04/01) - Testing Concurrency
**Tópicos:** Testar concorrência, Loom, stress tests
**Exercício:** Detectar races, stress tests
**Prompt-resumo:** "Testes concorrentes: Loom, stress, debug"

### DIA 52 (05/01) - PROJETO: Web Scraper
**Tópicos:** Scraper concorrente, async, rate limiting
**Exercício:** Scraper completo com Tokio/reqwest
**Prompt-resumo:** "Scraper: async Tokio, concorrência, errors"

---

## 🚀 FASE 5: PROJETO FINAL (Dias 53-60)

### DIAS 53-60 (06-14/01) - REST API Completa

**Opção A: Task Manager API**
- CRUD tarefas, auth, SQL, testes

**Opção B: URL Shortener**
- Encurtar URLs, stats, rate limit, cache

**Opção C: Chat Server**
- WebSockets, rooms, persistência

**Cronograma:**
- Dia 53: Planejamento, arquitetura, setup
- Dia 54-55: Core features (models, logic, DB)
- Dia 56-57: API layer (routes, handlers, middleware)
- Dia 58: Testes completos
- Dia 59: Documentação (README, API docs)
- Dia 60: Polish, refatoração, deploy

