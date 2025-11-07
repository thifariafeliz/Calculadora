// cria um módulo para o arquivo `func.rs`
mod func;
// importa tudo (*) do módulo `func`
use func::*;

fn main() {
    loop {
        println!("\nEscolha a operação que deseja fazer:");
        println!("1. Fatorial\n2. Arranjo\n3. Binomial\n4. Combinação\n5. Permutação simples\n6. Permutação com repeticões\n
            7. Adição de matrizes\n8. Subtração de matrizes\n9.Multiplicação de matrizes\n10. Determinante de matrizes\n0. Sair");
        let input: i32 = pega_inteiro();

        if input == -1 {
            println!("Opção inválida. Tente novamente.");
            continue;
        }

        match input {
            0 => {
                break;
            },
            1 => {
                println!("\n=======FATORIAL=======");
                println!("Digite um número para ver seu fatorial:");
                let numero: i32 = pega_inteiro();

                // Verifica se o número digitado é negativo
                if numero < 0 {
                    println!("Número inválido. Tente novamente.");
                    continue;
                }

                let fat: i32 = fatorial(numero);
                
                println!("O fatorial de {} é = {}", numero, fat);
            }
            2 => {
                println!("\n=======ARRANJO=======");
                println!("Digite a quantidade de elementos que deseja arranjar:");
                let elementos: i32 = pega_inteiro();
                println!("Digite a quantidade de lugares em que os números serão arranjados:");
                let lugares: i32 = pega_inteiro();
            
                // Verifica se algum número digitado foi menor que 1
                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para elementos e lugares no arranjo. Tente novamente.");
                    continue;
                }

               let arrj: i32 = arranjo();
               println!("O arranjo de {} elementos em {} lugares é = {}", elementos, lugares, arrj);
            },
            3 => {
                println!("\n=======BINOMIAL=======");
                println!("Digite a ordem do número binomial:");
                let n: i32 = pega_inteiro();
                println!("Digite a classe do número binomial:");
                let k: i32 = pega_inteiro();

                if n < 1 || k < 1 {
                    println!("Valores menores que 1 não são válidos para ordem ou classe de um número binomial. Tente novamente.");
                    continue;
                }
                 let binomial: i32 = binomial(n, k);
                     println!("O número binomial de ordem {} e classe {} é = {}", n, k, binomial);    
            },
            4 => {
                println!("\n=======COMBINAÇÃO=======");
                println!("Digite o número de elementos a serem combinados:");
                let elementos: i32 = pega_inteiro();
                println!("Digite o número de lugares em que os elementos serão combinados:");
                let lugares: i32 = pega_inteiro();

                if elementos < 1 || lugares < 1 {
                    println!("Valores menores que 1 não são válidos para o número de elementos ou lugares. Tente novamente.");
                    continue;
                }

                let combinacao: i32 = combinacao(elementos, lugares);
                   println!("O número de combinações possíveis com {} elementos tomados de {} em {} é = {}", elementos, lugares, lugares, combinacao);

            },
            5 => {
                println!("\n=======PERMUTAÇÃO SIMPLES=======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: i32 = pega_inteiro();

                if elementos < 1 {
                    println!("Valor menor 1 não é válido para o número de elementos na permutação. Tente novamente.");
                    continue;
                }

                let permutacao = permutacao();
                println!("O número de permutações possíveis com {} elementos é = {}", elementos, permutacao);
            },
            6 => {
                println!("\n=======PERMUTAÇÃO COM REPETIÇÃO=======");
                println!("Digite a quantidade de elementos que serão permutados:");
                let elementos: i32 = pega_inteiro();
                println!("Digite a quantidade de elementos que se repetem:");
                let quant: i32 = pega_inteiro();
                let mut repeticoes: Vec<i32> = Vec::new();  // cria um vetor para armazenar valores i32

                if elementos < 1 || quant < 1{
                    println!("Valores menores que 1 não são válidos para elementos, quantidade de repetições e repetições. Tente novamente.");
                    continue;
                }

                // Iteração para pegar números e enfiá-los no vetor.
                for i in 1..=quant {
                    println!("Digite a quantidade de repetições do elemento {}:", i);
                    repeticoes.push(pega_inteiro());
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

                let resultado = adicao_matrizes(mat1, mat2);

                println!("\nResultado da adição:");
                for linha in resultado {
                    println!("{:?}", linha);
    }
}

            },
            8 => { 
                println!("\n======= SUBTRAÇÃO DE MATRIZES =======");
                println!("Digite o número de linhas:");
                let linhas: i32 = pega_inteiro();
                println!("Digite o número de colunas:");
               let colunas: i32 = pega_inteiro();

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

            println!("\nResultado da subtração:");
            for linha in resultado {
            println!("{:?}", linha);


    
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

                println!("\nResultado da multiplicação:");
                for linha in resultado {
                 println!("{:?}", linha);
}

            },
            10 => {

            },
            _ => {  // mesmo que default em C, mas obrigatório neste caso.
                println!("Opção inválida. Tente novamente.");
                continue;
            }
        }
    }

    println!("Até mais!");
}
