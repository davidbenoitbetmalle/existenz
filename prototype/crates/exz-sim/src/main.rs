//! exz-sim: multi-zone simulation over the exz-core rules engine.
//!
//! Not a blockchain and not a forecast: a deterministic, seeded replay of the
//! protocol rules at zone scale, to observe how issuance, the accumulation
//! cap, the conversion tiers (Annex F.4), the circuit bound (Annex H.2) and
//! the coverage ration (Annex G.3) interact day by day. Three zones are
//! simulated by default:
//!
//! - pilot:  the Annex G.5 scenario (100k persons, u = 0.60, v = 0.80),
//!           endowment and inflow sized for full coverage.
//! - thin:   a sparse adoption zone (5k persons, u = 0.30, v = 0.50).
//! - stress: an under-endowed zone (allocated = 0.5 x H x D, inflow = 0.3 x D)
//!           where the ration queue and the coverage rule do visible work.
//!
//! Outputs a CSV (one row per day and zone), an SVG chart, and a stdout
//! summary with the per-person-per-year payout compared to the Annex G.1
//! projection.

use exz_core::{projected_daily_payout_cents, CapReason, DayStats, Params, World};
use std::env;
use std::fs;
use std::path::Path;

/// Deterministic splitmix64: same seed, same run, replayable forever.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }
    fn bps_hit(&mut self, bps: u64) -> bool {
        self.below(10_000) < bps
    }
}

/// Static description of one rollout zone.
struct ZoneSpec {
    id: u64,
    name: &'static str,
    persons: u64,
    merchants: u64,
    meal_price_cents: u64,
    /// u: probability that a person spends the daily allowance today.
    spend_bps: u64,
    /// v: fraction of confirmed credit the merchant converts daily.
    convert_bps: u64,
    /// Share of persons who never spend (accumulation-cap pressure).
    hoarder_bps: u64,
    /// Probability that a merchant with credit circuits part of it today.
    circuit_bps: u64,
    /// Endowment = allocated_coverage x H x D_projected.
    allocated_coverage: f64,
    /// Daily inflow = inflow_frac x D_projected.
    inflow_frac: f64,
}

impl ZoneSpec {
    fn plausible_population(&self) -> u64 {
        (self.persons + self.merchants - 1) / self.merchants
    }
}

fn default_specs() -> Vec<ZoneSpec> {
    vec![
        ZoneSpec {
            id: 1,
            name: "pilot",
            persons: 100_000,
            merchants: 150,
            meal_price_cents: 60,
            spend_bps: 6_000,
            convert_bps: 8_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            allocated_coverage: 1.0,
            inflow_frac: 1.0,
        },
        ZoneSpec {
            id: 2,
            name: "thin",
            persons: 5_000,
            merchants: 15,
            meal_price_cents: 40,
            spend_bps: 3_000,
            convert_bps: 5_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            allocated_coverage: 1.0,
            inflow_frac: 1.0,
        },
        ZoneSpec {
            id: 3,
            name: "stress",
            persons: 20_000,
            merchants: 40,
            meal_price_cents: 80,
            spend_bps: 9_000,
            convert_bps: 10_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            // Deliberately under-endowed: the ration queue must appear.
            allocated_coverage: 0.5,
            inflow_frac: 0.3,
        },
    ]
}

const CHART_COLORS: [&str; 3] = ["#d95f18", "#3a6ea5", "#6b4fa0"];

struct Args {
    days: u64,
    out: String,
    seed: u64,
}

fn parse_args() -> Args {
    let mut args = Args { days: 90, out: "./out".to_string(), seed: 1 };
    let argv: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--days" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(d) = argv[i].parse() {
                    args.days = d;
                }
            }
            "--out" if i + 1 < argv.len() => {
                i += 1;
                args.out = argv[i].clone();
            }
            "--seed" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(s) = argv[i].parse() {
                    args.seed = s;
                }
            }
            _ => {}
        }
        i += 1;
    }
    args
}

