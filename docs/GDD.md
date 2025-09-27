# Game Design Document — **Eryndor**

*A 2D side-scrolling MMORPG with skill-based progression inspired by RuneScape’s mastery system and MapleStory’s charm. Built in Rust using **SpaceTimeDB** for the server and **Bevy** for the client, with worlds authored in LDtk or Tiled. Set in the fractured realm of **Eryndor**, where elemental legacies, ancient rivalries, and living myths define player progression.*  

---

## 1) Vision & Pillars

**Vision**  
Eryndor is a place where history is written in action: the bow you string, the staff you wield, the ores you smelt, and the alliances you forge. The MMO blends nostalgic 2D sidescrolling charm with deep horizontal skill progression rooted in the world’s lore.  

**Design Pillars**
- **Your path is your weapon:** Train combat and non-combat skills through use, with no locked-in classes.  
- **Lore-driven progression:** Skills, zones, and items reflect the factions, myths, and histories of Eryndor.  
- **Social & cooperative:** Guilds, parties, events, and a living economy bring players together.  
- **Player economy focus:** Gathering, crafting, and trading tie into regional scarcity and cultural heritage.  
- **Server-authoritative & scalable:** Rust + SpaceTimeDB ensures real-time MMO simulation with persistence.  

---

## 2) The World of Eryndor

A land fractured by the **Shattering of the Leylines**, Eryndor is defined by elemental dominions, roaming clans, and ancient ruins.  

### Key Factions
- **The Winterbound** — keepers of frost magic and frozen ruins in the north.  
- **The Ember Lords** — descendants of forge-masters and lava cults in volcanic caverns.  
- **The Skyborne** — storm-worshipping clans who live in the peaks, mapping leyline storms.  
- **The Rootguard** — druids and defenders who anchor the balance in Greenbough Enclave.  
- **The Stone-Breakers** — barbarian tribes forged in war against titans and giants.  

### Zones
- **Shattered Vale (Starter Zone):** fractured farmlands, the first taste of leyline corruption.  
- **Frostbound Expanse:** icy tundra, frozen ruins of the Winterbound.  
- **Emberdepth Caverns:** volcanic caverns where molten rivers conceal ancient forges.  
- **Stormreach Peaks:** vertical cliffs, sky-bridges, and eternal tempests.  
- **Greenbough Enclave:** central city-hub and cultural crossroads.  

Each zone is tied to **specific skill development** and **lore progression**, ensuring exploration and mastery feel organic.  

---

## 3) Core Gameplay Loops

### Combat & Skill Loop
- Equip a weapon (bow, staff, sword, etc.)  
- Fight mobs tied to regional lore → gain XP in that weapon’s skill  
- Unlock active abilities and passive bonuses rooted in that faction’s traditions  
- Graduate to more dangerous regions  

### Gathering & Crafting Loop
- Explore Eryndor’s fractured lands → harvest resources (Winterbound crystals, Ember ore, Skyborne stormstones)  
- Refine them into materials → craft weapons, armor, consumables  
- Trade via markets → reinvest into skill growth or guild projects  

### Social Loop
- Form parties, guilds, and caravans  
- Join world events (leyline surges, roaming titans)  
- Contribute to shared goals (Enclave defense, guild banners, seasonal rituals)  

---

## 4) Progression: Skills & Leveling  

Progression mirrors RuneScape’s **use-based skills**, but infused with Eryndor’s lore.  

### Combat Skills
- **Ranger (Bow, Crossbow):** legacy of Vale scouts; mobility, traps, precision strikes.  
- **Ice Mage (Ice Staff, Winterbound Crystal):** freezing, slowing, shattering enemies.  
- **Fire Mage (Ember Staff, Volcanic Relic):** AoE damage, ignite effects, explosive bursts.  
- **Lightning Mage (Storm Rods, Skyborne Focus):** chaining attacks, high burst, mobility tools.  
- **Barbarian (2H Mace/Axe):** Stone-Breaker tradition; heavy hits, shockwaves, armor breaks.  
- **Blademaster (1H Sword/Dual Blades):** Rootguard defenders; parry, stance-swaps, swift combos.  
- **Defender (Shield/Hammer):** frontline guardians; crowd control, buffs, taunts.  

> Weapon **mastery trees** unlock lore-rooted skills: e.g., Ice Mage learns *Shardstorm*, a Winterbound rite spell.  

### Non-Combat Skills
- **Mining:** Emberdepth veins, ore-smelting traditions.  
- **Smithing:** forging Embersteel, armor, and weapons.  
- **Fletching:** bows/arrows from Greenbough forests.  
- **Alchemy:** Rootguard herbal brews, leyline potions.  
- **Cooking:** sustenance, buffs, and social feasts.  
- **Cartography:** Skyborne star-charts revealing secret routes.  
- **Beast Handling:** eventual pets/mounts tied to Eryndor wildlife.  

