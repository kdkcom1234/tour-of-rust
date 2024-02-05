fn main() {
  // 타입 추론
  let x = 13;
  println!("{}", x);

  // 변수 숨김(variable shadowing)
  // snake_case로 작성
  // 자료형 명시
  let x: f64 = 3.14159;
  println!("{}", x);

  let x;
  x = 0;
  println!("{}", x);
}