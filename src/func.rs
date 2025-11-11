// Função recursiva para calcular o fatorial de um número
pub fn fatorial(numero: i32) -> i32 {
    // Essa função será chamada consecutivamente várias vezes até que `numero` seja igual a 0
    if numero == 0 {
        return 1;
    };
    // Quando acontecer o primeiro retorno (valor 1), ele será multiplicado pelo valor do argumento
    // da penúltima função chamada, 2 * 1 retorna 2, assim sucessiivamente até chegar ao número
    // primordial passado para a primeira execução da função `fatorial`
    numero * fatorial(numero - 1)
}

pub fn arranjo(n: i32, p: i32) -> i32 {
    if p > n {
        println!("Erro: p não pode ser maior que n!");
        return 0;
    }
    fatorial(n) / fatorial(n - p)
}


pub fn binomial(n: i32, k: i32) -> i32 {
    if k < 0 || k > n {
        return 0;
    }
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}

pub fn combinacao(elementos: i32, lugares: i32) -> i32 {
    if lugares < 0 || lugares > elementos {
        return 0;
    }
    fatorial(elementos) / (fatorial(lugares) * fatorial(elementos - lugares))
}

pub fn permutacao_simples(n: i32) -> i32 {
    fatorial(n)
}

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

