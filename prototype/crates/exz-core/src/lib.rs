//! exz-core: the deterministic rules engine of the Existenz protocol.
//!
//! Reference implementation of the founding rules of the Existenz white paper:
//! equal daily issuance, capped accumulation, melting currency, the single-hop
//! merchant rule, and the coverage-indexed global fund. Dependency-free and
//! integer-only: balances are counted in milli-EXZ (mEXZ), local-currency
//! amounts in cents. Nothing here is a blockchain: this crate is the
//! executable specification that any future ledger runtime must satisfy, and
//! the simulation base the rules will be tuned against.
//!
//! White paper sections implemented here:
//! - Section 4 and 5.4: founding rules, Scenario B subsistence parameters
//! - Section 5.1/5.2: melting, cap, global fund (melting suspended in the fund)
//! - Annex C: melting exemption floor and targeted savings sub-wallets
//! - Annex F: single-hop rule, settlement credit, cap band, commission
//! - Annex G: payout formula, coverage ratio, reserve and rollout gate
//! - Annex H.2: bounded merchant circuit
//! - Revised Annex F.4 (proposed, pending paper integration): the
//!   solidarity scale rule. Declared covers are saturated at the zone's
//!   class table size (tanh soft cap), the zone's daily cover quota caps
//!   total counting at quota x (1 + headroom) through a degressive band,
//!   and a merchant declaring beyond its adjusted table sees its settlement
//!   credit melt at an accelerated rate.

use std::collections::{HashMap, HashSet, VecDeque};

/// Money of account, in milli-EXZ (1 EXZ = 1000 mEXZ).
pub type Mexz = u64;
/// Local-currency amounts, in cents.
pub type Cents = i64;

pub const MEXZ_PER_EXZ: Mexz = 1000;

/// The parametric rules of the protocol (Annex D, "parametric rules" tier).
/// Defaults follow the white paper's Scenario B (subsistence model) and the
/// proposed values of Annexes C, F, G and H.
#[derive(Clone, Debug, PartialEq)]
pub struct Params {
    /// Daily issuance per person: 2 EXZ (Scenario B).
    pub daily_issuance_mexz: Mexz,
    /// Accumulation cap, in days of issuance (cap_days x issuance).
    pub cap_days: Mexz,
    /// Melting rate per idle day, in basis points of the balance
    /// (Scenario B: one unit lost at every 60th, ~167 bps/day).
    pub melt_bps_per_day: u64,
    /// Melting starts after this many idle days; any exchange resets the clock.
    pub melt_idle_days: u64,
    /// Melting exemption floor, in days of issuance (Annex C.1: 14).
    pub floor_days: Mexz,
    /// Frozen sub-wallet bound, in days of issuance (Annex C.2: 60).
    pub subwallet_max_days: Mexz,
    /// Settlement credit melting-exempt window in days (Annex F.2: 7).
    pub settlement_window_days: u64,
    /// Days until a purchase is confirmed (Annex E.6 confirmation window).
    pub confirm_days: u64,
    /// Bounded merchant circuit, fraction of the weekly received credit
    /// (Annex H.2: 20%).
    pub circuit_fraction_bps: u64,
    /// Circuit accounting window in days (weekly: 7).
    pub circuit_window_days: u64,
    /// Verification commission on confirmed transactions (Annex F.5: 3%).
    pub commission_bps: u64,
    /// Commission daily cap, in daily allowances per merchant (Annex F.5: 50).
    pub commission_cap_allowances: u64,
    /// Initial conversion tier, in daily allowances (Annex F.4 Cap 3: 100).
    pub tier_initial_allowances: u64,
    /// Integrity index EWMA gain, in bps (0.2 per clean day = 2000).
    pub integrity_alpha_bps: u64,
    /// Reserve horizon H, in days (Annex G.3: 90).
    pub reserve_horizon_days: u64,
    /// Days of payout history used to estimate the steady-state daily payout.
    pub payout_avg_days: u64,
    /// Zone quota headroom, in bps (revised F.4): the zone's total counted
    /// covers cap at quota x (1 + headroom), and a merchant overshoots its
    /// adjusted table at table x (1 + headroom). Proposed: 20%.
    pub zone_headroom_bps: u64,
    /// Accelerated melting of an overshooting merchant's settlement credit,
    /// in bps per day, replacing the melt-free operational window for that
    /// volume (revised F.4). Proposed: 5% per day.
    pub overshoot_melt_bps_per_day: u64,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            daily_issuance_mexz: 2_000,
            cap_days: 2,
            melt_bps_per_day: 167,
            melt_idle_days: 1,
            floor_days: 14,
            subwallet_max_days: 60,
            settlement_window_days: 7,
            confirm_days: 1,
            circuit_fraction_bps: 2_000,
            circuit_window_days: 7,
            commission_bps: 300,
            commission_cap_allowances: 50,
            tier_initial_allowances: 100,
            integrity_alpha_bps: 2_000,
            reserve_horizon_days: 90,
            payout_avg_days: 7,
            zone_headroom_bps: 2_000,
            overshoot_melt_bps_per_day: 500,
        }
    }
}

