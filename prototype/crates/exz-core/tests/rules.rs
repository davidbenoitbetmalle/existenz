//! One test per founding rule of the protocol, mapped to the white paper.

use exz_core::*;

fn world_with_zone(allocated: Cents, inflow: Cents) -> World {
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 60, 2_000, 1_000_000, 1_000_000_000, allocated, inflow)
        .unwrap();
    w
}

#[test]
fn equal_issuance_and_cap_excess_flows_to_fund() {
    // Section 4 / 5.2: same issuance for everyone; above the cap,
    // units flow to the global fund.
    let mut w = world_with_zone(10_000_000, 0);
    w.enroll_person(1, 10).unwrap();
    w.enroll_person(1, 11).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..10 {
        w.new_day();
    }
    let cap = w.params.cap_mexz();
    let p0 = w.zones[0].persons[0].balance;
    let p1 = w.zones[0].persons[1].balance;
    assert_eq!(p0, p1, "issuance is equal");
    assert_eq!(p0, cap, "balances saturate at the cap");
    assert!(w.fund.exz > 0, "excess above the cap reached the fund");
    assert_eq!(w.fund.exz, w.stats.excess_to_fund_mexz);
}

#[test]
fn melting_idle_balances_above_the_floor() {
    // Section 5.1 / Annex C.1: idle balances above the floor melt; balances
    // at or below the floor are protected by design.
    //
    // Implementation note, honest and worth stating: with the paper's
    // proposed values (floor 14 days = 28 EXZ, cap 2 days = 4 EXZ), the
    // exemption floor dominates the accumulation cap, so person-level
    // melting is vacuous: the cap, not melting, does the anti-accumulation
    // work for individuals. The mechanism is therefore exercised here with
    // a parametric floor set below the cap (Annex D makes the floor a
    // parametric rule); the cap still guards accumulation.
    let mut params = Params::default();
    params.floor_days = 1; // floor = 2000 mEXZ, cap = 4000 mEXZ
    let mut w = World::new(params);
    w.add_zone(1, "test", 60, 2_000, 1_000_000, 1_000_000_000, 10_000_000, 0)
            .unwrap();
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..40 {
        w.new_day();
    }
    let p = &w.zones[0].persons[0];
    assert!(p.balance < w.params.cap_mexz(), "an idle hoarder melts below the cap");
    assert!(w.stats.melted_mexz > 0);

    // With a floor above the cap, melting never touches persons: the
    // paper's proposed configuration, verified as such.
    let mut w2 = world_with_zone(10_000_000, 0);
    w2.enroll_person(1, 10).unwrap();
    w2.enroll_merchant(1, 99).unwrap();
    for _ in 0..5 {
        w2.new_day();
    }
    let bal = w2.zones[0].persons[0].balance;
    for _ in 0..5 {
        w2.new_day();
    }
    assert_eq!(w2.zones[0].persons[0].balance, w2.params.cap_mexz());
    assert_eq!(w2.stats.melted_mexz, 0, "floor above cap: person melting is vacuous");
}

#[test]
fn activity_resets_the_melting_clock() {
    let mut params = Params::default();
    params.floor_days = 1;
    let mut w = World::new(params);
    w.add_zone(1, "test", 60, 2_000, 1_000_000, 1_000_000_000, 10_000_000, 0)
            .unwrap();
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..30 {
        w.new_day();
    }
    let melted_before = w.stats.melted_mexz;
    assert!(melted_before > 0);
    // Spend once: the idle clock resets, so the next day starts a new
    // idle period of melt_idle_days before melting resumes.
    w.spend(1, 10, 99, 1_000).unwrap();
    w.new_day();
    let spent_issued = w.zones[0].persons[0].balance;
    w.new_day();
    assert!(w.zones[0].persons[0].balance < spent_issued + w.params.daily_issuance_mexz);
    assert!(w.zones[0].persons[0].last_active_day == w.day - 2);
}

