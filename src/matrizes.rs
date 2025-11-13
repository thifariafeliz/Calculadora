// Função que faz a soma de duas matrizes e retorna a matriz resultado
pub fn adicao_matrizes(mat1: Vec<Vec<i128>>, mat2: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    if mat1.len() != mat2.len() || mat1[0].len() != mat2[0].len() {
        println!("erro! as matrizes devem ter colunas iguais, assim como suas colunas.");
        return vec![];
    }

    let mut resultado: Vec<Vec<i128>> = vec![vec![0; mat1[0].len()]; mat1.len()];

    for i in 0..mat1.len() {
        for j in 0..mat1[0].len() {
            resultado[i][j] = mat1[i][j] + mat2[i][j];
        }
    }

    resultado
}

// Função que faz subtração de duas matrizes e retorna a matriz resultado
pub fn subtracao_matrizes(mat1: Vec<Vec<i128>>, mat2: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    if mat1.len() != mat2.len() || mat1[0].len() != mat2[0].len() {
        println!("erro! as matrizes devem ter colunas iguais, assim como suas colunas.");
        return vec![];
    }

    let mut resultado: Vec<Vec<i128>> = vec![vec![0; mat1[0].len()]; mat1.len()];

    for i in 0..mat1.len() {
        for j in 0..mat1[0].len() {
            resultado[i][j] = mat1[i][j] - mat2[i][j];
        }
    }

    resultado
}

// Função que faz a multiplicação de duas matrizes e retorna a matriz resultado
pub fn multiplicacao_matrizes(mat1: Vec<Vec<i128>>, mat2: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    if mat1[0].len() != mat2.len() {
        println!("Erro! O número de colunas da primeira matriz deve corresponder ao número de linhas da segunda matriz.");
        return vec![];
    }

    let mut resultado: Vec<Vec<i128>> = vec![vec![0; mat1.len()]; mat2[0].len()];

    for i in 0..mat1.len() {
        for j in 0..mat2[0].len() {
            for k in 0..mat1[0].len() {
                resultado[i][j] = mat1[i][k] * mat2[k][j];
            }
        }
    }

    resultado
}

// Função que determina o determinante de uma matriz 2x2 e retorna um número inteiro
pub fn determinante_matrizes(matriz: Vec<Vec<i128>>) -> i128 {
    (matriz[0][0] * matriz[1][1]) - (matriz[1][0] * matriz[0][1])
}