impl Params {
    pub fn cap_mexz(&self) -> Mexz {
        self.cap_days.saturating_mul(self.daily_issuance_mexz)
    }
    pub fn floor_mexz(&self) -> Mexz {
        self.floor_days.saturating_mul(self.daily_issuance_mexz)
    }
    pub fn subwallet_max_mexz(&self) -> Mexz {
        self.subwallet_max_days.saturating_mul(self.daily_issuance_mexz)
    }
    /// Local-currency value of one EXZ in a zone: the BNB price of one meal.
    pub fn exz_cents(&self, meal_price_cents: u64) -> Cents {
        meal_price_cents as Cents
    }
    /// Local-currency value of the daily allowance (issuance x meal price).
    pub fn allowance_cents(&self, meal_price_cents: u64) -> Cents {
        (self.daily_issuance_mexz as Cents).saturating_mul(meal_price_cents as Cents)
            / MEXZ_PER_EXZ as Cents
    }
}

/// The Annex G.1 daily payout formula: D = N x u x v x 2p x (1 + c).
/// Kept in floating point because it is an analytical projection, not ledger
/// accounting.
pub fn projected_daily_payout_cents(
    n: u64,
    u: f64,
    v: f64,
    p_cents: u64,
    commission_bps: u64,
) -> f64 {
    let two_p = 2.0 * p_cents as f64;
    let cc = 1.0 + commission_bps as f64 / 10_000.0;
    n as f64 * u * v * two_p * cc
}

/// Fixed-point scale of `tanh_fixed` (1e12).
pub const TANH_FP: u128 = 1_000_000_000_000;

/// Deterministic integer tanh(num/den), returned in TANH_FP fixed point.
/// The solidarity scale rule (revised F.4) needs a saturating soft cap with
/// no cliff: identity near zero, asymptote at one. Uses
/// tanh(x) = (e^(2x) - 1) / (e^(2x) + 1) with a fixed-point Taylor series
/// of the exponential; for num/den >= 10 the deficit from one is below
/// 8e-10, far below the integer rounding that follows, so it saturates.
pub fn tanh_fixed(num: u64, den: u64) -> u128 {
    if den == 0 || num >= den.saturating_mul(10) {
        return TANH_FP;
    }
    // t = 2 x num/den in fixed point (t < 20 in real units).
    let t = 2u128 * u128::from(num) * TANH_FP / u128::from(den);
    // e^t by Taylor, term by term, all in fixed point.
    let mut term = TANH_FP;
    let mut sum = TANH_FP;
    for k in 1..=80u128 {
        term = term * t / (k * TANH_FP);
        if term == 0 {
            break;
        }
        sum += term;
    }
    (sum - TANH_FP) * TANH_FP / (sum + TANH_FP)
}

/// A human account holder. Merchants hold one of these too (Annex F.2:
/// the existence wallet never mingles with business streams).
#[derive(Clone, Debug)]
pub struct Person {
    pub id: u64,
    pub balance: Mexz,
    pub last_active_day: u64,
    pub alive: bool,
    /// Frozen labeled savings sub-wallets (Annex C.2). Melting is suspended
    /// there; the total is bounded by subwallet_max_days of issuance.
    pub subwallets: HashMap<String, Mexz>,
}

impl Person {
    pub fn new(id: u64) -> Self {
        Person { id, balance: 0, last_active_day: 0, alive: true, subwallets: HashMap::new() }
    }
}

/// Provisional purchase credit (awaiting the confirmation window).
/// Confirmed settlement credit buckets, tagged with the confirmation day for
/// the operational-window aging of Annex F.2.
type CreditBucket = (Mexz, u64);

#[derive(Clone, Debug)]
pub struct Merchant {
    pub id: u64,
    /// The merchant's own existence wallet (Annex F.2, stream 1).
    pub existence: Person,
    /// Purchases awaiting confirmation: (amount, transaction day).
    pub provisional: Vec<CreditBucket>,
    /// Confirmed settlement credit: (amount, confirmed day).
    pub confirmed: Vec<CreditBucket>,
    /// Distinct persons served today (accumulating during the day).
    pub today_presences: HashSet<u64>,
    /// Presences of the day being declared (Annex F.4 Cap 1 input): the
    /// closed day's statement, carried through the confirmation window.
    pub presences_declared: HashSet<u64>,
    /// Integrity index, in bps of 10_000 (Annex F.4 Cap 3).
    pub integrity_bps: u64,
    /// Received credit history: (day, amount), for the circuit bound.
    pub received_hist: Vec<(u64, Mexz)>,
    /// Circuit spending history: (day, amount).
    pub circuit_spent_hist: Vec<(u64, Mexz)>,
    pub converted_cents_total: Cents,
    pub commission_cents_total: Cents,
    pub queued_cents: Cents,
    /// True while the merchant declared beyond its adjusted table
    /// (table x (1 + headroom), revised F.4): accelerated melting applies.
    pub overshoot: bool,
}

impl Merchant {
    pub fn new(id: u64) -> Self {
        Merchant {
            id,
            existence: Person::new(id),
            provisional: Vec::new(),
            confirmed: Vec::new(),
            today_presences: HashSet::new(),
            presences_declared: HashSet::new(),
            integrity_bps: 0,
            received_hist: Vec::new(),
            circuit_spent_hist: Vec::new(),
            converted_cents_total: 0,
            commission_cents_total: 0,
            queued_cents: 0,
            overshoot: false,
        }
    }
    pub fn confirmed_total(&self) -> Mexz {
        self.confirmed.iter().map(|b| b.0).sum()
    }
}