fn main() {
    let args = parse_args();
    let specs = default_specs();
    let params = Params::default();
    let commission_bps = params.commission_bps;
    let horizon = params.reserve_horizon_days;
    let allowance = params.daily_issuance_mexz;
    let days = args.days;
    let n_zones = specs.len();

    // Projected daily payout D per the Annex G.1 formula, used to size the
    // endowment (H x D) and the daily inflow of each zone fund.
    let d_proj: Vec<f64> = specs
        .iter()
        .map(|s| {
            projected_daily_payout_cents(
                s.persons,
                s.spend_bps as f64 / 10_000.0,
                s.convert_bps as f64 / 10_000.0,
                s.meal_price_cents,
                commission_bps,
            )
        })
        .collect();

    let mut world = World::new(params.clone());
    for (i, s) in specs.iter().enumerate() {
        let allocated = (s.allocated_coverage * horizon as f64 * d_proj[i]) as i64;
        let inflow = (s.inflow_frac * d_proj[i]) as i64;
        world
            .add_zone(s.id, s.name, s.meal_price_cents, s.plausible_population(), allocated, inflow)
            .expect("zone setup");
        for p in 0..s.persons {
            world.enroll_person(s.id, p).expect("enroll person");
        }
        for m in 0..s.merchants {
            world.enroll_merchant(s.id, m).expect("enroll merchant");
        }
    }

    // One independent RNG stream per zone, all derived from the master seed.
    let mut rngs: Vec<Rng> = specs
        .iter()
        .map(|s| Rng::new(args.seed ^ (s.id.wrapping_mul(0x9E3779B97F4A7C15))))
        .collect();

    // Aggregated daily grid: [day][zone].
    let mut grid: Vec<Vec<DayStats>> =
        vec![vec![DayStats::default(); n_zones]; days as usize];
    // Which conversion cap bound, per zone (Annex F.4 diagnostics).
    let mut cap_counts: Vec<[u64; 4]> = vec![[0; 4]; n_zones];
    // Coverage ratio per (day, zone), recorded after the day's conversions.
    let mut coverage: Vec<Vec<f64>> = vec![vec![0.0; n_zones]; days as usize];

    for day in 1..=days {
        world.new_day();

        for (zi, s) in specs.iter().enumerate() {
            let zone = s.id;
            let hoarders = s.persons * s.hoarder_bps / 10_000;

            // 1. Persons spend the daily allowance with probability u,
            //    at a uniformly random merchant (single-hop rule, Annex F.1).
            for p in 0..s.persons {
                if p < hoarders {
                    continue;
                }
                if rngs[zi].bps_hit(s.spend_bps) {
                    let m = rngs[zi].below(s.merchants);
                    // A living person's balance is always at least one
                    // allowance after issuance (cap = 2 allowances), so the
                    // daily allowance can always be spent in full.
                    let _ = world.spend(zone, p, m, allowance);
                }
            }

            // 2. Merchants circuit part of their credit to other merchants
            //    (Annex H.2 bound: 20% of the weekly received credit).
            let merchants = s.merchants;
            for m in 0..merchants {
                if !rngs[zi].bps_hit(s.circuit_bps) {
                    continue;
                }
                let zi_world = world
                    .zones
                    .iter()
                    .position(|z| z.id == zone)
                    .expect("zone exists");
                let merchant = &world.zones[zi_world].merchants[m as usize];
                let confirmed = merchant.confirmed_total();
                if confirmed == 0 {
                    continue;
                }
                let window = world.params.circuit_window_days;
                let received_7d: u64 = merchant
                    .received_hist
                    .iter()
                    .filter(|(d, _)| world.day.saturating_sub(*d) <= window)
                    .map(|(_, a)| a)
                    .sum();
                let spent_7d: u64 = merchant
                    .circuit_spent_hist
                    .iter()
                    .filter(|(d, _)| world.day.saturating_sub(*d) <= window)
                    .map(|(_, a)| a)
                    .sum();
                let bound = received_7d * world.params.circuit_fraction_bps / 10_000;
                let headroom = bound.saturating_sub(spent_7d);
                if headroom == 0 {
                    continue;
                }
                let amount = confirmed.min(headroom);
                if amount == 0 {
                    continue;
                }
                let target = {
                    let t = rngs[zi].below(merchants - 1);
                    if t >= m {
                        t + 1
                    } else {
                        t
                    }
                };
                // Errors (bounds already reached) are part of the protocol:
                // a failed circuit is simply not executed.
                let _ = world.circuit_spend(zone, m, target, amount);
            }

            // 3. End-of-day conversion: each merchant converts the v
            //    fraction of its confirmed credit (Annexes F.3 to F.5).
            for m in 0..s.merchants {
                let zi_world = world
                    .zones
                    .iter()
                    .position(|z| z.id == zone)
                    .expect("zone exists");
                let confirmed = world.zones[zi_world].merchants[m as usize].confirmed_total();
                if confirmed == 0 {
                    continue;
                }
                let requested = confirmed * s.convert_bps / 10_000;
                if let Ok(receipt) = world.convert(zone, m, requested) {
                    let slot = match receipt.capped_by {
                        CapReason::NotCapped | CapReason::Request => 0,
                        CapReason::PhysicalCap => 1,
                        CapReason::PlausibilityCap => 2,
                        CapReason::TierCap => 3,
                    };
                    cap_counts[zi][slot] += 1;
                }
            }
        }

        for (zi, s) in specs.iter().enumerate() {
            coverage[(day - 1) as usize][zi] = world.coverage_ratio(s.id);
        }
    }

    // Fold the history into one row per (day, zone).
    for (d, zid, row) in &world.history {
        let zi = specs.iter().position(|s| s.id == *zid).expect("known zone");
        let cell = &mut grid[(*d - 1) as usize][zi];
        cell.spent_mexz += row.spent_mexz;
        cell.converted_cents += row.converted_cents;
        cell.commission_cents += row.commission_cents;
        cell.queued_cents += row.queued_cents;
        cell.melted_mexz += row.melted_mexz;
        cell.excess_mexz += row.excess_mexz;
        cell.reverted_mexz += row.reverted_mexz;
    }

    // ---- outputs ----------------------------------------------------------

    let out_dir = Path::new(&args.out);
    fs::create_dir_all(out_dir).expect("create out dir");

    let mut csv = String::from("day,zone,spent_mexz,converted_cents,commission_cents,queued_cents,melted_mexz,excess_to_fund_mexz,reverted_mexz,coverage_ratio\n");
    for (di, row_zones) in grid.iter().enumerate() {
        for (zi, cell) in row_zones.iter().enumerate() {
            let cov = coverage[di][zi];
            let cov = if cov.is_finite() { format!("{:.3}", cov) } else { "inf".to_string() };
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                di + 1,
                specs[zi].name,
                cell.spent_mexz,
                cell.converted_cents,
                cell.commission_cents,
                cell.queued_cents,
                cell.melted_mexz,
                cell.excess_mexz,
                cell.reverted_mexz,
                cov
            ));
        }
    }
    fs::write(out_dir.join("exz_sim.csv"), csv).expect("write csv");

    let svg = build_svg(&specs, &grid, &coverage, days);
    fs::write(out_dir.join("exz_sim.svg"), svg).expect("write svg");

    // ---- stdout summary ---------------------------------------------------

    println!("exz-sim: {} days, seed {}, {} zones", days, args.seed, n_zones);
    println!();

    for (zi, s) in specs.iter().enumerate() {
        let converted: i64 = world.zones
            [world.zones.iter().position(|z| z.id == s.id).unwrap()]
            .merchants
            .iter()
            .map(|m| m.converted_cents_total)
            .sum();
        let queued: i64 = world.zones
            [world.zones.iter().position(|z| z.id == s.id).unwrap()]
            .merchants
            .iter()
            .map(|m| m.queued_cents)
            .sum();
        let totals: DayStats = grid.iter().map(|r| r[zi]).fold(DayStats::default(), |a, b| DayStats {
            spent_mexz: a.spent_mexz + b.spent_mexz,
            converted_cents: a.converted_cents + b.converted_cents,
            commission_cents: a.commission_cents + b.commission_cents,
            queued_cents: a.queued_cents + b.queued_cents,
            melted_mexz: a.melted_mexz + b.melted_mexz,
            excess_mexz: a.excess_mexz + b.excess_mexz,
            reverted_mexz: a.reverted_mexz + b.reverted_mexz,
        });

        let eff_ppy = converted as f64 / 100.0 / s.persons as f64 * (365.0 / days as f64);
        let proj_ppy = d_proj[zi] * 365.0 / s.persons as f64 / 100.0;
        let final_cov = coverage[(days - 1) as usize][zi];

        println!("zone {} (id {}): {} persons, {} merchants, meal {}c", s.name, s.id, s.persons, s.merchants, s.meal_price_cents);
        println!("  spent (single-hop)          {:>14.2} EXZ", totals.spent_mexz as f64 / 1000.0);
        println!("  converted (paid)           {:>14.2} USD", converted as f64 / 100.0);
        println!("  commission charged          {:>14.2} USD", totals.commission_cents as f64 / 100.0);
        println!("  queued (rationed)           {:>14.2} USD", queued as f64 / 100.0);
        println!("  merchant credit reverted    {:>14.2} EXZ (7-day window)", totals.reverted_mexz as f64 / 1000.0);
        println!("  cap overflow to fund        {:>14.2} EXZ", totals.excess_mexz as f64 / 1000.0);
        println!("  melted (idle persons)       {:>14.2} EXZ", totals.melted_mexz as f64 / 1000.0);
        println!("  conversion caps hit         {:>14} physical / {} plausible / {} tier",
            cap_counts[zi][1], cap_counts[zi][2], cap_counts[zi][3]);
        println!("  effective payout            {:>14.2} USD/person/year", eff_ppy);
        println!("  Annex G.1 projection        {:>14.2} USD/person/year", proj_ppy);
        println!("  final coverage ratio        {:>14.3}", if final_cov.is_finite() { final_cov } else { -1.0 });
        println!();
    }

    println!("global fund (EXZ commons, melting suspended): {:.2} EXZ", world.fund.exz as f64 / 1000.0);
    println!("global coverage ratio: {:.3}", world.global_coverage_ratio());

    // Sanity check on the pilot zone: the simulated effective payout should
    // stay within 15% of the Annex G.1 projection for the same parameters.
    let pilot = 0;
    let s = &specs[pilot];
    let converted: i64 = world.zones
        [world.zones.iter().position(|z| z.id == s.id).unwrap()]
        .merchants
        .iter()
        .map(|m| m.converted_cents_total)
        .sum();
    let eff_ppy = converted as f64 / 100.0 / s.persons as f64 * (365.0 / days as f64);
    let proj_ppy = d_proj[pilot] * 365.0 / s.persons as f64 / 100.0;
    let dev = (eff_ppy - proj_ppy).abs() / proj_ppy;
    if dev <= 0.15 {
        println!("pilot sanity check: PASS (effective {:.2} vs projected {:.2} USD/person/year, deviation {:.1}%)", eff_ppy, proj_ppy, dev * 100.0);
    } else {
        println!("pilot sanity check: FAIL (effective {:.2} vs projected {:.2} USD/person/year, deviation {:.1}%)", eff_ppy, proj_ppy, dev * 100.0);
    }
    println!();
    println!("artifacts: {}/exz_sim.csv, {}/exz_sim.svg", args.out, args.out);
}

