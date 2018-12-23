#include <algorithm>
#include <vector>
#include <iostream>

int W, H, N;
std::vector<int> X1, X2, Y1, Y2;

bool fld[500 * 6][500 * 6];


void print_vector(const std::vector<int>& v) {
    for (const auto& elem : v) {
        std::cout << elem << ", ";
    }
    std::cout << std::endl;
}

int compress(std::vector<int>& x1, std::vector<int>& x2, int w) {
    std::vector<int> xs;
    for (int i = 0; i < N; i++) {
        for (int d = -1; d <= 1; d++) {
            int tx1 = x1[i] + d;
            int tx2 = x2[i] + d;
            if (1 <= tx1 && tx1 <= w) xs.push_back(tx1);
            if (1 <= tx2 && tx2 <= w) xs.push_back(tx2);
        }
    }
    std::sort(xs.begin(), xs.end());
    xs.erase(std::unique(xs.begin(), xs.end()), xs.end());
    std::cout << "xs" << std::endl;
    print_vector(xs);
    for (int i = 0; i < N; i++) {
        std::cout << "before x1[i]: " << x1[i];
        x1[i] = std::find(xs.begin(), xs.end(), x1[i]) - xs.begin();
        std::cout << " after x1[i]: " << x1[i];
        x2[i] = std::find(xs.begin(), xs.end(), x2[i]) - xs.begin();
        std::cout << std::endl;
    }
    return xs.size();
    // x2.resize(xs.size());
}

void solve() {
    std::cout << "X1" << std::endl;
    print_vector(X1);
    std::cout << "X2" << std::endl;
    print_vector(X2);
    W = compress(X1, X2, W);
    std::cout << "X1" << std::endl;
    print_vector(X1);
    std::cout << "X2" << std::endl;
    print_vector(X2);
    std::cout << "W: " << W << std::endl;
   
    std::cout << std::endl;
    std::cout << "Y1" << std::endl;
    print_vector(Y1);
    std::cout << "Y2" << std::endl;
    print_vector(Y2);
    H = compress(Y1, Y2, H);
    std::cout << "Y1" << std::endl;
    print_vector(Y1);
    std::cout << "Y2" << std::endl;
    print_vector(Y2);
    std::cout << "H: " << H << std::endl;
}

int main() {
    W = 100;
    H = 100;
    N = 5;
    X1 = {1, 15, 4, 9, 10};
    X2 = {6, 30, 4, 9, 10};
    Y1 = {4, 8, 1, 1, 6};
    Y2 = {4, 8, 10, 5, 10};
    solve();
}