#[test]
fn fund_melting_is_suspended() {
    // Section 5.1: the global fund accumulates; melting is suspended there.
    let mut w = world_with_zone(10_000_000, 0);
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..30 {
        w.new_day();
    }
    let fund_exz = w.fund.exz;
    assert!(fund_exz > 0);
    let before = w.fund.exz;
    w.new_day();
    // The fund only grows (from new excess/melting), never melts.
    assert!(w.fund.exz >= before);
}

#[test]
fn single_hop_person_to_merchant_only() {
    // Annex F.1: EXZ circulates exactly one hop. There is no operation
    // that moves merchant earnings to a person, and settlement credit
    // never re-enters anyone's balance.
    let mut w = world_with_zone(10_000_000, 0);
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    w.new_day();
    w.spend(1, 10, 99, 2_000).unwrap();
    let person_balance = w.zones[0].persons[0].balance;
    let merchant_wallet = w.zones[0].merchants[0].existence.balance;
    // The spend landed in provisional settlement credit, not the wallet.
    assert_eq!(w.zones[0].merchants[0].provisional.iter().map(|b| b.0).sum::<Mexz>(), 2_000);
    assert_eq!(w.zones[0].merchants[0].existence.balance, merchant_wallet);
    assert_eq!(w.zones[0].persons[0].balance, person_balance);
    // A dead person cannot spend.
    w.settle_death(1, 10).unwrap();
    assert!(matches!(w.spend(1, 10, 99, 1), Err(ExzError::DeceasedAccount)));
    // Insufficient balance is refused.
    w.enroll_person(1, 12).unwrap();
    assert!(matches!(w.spend(1, 12, 99, 4_000), Err(ExzError::InsufficientBalance)));
}

#[test]
fn settlement_credit_confirms_then_reverts() {
    // Annex F.2: provisional credit confirms after the window; confirmed
    // credit unconverted past the 7-day operational window reverts to the
    // fund (melting back to the commons instead of draining it).
    let mut w = world_with_zone(10_000_000, 0);
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    w.new_day();
    w.spend(1, 10, 99, 2_000).unwrap();
    assert_eq!(w.zones[0].merchants[0].confirmed_total(), 0);
    w.new_day(); // confirm_days = 1: the purchase is now confirmed
    assert_eq!(w.zones[0].merchants[0].confirmed_total(), 2_000);
    let fund_before = w.fund.exz;
    for _ in 0..8 {
        w.new_day(); // never converted: past the window it reverts
    }
    assert_eq!(w.zones[0].merchants[0].confirmed_total(), 0, "credit reverted to the fund");
    assert!(w.fund.exz >= fund_before);
    assert!(w.stats.reverted_mexz >= 2_000);
}

#[test]
fn cap_one_physical_binds_conversion() {
    // Annex F.4 Cap 1: convertible volume cannot exceed unique validated
    // presences x daily allowance, whatever the declared amounts.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.enroll_merchant(1, 99).unwrap();
    for i in 0..5 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day();
    for i in 0..5 {
        w.spend(1, 100 + i, 99, 2_000).unwrap();
    }
    w.new_day();
    // The merchant fabricates extra credit by some other channel and
    // requests a huge conversion: Cap 1 binds to 5 x 2000.
    let r = w.convert(1, 99, 100_000).unwrap();
    assert!(r.converted_mexz <= 5 * 2_000, "physical cap binds");
    assert_eq!(r.capped_by, CapReason::PhysicalCap);
}

#[test]
fn cap_two_plausibility_binds_conversion() {
    // Annex F.4 Cap 2: a sandwich cart cannot serve the whole zone.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.add_zone(2, "dense", 60, 10, 1_000_000, 1_000_000_000, 1_000_000_000, 0)
            .unwrap();
    w.enroll_merchant(2, 50).unwrap();
    for i in 0..30 {
        w.enroll_person(2, 200 + i).unwrap();
    }
    w.new_day();
    for i in 0..30 {
        w.spend(2, 200 + i, 50, 2_000).unwrap();
    }
    w.new_day();
    let r = w.convert(2, 50, 100_000).unwrap();
    // plausible_population = 10, so Cap 2 = 10 x 2000 = 20000.
    assert!(r.converted_mexz <= 10 * 2_000);
    assert_eq!(r.capped_by, CapReason::PlausibilityCap);
}

