mod domain;

fn main() {
    let n = 7;

    let temp = String::from("helloWorld");

    let label = if n % 2 == 0 { "even" } else { "odd" };


    // 소유권 이동에 관여를 함.
    // n이라는 변수가 있는데
    // score안의 중괄호에서 쓰면 안됨
    let score = {
        let mut base = "haha";
        base = temp.as_str();
        base
    };

    println!("{temp}, {score}");
}

