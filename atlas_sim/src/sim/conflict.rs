use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*, utils::hashbrown::HashMap},
    bevy_egui,
    config::{sim::AtlasSimConfig, AtlasConfig},
    rand::Rng,
    ui::{sidebar::*, UiEditableEnum},
    weighted_rand::table::WalkerTable,
    MakeUi, UiEditableEnum,
};

use crate::sim::{
    polity::{
        Polity, Tribute, POL_MILITARIST, SCI_MEDICINE, SCI_MILTECH, STR_FORTRESS, TRAD_DIPLOMATIC,
        TRAD_MILITANT,
    },
    region::Region,
    ui::ConflictUi,
    SimMapData,
};

#[derive(Clone)]
pub struct Conflict {
    pub id: u32,
    pub start_date: u32,
    pub concluded: bool,
    pub primary_attacker: Entity,
    pub primary_defender: Entity,
    pub attackers: HashMap<Entity, ConflictMember>,
    pub defenders: HashMap<Entity, ConflictMember>,
    //pub marker: Entity, // TODO simple conflict markers?
}

impl Conflict {
    pub fn into_ui(&self, _config: &AtlasSimConfig) -> ConflictUi {
        ConflictUi {
            start_date: self.start_date,
            primary_attacker: self.primary_attacker,
            primary_defender: self.primary_defender,
            attackers: self.attackers.values().map(|x| x.to_owned()).collect(),
            defenders: self.defenders.values().map(|x| x.to_owned()).collect(),
        }
    }

    pub fn add_member(&mut self, entity: Entity, color: [u8; 3], is_attacker: bool) {
        let member = ConflictMember::new(entity, color);
        if is_attacker {
            self.attackers.insert(entity, member)
        } else {
            self.defenders.insert(entity, member)
        };
    }

    pub fn is_primary_attacker(&self, entity: Entity) -> bool {
        self.primary_attacker == entity
    }

    pub fn is_primary_defender(&self, entity: Entity) -> bool {
        self.primary_defender == entity
    }

    pub fn is_member(&self, entity: &Entity, is_attacker: bool, is_any: bool) -> bool {
        if (is_any || is_attacker) && self.attackers.contains_key(entity) {
            return true;
        }
        if (is_any || !is_attacker) && self.defenders.contains_key(entity) {
            return true;
        }
        return false;
    }

    pub fn is_opposing(&self, us: &Entity, them: &Entity) -> bool {
        if self.attackers.contains_key(us) && self.defenders.contains_key(them) {
            return true;
        }
        if self.defenders.contains_key(us) && self.attackers.contains_key(them) {
            return true;
        }
        return false;
    }
}

#[derive(Clone, MakeUi)]
pub struct ConflictMember {
    #[name("Polity Id")]
    #[control(SidebarEntityLink)]
    pub entity: Entity,
    #[name("Polity Color")]
    #[control(SidebarColor)]
    pub color: [u8; 3],
    #[name("Material Strength")]
    #[control(SidebarSlider)]
    pub material: f32,
    #[name("Morale Strength")]
    #[control(SidebarSlider)]
    pub morale: f32,
    #[name("Attrition")]
    #[control(SidebarSlider)]
    pub attrition: f32,
    #[name("Contribution to Conflict")]
    #[control(SidebarSlider)]
    pub contribution: f32,
    #[name("Fortifications Strength")]
    #[control(SidebarSlider)]
    pub fortifications: f32,
    #[name("Engaged")]
    #[control(SidebarCheckbox)]
    pub engaged: bool,
    #[name("Combat Action")]
    #[control(SidebarEnumDropdown)]
    pub action: CombatAction,
}

impl ConflictMember {
    pub fn new(entity: Entity, color: [u8; 3]) -> Self {
        Self {
            entity,
            color,
            material: 0.0,
            morale: 0.0,
            attrition: 0.0,
            contribution: 0.0,
            fortifications: 0.0,
            engaged: false,
            action: CombatAction::Delay,
        }
    }
}