/// Per-zone slice of the global fund (Annex G.3: the coverage rule holds
/// per zone and globally).
#[derive(Clone, Debug)]
pub struct ZoneFund {
    pub allocated_cents: Cents,
    pub inflow_cents_per_day: Cents,
    /// Recent daily payout totals, oldest first (payout_avg_days entries).
    /// The coverage rule needs the DAILY total, not the per-event average:
    /// a zone where many merchants convert small amounts each day must not
    /// look better covered than it is.
    pub payout_hist: VecDeque<Cents>,
    /// Today's payouts, accumulated event by event, flushed into
    /// payout_hist as a single daily total at the next new_day.
    pub payout_today: Cents,
    /// Remaining ration budget for today (Annex G.3: the steady-state
    /// daily inflow), SHARED by all conversion requests of the day. It is
    /// reset at each new_day; each rationed payment consumes from it.
    pub ration_left: Cents,
    /// Conversion ration queue: (merchant id, amount cents), elder first.
    pub queue: VecDeque<(u64, Cents)>,
}

impl ZoneFund {
    pub fn new(allocated_cents: Cents, inflow_cents_per_day: Cents) -> Self {
        ZoneFund {
            allocated_cents,
            inflow_cents_per_day,
            payout_hist: VecDeque::new(),
            payout_today: 0,
            ration_left: 0,
            queue: VecDeque::new(),
        }
    }
}

/// The global fund: EXZ commons (melting suspended, Section 5.1) plus the
/// per-zone external endowments that honor conversion (Annex G).
#[derive(Clone, Debug, Default)]
pub struct Fund {
    /// EXZ units held by the commons: excess above caps, melted units,
    /// reverted settlement credit, balances of the deceased. Melting is
    /// suspended here (Section 5.2).
    pub exz: Mexz,
    pub zones: HashMap<u64, ZoneFund>,
}

#[derive(Clone, Debug, Default)]
pub struct WorldStats {
    pub melted_mexz: Mexz,
    pub overshoot_melted_mexz: Mexz,
    pub excess_to_fund_mexz: Mexz,
    pub reverted_mexz: Mexz,
    pub death_settlements_mexz: Mexz,
}

/// One rollout zone.
#[derive(Clone, Debug)]
pub struct Zone {
    pub id: u64,
    pub name: String,
    /// BNB price of one basic meal in this zone (Annex B): p.
    pub meal_price_cents: u64,
    /// Plausible served population per merchant location (Annex F.4 Cap 2): P.
    pub plausible_population: u64,
    /// Solidarity table size of the zone's class (revised F.4): the
    /// tanh saturation point of a merchant's counted covers. 0 = uncapped.
    pub scale_cap_presences: u64,
    /// Daily cover quota of the zone (revised F.4), from its malnutrition
    /// class profile: total counted covers cap at quota x (1 + headroom)
    /// through the degressive band. 0 = no zone quota.
    pub quota_covers: u64,
    /// Cover position declared for conversion today (revised F.4 band).
    pub covers_declared_today: u64,
    /// Covers counted today (revised F.4 band).
    pub covers_counted_today: u64,
    pub persons: Vec<Person>,
    pub merchants: Vec<Merchant>,
    /// id -> position maps, so lookups stay O(1) at zone scale.
    pub person_index: HashMap<u64, usize>,
    pub merchant_index: HashMap<u64, usize>,
}

/// Daily per-zone accounting row, used by the simulation and by tests.
/// `converted_cents` counts only what the fund ACTUALLY paid out (gross
/// plus commission), never the queued part; queue drain payments are
/// included and attributed to the day they were paid.
#[derive(Clone, Copy, Debug, Default)]
pub struct DayStats {
    pub spent_mexz: Mexz,
    pub converted_cents: Cents,
    pub commission_cents: Cents,
    pub queued_cents: Cents,
    pub melted_mexz: Mexz,
    pub excess_mexz: Mexz,
    pub reverted_mexz: Mexz,
    pub overshoot_melted_mexz: Mexz,
}

#[derive(Clone, Debug)]
pub struct World {
    pub params: Params,
    pub day: u64,
    pub zones: Vec<Zone>,
    pub fund: Fund,
    pub stats: WorldStats,
    /// One row per (day, zone), written at each conversion or day close.
    pub history: Vec<(u64, u64, DayStats)>,
}

#[derive(Clone, Debug)]
pub struct ConversionReceipt {
    pub converted_mexz: Mexz,
    pub gross_cents: Cents,
    pub commission_cents: Cents,
    pub queued_cents: Cents,
    pub capped_by: CapReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapReason {
    NotCapped,
    /// The merchant's own table saturation (tanh over declared covers).
    PhysicalCap,
    /// The zone's daily cover quota, through the degressive band.
    ZoneQuotaCap,
    PlausibilityCap,
    TierCap,
    Request,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExzError {
    UnknownZone,
    UnknownPerson,
    UnknownMerchant,
    DeceasedAccount,
    InsufficientBalance,
    NonTransferableCredit,
    MerchantToPersonForbidden,
    SubwalletLimitExceeded,
    UnknownSubwallet,
    CircuitLimitExceeded,
    InsufficientCredit,
    Empty,
}

impl std::fmt::Display for ExzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exz error: {:?}", self)
    }
}
impl std::error::Error for ExzError {}

impl World {
    pub fn new(params: Params) -> Self {
        World {
            params,
            day: 0,
            zones: Vec::new(),
            fund: Fund::default(),
            stats: WorldStats::default(),
            history: Vec::new(),
        }
    }

