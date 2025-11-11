// Função recursiva para calcular o fatorial de um número
pub fn fatorial(numero: i32) -> i32 {
    if numero == 0 {
        return 1;
    };
    numero * fatorial(numero - 1)
}

// Função para calcular o arranjo de `n` números em `p` lugares
pub fn arranjo(n: i32, p: i32) -> i32 {
    if p > n {
        println!("Erro: p não pode ser maior que n!");
        return 0;
    }
    fatorial(n) / fatorial(n - p)
}

// Função para calcular um número binomial de ordem `n` e classe `k`
pub fn binomial(n: i32, k: i32) -> i32 {
    if k < 0 || k > n {
        return 0;
    }
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}

// Função para calcular a combinação de n elementos em k lugares
pub fn combinacao(n: i32, k: i32) -> i32 {
    if k < 0 || k > n {
        return 0;
    }
    fatorial(n) / (fatorial(k) * fatorial(n - k))
}

// Função para calcular a permutação simples de um número
pub fn permutacao_simples(n: i32) -> i32 {
    fatorial(n)
}

// Calcula a quantidade de permutações com repetição
pub fn permutacao_repeticao(elementos: i32, repeticoes: Vec<i32>) -> i32 {
    let mut denominador: i32 = 1;

    for i in repeticoes.iter() {
        denominador *= fatorial(*i);
    }

    fatorial(elementos) / denominador
}