#[test]
fn cap_three_tier_grows_with_integrity() {
    // Annex F.4 Cap 3: a fresh merchant is bounded at 100 daily
    // allowances; clean days raise the integrity index and the tier.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.add_zone(2, "grow", 60, 10_000, 1_000_000, 1_000_000_000, 1_000_000_000, 0)
            .unwrap();
    w.enroll_merchant(2, 50).unwrap();
    for i in 0..500 {
        w.enroll_person(2, 200 + i).unwrap();
    }
    w.new_day();
    for i in 0..500 {
        w.spend(2, 200 + i, 50, 2_000).unwrap();
    }
    w.new_day();
    let r1 = w.convert(2, 50, 10_000_000).unwrap();
    assert!(r1.converted_mexz <= 100 * 2_000, "fresh merchant at the floor tier");
    assert_eq!(r1.capped_by, CapReason::TierCap);
    let integrity_first = w.zones.iter().find(|z| z.id == 2).unwrap().merchants[0].integrity_bps;
    assert!(integrity_first > 0, "a clean conversion raises integrity");
    for _ in 0..40 {
        w.new_day();
        let _ = w.convert(2, 50, 10_000_000).unwrap();
    }
    assert!(
        w.zones.iter().find(|z| z.id == 2).unwrap().merchants[0].integrity_bps > 9_000,
        "integrity converges to 1.0"
    );
}

#[test]
fn commission_is_paid_by_the_fund_and_capped() {
    // Annex F.5: 3% of each confirmed transaction, paid by the network,
    // capped at 50 daily allowances per day.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.enroll_merchant(1, 99).unwrap();
    for i in 0..20 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day();
    for i in 0..20 {
        w.spend(1, 100 + i, 99, 2_000).unwrap();
    }
    w.new_day();
    let allocated_before = w.fund.zones.get(&1).unwrap().allocated_cents;
    let r = w.convert(1, 99, 40_000).unwrap();
    assert_eq!(r.converted_mexz, 40_000);
    // gross = 40 EXZ x $0.60 = 2400 cents; commission = min(3%, cap 50x120).
    assert_eq!(r.gross_cents, 2_400);
    assert_eq!(r.commission_cents, 72); // 3% of 2400, well under the cap
    let spent = allocated_before - w.fund.zones.get(&1).unwrap().allocated_cents;
    assert_eq!(spent, 2_472);

    // Cap: a very large day is capped at 50 allowances x 120 cents = 6000.
    // 2,000 presences x 2 EXZ = 4,000 EXZ: gross 240,000 cents, 3% = 7,200
    // uncapped, so the 6,000-cent cap binds. The tier cap is raised
    // parametrically here so that Cap 3 does not bind before the commission
    // cap does (a fresh merchant would otherwise stop at 100 allowances).
    let mut params2 = Params::default();
    params2.tier_initial_allowances = 5_000;
    let mut w2 = World::new(params2);
    w2.add_zone(1, "test", 60, 2_000, 1_000_000, 1_000_000_000, 1_000_000_000, 0)
            .unwrap();
    w2.enroll_merchant(1, 99).unwrap();
    for i in 0..2_000 {
        w2.enroll_person(1, 100 + i).unwrap();
    }
    w2.new_day();
    for i in 0..2_000 {
        w2.spend(1, 100 + i, 99, 2_000).unwrap();
    }
    w2.new_day();
    let r2 = w2.convert(1, 99, 4_000_000).unwrap();
    assert_eq!(r2.gross_cents, 240_000);
    assert_eq!(r2.commission_cents, 6_000, "commission capped at 50 allowances");
}

