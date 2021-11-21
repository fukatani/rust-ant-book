#[derive(Clone)]
struct SegTree2D<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    n: usize,
    m: usize,
    dat: Vec<Vec<T>>,
    init: T,
    functor: F,
}

impl<T, F> SegTree2D<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    fn new(dat: &Vec<Vec<T>>, init: T, f: F) -> SegTree2D<T, F> {
        let n = dat.len();
        let m = dat[0].len();

        let kn = 65 - (n - 1).leading_zeros();
        let nn = 1 << kn;
        let km = 65 - (m - 1).leading_zeros();
        let mm = 1 << km;

        let mut dat2 = vec![vec![init; 2 * mm]; 2 * nn];
        for i in 0..n {
            for j in 0..m {
                dat2[nn + i][mm + j] = dat[i][j];
            }
        }
        let mut seg = SegTree2D {
            n: nn,
            m: mm,
            dat: dat2,
            init: init,
            functor: f,
        };
        seg.build();
        seg
    }

    fn build(&mut self) {
        for j in 0..self.m {
            for i in (0..self.n).rev() {
                self.dat[i][self.m + j] = (self.functor)(
                    self.dat[i << 1][self.m + j],
                    self.dat[i << 1 | 1][self.m + j],
                );
            }
        }
        for i in 0..self.n * 2 {
            for j in (0..self.m).rev() {
                self.dat[i][j] = (self.functor)(self.dat[i][j << 1], self.dat[i][j << 1 | 1]);
            }
        }
    }

    fn update(&mut self, x: usize, y: usize, value: T) {
        let mut i = x + self.n;
        let mut j = y + self.m;
        self.dat[i][j] = value;
        while j > 1 {
            j >>= 1;
            self.dat[i][j] = (self.functor)(self.dat[i][j << 1], self.dat[i][j << 1 | 1]);
        }
        j = y + self.m;
        while i > 1 {
            i >>= 1;
            self.dat[i][j] = (self.functor)(self.dat[i << 1][j], self.dat[i << 1 | 1][j]);
            while j > 1 {
                j >>= 1;
                self.dat[i][j] = (self.functor)(self.dat[i][j << 1], self.dat[i][j << 1 | 1]);
            }
            j = y + self.m;
        }
    }

    fn query(&self, mut lx: usize, mut rx: usize, mut ly: usize, mut ry: usize) -> T {
        // [Lx,Rx)×[Ly,Ry)
        lx += self.n;
        rx += self.n;
        ly += self.m;
        ry += self.m;
        let mut vlx = self.init;
        let mut vrx = self.init;
        while lx < rx {
            if lx & 1 == 1 {
                let mut vly = self.init;
                let mut vry = self.init;
                let mut ly1 = ly;
                let mut ry1 = ry;
                while ly1 < ry1 {
                    if ly1 & 1 == 1 {
                        vly = (self.functor)(vly, self.dat[lx][ly1]);
                        ly1 += 1;
                    }
                    if ry1 & 1 == 1 {
                        ry1 -= 1;
                        vry = (self.functor)(self.dat[lx][ry1], vry);
                    }
                    ly1 >>= 1;
                    ry1 >>= 1;
                }
                let vy = (self.functor)(vly, vry);
                vlx = (self.functor)(vlx, vy);
                lx += 1;
            }
            if rx & 1 == 1 {
                rx -= 1;
                let mut vly = self.init;
                let mut vry = self.init;
                let mut ly1 = ly;
                let mut ry1 = ry;
                while ly1 < ry1 {
                    if ly1 & 1 == 1 {
                        vly = (self.functor)(vly, self.dat[rx][ly1]);
                        ly1 += 1;
                    }
                    if ry1 & 1 == 1 {
                        ry1 -= 1;
                        vry = (self.functor)(self.dat[rx][ry1], vry);
                    }
                    ly1 >>= 1;
                    ry1 >>= 1;
                }
                let vy = (self.functor)(vly, vry);
                vrx = (self.functor)(vy, vrx);
            }
            lx >>= 1;
            rx >>= 1;
        }
        (self.functor)(vlx, vrx)
    }
}
