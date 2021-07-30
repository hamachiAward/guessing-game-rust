use std::io;
use std::cmp::Ordering; //比較
use rand::Rng; //ランダム

fn main() {
    println!("Guess the number!");

    //秘密の数字を生成
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    let mut count = 0;

    //ユーザが何度も予想できるようにループ
    loop {
        println!("Please input your guess.");

        //予想の変数を作成・入力
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //予想の変数の余分な部分（空白・改行）を削除、数値に変換
        let guess: u32 = match guess.trim().parse() {
            //Ok値が返る：成功
            Ok(num) => num,
            //数値の変換に失敗してErr値が返る：無視してloopの次のステップにスキップ
            //(_)の_は包括値：すべてのErr値にマッチ
            Err(_) => continue,
        };

        count += 1;
        println!("You guessed: {}", guess);

        //結果発表
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                //試合終了：ループから抜ける
                println!("You win!");
                break;
            }
        }
    }
    println!("{}回目で正解", count);
}
