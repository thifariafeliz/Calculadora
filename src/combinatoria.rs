// Função recursiva para calcular o fatorial de um número
pub fn fatorial(numero: u128) -> u128 {
    if numero == 0 {
        return 1;
    };
    numero * fatorial(numero - 1)
}

// Função para calcular o arranjo de `n` números em `p` lugares
pub fn arranjo(n: u128, p: u128) -> u128 {
    if p > n {
        println!("Erro: p não pode ser maior que n!");
        return 0;
    }
    fatorial(n) / fatorial(n - p)
}

// Função para calcular um número binomial de ordem `n` e classe `k`
pub fn binomial(n: u128, k: u128) -> u128 {
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}

// Função para calcular a combinação de n elementos em k lugares
pub fn combinacao(n: u128, k: u128) -> u128 {
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}

// Função para calcular a permutação simples de um número
pub fn permutacao_simples(n: u128) -> u128 {
    fatorial(n)
}

// Calcula a quantidade de permutações com repetição
pub fn permutacao_repeticao(elementos: u128, repeticoes: Vec<u128>) -> u128 {
    let mut denominador: u128 = 1;

    for i in repeticoes.iter() {
        denominador *= fatorial(*i);
    }

    fatorial(elementos) / denominador
}
