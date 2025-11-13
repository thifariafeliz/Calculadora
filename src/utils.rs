use std::io::{self, Write};

#[derive(Debug)]
pub enum Inteiro {
    Unsigned(u128),
    Signed(i128),
}

pub fn pega_inteiro(negativo: bool) -> Inteiro {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Falha na leitura do input.");

        if negativo == true {
            let input = if let Ok(num) = input.trim().parse::<i128>() {
                num
            } else {
                println!("Não foi possível converter input para i128. Tente novamente:");
                continue;
            };

            return Inteiro::Signed(input);
        } else {
            let input = if let Ok(num) = input.trim().parse::<u128>() {
                num
            } else {
                println!("Não foi possível converter input para u128. Tente novamente:");
                continue;
            };

            return Inteiro::Unsigned(input);
        }
    }
}

pub fn pega_matriz(linhas: u128, colunas: u128) -> Vec<Vec<i128>> {
    let mut matriz: Vec<Vec<i128>> = vec![vec![0; colunas as usize]; linhas as usize];

    for i in 0..linhas as usize {
        for j in 0..colunas as usize {
            println!("Digite o valor do índice [{}][{}]:", i, j);
            matriz[i][j] = if let Inteiro::Signed(num) = pega_inteiro(true) {
                num
            } else {
                println!("Não foi possível atribuir valor ao índice. Tente novamente.");
                return vec![];
            }
        }
    }

    matriz
}

pub fn imprime_matriz(matriz: Vec<Vec<i128>>) {
    for i in 0..matriz.len() {
        for j in 0..matriz[0].len() {
            print!("{:5} ", matriz[i][j]);
            io::stdout().flush().unwrap();
        }
        println!();
    }
}
