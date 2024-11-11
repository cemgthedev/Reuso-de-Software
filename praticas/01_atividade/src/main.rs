use std::io;

/* 
* Função que faz a media dos números positivos de um vetor
*/
fn media_positivos(vetor: [i32; 10]) -> Option<f32> {
    let mut soma: i32 = 0;

    for i in vetor.iter() {
        if *i > 0 {
            soma += *i;
        }
    }

    let lenght: f32 = vetor.len() as f32;

    if soma == 0 {
        None
    } else {
        Some(soma as f32 / lenght)
    }
}

/* 
* Função que faz o produto dos números pares de um vetor
*/
fn produto_pares(vetor: [i32; 10]) -> i32 {
    let mut prod: i32 = 1;

    for i in vetor.iter() {
        if *i % 2 == 0 {
            prod *= *i;
        }
    }

    if prod == 1 {
        0
    } else {
        prod
    }
}

/* 
* Função que exibe a questão 1
*/
fn questao_01_main() {
    let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];

    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {}", media),
        None => println!("Não há números positivos."),
    }

    let produto = produto_pares(numeros);
    println!("Produto dos números pares: {}", produto);
}

/* 
* Função auxiliar para ler um inteiro
*/
fn ler_inteiro() -> i32 {
    let mut input = String::new();
    let scanner = io::stdin();
    scanner.read_line(&mut input).expect("Failed to read line");
    
    let number:i32 = input.trim().parse().expect("Please type a number!");

    number
}

/* 
* Função que analisa uma tupla retornando uma tupla com a soma, o maior e o menor valor
*/
fn analisar_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    let mut maior:i32 = tupla.0;
    let mut menor:i32 = tupla.0;
    let mut sum:i32 = 0;
    
    // os valores da tupla foram adicionados a um vetor para utilização do método iter
    for i in [tupla.0, tupla.1, tupla.2].iter() {
        if *i > maior {
            maior = *i; // termina o loop com o maior valor
        } else if *i < menor {
            menor = *i; // termina o loop com o menor valor
        }

        sum += *i; // termina o loop com a soma dos valores
    }

    (sum, maior, menor)
}

/* 
* Função que exibe a questão 2
*/
fn questao_02_main() {
    let mut tuples: (i32, i32, i32) = (0, 0, 0);
    
    println!("Digite o primeiro valor da tupla:");
    tuples.0 = ler_inteiro();
    println!("Digite o segundo valor da tupla:");
    tuples.1 = ler_inteiro();
    println!("Digite o terceiro valor da tupla:");
    tuples.2 = ler_inteiro();
    
    let result:(i32, i32, i32) = analisar_tupla(tuples);

    println!("Tupla: {:?}\nSoma: {}\nMaior: {}\nMenor: {}", result, result.0, result.1, result.2);
}

/* 
* Função principal de menu onde o usuário escolhe qual questão deseja
*/
fn main() {
    let scanner = io::stdin();

    loop {
        let mut input = String::new();

        println!("Digite uma das opções:\n1 - Questão 1\n2 - Questão 2\n3 - Sair");
        scanner
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        match input.as_str() {
            "1" => questao_01_main(),
            "2" => questao_02_main(),
            "3" => break,
            _ => println!("Opção inválida"),
        }

    }
}