#[test]
fn conversion_respects_the_coverage_ration_and_queues() {
    // Annex G.3: while covered, same-day payment; below 1, conversion
    // rations by transparent queue within the steady-state daily budget.
    // 50 presences a day = 100 EXZ confirmed a day, due = 6,180 cents
    // (gross 6,000 + commission 180). The zone is endowed for exactly
    // H x D at first, but the inflow covers only 2,000 cents a day: the
    // allocation must eventually degrade, and the queue must appear.
    let due_daily = 6_180i64;
    let mut w = world_with_zone(90 * due_daily, 2_000);
    w.enroll_merchant(1, 99).unwrap();
    for i in 0..50 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    let mut queued_total = 0i64;
    let mut paid_total = 0i64;
    for _ in 0..80 {
        w.new_day();
        for i in 0..50 {
            w.spend(1, 100 + i, 99, 2_000).unwrap();
        }
        let r = w.convert(1, 99, 200_000).unwrap();
        if w.day >= 2 {
            assert!(r.converted_mexz > 0, "cap band allows the declared day");
        }
        queued_total += r.queued_cents;
        paid_total += r.gross_cents;
    }
    assert!(paid_total > 0, "covered days pay same-day");
    assert!(queued_total > 0, "once coverage degrades, requests queue");
    assert!(!w.fund.zones.get(&1).unwrap().queue.is_empty());
    assert!(w.coverage_ratio(1) < 1.0, "the zone is visibly under-covered");
    let merchant = w.zones.iter().find(|z| z.id == 1).unwrap().merchants[0].clone();
    assert!(merchant.queued_cents > 0);
    assert!(merchant.converted_cents_total > 0, "partial payments flow within the budget");
}

#[test]
fn rollout_gate_requires_horizon_coverage() {
    // Annex G.3 inflow rule: a zone is onboarded only when its H x D is
    // endowment-covered.
    let d = projected_daily_payout_cents(100_000, 0.6, 0.8, 60, 300);
    let horizon = 90u64;
    let required = d * horizon as f64;
    assert!(!World::can_onboard((required as i64) - 1, d, horizon));
    assert!(World::can_onboard(required as i64, d, horizon));
}

#[test]
fn death_settlement_joins_the_fund() {
    // Section 5.2: no heir, no estate; the balance joins the commons.
    let mut w = world_with_zone(10_000_000, 0);
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..3 {
        w.new_day();
    }
    w.freeze_to_subwallet(1, 10, "health", 1_000).unwrap();
    let expected = w.zones[0].persons[0].balance + 1_000;
    let fund_before = w.fund.exz;
    w.settle_death(1, 10).unwrap();
    assert_eq!(w.fund.exz, fund_before + expected);
    assert!(!w.zones[0].persons[0].alive);
    assert_eq!(w.zones[0].persons[0].balance, 0);
    assert!(w.zones[0].persons[0].subwallets.is_empty());
    // A deceased account cannot freeze either.
    assert!(matches!(
        w.freeze_to_subwallet(1, 10, "x", 1),
        Err(ExzError::DeceasedAccount)
    ));
}

#[test]
fn subwallets_are_bounded_and_melt_free() {
    // Annex C.2: up to 60 days of issuance, melting suspended inside.
    let mut params = Params::default();
    params.cap_days = 50; // cap = 100 EXZ, so a balance can accumulate
    params.floor_days = 1;
    params.melt_bps_per_day = 500;
    params.subwallet_max_days = 11; // 22,000 mEXZ, reachable within the balance
    let mut w = World::new(params.clone());
    w.add_zone(1, "test", 60, 2_000, 1_000_000, 1_000_000_000, 10_000_000, 0)
            .unwrap();
    w.enroll_person(1, 10).unwrap();
    w.enroll_merchant(1, 99).unwrap();
    for _ in 0..30 {
        w.new_day();
    }
    let max = params.subwallet_max_mexz();
    assert_eq!(max, 22_000);
    let bal = w.zones[0].persons[0].balance;
    assert!(bal > 22_000, "the hoarder accumulated above 11 EXZ, got {}", bal);
    w.freeze_to_subwallet(1, 10, "health", 20_000).unwrap();
    let frozen = w.zones[0].persons[0].subwallets["health"];
    assert_eq!(frozen, 20_000);
    // The bound refuses to exceed the sub-wallet maximum in total.
    assert!(matches!(
        w.freeze_to_subwallet(1, 10, "repair", 2_001),
        Err(ExzError::SubwalletLimitExceeded)
    ));
    // Melting is suspended on the frozen amount.
    for _ in 0..10 {
        w.new_day();
    }
    assert_eq!(w.zones[0].persons[0].subwallets["health"], frozen);
    // Spending from the sub-wallet respects the single hop.
    w.spend_from_subwallet(1, 10, "health", 99, 2_000).unwrap();
    assert_eq!(w.zones[0].persons[0].subwallets["health"], frozen - 2_000);
    assert_eq!(
        w.zones[0].merchants[0].provisional.iter().map(|b| b.0).sum::<Mexz>(),
        2_000
    );
    assert!(matches!(
        w.spend_from_subwallet(1, 10, "nonexistent", 99, 1),
        Err(ExzError::UnknownSubwallet)
    ));
}

