/// Cargo.toml の [dependencies] セクションなどに
/// itertools = "0.10"   を追加してください。
use itertools::Itertools;
use chrono::{Utc, Local, DateTime, Date};
/// 2次元座標を表す型エイリアス
type Point = (i32, i32);

/// 3点が厳密に同一直線上にあるかどうか（面積が0かどうか）
/// 面積 2 倍の値(外積)が 0 なら collinear
fn area2(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let (x1, y1) = *p1;
    let (x2, y2) = *p2;
    let (x3, y3) = *p3;
    (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)
}

/// 4点が厳密に同一直線上にあるかどうか
///
/// 4点 p1, p2, p3, p4 が同一直線上にあるとは、
/// p1, p2, p3 が共線 かつ p1, p2, p4 も共線 であること。
fn four_points_are_collinear(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    area2(p1, p2, p3) == 0 && area2(p1, p2, p4) == 0
}

/// 4点が同一円周上かどうかを判定するための行列式 (circumcircle 判定)
///
/// 以下の 4x4 行列の行列式が 0 になるかどうかで判定する:
///
/// | x_i^2 + y_i^2   x_i   y_i   1 |
/// | x_j^2 + y_j^2   x_j   y_j   1 |
/// | x_k^2 + y_k^2   x_k   y_k   1 |
/// | x_l^2 + y_l^2   x_l   y_l   1 |
///
/// 絶対値がごく小さい(浮動小数で 0 相当)なら同一円周上。
/// ただし、4点が厳密に同一直線上の場合は「同一円周上」とみなさない。
fn four_points_are_concyclic(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    // まず4点が同一直線上なら false を返す
    if four_points_are_collinear(p1, p2, p3, p4) {
        return true;
    }

    // 4x4行列を作成し、行列式が 0 かどうか調べる
    fn det4(m: [[f64; 4]; 4]) -> f64 {
        // 行列式を直接計算(展開法 or その他の方法)
        // ここでは展開法(サラスの公式)をなるべく避け、汎用的に書いても良いですが
        // コード量が多くなるため、ラプラス展開など適当な実装を簡易的に行います。
        //
        // Rust で n=4 の行列式を愚直に書くなら、子行列の 3x3 の行列式と符号を使った
        // ラプラス展開が分かりやすいです。
        //
        // ここでは簡単のために余因子展開をベタ書きします。

        let mut d: f64 = 0.0;
        for i in 0..4 {
            // 余因子 C(i,0) = (-1)^(i+0) * det(M_i0) (M_i0 は行0列iを除いた3x3小行列)
            let mut sub = [[0.0; 3]; 3];
            for (sub_row, row) in (0..4).filter(|&r| r != 0).enumerate() {
                let mut sub_col_idx = 0;
                for col in 0..4 {
                    if col == i {
                        continue;
                    }
                    sub[sub_row][sub_col_idx] = m[row][col];
                    sub_col_idx += 1;
                }
            }
            let sign = if (i + 0) % 2 == 0 { 1.0 } else { -1.0 };
            d += sign * m[0][i] * det3(sub);
        }
        d
    }

    fn det3(m: [[f64; 3]; 3]) -> f64 {
        // 3x3 行列式
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    let (x1, y1) = *p1;
    let (x2, y2) = *p2;
    let (x3, y3) = *p3;
    let (x4, y4) = *p4;

    let mat = [
        [(x1 * x1 + y1 * y1) as f64, x1 as f64, y1 as f64, 1.0],
        [(x2 * x2 + y2 * y2) as f64, x2 as f64, y2 as f64, 1.0],
        [(x3 * x3 + y3 * y3) as f64, x3 as f64, y3 as f64, 1.0],
        [(x4 * x4 + y4 * y4) as f64, x4 as f64, y4 as f64, 1.0],
    ];

    let det_value = det4(mat);

    // 浮動小数点誤差を考慮し、絶対値が非常に小さければ 0 とみなす
    const EPS: f64 = 1.0e-12;
    det_value.abs() < EPS
}

/// 部分集合 subset 内に 4 点が同一円周上となる組合せが一つでもあれば true
fn has_any_4_concyclic(subset: &[Point]) -> bool {
    // 4点組合せをすべて試し、1組でも同一円周上があれば true を返す
    for comb4 in subset.iter().combinations(4) {
        let p1 = comb4[0];
        let p2 = comb4[1];
        let p3 = comb4[2];
        let p4 = comb4[3];
        if four_points_are_concyclic(p1, p2, p3, p4) {
            return true;
        }
    }
    false
}

fn main() {
    let hen = 6;
    let mut all_points: Vec<Point> = (0..=hen)
        .flat_map(|x| (0..=hen).map(move |y| (x, y)))
        .collect();
    println!("大きさは{}x{}", hen + 1, hen + 1);
    for n in 13..=25 {
        // 25点から n 点を選ぶ
        // itertools の combinations を使う
        let mut found_good_subset = false;
        println!("{}", all_points.len());
        let mut t:i64 = 0;
        for subset in all_points.iter().combinations(n) {
            t = t + 1;
            // 同一円周上となる4点が存在するかをチェック
            if !has_any_4_concyclic(&subset.iter().map(|&&t| t).collect::<Vec<(i32, i32)>>()) {
                // もし同一円周上4点が存在しなければOK
                found_good_subset = true;
                println!(
                    "同一円周上となる4点を含まない部分集合が存在する n = {},{:#?}",
                    n, subset
                );
                break;
            }
            if (t % 10000000) == 0 {
                println!("{}:{} 千万回目",  Local::now(),t / 10000000); // 合計25億
            }
        }
        if !found_good_subset {
            println!("{} 存在しない", n);
            break;
        }
    }

    // もし何も見つからなければ n=0? (通常はあり得ないが念のため)
    println!("条件を満たす部分集合は見つかりませんでした。");
}
