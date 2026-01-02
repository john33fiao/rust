use std::io; // 표준 입출력 라이브러리
use rand::Rng; // 난수 생성 라이브러리

fn main() {
    println!("업다운 게임");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 1부터 100까지 난수 생성

    println!("비밀 숫자: {secret_number}"); // 디버깅용, 나중에 지울 것

    println!("1부터 100까지 숫자 중 하나를 맞춰보세요!");

    let mut guess = String::new(); // 변경 가능한 변수는 mut을 쓴다
    // 러스트에서 변수는 기본적으로 불변을 가정함

    // 얘는 io::stdin().readline(&mut guess).expect("잘못된 입력입니다."); 로도 표현 가능
    io::stdin()
        .read_line(&mut guess) // 임시저장 및 result 반환(열거형)
        .expect("잘못된 입력입니다."); // Err 반환하면 경고문구 표시 후 종료
        // expect는 Result 열거형의 메서드
        // expect 안쓰면 컴파일러에서 경고 발생 (컴파일은 되지만)
    
    println!("당신이 추측한 숫자: {guess}"); // {}는 플레이스홀더, 변수 값 출력

    // 자리표시자는 이렇게도 사용가능

    // println!("자리표시자 사용 예시:");

    // let x = 5;
    // let y = 10;
    // println!("x = {x}, y = {}, x + y = {}", y, x + y); // 근데 보통 이렇게 두 개 이상이면 헷갈리니까 잘 안쓰는듯
}
