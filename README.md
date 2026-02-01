# Study-Rust

Rust 학습 저장소 (The Rust Programming Language 기반)

---

## 프로젝트 구조

```
Study-Rust/
├── docs/
│   ├── why-rust.md           # Rust 탄생 배경, 실행 방식 비교, Java/Kotlin과 차이
│   └── memory-management.md  # 메모리 영역 구조, String Pool, 힙 정리 방식
├── hello_world/              # rustc로 직접 컴파일하는 예제
├── hello_cargo/              # cargo new로 생성한 프로젝트
└── guessing_game/            # 숫자 맞추기 게임 (Chapter 2)
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## Cargo (빌드 도구)

Cargo는 Rust의 빌드 시스템 + 패키지 매니저이다. Java/Kotlin의 **Gradle**에 해당한다.

| Java/Kotlin | Rust |
|---|---|
| Gradle | Cargo |
| `build.gradle` | `Cargo.toml` |
| Maven Central | crates.io |
| `./gradlew build` | `cargo build` |
| `./gradlew run` | `cargo run` |
| `./gradlew test` | `cargo test` |

### 저장소 관리 주체

- Maven Central → Sonatype (미국 회사)
- crates.io → Rust Foundation (AWS, Google, Microsoft 등이 후원하는 비영리 재단)

### 로컬 캐시 경로 (macOS)

- Maven → `~/.m2/repository/`
- Cargo → `~/.cargo/registry/`

---

## 기본 문법

### 변수 선언

```rust
let x = 5;       // 불변 (Kotlin의 val)
let mut y = 5;   // 가변 (Kotlin의 var)
```

### 함수 선언

```rust
fn main() {}     // fn = 함수 선언 키워드 (Kotlin의 fun)
```

### 매크로 vs 함수

```rust
println!("hello")  // ! 있으면 매크로
// println("hello") // ! 없으면 함수 (이 함수는 존재하지 않음)
```

---

## 학습 노트

- [Rust는 왜 만들어졌는가?](docs/why-rust.md) — 실행 방식 비교, OS 접근 구조, Java/Kotlin 대비 장점
- [메모리 관리](docs/memory-management.md) — 메모리 영역 구조, 데이터/스택/힙 차이, String Pool이 없는 이유
