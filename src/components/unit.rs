use bevy::prelude::*;
use crate::components::faction::Ownership;

// Core unit identity - requires Statsheet and Ownership components
#[derive(Component, Debug, Clone)]
#[require(Statsheet, Ownership)]
pub struct Unit {
    pub name: String, 
    pub unit_type: UnitType,
    pub state: UnitState,
    pub target: Option<Entity>,
}

// Unified stat sheet for all unit attributes and derived stats
#[derive(Component, Debug, Clone)]
pub struct Statsheet {
    // Primary attributes
    pub strength: f32,
    pub agility: f32, 
    pub intelligence: f32,
    
    // Base stats (before attribute calculations)
    pub base_health: f32,
    pub base_mana: f32,
    pub base_damage: f32,
    pub base_armor: f32,
    pub base_attack_speed: f32,
    pub base_move_speed: f32,
    
    // Current state
    pub health: f32,
    pub mana: f32,
    
    // Derived stats (calculated from attributes)
    pub max_health: f32,
    pub max_mana: f32,
    pub damage: f32,
    pub armor: f32,
    pub attack_speed: f32,
    pub move_speed: f32,
    
    // Combat properties
    pub attack_type: AttackType,
    pub armor_type: ArmorType,
    pub attack_range: f32,
    
    // Other properties
    pub turn_rate: f32,
    pub sight_range: f32,
}

// Unit type categorization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitType {
    Hero,
    Building,
    Melee,
    Ranged,
    Caster,
    Worker,
}

// Player/faction ownership
// Removed PlayerId enum - now using Ownership component from faction.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackType {
    Normal,
    Pierce,
    Siege,
    Magic,
    Chaos,
    Hero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorType {
    Unarmored,
    Light,
    Medium,
    Heavy,
    Fortified,
    Hero,
}

// Current unit state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitState {
    Idle,
    Moving,
    Attacking,
    Casting,
    Constructing,
    Harvesting,
    Dead,
}

// Default implementation for Unit
impl Default for Unit {
    fn default() -> Self {
        Self {
            name: "Default Unit".to_string(),
            unit_type: UnitType::Melee,
            state: UnitState::Idle,
            target: None,
        }
    }
}

impl Statsheet {
    // Calculate derived stats based on attributes
    pub fn calculate_derived_stats(&mut self) {
        // Health: Base + (Strength * 20)
        self.max_health = self.base_health + (self.strength * 20.0);
        
        // Mana: Base + (Intelligence * 15)
        self.max_mana = self.base_mana + (self.intelligence * 15.0);
        
        // Damage: Base + (Primary Attribute * 1.0)
        // Primary attribute depends on unit type, but let's use strength for now
        self.damage = self.base_damage + (self.strength * 1.0);
        
        // Armor: Base + (Agility * 0.15)
        self.armor = self.base_armor + (self.agility * 0.15);
        
        // Attack Speed: Base + (Agility * 0.01)
        self.attack_speed = self.base_attack_speed + (self.agility * 0.01);
        
        // Move Speed: Base + (Agility * 0.005)
        self.move_speed = self.base_move_speed + (self.agility * 0.005);
    }
    
    // Helper to initialize stats
    pub fn initialize(&mut self) {
        // Set current values to max
        self.health = self.max_health;
        self.mana = self.max_mana;
    }
}

impl Default for Statsheet {
    fn default() -> Self {
        let mut stats = Self {
            // Primary attributes (WC3-like baseline)
            strength: 18.0,
            agility: 18.0,
            intelligence: 18.0,
            
            // Base stats (before attributes)
            base_health: 100.0,
            base_mana: 0.0,
            base_damage: 10.0,
            base_armor: 0.0,
            base_attack_speed: 1.0,
            base_move_speed: 3.0,
            
            // Current state (will be set by initialize)
            health: 0.0,
            mana: 0.0,
            
            // Derived stats (will be calculated)
            max_health: 0.0,
            max_mana: 0.0,
            damage: 0.0,
            armor: 0.0,
            attack_speed: 0.0,
            move_speed: 0.0,
            
            // Combat properties
            attack_type: AttackType::Normal,
            armor_type: ArmorType::Medium,
            attack_range: 1.5,
            
            // Other properties
            turn_rate: 0.5,
            sight_range: 10.0,
        };
        
        // Calculate derived stats
        stats.calculate_derived_stats();
        
        // Set initial values
        stats.initialize();
        
        stats
    }
}

// No UnitBundle implementation needed with the required components approach

// Systems for unit behavior will be implemented later
pub fn unit_system() {
    // This will be implemented with actual unit logic
}