#[test]
fn merchant_circuit_is_bounded_to_the_weekly_fraction() {
    // Annex H.2: at most 20% of the weekly received credit, and the
    // credit never leaves the settlement-credit state.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.enroll_merchant(1, 99).unwrap();
    w.enroll_merchant(1, 100).unwrap();
    for i in 0..10 {
        w.enroll_person(1, 200 + i).unwrap();
    }
    w.new_day();
    for i in 0..10 {
        w.spend(1, 200 + i, 99, 2_000).unwrap();
    }
    w.new_day();
    // Merchant 99 received 20 EXZ this week: circuit bound = 4 EXZ.
    assert!(matches!(w.circuit_spend(1, 99, 100, 4_001), Err(ExzError::CircuitLimitExceeded)));
    w.circuit_spend(1, 99, 100, 4_000).unwrap();
    // The receiving merchant holds provisional settlement credit, and
    // the sender's existence wallet was untouched.
    assert_eq!(
        w.zones[0].merchants.iter().find(|m| m.id == 100).unwrap().provisional.iter().map(|b| b.0).sum::<Mexz>(),
        4_000
    );
    let m99 = w.zones[0].merchants.iter().find(|m| m.id == 99).unwrap();
    assert_eq!(m99.existence.balance, 4_000, "existence wallet untouched by circuit (2 days of issuance)");
    // No self-circuit.
    assert!(matches!(w.circuit_spend(1, 99, 99, 1), Err(ExzError::Empty)));
}

#[test]
fn fabricated_volume_returns_to_the_fund_and_slashes_integrity() {
    // Annex F.6: dishonesty is corrected; units return to the commons and
    // the tier cap falls.
    let mut w = world_with_zone(1_000_000_000, 0);
    w.enroll_merchant(1, 99).unwrap();
    for i in 0..10 {
        w.enroll_person(1, 200 + i).unwrap();
    }
    w.new_day();
    for i in 0..10 {
        w.spend(1, 200 + i, 99, 2_000).unwrap();
    }
    w.new_day();
    let _ = w.convert(1, 99, 4_000).unwrap(); // raises integrity
    let integrity = w.zones[0].merchants[0].integrity_bps;
    let fund_before = w.fund.exz;
    w.flag_fabricated(1, 99, 2_000).unwrap();
    assert!(w.zones[0].merchants[0].integrity_bps < integrity);
    assert_eq!(w.fund.exz, fund_before + 2_000);
}

#[test]
fn annex_g_formula_reproduces_the_white_paper_table() {
    // G.2/G.5: N = 100k, u = 60%, v = 80%, p = $0.60 => ~22 M USD/yr,
    // ~220 USD per person per year (order of magnitude, within 5%).
    let d = projected_daily_payout_cents(100_000, 0.6, 0.8, 60, 300);
    let year_usd = d * 365.0 / 100.0;
    assert!((year_usd - 21.65e6).abs() / 21.65e6 < 0.05, "yearly payout {}", year_usd);
    let per_person = year_usd / 100_000.0;
    assert!(per_person > 200.0 && per_person < 240.0, "per person per year {}", per_person);
    // Global scale check: 8.1 B, u=60%, v=80%: the G.2 table's ~4.8 B USD/day.
    let dg = projected_daily_payout_cents(8_100_000_000, 0.6, 0.8, 60, 300);
    assert!((dg / 100.0 - 4.8e9).abs() / 4.8e9 < 0.05, "global daily {}", dg / 100.0);
}