---

## 5) Combat Systems

- **Sidescrolling combat**: platforming mechanics (ledge grabs, jump arcs, one-way platforms).  
- **Elemental interactions:** Freeze → Shatter; Ignite + Shock → Overload.  
- **Boss mechanics:** stagger meters, AoE telegraphs, lore-unique abilities.  
- **PvE focus:** mob hunts, dungeons, world bosses tied to faction lore.  
- **PvP optional:** duels and arena combat framed as cultural trials.  

---

## 6) World & Content

### Example Zone Flow
1. **Shattered Vale** → intro to farming, hunting, Ranger basics.  
2. **Frostbound Expanse** → ice mobs, frozen mining nodes, Ice Mage path.  
3. **Emberdepth Caverns** → Barbarian, Fire Mage growth; rare ores.  
4. **Stormreach Peaks** → Lightning Mage, Cartography quests, vertical design.  
5. **Greenbough Enclave** → central trade hub; crafting, guilds, seasonal events.  

### Activities
- **Faction Quests:** tied to Winterbound/Ember/Skyborne/Rootguard lore.  
- **World Events:** leyline surges (rare mobs/resources), caravan escorts.  
- **Dungeons:** Emberforges, Winterbound Crypts, Skyborne Trials.  

---

## 7) Economy & Crafting  

- **Faction resources**: Winterbound crystals, Ember ore, Skyborne stormstones.  
- **Refining mini-games:** smithing bellows, alchemy timing, fletching accuracy.  
- **Markets:** regional scarcity encourages trading across zones.  
- **Contracts:** guild work orders, player commissions.  
- **Sinks:** guild projects, banner cosmetics, seasonal rituals.  

---

## 8) Social Systems

- **Parties:** loot-sharing, skill-boosting synergies.  
- **Guilds:** lore-flavored banners, faction allegiances, shared projects.  
- **Chat & Emotes:** region, party, guild channels; gesture-based emotes.  
- **Safety:** profanity filters, reporting, spam throttles.  

---

## 9) Technical Design  

### Server (Rust + SpaceTimeDB)
- **Tables:** Characters, Skills, Inventory, Zones, Entities, Markets, Guilds.  
- **Modules:** Combat resolution, XP gain, crafting recipes, market engine.  
- **Sharding:** zones run on independent partitions; travel via transfers.  
- **Interest Management:** spatial hashing to limit entity updates.  

**Example: Skills Table**  
```sql
TABLE skills (
  character_id UUID,
  skill_id TEXT,   -- e.g., "ranger", "ice_mage", "smithing"
  level INT,
  xp BIGINT,
  faction TEXT,    -- "Winterbound", "Ember Lords", etc.
  PRIMARY KEY (character_id, skill_id)
);
```

### Client (Bevy)
- **ECS components:** Position, Velocity, Health, Weapon, StatusEffects, Interactable.  
- **Systems:** Input prediction, reconciliation, world streaming, combat state machines.  
- **UI:** Lore-styled HUD, skill trees, market, inventory, guild banners.  
- **World loading:** LDtk/Tiled import → chunked zone streaming.  

---

## 10) Roadmap  

**MVP (1 Zone)**
- Shattered Vale, Ranger/Ice Mage/Barbarian skills, basic market, parties, chat.  

**Alpha**
- Add Frostbound, Emberdepth, Stormreach.  
- Expand crafting (smithing, alchemy, fletching).  
- Guilds v1.  

**Beta**
- PvP arenas (Skyborne trials), housing/guild halls, seasonal rituals.  
- 5+ fully developed zones with world events.  

---

## 11) Art & Audio Direction  

- **Pixel art 1.5D:** vibrant but readable silhouettes; each faction has a distinct visual style.  
- **Animations:** snappy, clear telegraphs for combat.  
- **Audio:** ambient soundscapes tied to lore — whispering winds in Stormreach, forge roars in Emberdepth.  

---

## 12) Risks & Mitigations  

- **Scale:** per-zone shards and strong interest management.  
- **Content treadmill:** procedural events (leyline surges) reduce grind.  
- **Economy inflation:** strong sinks and server-validated trade.  
- **Cheating:** server-authoritative validation, no client-side XP/loot authority.  

---

## 13) MVP Definition of Done  

- Stable server with 200–300 CCU per shard.  
- Shattered Vale playable end-to-end with 3 combat skills and 2 non-combat skills.  
- Basic market, party system, and world event working.  
- Client runs at 60 FPS in target zones.  
