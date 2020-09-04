const eps: f64 = 1e-8;

type Mat = Vec<Vec<f64>>;
type Vector = Vec<f64>;

fn gauss_jordan(A: &Mat, b: &Vector) -> Option<Vector> {
    let n = A.len();
    let mut B: Mat = vec![vec![0.0; n + 1]; n];

    for i in 0..n {
        for j in 0..n {
            B[i][j] = A[i][j];
        }
        B[i][n] = b[i];
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if B[j][i].abs() > B[pivot][i].abs() {
                pivot = j;
            }
        }
        B.swap(i, pivot);

        if B[i][i].abs() < eps {
            // no solution;
            return None;
        }

        for j in i + 1..n + 1 {
            B[i][j] /= B[i][i];
        }

        for j in 0..n {
            if i != j {
                for k in i + 1..n + 1 {
                    B[j][k] -= B[j][i] * B[i][k];
                }
            }
        }
    }
    let mut x = vec![0.0; n];
    for i in 0..n {
        x[i] = B[i][n];
    }
    Some(x)
}

fn main() {
    let A: Mat = vec![
        vec![1.0, -2.0, 3.0],
        vec![4.0, -5.0, 6.0],
        vec![7.0, -8.0, 10.0],
    ];

    let b: Vector = vec![6.0, 12.0, 21.0];
    let x = gauss_jordan(&A, &b);
    println!("{:?}", x);
}
