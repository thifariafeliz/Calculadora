use std::io;
// a palavra-chave `pub` nos permite usar a função no arquivo que está importando este arquivo.

// Função recursiva para calcular o fatorial de um número
pub fn fatorial(numero: i32) -> i32 {
    // Essa função será chamada consecutivamente várias vezes até que `numero` seja igual a 0
    if numero == 0 { return 1 };
    // Quando acontecer o primeiro retorno (valor 1), ele será multiplicado pelo valor do argumento
    // da penúltima função chamada, 2 * 1 retorna 2, assim sucessiivamente até chegar ao número
    // primordial passado para a primeira execução da função `fatorial`
    numero * fatorial(numero - 1)
}

// Função para calcular um número elevado a uma potência, sendo `base` o número, e `pot` a potência
pub fn potencia(base: i32, pot: i32) -> i32 {
    // Parecida com a função `fatorial`, será chamada várias vezes até que o argumento `pot` seja 0
    if pot == 0 { return 1 };

    // Quando 1 for retornado, a base irá multiplicar este valor, e depois o resultado será sempre
    // multiplicado pela base. Assim é como se fosse: 10*1=10, 10*10=100, 100*10=1000
    base * potencia(base, pot - 1)
}

/*
pub fn arranjo() -> i32 {

}


pub fn binomial(n: i32, k: i32) -> i32 {
    if k < 0 || k > n {
        return 0; // fora do domínio
    }
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}


pub fn combinacao(elementos: i32, lugares: i32) -> i32 {
    if lugares < 0 || lugares > elementos {
        return 0;
    }
    fatorial(elementos) / (fatorial(lugares) * fatorial(elementos - lugares))
}



pub fn permutacao_simples() -> i32 {

}
*/

// Calcula a quantidade de permutações com repetição
pub fn permutacao_repeticao(elementos: i32, repeticoes: Vec<i32>) -> i32 {
    let mut denominador: i32 = 1;

    // Itera sobre os elementos do vetor `repeticoes` e multiplica o fatorial de cada um pelo valor do denominador
    for i in repeticoes.iter() {
        denominador *= fatorial(*i);
    }

    // Retorna o a quantidade de permutações
    fatorial(elementos) / denominador
}

// Pega um valor inteiro a partir da digitação do usuário
pub fn pega_inteiro() -> i32 {
    // Cria uma nova string para armazenar a linha do usuário
    let mut input = String::new();

    // função que pega input do usuário
    io::stdin()
        .read_line(&mut input) //precisa passar como referência mutável
        .expect("Falha na leitura do input do usuário."); // precisa do expect em caso de falha

    // faz um match que verifica se o número, depois de `trim` pode ser convertido para `i32` por
    // meio de `.parse::<i32>()`
    match input.trim().parse::<i32>() {
        // em caso positivo retorna o número digitado
        Ok(num) => num,
        // em caso negativo retorna `-1`, neste programa é válido já que não usaremos valores
        // negativos em momento algum.
        Err(_) => -1,
    }
}
