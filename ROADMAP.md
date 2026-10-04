# Rust: 0 → Hero Roadmap

Reference หลัก: [The Rust Book](https://doc.rust-lang.org/book/) (`rustup doc --book` เปิด offline ได้)
เสริม: [Rust by Example](https://doc.rust-lang.org/rust-by-example/), [Rustlings](https://github.com/rust-lang/rustlings)

ทำเสร็จแต่ละข้อ → เปลี่ยน `[ ]` เป็น `[x]`. เวลาเป็นแค่ประมาณ (เรียน ~1 ชม./วัน).

---

## Phase 0 — Setup (วันที่ 1)
- [x] `git init` + `.gitignore` (`/target`)
- [x] สร้าง Cargo workspace ที่ root (`Cargo.toml` มี `[workspace]`)
- [x] สร้าง crate แรก `cargo new lessons/00-hello --name hello` แล้ว `cargo run -p hello`
- [x] Editor: Zed (rust-analyzer + debugger built-in) เปิด inlay hints + clippy on save
- [x] รู้จัก `cargo check` / `build` / `run` / `test` / `fmt` / `clippy` / `doc --open`

## Phase 1 — Basics (สัปดาห์ 1) · Book ch.1–3
- [x] variables, `mut`, shadowing, constants
- [x] scalar/compound types, type inference
- [x] functions, expressions vs statements
- [x] `if`, `loop`, `while`, `for`, ranges
- [x] 🛠 Mini: Guessing Game (Book ch.2), °C↔°F converter, FizzBuzz, Fibonacci

## Phase 2 — Ownership (สัปดาห์ 2) · Book ch.4  ⚠️ หัวใจของ Rust
- [x] ownership rules, move vs copy, `Clone`
- [x] references `&` / `&mut`, borrowing rules
- [ ] slices (`&str`, `&[T]`), `String` vs `&str`
- [ ] 🛠 Mini: `first_word`, reverse words, palindrome checker

## Phase 3 — Structs, Enums, Pattern Matching (สัปดาห์ 3) · Book ch.5–6
- [ ] struct, tuple struct, `impl`, methods, associated fns
- [ ] `#[derive(Debug, Clone, PartialEq)]`
- [ ] enum with data, `Option<T>`, `match`, `if let`, `let else`
- [ ] 🛠 Mini: Shape area calculator, traffic-light state machine

## Phase 4 — Modules & Collections (สัปดาห์ 4) · Book ch.7–8
- [ ] packages / crates / modules, `mod`, `pub`, `use`, แยกไฟล์
- [ ] `Vec<T>`, `String`, `HashMap<K, V>`
- [ ] 🛠 Mini: word frequency counter, employee directory CLI (stdin)

## Phase 5 — Error Handling (สัปดาห์ 5) · Book ch.9
- [ ] `panic!` vs `Result<T, E>`, `?` operator
- [ ] custom error enum, `impl From`, `Box<dyn Error>`
- [ ] crates: `anyhow`, `thiserror`
- [ ] 🛠 Mini: key=value config file parser พร้อม error ที่ดี

## Phase 6 — Generics, Traits, Lifetimes (สัปดาห์ 6–7) · Book ch.10
- [ ] generic fn/struct/enum
- [ ] traits, default impl, trait bounds, `impl Trait`, `where`
- [ ] std traits: `Display`, `From`/`Into`, `Iterator`, `Default`
- [ ] lifetimes `'a`, elision rules, `'static`
- [ ] 🛠 Mini: generic `Stack<T>`, `Summary` trait สำหรับหลาย type

## Phase 7 — Testing + Project #1 (สัปดาห์ 8) · Book ch.11–12
- [ ] unit tests `#[cfg(test)]`, integration tests `tests/`, doc tests
- [ ] 🛠 minigrep (Book ch.12)
- [ ] 🏆 **Project #1: `todo` CLI** — `clap` args, `serde` + JSON เก็บลงไฟล์, มี tests

## Phase 8 — Functional Rust (สัปดาห์ 9) · Book ch.13–14
- [ ] closures, `Fn`/`FnMut`/`FnOnce`
- [ ] iterators, adapters (`map`, `filter`, `fold`, `collect`), เขียน `Iterator` เอง
- [ ] Cargo: profiles, workspaces, publishing, docs comments `///`
- [ ] 🛠 refactor minigrep ด้วย iterators

## Phase 9 — Smart Pointers (สัปดาห์ 10) · Book ch.15
- [ ] `Box<T>`, `Deref`, `Drop`
- [ ] `Rc<T>`, `RefCell<T>`, interior mutability, `Weak<T>`
- [ ] 🛠 Mini: linked list, binary tree

## Phase 10 — Concurrency (สัปดาห์ 11) · Book ch.16
- [ ] `thread::spawn`, `move` closures, `join`
- [ ] channels `mpsc`, `Arc<Mutex<T>>`, `Send`/`Sync`
- [ ] crate `rayon`
- [ ] 🛠 Mini: parallel file hasher / word counter หลายไฟล์

## Phase 11 — Async (สัปดาห์ 12) · Book ch.17
- [ ] `async`/`await`, `Future`, runtime `tokio`
- [ ] `tokio::spawn`, `join!`, `select!`
- [ ] 🛠 Mini: ดึงหลาย URL พร้อมกันด้วย `reqwest`

## Phase 12 — Advanced (สัปดาห์ 13) · Book ch.18–20
- [ ] trait objects `dyn Trait` vs generics
- [ ] patterns ขั้นสูง
- [ ] advanced traits/types, macros เบื้องต้น (`macro_rules!`), `unsafe` (รู้จัก)
- [ ] 🛠 Book ch.21: multithreaded web server

## Phase 13 — 🏆 Capstone (สัปดาห์ 14–16) เลือก 1
- [ ] **A. REST API**: `axum` + `sqlx` (SQLite) + `serde` + `tracing` + tests
- [ ] **B. TUI app**: `ratatui` + `crossterm` (เช่น task manager, system monitor)
- [ ] **C. CLI tool จริง**: เช่น file organizer / log analyzer publish ลง crates.io
