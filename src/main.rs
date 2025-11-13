mod combinatoria;
use combinatoria::*;
mod matrizes;
use matrizes::*;
mod utils;
use utils::*;

fn main() {
    loop {
        println!("\nEscolha a operação que deseja fazer:");
        println!("1. Fatorial\n2. Arranjo\n3. Binomial\n4. Combinação\n5. Permutação simples\n6. Permutação com repeticões
7. Adição de matrizes\n8. Subtração de matrizes\n9. Multiplicação de matrizes\n10. Determinante de matrizes\n11. Triângulo de Pascal\n0. Sair");
        let input: u8 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
            num as u8
        } else {
            continue;
        };

        match input {
            0 => {
                break;
            },

            1 => {
                println!("\n======= FATORIAL =======");
                println!("Digite um número para ver seu fatorial:");
                let numero: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                let fat: u128 = fatorial(numero);
                
                println!("O fatorial de {} é = {}", numero, fat);
            },

            2 => {
                println!("\n======= ARRANJO =======");
                println!("Digite a quantidade de elementos que deseja arranjar:");
                let elementos: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if elementos > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                println!("Digite a quantidade de lugares em que os números serão arranjados:");
                let lugares: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if lugares > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para elementos e lugares no arranjo. Tente novamente.");
                    continue;
                } else if lugares > elementos {
                    println!("O número de lugares não pode ser maior que o número de elementos no arranjo. Tente novamente.");
                    continue;
                }

                let arrj: u128 = arranjo(elementos, lugares);
                println!("O arranjo de {} elementos em {} lugares é = {}", elementos, lugares, arrj);
            },

            3 => {
                println!("\n======= BINOMIAL =======");
                println!("Digite a ordem do número binomial:");
                let n: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if n > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                println!("Digite a classe do número binomial:");
                let k: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if k > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                if n < 1 || k < 1 {
                    println!("Valores menores que 1 não são válidos para ordem ou classe de um número binomial. Tente novamente.");
                    continue;
                } else if k > n {
                    println!("O número da classe não pode ser maior que o número da ordem no número binoial. Tente Novamente.");
                    continue;
                }

                let binomial: u128 = binomial(n, k);
                println!("O número binomial de ordem {} e classe {} é = {}", n, k, binomial);    
            },

            4 => {
                println!("\n======= COMBINAÇÃO =======");
                println!("Digite o número de elementos a serem combinados:");
                let elementos: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if elementos > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                println!("Digite o número de lugares em que os elementos serão combinados:");
                let lugares: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if lugares > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para o número de elementos ou lugares. Tente novamente.");
                    continue;
                } else if lugares > elementos {
                    println!("O número de lugares não pode ser maior que o núemro de elementos na combinação. Tente novamente.");
                    continue;
                }

                let combinacao: u128 = combinacao(elementos, lugares);
                   println!("O número de combinações possíveis com {} elementos tomados de {} em {} é = {}", elementos, lugares, lugares, combinacao);

            },

            5 => {
                println!("\n======= PERMUTAÇÃO SIMPLES =======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if elementos > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                } else if elementos < 1 {
                    println!("Valor menor 1 não é válido para o número de elementos na permutação. Tente novamente.");
                    continue;
                }

                let permutacao: u128 = permutacao_simples(elementos);
                println!("O número de permutações possíveis com {} elementos é = {}", elementos, permutacao);
            },

            6 => {
                println!("\n======= PERMUTAÇÃO COM REPETIÇÃO =======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if elementos > 34 {
                    println!("Não é possível fazer o fatorial de números maiores que 34 neste programa. Tente novamente.");
                    continue;
                }

                println!("Digite a quantidade de elementos que se repetem:");
                let quant: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if quant > elementos {
                    println!("Não é possível que a quantidade de elementos que se repetem seja maior que a quantidade de elementos total. Tente novamente.");
                    continue;
                }

                let mut repeticoes: Vec<u128> = Vec::new();  // cria um vetor para armazenar valores i32
                let mut soma_repeticoes: u128 = 0;

                if elementos < 1 || quant < 1{
                    println!("Valores menores que 1 não são válidos para elementos, quantidade de repetições e repetições. Tente novamente.");
                    continue;
                }

                // Iteração para pegar números e enfiá-los no vetor.
                for i in 1..=quant {
                    println!("Digite a quantidade de repetições do elemento {}:", i);
                    let numero: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                        num
                    } else {
                        continue;
                    };
                    repeticoes.push(numero);
                }

                for i in 0..repeticoes.len() {
                    soma_repeticoes += repeticoes[i];
                }

                if soma_repeticoes > elementos {
                    println!("Não é possível que o total de repetições seja maior que o número de elementos. Tente novamente.");
                    continue;
                }

                println!("{soma_repeticoes}");
                
                let permutacao: u128 = permutacao_repeticao(elementos, repeticoes);

                println!("A quantidade de permutações de {} elementos {} elementos repetidos é = {}", elementos, quant, permutacao);
            },

            7 => {
                println!("\n======= ADIÇÃO DE MATRIZES =======");
                println!("Digite o número de linhas:");
                let linhas: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };
                println!("Digite o número de colunas:");
                let colunas: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if linhas < 1 || colunas < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                continue;
                }

                println!("\nDigite os valores da primeira matriz:");
                let mat1: Vec<Vec<i128>> = pega_matriz(linhas, colunas);

                println!("\nDigite os valores da segunda matriz:");
                let mat2: Vec<Vec<i128>> = pega_matriz(linhas, colunas);

                let resultado = adicao_matrizes(mat1, mat2);

                println!("\nResultado da adição:");
                imprime_matriz(resultado);
            },

            8 => { 
                println!("\n======= SUBTRAÇÃO DE MATRIZES =======");
                println!("Digite o número de linhas:");
                let linhas: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };
                println!("Digite o número de colunas:");
                let colunas: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if linhas < 1 || colunas < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                }

                println!("\nDigite os valores da primeira matriz:");
                let mat1: Vec<Vec<i128>> = pega_matriz(linhas, colunas);

                println!("\nDigite os valores da segunda matriz:");
                let mat2: Vec<Vec<i128>> = pega_matriz(linhas, colunas);              

                let resultado = subtracao_matrizes(mat1, mat2);

                println!("\nResultado da subtração:");
                imprime_matriz(resultado);
            },

            9 => {
                println!("\n======= MULTIPLICAÇÃO DE MATRIZES =======");

                println!("Digite o número de linhas da primeira matriz:");
                let l1: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };
                println!("Digite o número de colunas da primeira matriz:");
                let c1: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                println!("Digite o número de linhas da segunda matriz:");
                let l2: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };
                println!("Digite o número de colunas da segunda matriz:");
                let c2: u128 = if let Inteiro::Unsigned(num) = pega_inteiro(false) {
                    num
                } else {
                    continue;
                };

                if l1 < 1 || c1 < 1 || l2 < 1 || c2 < 1 {
                    println!("Não é possível que o número de linhas ou colunas seja menor que 1. Tente novamente.");
                    continue;
                }

                println!("\nDigite os valores da primeira matriz:");
                let mat1: Vec<Vec<i128>> = pega_matriz(l1, c1);

                println!("\nDigite os valores da segunda matriz:");
                let mat2: Vec<Vec<i128>> = pega_matriz(l2, c2);

                let resultado = multiplicacao_matrizes(mat1, mat2);

                println!("\nResultado da multiplicação:");
                imprime_matriz(resultado);
            },

            10 => {
                println!("\n======= DETERMINANTE DE MATRIZES =======");

                println!("\nDigite os valores da matriz:");
                let matriz: Vec<Vec<i128>> = pega_matriz(2, 2);

                let resultado = determinante_matrizes(matriz);

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
