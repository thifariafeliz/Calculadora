mod combinatoria;
use combinatoria::*;
mod matrizes;
use matrizes::*;
use std::io::{self, Write};

pub fn pega_inteiro() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Falha na leitura do input do usuário.");

        if let Ok(num) = input.trim().parse::<i32>() {
            return num;
        } else {
            println!("Não foi possível converter para número. Digite um valor inteiro válido:");
        }
    }
}

fn main() {
    loop {
        println!("\nEscolha a operação que deseja fazer:");
        println!("1. Fatorial\n2. Arranjo\n3. Binomial\n4. Combinação\n5. Permutação simples\n6. Permutação com repeticões
7. Adição de matrizes\n8. Subtração de matrizes\n9. Multiplicação de matrizes\n10. Determinante de matrizes\n11. Triângulo de Pascal\n0. Sair");
        let input: i32 = pega_inteiro();

        match input {
            0 => {
                break;
            },

            1 => {
                println!("\n======= FATORIAL =======");
                println!("Digite um número para ver seu fatorial:");
                let numero: i32 = pega_inteiro();

                if numero < 0 {
                    println!("Não é possível obter o fatorial de um número negativo. Tente novamente.");
                    continue;
                }

                let fat: i32 = fatorial(numero);
                
                println!("O fatorial de {} é = {}", numero, fat);
            },

            2 => {
                println!("\n======= ARRANJO =======");
                println!("Digite a quantidade de elementos que deseja arranjar:");
                let elementos: i32 = pega_inteiro();
                println!("Digite a quantidade de lugares em que os números serão arranjados:");
                let lugares: i32 = pega_inteiro();

                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para elementos e lugares no arranjo. Tente novamente.");
                    continue;
                } else if lugares > elementos {
                    println!("O número de lugares não pode ser maior que o número de elementos no arranjo. Tente novamente.");
                    continue;
                }

                let arrj: i32 = arranjo(elementos, lugares);
                println!("O arranjo de {} elementos em {} lugares é = {}", elementos, lugares, arrj);
            },

            3 => {
                println!("\n======= BINOMIAL =======");
                println!("Digite a ordem do número binomial:");
                let n: i32 = pega_inteiro();
                println!("Digite a classe do número binomial:");
                let k: i32 = pega_inteiro();

                if n < 1 || k < 1 {
                    println!("Valores menores que 1 não são válidos para ordem ou classe de um número binomial. Tente novamente.");
                    continue;
                } else if k > n {
                    println!("O número da classe não pode ser maior que o número da ordem no número binoial. Tente Novamente.");
                    continue;
                }

                let binomial: i32 = binomial(n, k);
                println!("O número binomial de ordem {} e classe {} é = {}", n, k, binomial);    
            },

            4 => {
                println!("\n======= COMBINAÇÃO =======");
                println!("Digite o número de elementos a serem combinados:");
                let elementos: i32 = pega_inteiro();
                println!("Digite o número de lugares em que os elementos serão combinados:");
                let lugares: i32 = pega_inteiro();

                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para o número de elementos ou lugares. Tente novamente.");
                    continue;
                } else if lugares > elementos {
                    println!("O número de lugares não pode ser maior que o núemro de elementos na combinação. Tente novamente.");
                    continue;
                }

                let combinacao: i32 = combinacao(elementos, lugares);
                   println!("O número de combinações possíveis com {} elementos tomados de {} em {} é = {}", elementos, lugares, lugares, combinacao);

            },

            5 => {
                println!("\n======= PERMUTAÇÃO SIMPLES =======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: i32 = pega_inteiro();

                if elementos < 1 {
                    println!("Valor menor 1 não é válido para o número de elementos na permutação. Tente novamente.");
                    continue;
                }

                let permutacao = permutacao_simples(elementos);
                println!("O número de permutações possíveis com {} elementos é = {}", elementos, permutacao);
            },

            6 => {
                println!("\n======= PERMUTAÇÃO COM REPETIÇÃO =======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: i32 = pega_inteiro();
                println!("Digite a quantidade de elementos que se repetem:");
                let quant: i32 = pega_inteiro();

                if quant > elementos {
                    println!("Não é possível que a quantidade de elementos que se repetem seja maior que a quantidade de elementos total. Tente novamente.");
                    continue;
                }

                let mut repeticoes: Vec<i32> = Vec::new();  // cria um vetor para armazenar valores i32
                let mut soma_repeticoes: i32 = 0;

                if elementos < 1 || quant < 1{
                    println!("Valores menores que 1 não são válidos para elementos, quantidade de repetições e repetições. Tente novamente.");
                    continue;
                }

                // Iteração para pegar números e enfiá-los no vetor.
                for i in 1..=quant {
                    println!("Digite a quantidade de repetições do elemento {}:", i);
                    repeticoes.push(pega_inteiro());
                }

                for i in 0..repeticoes.len() {
                    soma_repeticoes += repeticoes[i];
                }

                if soma_repeticoes > elementos {
                    println!("Não é possível que o total de repetições seja maior que o número de elementos. Tente novamente.");
                    continue;
                }
                
                let permutacao: i32 = permutacao_repeticao(elementos, repeticoes);

                println!("A quantidade de permutações de {} elementos {} elementos repetidos é = {}", elementos, quant, permutacao);
            },

            7 => {
                println!("\n======= ADIÇÃO DE MATRIZES =======");
                println!("Digite o número de linhas:");
                let linhas: i32 = pega_inteiro();
                println!("Digite o número de colunas:");
                let colunas: i32 = pega_inteiro();

                if linhas < 1 || colunas < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                    continue;
                }

                let mut mat1: Vec<Vec<i32>> = vec![vec![0; colunas as usize]; linhas as usize];
                let mut mat2: Vec<Vec<i32>> = vec![vec![0; colunas as usize]; linhas as usize];

                println!("\nDigite os valores da primeira matriz:");
                for i in 0..linhas {
                    for j in 0..colunas {
                        println!("mat1[{}][{}] =", i, j);
                        mat1[i as usize][j as usize] = pega_inteiro();
                    }
                }
 ..
                println!("\nDigite os valores da segunda matriz:");
                for i in 0..linhas {
                    for j in 0..colunas {
                        println!("mat2[{}][{}] =", i, j);
                        mat2[i as usize][j as usize] = pega_inteiro();
                    }
                }

                let resultado = adicao_matrizes(mat1, mat2);

                if resultado.is_empty() {
                    println!("Erro, resultado voltou um vetor vazio. Tente novamente.");
                    continue;
                }
                
                println!("\nResultado da adição:");
                for i in 0..linhas {
                    for j in 0..colunas {
                        print!("{} ", resultado[i as usize][j as usize]);
                        io::stdout().flush().unwrap();
                    }
                    println!();
                }
            },

            8 => { 
                println!("\n======= SUBTRAÇÃO DE MATRIZES =======");
                println!("Digite o número de linhas:");
                let linhas: i32 = pega_inteiro();
                println!("Digite o número de colunas:");
                let colunas: i32 = pega_inteiro();

                if linhas < 1 || colunas < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                }

                let mut mat1: Vec<Vec<i32>> = vec![vec![0; colunas as usize]; linhas as usize];
                let mut mat2: Vec<Vec<i32>> = vec![vec![0; colunas as usize]; linhas as usize];

                println!("\nDigite os valores da primeira matriz:");
                for i in 0..linhas {
                    for j in 0..colunas {
                        println!("mat1[{}][{}] =", i, j);
                        mat1[i as usize][j as usize] = pega_inteiro();
                    }
                }

                println!("\nDigite os valores da segunda matriz:");
                for i in 0..linhas {
                    for j in 0..colunas {
                    println!("mat2[{}][{}] =", i, j);
                    mat2[i as usize][j as usize] = pega_inteiro();
                    }
                }               

                let resultado = subtracao_matrizes(mat1, mat2);
                
                if resultado.is_empty() {
                    println!("Erro, resultado voltou um vetor vazio. Tente novamente.");
                    continue;
                }

                println!("\nResultado da subtração:");
                for i in 0..linhas {
                    for j in 0..colunas {
                        print!("{} ", resultado[i as usize][j as usize]);
                        io::stdout().flush().unwrap();
                    }
                    println!();
                }
            },

            9 => {
                println!("\n======= MULTIPLICAÇÃO DE MATRIZES =======");

                println!("Digite o número de linhas da primeira matriz:");
                let l1: i32 = pega_inteiro();
                println!("Digite o número de colunas da primeira matriz:");
                let c1: i32 = pega_inteiro();

                println!("Digite o número de linhas da segunda matriz:");
                let l2: i32 = pega_inteiro();
                println!("Digite o número de colunas da segunda matriz:");
                let c2: i32 = pega_inteiro();

                if l1 < 1 || c1 < 1 || l2 < 1 || c2 < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                    continue;
                }


                let mut mat1: Vec<Vec<i32>> = vec![vec![0; c1 as usize]; l1 as usize];
                let mut mat2: Vec<Vec<i32>> = vec![vec![0; c2 as usize]; l2 as usize];

                println!("\nDigite os valores da primeira matriz:");
                for i in 0..l1 {
                    for j in 0..c1 {
                        println!("mat1[{}][{}] =", i, j);
                        mat1[i as usize][j as usize] = pega_inteiro();
                    }
                }

                println!("\nDigite os valores da segunda matriz:");
                for i in 0..l2 {
                    for j in 0..c2 {
                        println!("mat2[{}][{}] =", i, j);
                        mat2[i as usize][j as usize] = pega_inteiro();
                    }
                }

                let resultado = multiplicacao_matrizes(mat1, mat2);

                if resultado.is_empty() {
                    println!("Erro, resultado voltou um vetor vazio. Tente novamente.");
                    continue;
                }

                println!("\nResultado da multiplicação:");
                for i in 0..l1 {
                    for j in 0..c2 {
                        print!("{} ", resultado[i as usize][j as usize]);
                        io::stdout().flush().unwrap();
                    }
                    println!();
                }
            },

            10 => {
                println!("\n======= DETERMINANTE DE MATRIZES =======");

                let mut mat1: Vec<Vec<i32>> = vec![vec![0; 2]; 2];

                println!("\nDigite os valores da matriz:");
                for i in 0..2 {
                    for j in 0..2 {
                        println!("mat1[{}][{}] =", i, j);
                        mat1[i as usize][j as usize] = pega_inteiro();
                    }
                }

                let resultado = determinante_matrizes(mat1);



                println!("\nDeterminante da Matriz = {}", resultado);
            },

            _ => { 
                println!("Opção inválida. Tente novamente.");
                continue;
            }
        }
    }

    println!("Até mais!");
}