#[test]
fn coverage_uses_daily_payout_totals_not_event_averages() {
    // Regression test, found by the exz-sim multi-zone run: the payout
    // history must hold one TOTAL per complete day, not one entry per
    // conversion event. The per-event version fed the coverage ratio an
    // event-sized daily estimate (D-hat divided by the number of daily
    // events), making a zone look far better covered than it was and
    // letting an under-endowed zone overspend its allocation into
    // negative values. Three merchants converting on the same day must
    // collapse into a single 618-cent day total.
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 100, 1_000, 1_000_000, 1_000_000_000, 1_000_000, 1_000)
        .unwrap();
    for m in [10u64, 20, 30] {
        w.enroll_merchant(1, m).unwrap();
    }
    for i in 0..3u64 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day(); // day 1: everyone spends at their merchant
    for (i, m) in [10u64, 20, 30].iter().enumerate() {
        w.spend(1, 100 + i as u64, *m, 2_000).unwrap();
    }
    w.new_day(); // day 2: all three purchases confirmed
    for m in [10u64, 20, 30] {
        let r = w.convert(1, m, 2_000).unwrap();
        assert_eq!(r.queued_cents, 0, "day-2 zone is still trivially covered");
        assert_eq!(r.gross_cents, 200);
    }
    // Three same-day payouts of 206 cents (gross 200 + commission 6).
    w.new_day(); // day 3: the day-2 total 618 is flushed as ONE entry
    let zf = w.fund.zones.get(&1).unwrap();
    let hist: Vec<i64> = zf.payout_hist.iter().copied().collect();
    assert_eq!(hist, vec![0, 0, 618], "one total per complete day");
    assert_eq!(zf.payout_today, 0, "the daily accumulator is reset on flush");
    // The estimate averages the three complete days, 618 / 3.
    let estimate = w.daily_payout_estimate(1);
    assert!((estimate - 206.0).abs() < 1e-9, "daily estimate {}, expected 206", estimate);
    // Coverage from daily totals: 1M + 3 x 1000 inflow, minus 618 paid,
    // over H x D-hat.
    let coverage = w.coverage_ratio(1);
    let expected = 1_002_382.0 / (90.0 * 206.0);
    assert!((coverage - expected).abs() < 1e-6, "coverage from daily totals, got {}", coverage);
}

#[test]
fn ration_budget_is_shared_across_the_day() {
    // Regression test, found by the exz-sim multi-zone run: the ration
    // budget is the zone's steady-state DAILY inflow, shared by every
    // conversion request of the day. The buggy version gave each request
    // a fresh full budget, so an under-covered zone could pay up to
    // (number of merchants) times its inflow each day. Here three
    // merchants ask 206 cents each under a 300-cent daily budget: the
    // first is paid in full, the second partially, the third nothing.
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 100, 10_000, 1_000_000, 1_000_000_000, 1_000, 300)
        .unwrap();
    for m in [10u64, 20, 30] {
        w.enroll_merchant(1, m).unwrap();
    }
    for i in 0..3u64 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day(); // day 1: everyone spends at their merchant
    for (i, m) in [10u64, 20, 30].iter().enumerate() {
        w.spend(1, 100 + i as u64, *m, 2_000).unwrap();
    }
    w.new_day(); // day 2: still trivially covered, all three convert
    for m in [10u64, 20, 30] {
        let r = w.convert(1, m, 2_000).unwrap();
        assert_eq!(r.queued_cents, 0);
    }
    // Day 2 spends again, so day 3 has confirmed credit to convert.
    for (i, m) in [10u64, 20, 30].iter().enumerate() {
        w.spend(1, 100 + i as u64, *m, 2_000).unwrap();
    }
    w.new_day(); // day 3: coverage now below 1 (tiny endowment)
    assert!(w.coverage_ratio(1) < 1.0, "zone must be under-covered");
    assert_eq!(w.fund.zones.get(&1).unwrap().ration_left, 300);
    let r1 = w.convert(1, 10, 2_000).unwrap();
    assert_eq!(r1.queued_cents, 0, "first request paid in full");
    assert_eq!(r1.gross_cents, 200);
    let r2 = w.convert(1, 20, 2_000).unwrap();
    let r2_paid = r2.gross_cents + r2.commission_cents - r2.queued_cents;
    assert_eq!(r2_paid, 94, "second request gets the budget remainder");
    assert_eq!(r2.queued_cents, 112, "second request queues the unpaid part");
    let r3 = w.convert(1, 30, 2_000).unwrap();
    let r3_paid = r3.gross_cents + r3.commission_cents - r3.queued_cents;
    assert_eq!(r3_paid, 0, "third request: budget exhausted");
    assert_eq!(r3.queued_cents, 206, "third request fully queued");
    assert_eq!(w.fund.zones.get(&1).unwrap().ration_left, 0);
}