#[derive(Clone, UiEditableEnum)]
pub enum CombatAction {
    /// Disengage and do nothing.
    Surrender,
    /// Disengage but contribute defence.
    Delay,
    /// Engage with material damage bonus.
    Assault,
    /// Engage with morale damage bonus.
    Maneouver,
    /// Engage with increased attack and reduced defence.
    Charge,
    /// Engage with reduced attack and increased defence.
    Rally,
    /// Engage with fortification damage bonus.
    Siege,
    /// Disengage but deal some damage.
    Skirmish,
    /// Disengage with fortification def bonus.
    Fortify,
}

#[derive(Component)]
pub struct ConflictMarker {
    /// Conflict ID.
    pub id: u32,
}

impl Conflict {
    pub fn update(
        &mut self,
        config: &AtlasSimConfig,
        polities: &mut Query<&mut Polity>,
        regions: &mut Query<&mut Region>,
        rng: &mut impl Rng,
        extras: &mut SimMapData,
    ) {
        // Choose an action if not capitulated.
        let defenders_lost = Self::choose_actions(
            &mut self.defenders,
            config.rules.combat.base_defender_attrition,
            &config.rules.combat.action_table_defender,
            rng,
        );
        let attackers_lost = Self::choose_actions(
            &mut self.attackers,
            config.rules.combat.base_attacker_attrition,
            &config.rules.combat.action_table_attacker,
            rng,
        );
        // If one side is fully defeated, end the conflict.
        if attackers_lost || defenders_lost {
            self.conclude(defenders_lost, polities, regions, extras, config, rng);
            return;
        }
        // Apply defender action.
        let (mat_d_atk, mor_d_atk, mat_d_def, mor_d_def, active_d, siege_d) =
            Self::handle_actions(config, polities, rng, &mut self.defenders);
        let material_d = self.defenders.values().fold(0.0, |acc, x| acc + x.material);
        // Apply attacker action.
        let (mat_a_atk, mor_a_atk, mat_a_def, mor_a_def, active_a, siege_a) =
            Self::handle_actions(config, polities, rng, &mut self.attackers);
        let material_a = self.attackers.values().fold(0.0, |acc, x| acc + x.material);
        // Resolve combat.
        let mat_a_mod = (mat_a_atk / mat_d_def)
            .clamp(0.33, 3.0)
            .powf(config.rules.combat.material_advantage);
        let mor_a_mod = (mor_a_atk / mor_d_def)
            .clamp(0.33, 3.0)
            .powf(config.rules.combat.morale_advantage);
        let mat_d_mod = (mat_d_atk / mat_a_def)
            .clamp(0.33, 3.0)
            .powf(config.rules.combat.material_advantage);
        let mor_d_mod = (mor_d_atk / mor_a_def)
            .clamp(0.33, 3.0)
            .powf(config.rules.combat.morale_advantage);
        let mod_a = mat_a_mod * mor_a_mod;
        let mod_d = mat_d_mod * mor_d_mod;
        let mat_d_dmg = (material_a * config.rules.combat.fatality * mod_a) / active_d as f32;
        let mat_a_dmg = (material_d * config.rules.combat.fatality * mod_d) / active_a as f32;
        let mor_d_dmg = (material_a * config.rules.combat.fragility * mod_a) / active_d as f32;
        let mor_a_dmg = (material_d * config.rules.combat.fragility * mod_d) / active_a as f32;
        let siege_d_dmg = (siege_a * mod_a) / active_d as f32;
        let siege_a_dmg = (siege_d * mod_d) / active_a as f32;
        // Deal damage (defenders).
        for (polity, member) in self.defenders.iter_mut() {
            let mut polity = polities.get_mut(*polity).unwrap();
            member.deal_damage(config, &mut polity, mat_d_dmg, mor_d_dmg, siege_d_dmg);
        }
        // Deal damage (attackers).
        for (polity, member) in self.attackers.iter_mut() {
            let mut polity = polities.get_mut(*polity).unwrap();
            member.deal_damage(config, &mut polity, mat_a_dmg, mor_a_dmg, siege_a_dmg);
        }
    }

