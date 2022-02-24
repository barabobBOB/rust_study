use std::io; // 프로그램의 범위
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫지를 맞춰봅시다!");
    let secret_number = rand::thread_rng().gen_range(1,11);
    //println!("사용자가 맞춰야할 숫자 : {}", secret_number);
    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요 -> ");
        let mut guess = String::new();
        // ::문법 -> new라는 함수가 String타입에 연관함수
        // 시작을 std::io해서 굳이 use를 안해줘도 ㄱㅊ
        io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("올바른 값을 입력해주세요");
                continue;
            }
        };
        // trim 양쪽 공백문자 제거 왜냐? 입력시 ex) 5/n 처럼 되니까
        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다."),
            Ordering::Greater => println!("입력한 숫자가 큽니다."),
            Ordering::Equal => {
                println!("정답");
                break;
            },
        }
    }
    // println!("사용자가 맞춰야할 숫자 : {}", secret_number);
}

/*
1. 러스트에서의 들여쓰기는 탭 1번이 아니라 공백문자 4개 이용함
2. println! -> 러스트 매크로 라고 부름 지금은 ! 기호를 보면 함수가 아닌 매크로를 호출한다는 사실만 알아두자
3. Hello world 를 println! 매크로의 인수로 전달되서 화면에 출력
4. 세미콜론으로 끝남

rust 도 C C++ 마냥 GCC로 파일을 실행하고 바이너리 파일이 생성되듯이
러스트도 rustc main.rs를 실행하면 main 이라는 바이너리 파일이 생성 ./main을 실행하면
main.rs 에서 만들 파일을 컴파일한 바이너리 파일이 실행되는것이다
 */
//mut를 붙혀야 가변변수
