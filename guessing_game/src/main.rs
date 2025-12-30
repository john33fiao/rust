use std::io; // 표준 입출력 라이브러리

fn main() {
    println!("업다운 게임");

    println!("1부터 100까지 숫자 중 하나를 맞춰보세요!");

    let mut guess = String::new(); // 변경 가능한 변수는 mut을 쓴다
    // 러스트에서 변수는 기본적으로 불변을 가정함

    io::stdin()
        .read_line(&mut guess)
        .expect("잘못된 입력입니다.");
    
    println!("당신이 추측한 숫자: {guess}");
}