    fn choose_actions(
        members: &mut HashMap<Entity, ConflictMember>,
        attrition: f32,
        action_table: &WalkerTable,
        rng: &mut impl Rng,
    ) -> bool {
        let mut lost = true;
        for (_, member) in members.iter_mut() {
            member.attrition += attrition;
            if member.attrition >= 1.0 {
                member.action = CombatAction::Surrender;
                lost &= true;
                continue;
            }
            lost = false;
            member.action = match action_table.next_rng(rng) {
                0 => CombatAction::Assault,
                1 => CombatAction::Maneouver,
                2 => CombatAction::Charge,
                3 => CombatAction::Rally,
                4 => CombatAction::Skirmish,
                5 => CombatAction::Delay,
                6 => CombatAction::Siege,
                7 => CombatAction::Fortify,
                _ => unreachable!(),
            };
        }
        lost
    }

    fn handle_actions(
        config: &AtlasSimConfig,
        polities: &mut Query<&mut Polity>,
        rng: &mut impl Rng,
        members: &mut HashMap<Entity, ConflictMember>,
    ) -> (f32, f32, f32, f32, u32, f32) {
        let randomness = (-config.rules.combat.randomness)..=config.rules.combat.randomness;
        let mut mat_atk_sum = 0.0;
        let mut mor_atk_sum = 0.0;
        let mut mat_def_sum = 0.0;
        let mut mor_def_sum = 0.0;
        let mut active = 0;
        let mut siege_sum = 0.0;
        for (polity, member) in members {
            // Do nothing if surrendered.
            match member.action {
                CombatAction::Surrender => continue,
                _ => active += 1,
            }
            let mut polity = polities.get_mut(*polity).unwrap();
            let roll = 1.0 + rng.gen_range(randomness.clone());
            // Reinforce from the polity.
            let (mat, mor) = polity.mobilize(config);
            member.material += mat;
            member.morale = (member.morale + mor).min(member.material * config.rules.combat.morale_cap);
            member.fortifications = polity.capacities[STR_FORTRESS];
            // Handle combat action.
            let (mat_atk, mor_atk, mat_def, mor_def, sg) = member.handle_action(&config, &polity, roll);
            member.contribution += mat_atk + mor_atk + mat_def + mor_def + sg;
            mat_atk_sum += mat_atk;
            mor_atk_sum += mor_atk;
            mat_def_sum += mat_def;
            mor_def_sum += mor_def;
            siege_sum += sg;
        }
        (
            mat_atk_sum.max(0.001),
            mor_atk_sum.max(0.001),
            mat_def_sum.max(0.001),
            mor_def_sum.max(0.001),
            active,
            siege_sum,
        )
    }

