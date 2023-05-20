use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
  left + right
}

async fn something_great_async_function() -> i32 {
  // let ans = async_add(2, 3).await;
  // println!("{}", ans);
  // ans
  let ans1 = async_add(2, 3).await;
  let ans2 = async_add(3, 4).await;
  let ans3 = async_add(4, 5).await;
  let result = ans1 + ans2 + ans3;
  result
}

fn main() {
  executor::block_on(something_great_async_function());
}
