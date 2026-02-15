# JSON Parser (from scratch)

학습용으로 외부 라이브러리 없이 JSON 토크나이저와 파서를 직접 구현합니다. 이 패키지는 템플릿만 제공하며 실제 구현은 스스로 진행합니다.

## 구현 순서
- Tokenizer부터: 공백 -> 구두점 -> 키워드 -> 문자열 -> 숫자 -> EOF
- Parser: `parse_value` 구현 -> `array` -> `object` -> 마지막 `EOF` 검사
- 테스트: 성공 케이스 먼저, 이후 실패 케이스(에러 위치 포함) 추가

## 모듈 구조 제안
- `src/error.rs` — 에러 타입(`Error`)과 `Result<T>` 별칭
- `src/token.rs` — `Token` 열거형, `Tokenizer` 구조체(`next_token`)와 `tokenize`
- `src/value.rs` — `JsonValue`(Null, Bool, Number, String, Array, Object)
- `src/parser.rs` — 재귀 하강 파서(`parse`, `Parser`, `parse_value/object/array`)
- `src/lib.rs` — 위 모듈을 `pub`으로 노출 및 최종 API 정리

## Tokenizer 가이드
- 공백 스킵: space, tab, CR, LF 를 전부 무시 (토큰 직전마다 호출)
- 구두점: `{ } [ ] : ,` 각각 일치하는 토큰 반환
- 키워드: `null`, `true`, `false` 정확히 매칭
- 문자열(`"..."`):
  - 이스케이프 처리: `\\" \\\\ \\/ \\b \\f \\n \\r \\t`
  - 유니코드: `\\uXXXX` (16진 4자리). 선택(고급): 서러게이트 페어 결합
  - 닫는 따옴표 없으면 `UnexpectedEOF`
- 숫자 패턴: `-? (0|[1-9][0-9]*) (\\.[0-9]+)? ([eE][+-]?[0-9]+)?`
  - 선행 0 금지(정수부가 0이면 단독 0만 허용)
  - 소수점 뒤/지수 뒤는 최소 1자리 필요
  - 매칭 슬라이스를 `f64`로 파싱, 실패 시 `InvalidNumber`
- EOF: 입력 소진 시 `Token::EOF`
- 위치(pos): 바이트 인덱스로 관리, 에러에 포함

## Parser 가이드
- Grammar
  - value := object | array | string | number | true | false | null
  - object := `{` (string `:` value) (`,` string `:` value)* `}` | `{` `}`
  - array  := `[` value (`,` value)* `]` | `[` `]`
- `parse_value`에서 `peek()`로 분기하고 해당 토큰 소비(`bump()`)
- `object/array`는 첫 토큰 소비 후 빈 케이스(`}` 또는 `]`) 처리
- 각 구문에서 예상 토큰이 없으면 `ExpectedToken { expected, pos }`
- 최종적으로 값 하나 파싱 후 `EOF`만 남아야 하며, 아니면 `TrailingCharacters`

## 테스트 전략
- Tokenizer 단위 테스트: 구두점/키워드, 문자열 이스케이프/유니코드, 숫자 변형
- Parser 통합 테스트: 원시 타입/배열/객체/공백, 실패 케이스(trailing, 닫힘 누락 등)
- 작은 입력부터 `cargo test -q`로 반복

## 시작하는 법
1. 각 모듈 파일을 생성하고(예: `src/token.rs`) `lib.rs`에서 `pub mod token;` 식으로 노출
2. Tokenizer를 먼저 완성하고 간단한 테스트 작성
3. Parser를 구현하고 통합 테스트 작성
4. 필요하면 `examples/`에 샘플 추가 후 `cargo run --example your_example`

행운을 빕니다! 문제가 생기면 실패 케이스 입력과 현재 토큰/위치 정보를 출력해 디버깅하세요.