    fn conclude(
        &mut self,
        attackers_won: bool,
        polities: &mut Query<&mut Polity>,
        regions: &mut Query<&mut Region>,
        extras: &mut SimMapData,
        config: &AtlasSimConfig,
        rng: &mut impl Rng,
    ) {
        let mut winner_contrib = 0.0;
        let (winners, losers) = if attackers_won {
            (&mut self.attackers, &mut self.defenders)
        } else {
            (&mut self.defenders, &mut self.attackers)
        };
        // Calculate final contribution ratios.
        for (winner_e, member) in winners.iter_mut() {
            let winner_p = polities.get_mut(*winner_e).unwrap();
            member.contribution *= winner_p.get_tradition_multiplier(config, TRAD_DIPLOMATIC);
            winner_contrib += member.contribution;
        }
        for (loser_e, member) in losers.iter_mut() {
            let loser_p = polities.get_mut(*loser_e).unwrap();
            member.contribution *= loser_p.get_tradition_multiplier(config, TRAD_DIPLOMATIC);
        }
        // Make region claims.
        let mut region_claims_num = HashMap::<Entity, HashMap<Entity, u32>>::default(); // <Winner <Loser, count>>
        let mut tribute_claims = HashMap::<Entity, Vec<Tribute>>::default(); // <Loser, [Tribute]>
        let mut region_claims = HashMap::<Entity, (Entity, f32)>::default(); // <Region, (Winner, claim)>
        for (winner_e, winner_m) in winners.iter_mut() {
            let winner_e = *winner_e;
            let contribution_ratio = (winner_m.contribution / winner_contrib).max(0.0);
            // NOTE: Unsafe is ok, the same polity cannot be a winner and loser in a conflict.
            let mut winner_p = unsafe { polities.get_unchecked(winner_e).unwrap() };
            let mut claims = HashMap::<Entity, u32>::default();
            for (loser_e, loser_m) in losers.iter_mut() {
                let mut loser_p = unsafe { polities.get_unchecked(*loser_e).unwrap() };
                let loser_diplomacy = loser_p.get_tradition_multiplier(config, TRAD_DIPLOMATIC);
                let wars =
                    extras
                        .war_map
                        .dec_war_map_num(winner_e, *loser_e, config.rules.diplomacy.truce_length);
                // If winner borders loser, claim border regions.
                // Lose right to tribute when claiming land.
                if let Some((borders, relation, _)) = winner_p.neighbours.get_mut(loser_e) {
                    // Reset relations if they exist and there are no other wars between them.
                    if wars == 0 {
                        *relation = 0.0;
                        if let Some((_, relation, _)) = loser_p.neighbours.get_mut(&winner_e) {
                            *relation = 0.0;
                        }
                    }
                    //
                    let mut regions_taken = 0;
                    let difficulty = (loser_m.contribution * config.rules.diplomacy.claim_difficulty
                        / winner_m.contribution)
                        .min(1.0);
                    let mut skip_regions = (difficulty * borders.len() as f32) as u32;
                    for region in borders.iter() {
                        // Sanity check: what if someone from another war got this region and it haven't updated yet?
                        if !loser_p.regions.contains(region) {
                            continue;
                        }
                        if let Some((claimee, strength)) = region_claims.get_mut(region) {
                            // If someone made a claim, but they have lower contribution, take it.
                            if contribution_ratio > *strength {
                                if skip_regions > 0 {
                                    skip_regions -= 1;
                                    continue;
                                }
                                *region_claims_num
                                    .get_mut(claimee)
                                    .unwrap()
                                    .get_mut(loser_e)
                                    .unwrap() -= 1;
                                *claimee = winner_e;
                                *strength = contribution_ratio;
                                regions_taken += 1;
                            }
                        } else {
                            if skip_regions > 0 {
                                skip_regions -= 1;
                                continue;
                            }
                            region_claims.insert(*region, (winner_e, contribution_ratio));
                            regions_taken += 1;
                        }
                    }
                    claims.insert(*loser_e, regions_taken);
                }
                // Preliminary tribute calculation.
                let tribute = Tribute::new(
                    winner_e,
                    contribution_ratio / loser_diplomacy,
                    config.rules.diplomacy.tribute_time,
                );
                if let Some(vec) = tribute_claims.get_mut(loser_e) {
                    vec.push(tribute);
                } else {
                    tribute_claims.insert(*loser_e, vec![tribute]);
                }
            }
            region_claims_num.insert(winner_e, claims);
        }
        // Resolve tribute claims.
        for (loser_e, mut tributes) in tribute_claims.drain() {
            let mut loser_p = polities.get_mut(loser_e).unwrap();
            // Do not pay tribute to a winner that claimed our regions.
            tributes.retain(|tribute| {
                if let Some(x) = region_claims_num.get(&tribute.receiver) {
                    if let Some(x) = x.get(&loser_e) {
                        return *x == 0;
                    }
                }
                true
            });
            // Make the winner a global tribute account.
            for tribute in tributes.iter() {
                let _ = extras.tributes.try_insert(tribute.receiver, (0.0, 0.0));
            }
            // Give the loser a list of tributes to pay.
            loser_p.tributes.push(tributes);
        }
        // Resolve region claims.
        for (region_e, (winner_e, _)) in region_claims.drain() {
            let mut region = regions.get_mut(region_e).unwrap();
            let mut loser_p = polities.get_mut(region.polity).unwrap();
            let pos = config.index_to_map_i(region.city_position);
            region.need_visual_update = true;
            loser_p.population -= region.population;
            loser_p.regions.remove(&region_e);
            loser_p.rtree.remove(&pos);
            region.rebel_rate = (region.rebel_rate
                + config.rules.diplomacy.base_rebel_rate
                + loser_p.policies[POL_MILITARIST] * loser_p.get_tradition_multiplier(config, TRAD_MILITANT))
            .clamp(0.0, 2.0);
            let mut winner_p = polities.get_mut(winner_e).unwrap();
            region.polity = winner_e;
            region.color_l = rng.gen_range(-0.1..=0.1);
            winner_p.population += region.population;
            winner_p.regions.insert(region_e);
            winner_p.rtree.insert(pos);
            for i in region.tiles.iter() {
                extras.tile_polity[*i as usize] = Some(winner_e);
            }
            // War damage.
            region.development /= 2.0;
            for str in region.structures.iter_mut() {
                *str /= 2.0;
            }
        }
        // Demobilize and clean up.
        for (polity, member) in self.defenders.iter_mut() {
            let mut polity = polities.get_mut(*polity).unwrap();
            polity.demobilize(self.id, member.material, member.morale, config);
        }
        for (polity, member) in self.attackers.iter_mut() {
            let mut polity = polities.get_mut(*polity).unwrap();
            polity.demobilize(self.id, member.material, member.morale, config);
        }
        self.concluded = true;
    }
}

