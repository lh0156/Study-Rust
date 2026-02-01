struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// trait 정의
trait Animal {
    fn speak(&self);
}

// Dog에 Animal 구현
impl Animal for Dog {
    fn speak(&self) {
        println!("{}: 멍멍!", self.name);
    }
}

// Cat에 Animal 구현
impl Animal for Cat {
    fn speak(&self) {
        println!("{}: 야옹~", self.name);
    }
}

// &dyn Animal → 런타임에 vtable을 조회해서 speak()을 찾음
fn interact(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let dog = Dog { name: String::from("바둑이") };
    let cat = Cat { name: String::from("나비") };

    // 같은 함수에 다른 타입을 넘김
    interact(&dog); // Dog vtable → dog_speak() → "바둑이: 멍멍!"
    interact(&cat); // Cat vtable → cat_speak() → "나비: 야옹~"

    // dyn의 진짜 장점: 다른 타입을 한 컬렉션에 담기
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    println!("\n--- 전체 동물 ---");
    for animal in &animals {
        animal.speak(); // 각각의 vtable을 따라감
    }
}