    /// Annex G.3 rollout gate: a zone may be onboarded only if its
    /// H x D is endowment-covered by the available allocation.
    pub fn can_onboard(available_cents: Cents, projected_daily_cents: f64, horizon_days: u64) -> bool {
        available_cents as f64 >= projected_daily_cents * horizon_days as f64
    }

    pub fn add_zone(
        &mut self,
        id: u64,
        name: &str,
        meal_price_cents: u64,
        plausible_population: u64,
        scale_cap_presences: u64,
        quota_covers: u64,
        allocated_cents: Cents,
        inflow_cents_per_day: Cents,
    ) -> Result<(), ExzError> {
        if self.zones.iter().any(|z| z.id == id) {
            return Err(ExzError::UnknownZone);
        }
        self.zones.push(Zone {
            id,
            name: name.to_string(),
            meal_price_cents,
            plausible_population,
            scale_cap_presences,
            quota_covers,
            covers_declared_today: 0,
            covers_counted_today: 0,
            persons: Vec::new(),
            merchants: Vec::new(),
            person_index: HashMap::new(),
            merchant_index: HashMap::new(),
        });
        self.fund.zones.insert(id, ZoneFund::new(allocated_cents, inflow_cents_per_day));
        Ok(())
    }

    pub fn enroll_person(&mut self, zone_id: u64, person_id: u64) -> Result<(), ExzError> {
        let z = self.zone_mut(zone_id)?;
        let pos = z.persons.len();
        z.persons.push(Person::new(person_id));
        z.person_index.insert(person_id, pos);
        Ok(())
    }

    pub fn enroll_merchant(&mut self, zone_id: u64, merchant_id: u64) -> Result<(), ExzError> {
        let z = self.zone_mut(zone_id)?;
        let pos = z.merchants.len();
        z.merchants.push(Merchant::new(merchant_id));
        z.merchant_index.insert(merchant_id, pos);
        Ok(())
    }

    fn zone_mut(&mut self, zone_id: u64) -> Result<&mut Zone, ExzError> {
        let i = self
            .zones
            .iter_mut()
            .position(|z| z.id == zone_id)
            .ok_or(ExzError::UnknownZone)?;
        Ok(&mut self.zones[i])
    }

    // ---- day engine -------------------------------------------------------

    /// Advance one day: issuance, melting, credit aging, fund replenishment.
    /// Spends and conversions for day `self.day` happen after this call.
    pub fn new_day(&mut self) {
        self.day += 1;
        let day = self.day;
        let params = self.params.clone();
        let issuance = params.daily_issuance_mexz;
        let cap = params.cap_mexz();
        let floor = params.floor_mexz();

        for z in 0..self.zones.len() {
            // A new day of the solidarity quota band (revised F.4).
            self.zones[z].covers_declared_today = 0;
            self.zones[z].covers_counted_today = 0;
            // The new day starts with an empty presence book; yesterday's
            // presences become the declared statement that Cap 1 of the
            // day's conversion requests is checked against.
            for m in 0..self.zones[z].merchants.len() {
                let merchant = &mut self.zones[z].merchants[m];
                merchant.presences_declared =
                    std::mem::take(&mut merchant.today_presences);
            }

            // Existence wallets (persons and merchants alike): issuance,
            // cap overflow to the fund, melting of idle balances.
            let mut excess = 0u64;
            let mut melted = 0u64;
            for p in self.zones[z].persons.iter_mut() {
                if !p.alive {
                    continue;
                }
                p.balance = p.balance.saturating_add(issuance);
                if p.balance > cap {
                    excess += p.balance - cap;
                    p.balance = cap;
                }
                if day.saturating_sub(p.last_active_day) >= params.melt_idle_days
                    && p.balance > floor
                {
                    let melt = (u128::from(p.balance) * u128::from(params.melt_bps_per_day)
                        / 10_000) as u64;
                    p.balance -= melt;
                    melted += melt;
                }
            }
            for mid in 0..self.zones[z].merchants.len() {
                let p = &mut self.zones[z].merchants[mid].existence;
                p.balance = p.balance.saturating_add(issuance);
                if p.balance > cap {
                    excess += p.balance - cap;
                    p.balance = cap;
                }
                if day.saturating_sub(p.last_active_day) >= params.melt_idle_days
                    && p.balance > floor
                {
                    let melt = (u128::from(p.balance) * u128::from(params.melt_bps_per_day)
                        / 10_000) as u64;
                    p.balance -= melt;
                    melted += melt;
                }
            }

            // Settlement credit aging (Annex F.2): provisionals confirm,
            // confirmed credit past the operational window reverts to the fund.
            let mut reverted = 0u64;
            for m in 0..self.zones[z].merchants.len() {
                let merchant = &mut self.zones[z].merchants[m];
                let mut still_provisional = Vec::new();
                for (amount, tx_day) in merchant.provisional.drain(..) {
                    if day.saturating_sub(tx_day) >= params.confirm_days {
                        merchant.confirmed.push((amount, day));
                    } else {
                        still_provisional.push((amount, tx_day));
                    }
                }
                merchant.provisional = still_provisional;
                let mut keep = Vec::new();
                for (amount, confirmed_day) in merchant.confirmed.drain(..) {
                    if day.saturating_sub(confirmed_day) > params.settlement_window_days {
                        reverted += amount;
                    } else {
                        keep.push((amount, confirmed_day));
                    }
                }
                merchant.confirmed = keep;
                // Prune the circuit histories outside the window.
                merchant
                    .received_hist
                    .retain(|(d, _)| day.saturating_sub(*d) <= params.circuit_window_days);
                merchant
                    .circuit_spent_hist
                    .retain(|(d, _)| day.saturating_sub(*d) <= params.circuit_window_days);
            }

            // Solidarity scale overshoot (revised F.4): a merchant that
            // declared beyond its adjusted table (table x (1 + headroom))
            // is in overshoot for the day; its confirmed settlement credit
            // melts at the accelerated rate instead of enjoying the
            // melt-free operational window. The melted units join the
            // EXZ commons, like all melting.
            let p_max = self.zones[z].scale_cap_presences;
            let threshold = p_max + p_max * params.zone_headroom_bps / 10_000;
            let mut overshoot_melted = 0u64;
            for m in 0..self.zones[z].merchants.len() {
                let merchant = &mut self.zones[z].merchants[m];
                merchant.overshoot =
                    merchant.presences_declared.len() as u64 > threshold;
                if merchant.overshoot {
                    let credit = merchant.confirmed_total();
                    let melt =
                        credit * params.overshoot_melt_bps_per_day / 10_000;
                    if melt > 0 {
                        take_from_buckets(&mut merchant.confirmed, melt);
                        overshoot_melted += melt;
                    }
                }
            }

            // Fund replenishment (Annex G.3 inflow rule), and close of the
            // payout ledger: today's events become one daily total.
            let zid = self.zones[z].id;
            if let Some(zf) = self.fund.zones.get_mut(&zid) {
                zf.allocated_cents += zf.inflow_cents_per_day;
                zf.payout_hist.push_back(zf.payout_today);
                zf.payout_today = 0;
                zf.ration_left = zf.inflow_cents_per_day;
                let max_hist = params.payout_avg_days as usize;
                while zf.payout_hist.len() > max_hist {
                    zf.payout_hist.pop_front();
                }
            }
            self.fund.exz += excess + melted + reverted + overshoot_melted;
            self.stats.melted_mexz += melted;
            self.stats.excess_to_fund_mexz += excess;
            self.stats.reverted_mexz += reverted;
            self.stats.overshoot_melted_mexz += overshoot_melted;
            self.push_history(zid, DayStats {
                melted_mexz: melted,
                excess_mexz: excess,
                reverted_mexz: reverted,
                overshoot_melted_mexz: overshoot_melted,
                ..Default::default()
            });
        }
    }

