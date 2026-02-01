
# C vs Rust vs Kotlin: 다형성의 원리

## 다형성이란?

"같은 호출 방식으로 다른 동작을 실행하는 것"

```
speak() 호출
  ├─ Dog  → "멍멍!"
  ├─ Cat  → "야옹~"
  └─ Duck → "꽥꽥!"
```

타입마다 다른 함수가 실행되어야 한다. 이걸 어떻게 구현하느냐가 핵심이다.

---

## vtable이란?

**Virtual Method Table**의 줄임말. "함수 주소를 모아놓은 표"다.

### 문제 상황

```
Animal 타입의 변수 a가 있다.
a.speak()을 호출했다.
그런데 a가 Dog인지 Cat인지는 런타임에만 알 수 있다.
→ 어떤 speak()을 호출해야 하는가?
```

### 해결: 함수 주소를 표로 만들어두자

```mermaid
graph LR
    subgraph vtable_dog["Dog의 vtable"]
        DT["[0] speak → 0x1000<br/>[1] eat → 0x1004"]
    end

    subgraph vtable_cat["Cat의 vtable"]
        CT["[0] speak → 0x2000<br/>[1] eat → 0x2004"]
    end

    DT --> |"0x1000"| DS["dog_speak()<br/>멍멍!"]
    DT --> |"0x1004"| DE["dog_eat()<br/>사료 먹기"]
    CT --> |"0x2000"| CS["cat_speak()<br/>야옹~"]
    CT --> |"0x2004"| CE["cat_eat()<br/>생선 먹기"]
```

각 타입마다 vtable이 하나씩 존재한다. vtable 안에는 해당 타입의 메서드들의 **메모리 주소**가 순서대로 들어있다.

### 호출 과정

```mermaid
sequenceDiagram
    participant Code as a.speak() 호출
    participant Obj as 객체 (a)
    participant VT as vtable
    participant Fn as 실제 함수

    Code->>Obj: a가 누구야?
    Obj->>VT: 내 vtable은 여기야 (포인터)
    VT->>Fn: speak은 [0]번 = 0x1000
    Fn->>Code: dog_speak() 실행 → "멍멍!"
```

1. 객체에게 "너의 vtable 어딨어?" 물어봄
2. vtable에서 "speak은 몇 번째?" 찾음
3. 그 주소에 있는 함수를 실행

### 메모리 배치

```
Dog 객체                          Dog vtable
┌──────────────────┐             ┌──────────────────────┐
│ vtable_ptr: ─────┼────────────→│ [0] speak: 0x1000    │──→ dog_speak()
│ name: "바둑이"    │             │ [1] eat:   0x1004    │──→ dog_eat()
└──────────────────┘             └──────────────────────┘

Cat 객체                          Cat vtable
┌──────────────────┐             ┌──────────────────────┐
│ vtable_ptr: ─────┼────────────→│ [0] speak: 0x2000    │──→ cat_speak()
│ name: "나비"      │             │ [1] eat:   0x2004    │──→ cat_eat()
└──────────────────┘             └──────────────────────┘
```

핵심: **객체 안에 vtable 포인터가 숨어있다.** 이 포인터가 자기 타입의 함수 목록을 가리킨다.

### 각 언어에서 vtable을 누가 만드는가?

```mermaid
graph TD
    VT["vtable (함수 주소 표)"]

    VT --> |"C"| C_who["프로그래머가 직접 만듦<br/>struct에 함수 포인터를 넣는 게 곧 vtable"]
    VT --> |"Kotlin"| VM_who["JVM이 자동 생성<br/>클래스 로딩 시 vtable 구성"]
    VT --> |"Rust (dyn)"| Rust_who["컴파일러가 자동 생성<br/>&dyn Trait 사용 시 vtable 생김"]
    VT --> |"Rust (impl)"| Rust_no["vtable 없음!<br/>컴파일 시 함수 직접 연결"]
```

### vtable의 비용

vtable을 사용하면 함수 호출마다 **간접 참조(indirection)**가 발생한다.

```
직접 호출:     dog_speak()          → 1단계
vtable 호출:   객체 → vtable → 함수  → 2단계 (포인터 2번 따라감)
```

