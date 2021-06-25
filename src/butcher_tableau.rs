
struct ButcherTableau<const COUNT: usize>{
    num_stages : usize,
    c : [f64;COUNT],
    b : [f64;COUNT],
    a : [[f64;COUNT];COUNT]
}

struct ButcherTableauAdapt<const COUNT: usize>{
    bt : ButcherTableau<COUNT>,
    b_star : [f64;COUNT]
}


static RK4: &'static ButcherTableau<4> = ButcherTableau{
    num_stages : 4,
    c:[0,0.5,0.5,1],
    b:[1.0/6.0,1.0/3.0,1.0/3.0,1.0/6.0],
    a:[ [0.0, 0.0, 0.0, 0.0],
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0]]
};

static BOGACKI_SHAMPINE_2nd3rd : &'static ButcherTableauAdapt<4> = ButcherTableauAdapt{
    bt : ButcherTableau{
        num_stages : 4,
        c:[0,0.5,0.75,1],
        b:[2.0/9.0,1.0/3.0,4.0/9.0,0.0],
        a:[ [0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0]]
    },
}

//      //   0  |
//      //  c1  | a10
//      //  c2  | a20 a21
//      //  c3  | a30 a31 a32
//      //  .   |  .   .   .  .
//      //  .   |  .   .   .    .
//      //  .   |  .   .   .      .
//      //  cn  | an0 an1 an2 . . .a[n,n-1]
//      //  ---------------------------
//      //      |  b0  b1  b2 . . . b[n-1]  bn

// public class ButcherTableau implements Serializable {
 
//      public final double[] c; // 1st element unused.
//      public final double[][] a; // only half used...
//      public final double[] b;
//      public final double[] bStar;
//      public final boolean isFSAL;
 
//      public ButcherTableau(double[][] a, double[] b, double[] c, double[] bStar, boolean isFSAL) {
//          this.a = a;
//          this.b = b;
//          this.c = c;
//          this.bStar = bStar;
//          this.isFSAL = isFSAL;
//      }
 
//      public int getEvalsPerCall() {
//          if (isFSAL) {
//              return b.length - 1;
//          } else {
//              return b.length;
//          }
//      }
//      //   n = s-1
//      //
//      //   0  |
//      //  c1  | a10
//      //  c2  | a20 a21
//      //  c3  | a30 a31 a32
//      //  .   |  .   .   .  .
//      //  .   |  .   .   .    .
//      //  .   |  .   .   .      .
//      //  cn  | an0 an1 an2 . . .a[n,n-1]
//      //  ---------------------------
//      //      |  b0  b1  b2 . . . b[n-1]  bn
//      //
 
//  }