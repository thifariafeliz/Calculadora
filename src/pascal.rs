pub fn pascal(mut matriz: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let linhas = matriz.len();
    let colunas = matriz[0].len();

    for i in 0..linhas {
        for j in 0..colunas {
            if j == 0 {
                matriz[i][j] = 1;
            } else {
                if i != 0 {
                    matriz[i][j] = matriz[i-1][j-1] + matriz[i-1][j];
                }
            }
        }
    }

    matriz
}