이 비용은 작지만, 초당 수백만 번 호출되는 코드에서는 차이가 난다.
Rust가 `&impl Trait`(정적 디스패치)를 기본으로 하는 이유가 이것이다.
vtable이 필요 없으면 아예 만들지 않는다.

---

## 전체 구조 비교

```mermaid
graph TB
    subgraph problem["문제: 다형성"]
        Q["speak()을 호출했을 때<br/>Dog은 멍멍, Cat은 야옹<br/>어떻게?"]
    end

    Q --> C
    Q --> Rust
    Q --> Kotlin_sub

    subgraph C["C - 함수 포인터"]
        C1["struct에 함수 포인터 직접 저장"]
        C2["프로그래머가 수동으로 vtable 구성"]
        C3["검증 없음 - 런타임에 터짐"]
        C1 --> C2 --> C3
    end

    subgraph Rust["Rust - Trait"]
        R1["trait으로 계약 정의"]
        R2["impl로 타입별 구현"]
        R3["컴파일러가 검증 + 정적/동적 선택"]
        R1 --> R2 --> R3
    end

    subgraph Kotlin_sub["Kotlin - Interface + 상속"]
        K1["interface/abstract class로 계약 정의"]
        K2["class에서 상속/구현"]
        K3["VM이 vtable 관리 - 항상 런타임"]
        K1 --> K2 --> K3
    end
```

---

## 각 언어의 구현 방식

### C - 함수 포인터 (수동)

```c
// 함수 포인터 테이블 = 수동 vtable
typedef struct {
    const char* name;
    void (*speak)(void);
    void (*eat)(void);
} Animal;

void dog_speak() { printf("멍멍!\n"); }
void dog_eat()   { printf("사료 먹기\n"); }

Animal dog = { "강아지", dog_speak, dog_eat };
dog.speak();  // 함수 포인터를 통해 호출
```

### Rust - Trait (컴파일러가 관리)

```rust
trait Animal {
    fn speak(&self);
    fn eat(&self);
}

struct Dog { name: String }

impl Animal for Dog {
    fn speak(&self) { println!("멍멍!"); }
    fn eat(&self)   { println!("사료 먹기"); }
}
```

### Kotlin - Interface

```kotlin
interface Animal {
    fun speak()
    fun eat()
}

class Dog(val name: String) : Animal {
    override fun speak() = println("멍멍!")
    override fun eat()   = println("사료 먹기")
}
```

---

## 메모리에서 실제로 일어나는 일

```mermaid
graph LR
    subgraph C_mem["C - 메모리"]
        CS["Dog struct"]
        CS --> |"speak: 0x1000"| CF1["dog_speak()"]
        CS --> |"eat: 0x1004"| CF2["dog_eat()"]
    end

    subgraph Rust_static["Rust 정적 디스패치 - 메모리"]
        RS["Dog struct"]
        RS --> |"컴파일 시 직접 연결"| RF1["dog_speak() 직접 호출"]
    end

    subgraph Rust_dynamic["Rust 동적 디스패치 - 메모리"]
        RD["Dog 참조"]
        RD --> |"vtable 포인터"| RV["vtable"]
        RV --> RDF1["dog_speak()"]
        RV --> RDF2["dog_eat()"]
    end

    subgraph Kotlin_mem["Kotlin - 메모리"]
        KS["Dog 객체"]
        KS --> |"클래스 메타데이터"| KV["vtable (JVM 관리)"]
        KV --> KF1["dog_speak()"]
        KV --> KF2["dog_eat()"]
    end
```

---

## 디스패치 방식 비교

```mermaid
graph TD
    Call["animal.speak() 호출"] --> How{"어떻게 함수를 찾는가?"}

    How --> |"C"| C_way["함수 포인터 주소를 읽어서<br/>해당 주소로 점프<br/>(항상 런타임)"]

    How --> |"Rust (기본)"| Rust_static["컴파일 타임에 타입 확정<br/>→ 함수 직접 호출 코드 생성<br/>(비용 0, monomorphization)"]

    How --> |"Rust (dyn)"| Rust_dynamic["vtable에서 함수 주소 조회<br/>→ 해당 주소로 점프<br/>(런타임, C와 동일)"]

    How --> |"Kotlin"| VM_way["JVM이 객체의 클래스 메타데이터에서<br/>vtable 조회 → 함수 호출<br/>(항상 런타임)"]

    C_way --> Runtime["런타임 비용 있음"]
    Rust_dynamic --> Runtime
    VM_way --> Runtime
    Rust_static --> Zero["런타임 비용 없음"]
```