#[test]
fn tanh_fixed_matches_reference_values() {
    // The soft-cap kernel of the solidarity scale rule, checked against
    // independently computed values.
    let close = |num: u64, den: u64, expected: f64| {
        let got = exz_core::tanh_fixed(num, den) as f64 / 1e12;
        assert!(
            (got - expected).abs() < 1e-9,
            "tanh({}/{}) = {}, expected {}",
            num, den, got, expected
        );
    };
    close(1, 1, 0.7615941559557649);
    close(20, 30, 0.5827829453479102);
    close(50, 30, 0.9311096086675776);
    close(100, 30, 0.9974579674738373);
    close(9, 1, 0.999999969540041);
    // Identity near zero, where small merchants must be counted whole.
    let small = exz_core::tanh_fixed(50, 1_000_000) as f64 / 1e12;
    assert!((small - 5e-5).abs() < 1e-9);
    // Saturation at one for large ratios.
    assert_eq!(exz_core::tanh_fixed(10, 1), 1_000_000_000_000);
    assert_eq!(exz_core::tanh_fixed(1_000, 1), 1_000_000_000_000);
}

#[test]
fn scale_cap_counts_declared_covers_with_tanh() {
    // Revised F.4, step 1: a 30-cover table counts 20 declared covers as
    // round(30 x tanh(20/30)) = 17. The cap binds below what was received.
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 100, 1_000_000, 30, 1_000_000_000, 1_000_000_000, 1_000_000)
        .unwrap();
    w.enroll_merchant(1, 10).unwrap();
    for i in 0..20u64 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day();
    for i in 0..20u64 {
        w.spend(1, 100 + i, 10, 2_000).unwrap();
    }
    w.new_day();
    let r = w.convert(1, 10, 200_000).unwrap();
    assert_eq!(r.converted_mexz, 17 * 2_000, "17 counted covers");
    assert_eq!(r.capped_by, CapReason::PhysicalCap);
}

#[test]
fn a_hundred_cover_restaurant_counts_no_more_than_its_table() {
    // Revised F.4, step 1 at saturation: a 30-cover table counts 50
    // declared covers as 28 and 100 as 30: doubling the clientele past the
    // table adds almost nothing. The big venue has no rational reason to
    // absorb the flow.
    let mut run = |covers: u64| -> (u64, CapReason) {
        let mut w = World::new(Params::default());
        w.add_zone(1, "test", 100, 1_000_000, 30, 1_000_000_000, 1_000_000_000, 1_000_000)
            .unwrap();
        w.enroll_merchant(1, 10).unwrap();
        for i in 0..covers {
            w.enroll_person(1, 100 + i).unwrap();
        }
        w.new_day();
        for i in 0..covers {
            w.spend(1, 100 + i, 10, 2_000).unwrap();
        }
        w.new_day();
        let r = w.convert(1, 10, 1_000_000_000).unwrap();
        (r.converted_mexz, r.capped_by)
    };
    let (converted_50, by_50) = run(50);
    let (converted_100, by_100) = run(100);
    assert_eq!(converted_50, 28 * 2_000); // round(30 x tanh(50/30)) = 28
    assert_eq!(converted_100, 30 * 2_000); // round(30 x tanh(100/30)) = 30
    assert_eq!(by_50, CapReason::PhysicalCap);
    assert_eq!(by_100, CapReason::PhysicalCap);
    // Doubling from 50 to 100 covers adds only two counted covers.
    assert_eq!(converted_100 - converted_50, 2 * 2_000);
}