impl ConflictMember {
    fn handle_action(
        &mut self,
        config: &AtlasSimConfig,
        polity: &Polity,
        roll: f32,
    ) -> (f32, f32, f32, f32, f32) {
        let mat = self.get_material(config, &polity, roll);
        let mor = self.get_morale(config, &polity, roll);
        self.engaged = true;
        match self.action {
            CombatAction::Surrender => {
                self.engaged = false;
                (0.0, 0.0, 0.0, 0.0, 0.0)
            }
            CombatAction::Delay => {
                self.engaged = false;
                let bonus = (1.0 + config.rules.combat.delay_bonus) * config.rules.combat.delay_penalty;
                let penalty = (1.0 - config.rules.combat.delay_bonus) * config.rules.combat.delay_penalty;
                (mat * bonus, mor * bonus, mat * penalty, mor * penalty, 0.0)
            }
            CombatAction::Skirmish => {
                self.engaged = false;
                let bonus = (1.0 + config.rules.combat.skirmish_bonus) * config.rules.combat.skirmish_penalty;
                let penalty =
                    (1.0 - config.rules.combat.skirmish_bonus) * config.rules.combat.skirmish_penalty;
                (mat * bonus, mor * bonus, mat * penalty, mor * penalty, 0.0)
            }
            CombatAction::Assault => {
                let bonus = 1.0 + config.rules.combat.assault_bonus;
                let penalty = 1.0 - config.rules.combat.assault_bonus;
                (mat * bonus, mor * penalty, mat * bonus, mor * penalty, 0.0)
            }
            CombatAction::Maneouver => {
                let bonus = 1.0 + config.rules.combat.maneouver_bonus;
                let penalty = 1.0 - config.rules.combat.maneouver_bonus;
                (mat * penalty, mor * bonus, mat * penalty, mor * bonus, 0.0)
            }
            CombatAction::Rally => {
                let bonus = 1.0 + config.rules.combat.rally_bonus;
                let penalty = 1.0 - config.rules.combat.rally_bonus;
                (mat * penalty, mor * penalty, mat * bonus, mor * bonus, 0.0)
            }
            CombatAction::Charge => {
                let bonus = 1.0 + config.rules.combat.charge_bonus;
                let penalty = 1.0 - config.rules.combat.charge_bonus;
                (mat * bonus, mor * bonus, mat * penalty, mor * penalty, 0.0)
            }
            CombatAction::Siege => {
                let bonus = config.rules.combat.siege_bonus;
                let penalty = config.rules.combat.siege_penalty;
                let siege = (mat + mor) * bonus;
                let mat = mat * penalty;
                let mor = mor * penalty;
                (mat, mor, mat, mor, siege)
            }
            CombatAction::Fortify => {
                self.engaged = false;
                let bonus = config.rules.combat.fortify_bonus * self.fortifications;
                let penalty = config.rules.combat.fortify_penalty;
                (mat * penalty, mor * penalty, mat + bonus, mor + bonus, 0.0)
            }
        }
    }