/// Hand-rolled SVG line charts, no dependencies. Two stacked panels:
/// coverage ratio per zone (reference line at 1.0), and daily converted
/// payout per zone.
fn build_svg(specs: &[ZoneSpec], grid: &[Vec<DayStats>], coverage: &[Vec<f64>], days: u64) -> String {
    let width = 960.0;
    let panel_h = 360.0;
    let margin_l = 70.0;
    let margin_r = 20.0;
    let margin_t = 34.0;
    let plot_h = panel_h - margin_t - 40.0;
    let total_h = panel_h * 2.0 + 20.0;

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{0}\" height=\"{1}\" viewBox=\"0 0 {0} {1}\" font-family=\"system-ui, sans-serif\" font-size=\"12\">\n",
        width as u32, total_h as u32
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#faf7f1\"/>\n");

    let x_of = |day: usize| -> f64 {
        margin_l + (day as f64 / (days as f64 - 1.0).max(1.0)) * (width - margin_l - margin_r)
    };

    // Panel 1: coverage ratio.
    let y1 = 0.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Coverage ratio per zone (R / (H x D-hat), Annex G.3)</text>\n",
        margin_l, y1 + 20.0
    ));
    let mut cov_max = 0.0f64;
    for row in coverage {
        for v in row {
            if v.is_finite() && *v > cov_max {
                cov_max = *v;
            }
        }
    }
    cov_max = (cov_max.max(1.0) * 1.1).min(4.0);
    let y_cov = |v: f64| -> f64 { y1 + panel_h - 40.0 - (v / cov_max).clamp(0.0, 1.0) * plot_h };
    // axes
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t}\" x2=\"{m}\" y2=\"{b}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l,
        t = y1 + margin_t,
        b = y1 + panel_h - 40.0
    ));
    for (label, val) in [("0", 0.0f64), ("1.0", 1.0f64), (&format!("{:.1}", cov_max), cov_max)] {
        let v = if val > cov_max { cov_max } else { val };
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{}</text>\n",
            margin_l - 8.0,
            y_cov(v) + 4.0,
            label
        ));
    }
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0,
        y1 + panel_h - 12.0
    ));
    // reference line at coverage = 1.0
    if cov_max >= 1.0 {
        svg.push_str(&format!(
            "<line x1=\"{m}\" y1=\"{y:.1}\" x2=\"{e:.1}\" y2=\"{y:.1}\" stroke=\"#3a230d\" stroke-dasharray=\"6 4\" stroke-width=\"1\" opacity=\"0.6\"/>\n",
            m = margin_l,
            y = y_cov(1.0),
            e = width - margin_r
        ));
    }
    for (zi, _s) in specs.iter().enumerate() {
        let mut pts = String::new();
        for (di, row) in coverage.iter().enumerate() {
            let v = row[zi];
            let v = if v.is_finite() { v.min(cov_max) } else { cov_max };
            pts.push_str(&format!("{:.1},{:.1} ", x_of(di), y_cov(v)));
        }
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            pts.trim_end(),
            CHART_COLORS[zi % 3]
        ));
    }
    // legend
    let mut lx = margin_l;
    for (zi, s) in specs.iter().enumerate() {
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
            lx,
            y1 + 26.0,
            CHART_COLORS[zi % 3]
        ));
        svg.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{}\" fill=\"#3a230d\">{}</text>\n",
            lx + 17.0,
            y1 + 37.0,
            s.name
        ));
        lx += 17.0 + 14.0 * s.name.len() as f64;
    }

    // Panel 2: daily converted payout.
    let y2 = panel_h + 20.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Daily converted payout per zone (USD cents)</text>\n",
        margin_l,
        y2 + 20.0
    ));
    let mut pay_max = 1.0f64;
    for row in grid {
        for cell in row {
            if cell.converted_cents > pay_max as i64 {
                pay_max = cell.converted_cents as f64;
            }
        }
    }
    pay_max *= 1.1;
    let y_pay = |v: f64| -> f64 { y2 + panel_h - 40.0 - (v / pay_max).clamp(0.0, 1.0) * plot_h };
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t}\" x2=\"{m}\" y2=\"{b}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l,
        t = y2 + margin_t,
        b = y2 + panel_h - 40.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">0</text>\n",
        margin_l - 8.0,
        y_pay(0.0) + 4.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{:.0}c</text>\n",
        margin_l - 8.0,
        y_pay(pay_max / 1.1) + 4.0,
        pay_max / 1.1
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0,
        y2 + panel_h - 12.0
    ));
    for (zi, _s) in specs.iter().enumerate() {
        let mut pts = String::new();
        for (di, row) in grid.iter().enumerate() {
            pts.push_str(&format!(
                "{:.1},{:.1} ",
                x_of(di),
                y_pay(row[zi].converted_cents as f64)
            ));
        }
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            pts.trim_end(),
            CHART_COLORS[zi % 3]
        ));
    }
    let mut lx = margin_l;
    for (zi, s) in specs.iter().enumerate() {
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
            lx,
            y2 + 26.0,
            CHART_COLORS[zi % 3]
        ));
        svg.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{}\" fill=\"#3a230d\">{}</text>\n",
            lx + 17.0,
            y2 + 37.0,
            s.name
        ));
        lx += 17.0 + 14.0 * s.name.len() as f64;
    }

    svg.push_str("</svg>\n");
    svg
}