#[test]
fn zone_quota_caps_counting_twenty_percent_above_quota() {
    // Revised F.4, step 2: with a 1000-cover quota and 20% headroom, the
    // zone counts at full weight up to the quota, then progressively less,
    // and never above quota + 20% = 1200 covers. Fifteen merchants
    // declaring 100 covers each: the first ten count 100, then the band
    // degrades (87, 62, 37, 12) and closes at zero.
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 100, 10_000, 1_000_000, 1_000, 1_000_000_000, 1_000_000_000)
        .unwrap();
    for m in 0..15u64 {
        w.enroll_merchant(1, m).unwrap();
    }
    for i in 0..1_500u64 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day();
    for m in 0..15u64 {
        for i in 0..100u64 {
            w.spend(1, 100 + m * 100 + i, m, 2_000).unwrap();
        }
    }
    w.new_day();
    let expected_converted = [
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, // up to quota
        87, 62, 37, 12, // degressive band
        0, // closed
    ];
    for (m, expected) in expected_converted.iter().enumerate() {
        let r = w.convert(1, m as u64, 200_000).unwrap();
        assert_eq!(
            r.converted_mexz,
            expected * 2_000,
            "merchant {} converted {}",
            m,
            r.converted_mexz
        );
        if *expected < 100 && *expected > 0 {
            assert_eq!(r.capped_by, CapReason::ZoneQuotaCap);
        }
    }
    let zone = w.zones.iter().find(|z| z.id == 1).unwrap();
    // Total counted: 1000 + 198 = 1198, at most quota x 1.2 = 1200.
    assert_eq!(zone.covers_counted_today, 1_198);
    assert!(zone.covers_counted_today <= zone.quota_covers * 12 / 10);
}

#[test]
fn overshooting_volume_melts_settlement_credit_faster() {
    // Revised F.4, step 3: a 30-cover table with 20% headroom admits 36
    // covers. A merchant that declared 40 is in overshoot: its confirmed
    // settlement credit melts at 5% per day, outside the melt-free
    // operational window. A merchant at 20 covers keeps its credit whole.
    let mut w = World::new(Params::default());
    w.add_zone(1, "test", 100, 1_000_000, 30, 1_000_000_000, 1_000_000_000, 1_000_000)
        .unwrap();
    w.enroll_merchant(1, 10).unwrap(); // will declare 40 covers
    w.enroll_merchant(1, 20).unwrap(); // will declare 20 covers
    for i in 0..60u64 {
        w.enroll_person(1, 100 + i).unwrap();
    }
    w.new_day();
    for i in 0..40u64 {
        w.spend(1, 100 + i, 10, 2_000).unwrap(); // 40 covers at merchant 10
    }
    for i in 0..20u64 {
        w.spend(1, 100 + 40 + i, 20, 2_000).unwrap(); // 20 covers at merchant 20
    }
    w.new_day(); // purchases confirm, then the overshoot melt applies
    let zone = w.zones.iter().find(|z| z.id == 1).unwrap();
    let m10 = zone.merchants.iter().find(|m| m.id == 10).unwrap();
    let m20 = zone.merchants.iter().find(|m| m.id == 20).unwrap();
    assert!(m10.overshoot, "40 covers exceeds the 36-cover adjusted table");
    assert!(!m20.overshoot);
    // 80_000 confirmed melted at 5% = 4_000 to the fund; 76_000 remain.
    assert_eq!(m10.confirmed_total(), 76_000);
    assert_eq!(m20.confirmed_total(), 40_000);
    assert_eq!(w.stats.overshoot_melted_mexz, 4_000);
    assert_eq!(w.fund.exz, 4_000);
}