    fn deal_damage(
        &mut self,
        config: &AtlasSimConfig,
        polity: &mut Polity,
        mat_dmg: f32,
        mor_dmg: f32,
        siege: f32,
    ) {
        match self.action {
            CombatAction::Surrender => return,
            _ => {}
        }
        let mut mat_dmg_left = mat_dmg;
        let mut mor_dmg_left = mor_dmg;
        let total_pop = polity.population + polity.jobs.military;
        // Deal damage to army if engaged.
        if self.engaged && self.morale > 0.0 && self.material > 0.0 {
            let mor_diff = self.morale - mor_dmg_left;
            if mor_diff > 0.0 {
                self.morale = mor_diff;
                mor_dmg_left = 0.0;
            } else {
                self.morale = 0.0;
                mor_dmg_left += -mor_diff * config.rules.combat.breakdown;
            }
            let mut loss;
            let mat_diff = self.material - mor_dmg_left;
            if mat_diff > 0.0 {
                loss = mor_dmg_left;
                self.material = mat_diff;
            } else {
                self.material = 0.0;
                loss = self.material;
            }
            let mat_diff = self.material - mat_dmg_left;
            if mat_diff > 0.0 {
                loss += mat_dmg_left;
                self.material = mat_diff;
                mat_dmg_left = 0.0;
            } else {
                loss += self.material;
                self.material = 0.0;
                mat_dmg_left = -mat_diff;
            }
            let loss_ratio = loss / total_pop;
            polity.deal_military_damage(loss);
            self.attrition += loss_ratio * config.rules.combat.combat_attrition;
        }
        // Deal leftover damage to fortifications.
        if self.fortifications > 0.0 {
            let miltech = polity.get_tech_multiplier(config, SCI_MILTECH);
            let old_forts = self.fortifications;
            let mat_diff =
                self.fortifications * miltech - (mat_dmg_left + siege) / config.rules.combat.fort_damage;
            if mat_diff > 0.0 {
                self.fortifications = mat_diff / miltech;
            } else {
                self.fortifications = 0.0;
                mat_dmg_left = -mat_diff;
            }
            polity.deal_fort_damage(old_forts - self.fortifications);
        }
        // Deal leftover damage as attrition.
        if mat_dmg_left > 0.0 {
            let mat_dmg_left = mat_dmg_left / polity.get_tech_multiplier(config, SCI_MEDICINE);
            self.attrition += (mat_dmg_left * config.rules.combat.civilian_attrition) / total_pop;
            polity.deal_civilian_damage(
                ((mat_dmg_left * config.rules.combat.civilian_damage) / total_pop)
                    .min(config.rules.combat.civilian_damage_max),
            );
        }
    }

    #[inline(always)]
    fn get_material(&self, config: &AtlasSimConfig, polity: &Polity, roll: f32) -> f32 {
        self.material * polity.get_tech_multiplier(config, SCI_MILTECH) * roll
    }

    #[inline(always)]
    fn get_morale(&self, config: &AtlasSimConfig, polity: &Polity, roll: f32) -> f32 {
        self.morale * polity.get_tradition_multiplier(config, TRAD_MILITANT) * roll
    }
}