    fn push_history(&mut self, zone_id: u64, row: DayStats) {
        if let Some(last) = self.history.last_mut() {
            if last.0 == self.day && last.1 == zone_id {
                last.2.spent_mexz += row.spent_mexz;
                last.2.converted_cents += row.converted_cents;
                last.2.commission_cents += row.commission_cents;
                last.2.queued_cents += row.queued_cents;
                last.2.melted_mexz += row.melted_mexz;
                last.2.excess_mexz += row.excess_mexz;
                last.2.reverted_mexz += row.reverted_mexz;
                last.2.overshoot_melted_mexz += row.overshoot_melted_mexz;
                return;
            }
        }
        self.history.push((self.day, zone_id, row));
    }

    // ---- single-hop economy ----------------------------------------------

    /// The only person-to-merchant transfer in the protocol (Annex F.1).
    pub fn spend(
        &mut self,
        zone_id: u64,
        person_id: u64,
        merchant_id: u64,
        amount: Mexz,
    ) -> Result<(), ExzError> {
        let day = self.day;
        let zi = self.zone_index(zone_id)?;
        let p = *self.zones[zi]
            .person_index
            .get(&person_id)
            .ok_or(ExzError::UnknownPerson)?;
        if !self.zones[zi].persons[p].alive {
            return Err(ExzError::DeceasedAccount);
        }
        let m = *self.zones[zi]
            .merchant_index
            .get(&merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        if self.zones[zi].persons[p].balance < amount {
            return Err(ExzError::InsufficientBalance);
        }
        let pid = self.zones[zi].persons[p].id;
        self.zones[zi].persons[p].balance -= amount;
        self.zones[zi].persons[p].last_active_day = day;
        let merchant = &mut self.zones[zi].merchants[m];
        merchant.provisional.push((amount, day));
        merchant.received_hist.push((day, amount));
        merchant.today_presences.insert(pid);
        let zid = zone_id;
        self.push_history(zid, DayStats { spent_mexz: amount, ..Default::default() });
        Ok(())
    }

    /// Freeze balance into a labeled sub-wallet (Annex C.2): melting
    /// suspended, total bounded by subwallet_max_days of issuance.
    pub fn freeze_to_subwallet(
        &mut self,
        zone_id: u64,
        person_id: u64,
        label: &str,
        amount: Mexz,
    ) -> Result<(), ExzError> {
        let zi = self.zone_index(zone_id)?;
        let p = *self.zones[zi]
            .person_index
            .get(&person_id)
            .ok_or(ExzError::UnknownPerson)?;
        let person = &mut self.zones[zi].persons[p];
        if !person.alive {
            return Err(ExzError::DeceasedAccount);
        }
        if person.balance < amount {
            return Err(ExzError::InsufficientBalance);
        }
        let current: Mexz = person.subwallets.values().sum();
        if current.saturating_add(amount) > self.params.subwallet_max_mexz() {
            return Err(ExzError::SubwalletLimitExceeded);
        }
        person.balance -= amount;
        *person.subwallets.entry(label.to_string()).or_insert(0) += amount;
        Ok(())
    }

    /// Spend from a sub-wallet to a merchant (single hop preserved).
    pub fn spend_from_subwallet(
        &mut self,
        zone_id: u64,
        person_id: u64,
        label: &str,
        merchant_id: u64,
        amount: Mexz,
    ) -> Result<(), ExzError> {
        let day = self.day;
        let zi = self.zone_index(zone_id)?;
        let p = *self.zones[zi]
            .person_index
            .get(&person_id)
            .ok_or(ExzError::UnknownPerson)?;
        if !self.zones[zi].persons[p].alive {
            return Err(ExzError::DeceasedAccount);
        }
        let m = *self.zones[zi]
            .merchant_index
            .get(&merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        let have = *self.zones[zi].persons[p]
            .subwallets
            .get(label)
            .ok_or(ExzError::UnknownSubwallet)?;
        if have < amount {
            return Err(ExzError::InsufficientBalance);
        }
        let pid = self.zones[zi].persons[p].id;
        self.zones[zi].persons[p]
            .subwallets
            .insert(label.to_string(), have - amount);
        let merchant = &mut self.zones[zi].merchants[m];
        merchant.provisional.push((amount, day));
        merchant.received_hist.push((day, amount));
        merchant.today_presences.insert(pid);
        self.push_history(zone_id, DayStats { spent_mexz: amount, ..Default::default() });
        Ok(())
    }

    /// Annex H.2 bounded merchant circuit: settlement credit spent on
    /// basic-need goods with another merchant, within the capped fraction
    /// of the weekly received credit. The credit never leaves the
    /// settlement-credit state.
    pub fn circuit_spend(
        &mut self,
        zone_id: u64,
        from_merchant_id: u64,
        to_merchant_id: u64,
        amount: Mexz,
    ) -> Result<(), ExzError> {
        let day = self.day;
        if amount == 0 {
            return Err(ExzError::Empty);
        }
        let zi = self.zone_index(zone_id)?;
        let from = *self.zones[zi]
            .merchant_index
            .get(&from_merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        let to = *self.zones[zi]
            .merchant_index
            .get(&to_merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        if from == to {
            return Err(ExzError::Empty);
        }
        let window = self.params.circuit_window_days;
        let frac = self.params.circuit_fraction_bps;
        let received_7d: Mexz = self.zones[zi].merchants[from]
            .received_hist
            .iter()
            .filter(|(d, _)| day.saturating_sub(*d) <= window)
            .map(|(_, a)| a)
            .sum();
        let spent_7d: Mexz = self.zones[zi].merchants[from]
            .circuit_spent_hist
            .iter()
            .filter(|(d, _)| day.saturating_sub(*d) <= window)
            .map(|(_, a)| a)
            .sum();
        let bound = ((u128::from(received_7d) * u128::from(frac)) / 10_000) as u64;
        if spent_7d.saturating_add(amount) > bound {
            return Err(ExzError::CircuitLimitExceeded);
        }
        if self.zones[zi].merchants[from].confirmed_total() < amount {
            return Err(ExzError::InsufficientCredit);
        }
        take_from_buckets(&mut self.zones[zi].merchants[from].confirmed, amount);
        self.zones[zi].merchants[from].circuit_spent_hist.push((day, amount));
        let to_id = self.zones[zi].merchants[to].id;
        let merchant = &mut self.zones[zi].merchants[to];
        merchant.provisional.push((amount, day));
        merchant.received_hist.push((day, amount));
        merchant.today_presences.insert(to_id);
        Ok(())
    }

    /// Fabrication penalty (Annex F.6 retrospective adjustment): declared
    /// units return to the fund and the integrity index is slashed, which
    /// lowers the merchant's tier cap.
    pub fn flag_fabricated(
        &mut self,
        zone_id: u64,
        merchant_id: u64,
        amount: Mexz,
    ) -> Result<(), ExzError> {
        let zi = self.zone_index(zone_id)?;
        let m = *self.zones[zi]
            .merchant_index
            .get(&merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        let merchant = &mut self.zones[zi].merchants[m];
        if merchant.confirmed_total() < amount {
            return Err(ExzError::InsufficientCredit);
        }
        take_from_buckets(&mut merchant.confirmed, amount);
        merchant.integrity_bps /= 2;
        self.fund.exz += amount;
        Ok(())
    }

    /// End-of-life settlement (Section 5.2): remaining balance and
    /// sub-wallets join the fund; no heir, no estate.
    pub fn settle_death(&mut self, zone_id: u64, person_id: u64) -> Result<(), ExzError> {
        let zi = self.zone_index(zone_id)?;
        let p = *self.zones[zi]
            .person_index
            .get(&person_id)
            .ok_or(ExzError::UnknownPerson)?;
        let person = &mut self.zones[zi].persons[p];
        if !person.alive {
            return Err(ExzError::DeceasedAccount);
        }
        let total = person.balance + person.subwallets.values().sum::<Mexz>();
        person.balance = 0;
        person.subwallets.clear();
        person.alive = false;
        self.fund.exz += total;
        self.stats.death_settlements_mexz += total;
        Ok(())
    }

    // ---- conversion and the fund ------------------------------------------

    /// Estimated steady-state daily payout of the zone: the mean of the
    /// recent payouts (payout_avg_days), matching the coverage rule of G.3.
    /// Average DAILY payout over the last payout_avg_days complete days.
    pub fn daily_payout_estimate(&self, zone_id: u64) -> f64 {
        match self.fund.zones.get(&zone_id) {
            Some(zf) if !zf.payout_hist.is_empty() => {
                zf.payout_hist.iter().map(|&c| c as f64).sum::<f64>() / zf.payout_hist.len() as f64
            }
            _ => 0.0,
        }
    }

    /// Coverage ratio R / (H x D) of a zone (Annex G.3).
    pub fn coverage_ratio(&self, zone_id: u64) -> f64 {
        let (allocated, hist, inflow) = match self.fund.zones.get(&zone_id) {
            Some(zf) => (zf.allocated_cents, &zf.payout_hist, zf.inflow_cents_per_day),
            None => return 0.0,
        };
        let d = if hist.is_empty() {
            inflow as f64
        } else {
            hist.iter().map(|&c| c as f64).sum::<f64>() / hist.len() as f64
        };
        if d <= 0.0 {
            return f64::INFINITY;
        }
        allocated as f64 / (self.params.reserve_horizon_days as f64 * d)
    }

    /// Global coverage: sum of allocations over sum of H x D per zone.
    pub fn global_coverage_ratio(&self) -> f64 {
        let mut num = 0.0;
        let mut den = 0.0;
        for z in &self.zones {
            let d = self.daily_payout_estimate(z.id);
            let allocated = self
                .fund
                .zones
                .get(&z.id)
                .map(|zf| zf.allocated_cents as f64)
                .unwrap_or(0.0);
            if d > 0.0 {
                num += allocated;
                den += self.params.reserve_horizon_days as f64 * d;
            }
        }
        if den <= 0.0 {
            return f64::INFINITY;
        }
        num / den
    }

    /// The merchant's end-of-day conversion (Annexes F.3 to F.5 and G.3).
    /// The binding cap is the smallest of the three; the fund pays same-day
    /// within the coverage ration, or the request queues transparently.
    pub fn convert(
        &mut self,
        zone_id: u64,
        merchant_id: u64,
        requested_mexz: Mexz,
    ) -> Result<ConversionReceipt, ExzError> {
        let zi = self.zone_index(zone_id)?;
        let m = *self.zones[zi]
            .merchant_index
            .get(&merchant_id)
            .ok_or(ExzError::UnknownMerchant)?;
        let allowance = self.params.daily_issuance_mexz;
        let meal_price = self.zones[zi].meal_price_cents;
        let plausible = self.zones[zi].plausible_population;
        let tier_initial = self.params.tier_initial_allowances;
        let commission_bps = self.params.commission_bps;
        let commission_cap_cents = self.params.commission_cap_allowances as i64
            * self.params.allowance_cents(meal_price);
        let integrity_alpha = self.params.integrity_alpha_bps;

        let merchant = &self.zones[zi].merchants[m];
        let confirmed = merchant.confirmed_total();
        if confirmed == 0 && requested_mexz == 0 {
            return Ok(ConversionReceipt {
                converted_mexz: 0,
                gross_cents: 0,
                commission_cents: 0,
                queued_cents: 0,
                capped_by: CapReason::Request,
            });
        }
        // The solidarity scale rule (revised F.4), then the nested caps;
        // the binding one is the smallest.
        //
        // Step 1, merchant table: the declared covers P saturate at the
        // zone's class table size P_max, softly:
        //     P_eff = P_max x tanh(P / P_max)
        // identity for small P, asymptote P_max, no cliff. Rounded to the
        // nearest cover, then clamped by the declared count.
        let p_decl = merchant.presences_declared.len() as u64;
        let integrity_bps = merchant.integrity_bps;
        let p_max = self.zones[zi].scale_cap_presences;
        let p_eff = if p_max == 0 {
            p_decl
        } else {
            let raw = (u128::from(p_max) * tanh_fixed(p_decl, p_max)
                + TANH_FP / 2)
                / TANH_FP;
            p_decl.min(raw as u64)
        };
        //
        // Step 2, zone quota band: covers count at full weight up to the
        // zone's quota Q; beyond, the marginal weight decays linearly to
        // zero at Q x (1 + 2 x headroom). The zone's total counted covers
        // therefore never exceed Q x (1 + headroom) (with the proposed
        // 20% headroom: twenty percent above the quota, no cliff).
        let quota = self.zones[zi].quota_covers;
        let head = self.params.zone_headroom_bps;
        let counted = if quota == 0 {
            p_eff
        } else {
            let hq = quota * head / 10_000;
            let w = quota + 2 * hq;
            let vd = self.zones[zi].covers_declared_today;
            let u = p_eff.min(quota.saturating_sub(vd.min(quota)));
            let t0 = vd.max(quota);
            let s = (vd + p_eff).min(w);
            let n2 = if s > t0 {
                ((w - t0) * (w - t0) - (w - s) * (w - s)) / (2 * (w - quota))
            } else {
                0
            };
            u + n2
        };
        self.zones[zi].covers_declared_today += p_eff;
        self.zones[zi].covers_counted_today += counted;
        let cap1 = counted * allowance;
        let cap2 = plausible * allowance;
        let tier_floor = tier_initial * allowance;
        let cap3 = if cap2 > tier_floor {
            tier_floor
                + ((u128::from(cap2 - tier_floor) * u128::from(integrity_bps))
                    / 10_000) as u64
        } else {
            cap2
        };
        let capped = confirmed.min(cap1).min(cap2).min(cap3);
        let converted = requested_mexz.min(capped);
        let capped_by = if converted == requested_mexz {
            CapReason::NotCapped
        } else if capped == cap1 {
            if counted < p_eff {
                CapReason::ZoneQuotaCap
            } else {
                CapReason::PhysicalCap
            }
        } else if capped == cap2 {
            CapReason::PlausibilityCap
        } else if capped == cap3 {
            CapReason::TierCap
        } else {
            CapReason::Request
        };

        let gross = (converted as i64).saturating_mul(meal_price as i64)
            / MEXZ_PER_EXZ as i64;
        let uncapped_commission = gross.saturating_mul(commission_bps as i64) / 10_000;
        let commission = uncapped_commission.min(commission_cap_cents);

        // Coverage rule (G.3): pay same-day while covered, else queue.
        let coverage = self.coverage_ratio(zone_id);
        let mut queued = 0i64;
        if coverage >= 1.0 {
            let zf = self.fund.zones.get_mut(&zone_id).unwrap();
            zf.allocated_cents -= gross + commission;
            self.record_payout(zone_id, gross + commission);
            let merchant = &mut self.zones[zi].merchants[m];
            take_from_buckets(&mut merchant.confirmed, converted);
            merchant.converted_cents_total += gross;
            merchant.commission_cents_total += commission;
        } else {
            // Rationed: pay the queue first (elder first), then this
            // request, within the steady-state daily budget (the inflow
            // rule of G.3). Unpaid amounts join the transparent queue.
            let mut budget = self
                .fund
                .zones
                .get(&zone_id)
                .map(|zf| zf.ration_left)
                .unwrap_or(0);
            while budget > 0 {
                let (qid, qamt) = match self.fund.zones.get(&zone_id).unwrap().queue.front() {
                    Some(&x) => x,
                    None => break,
                };
                let pay = qamt.min(budget);
                let zf = self.fund.zones.get_mut(&zone_id).unwrap();
                zf.allocated_cents -= pay;
                zf.ration_left -= pay;
                self.record_payout(zone_id, pay);
                self.credit_converted_zone(zone_id, qid, pay);
                budget -= pay;
                if pay >= qamt {
                    self.fund.zones.get_mut(&zone_id).unwrap().queue.pop_front();
                } else {
                    self.fund.zones.get_mut(&zone_id).unwrap().queue[0].1 = qamt - pay;
                    break;
                }
            }
            let due = gross + commission;
            let own_pay = due.min(budget);
            if own_pay > 0 {
                let zf = self.fund.zones.get_mut(&zone_id).unwrap();
                zf.allocated_cents -= own_pay;
                zf.ration_left -= own_pay;
                self.record_payout(zone_id, own_pay);
                let merchant = &mut self.zones[zi].merchants[m];
                merchant.converted_cents_total += own_pay;
                let converted_paid = (u128::from(converted) * u128::from(own_pay as u64))
                    / u128::from(due as u64);
                take_from_buckets(&mut merchant.confirmed, converted_paid as u64);
            }
            queued = due - own_pay;
            let merchant = &mut self.zones[zi].merchants[m];
            merchant.queued_cents += queued;
            if queued > 0 {
                self.fund
                    .zones
                    .get_mut(&zone_id)
                    .unwrap()
                    .queue
                    .push_back((merchant_id, queued));
            }
        }

        // Integrity index: a clean conversion raises the tier (Annex F.4).
        let merchant = &mut self.zones[zi].merchants[m];
        let alpha = integrity_alpha;
        merchant.integrity_bps = merchant.integrity_bps
            + ((10_000 - merchant.integrity_bps) * alpha) / 10_000;

        let receipt = ConversionReceipt {
            converted_mexz: converted,
            gross_cents: gross,
            commission_cents: commission,
            queued_cents: queued,
            capped_by,
        };
        self.push_history(zone_id, DayStats {
            // Actual payments were already recorded via record_payout;
            // this row adds what was charged and what queued.
            commission_cents: receipt.commission_cents,
            queued_cents: receipt.queued_cents,
            ..Default::default()
        });
        Ok(receipt)
    }

    fn record_payout(&mut self, zone_id: u64, cents: Cents) {
        let zf = self.fund.zones.get_mut(&zone_id).unwrap();
        zf.payout_today += cents;
        self.push_history(zone_id, DayStats { converted_cents: cents, ..Default::default() });
    }

    fn credit_converted_zone(&mut self, zone_id: u64, merchant_id: u64, cents: Cents) {
        let zi = self.zone_index(zone_id).unwrap();
        if let Some(&m) = self.zones[zi].merchant_index.get(&merchant_id) {
            self.zones[zi].merchants[m].converted_cents_total += cents;
        }
    }

    fn zone_index(&self, zone_id: u64) -> Result<usize, ExzError> {
        self.zones
            .iter()
            .position(|z| z.id == zone_id)
            .ok_or(ExzError::UnknownZone)
    }
}

/// Remove `amount` from credit buckets, oldest first.
fn take_from_buckets(buckets: &mut Vec<CreditBucket>, amount: Mexz) {
    let mut left = amount;
    let mut out: Vec<CreditBucket> = Vec::new();
    for (a, d) in buckets.drain(..) {
        if left == 0 {
            out.push((a, d));
            continue;
        }
        if a <= left {
            left -= a;
        } else {
            out.push((a - left, d));
            left = 0;
        }
    }
    *buckets = out;
}
