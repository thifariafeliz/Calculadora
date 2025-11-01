// cria um módulo para o arquivo `func.rs`
mod func;
// importa tudo (*) do módulo `func`
use func::*;

fn main() {
    loop {
        println!("\nEscolha a operação que deseja fazer:");
        println!("1. Fatorial\n2. Arranjo\n3. Binomial\n4. Combinação\n5. Permutação simples\n6. Permutação com repeticões\n0. Sair");
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

                //let arrj: i32 = arranjo();
                //println!("O arranjo de {} elementos em {} lugares é = {}", elementos, lugares, arrj);
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

                //let permutacao = permutacao();
                //println!("O número de permutações possíveis com {} elementos é = {}", elementos, permutacao);
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
            _ => {  // mesmo que default em C, mas obrigatório neste caso.
                println!("Opção inválida. Tente novamente.");
                continue;
            }
        }
    }

    println!("Até mais!");
}
