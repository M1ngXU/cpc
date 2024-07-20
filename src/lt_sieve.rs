/// copied from https://github.com/TecTrixer/cp-template/blob/main/template.rs
pub fn lp(n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut lp = vec![0; n + 1];
    let mut pr = vec![];
    for i in 2..=n {
        if lp[i] == 0 {
            lp[i] = i;
            pr.push(i);
        }
        let mut j = 0;
        while i * pr[j] <= n {
            lp[i * pr[j]] = pr[j];
            if pr[j] == lp[i] {
                break;
            }
            j += 1;
        }
    }
    (lp, pr)
}
