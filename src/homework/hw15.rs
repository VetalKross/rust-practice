use std::io::{self, Write};

fn main() {
    let mut solutions = Vec::new();
    let digits = [1,2,3,4,5,6,7,8];

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x==m||x==u { continue; }
                for &a in &digits {
                    if [m,u,x].contains(&a) { continue; }
                    let muxa = 1000*m + 100*u + 10*x + a;
                    let prod = muxa * a;
                    if prod < 1000 || prod > 9999 { continue; }
                    let s = prod / 1000;
                    let l = (prod / 100) % 10;
                    let o = (prod / 10) % 10;
                    let n = prod % 10;
                    if [s,l,o,n].iter().any(|&d| d<1 || d>8) { continue; }
                    if s==m||s==u||s==x||s==a { continue; }
                    if l==m||l==u||l==x||l==a|| [s].contains(&l) { continue; }
                    if o==m||o==u||o==x||o==a|| [s,l].contains(&o) { continue; }
                    if n==m||n==u||n==x||n==a|| [s,l,o].contains(&n) { continue; }

                    solutions.push((m,u,x,a,s,l,o,n));
                }
            }
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for &(m,u,x,a,s,l,o,n) in &solutions {
        writeln!(handle, "  {}{}{}{}", m, u, x, a).unwrap();
        writeln!(handle, "×   {}", a).unwrap();
        writeln!(handle, "------+").unwrap();
        writeln!(handle, "  {}{}{}{}", s, l, o, n).unwrap();
        writeln!(handle).unwrap();
    }

    writeln!(handle, "Загалом знайдено {} рішення(й).", solutions.len()).unwrap();
}
//Всього — 2 рішення.