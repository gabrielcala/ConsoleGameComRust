use std::io;
//let texto_estatico: &str = "Isso é um texto imutável";
//let mut texto_mutavel: String = String::from("Isso é um texto mutável");
fn main() {
    let mut score: i32 = 0;

    loop {
        println!("Bem-vindo à floresta misteriosa");

        println!("Por favor choise uma opção:");
        println!("1 - Entrar na caverna escura");
        println!("2 - Seguir no caminho iluminado");
        println!("3 - Cruzar a ponte frágil");
        println!("4 - Descansar na beira do riacho");

        let mut player_choise: String = String::new();

        let _ =io::stdin().read_line(&mut player_choise);

        let choise: i32 = match player_choise.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if choise == 1 {
            println!("Você entrou na caverna escura e econtrou um tesouro parabéns!");

            score += 30;
            println!("seu score atual é: {}", score);
        } else if choise == 2 {
            println!("Você encontrou um Ogro poderoso, mas com sorte conseguiu escapar!");

            score -= 20;
            println!("seu score atual é: {}", score);
        } else if choise == 3 {
            println!("A ponte se quebrou com sorte você conseguiu nadar de volta para a margem!");

            score += 20;
            println!("seu score atual é: {}", score);
        } else if choise == 4 {
            println!("Você conseguiu recuperar um pouco das suas forças!");

            score += 10;
            println!("seu score atual é: {}", score);
        }

        if score >= 100 {
            println!("Parabéns você é um verdadeiro aventureiro!");
            break;
        } else if score <= 0 {
            println!("Que pena você perdeu");
            break;
        }
    }

    println!("Obrigado por jogar 'A floresta misteriosa!'");
}