---

## Rust의 정적 vs 동적 디스패치

```mermaid
graph TD
    Q{"다형성이 필요한가?"} --> |"YES"| B{"컴파일 타임에<br/>타입을 알 수 있나?"}
    Q --> |"NO"| A["struct + impl 만으로 충분"]

    B --> |"YES"| Static["&impl Trait (정적 디스패치)<br/>컴파일러가 타입별 함수를 각각 생성<br/>비용 0"]
    B --> |"NO"| Dynamic["&dyn Trait (동적 디스패치)<br/>런타임에 vtable 조회<br/>C/Kotlin과 동일한 비용"]

    Static --> Ex1["예: 제네릭 함수<br/>fn notify(item: &impl Summary)"]
    Dynamic --> Ex2["예: 여러 타입을 한 컬렉션에 담기<br/>Vec&lt;Box&lt;dyn Animal&gt;&gt;"]
```

---

## 안전성 비교

```mermaid
graph TD
    subgraph C_safe["C"]
        C_Q{"함수 포인터에<br/>엉뚱한 함수 넣으면?"}
        C_Q --> C_A["컴파일 됨 ✅"]
        C_A --> C_B["런타임에 터짐 💥"]
    end

    subgraph Rust_safe["Rust"]
        R_Q{"trait 메서드를<br/>빼먹으면?"}
        R_Q --> R_A["컴파일 에러 ❌"]
        R_A --> R_B["실행 자체가 안 됨<br/>→ 안전"]
    end

    subgraph Kotlin_safe["Kotlin"]
        K_Q{"interface 메서드를<br/>빼먹으면?"}
        K_Q --> K_A["컴파일 에러 ❌"]
        K_A --> K_B["실행 자체가 안 됨<br/>→ 안전"]
    end
```

---

## 상속 vs 합성

```mermaid
graph TD
    subgraph Kotlin_inherit["Kotlin - 상속 기반"]
        KA["open class Animal"] --> |"extends"| KD["class Dog : Animal"]
        KA --> |"extends"| KC["class Cat : Animal"]
        KI["interface Speakable"] --> |"implements"| KD
        KI --> |"implements"| KC
        KNote["데이터 + 행동이 클래스 안에 묶임<br/>부모의 구현을 물려받음<br/>다이아몬드 문제 발생 가능"]
    end

    subgraph Rust_compose["Rust - 합성 기반"]
        RS1["struct Dog (데이터만)"]
        RS2["struct Cat (데이터만)"]
        RT1["trait Animal (행동 계약)"]
        RT2["trait Pet (행동 계약)"]
        RT1 --> |"impl for"| RS1
        RT1 --> |"impl for"| RS2
        RT2 --> |"impl for"| RS1
        RT2 --> |"impl for"| RS2
        RNote["데이터와 행동이 분리됨<br/>상속 없음, trait을 자유롭게 조합<br/>다이아몬드 문제 없음"]
    end
```

---

## 최종 비교표

| | C | Rust | Kotlin |
|---|---|---|---|
| **다형성 도구** | 함수 포인터 | trait | interface + 상속 |
| **vtable 관리** | 프로그래머 | 컴파일러 | JVM |
| **컴파일 타임 검증** | 없음 | 있음 | 있음 |
| **정적 디스패치** | 수동으로 가능 | `&impl Trait` | 없음 (JIT가 최적화) |
| **동적 디스패치** | 항상 | `&dyn Trait` | 항상 |
| **상속** | 없음 | 없음 | 있음 |
| **런타임 비용** | 함수 포인터 조회 | 선택 가능 (0 또는 조회) | JVM 오버헤드 |
| **메모리 관리** | 수동 (malloc/free) | 소유권 시스템 (자동) | GC |

---

## 한 줄 요약

> **밑바닥은 전부 vtable(함수 포인터 테이블)이다.**
>
> - **C**: 직접 만들고 직접 관리. 빠르지만 위험.
> - **Kotlin**: JVM이 만들고 JVM이 관리. 안전하지만 항상 런타임 비용.
> - **Rust**: 컴파일러가 만들고 검증. 안전하면서 비용도 선택 가능 (정적이면 0).
