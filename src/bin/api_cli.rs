/// An ugly CLI to test open5e.com API calls
/// Will be removed once work on the GUI starts and basic unit-tests are added for the API

use dnd_character_creator::DndApiClient;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DndApiClient::new();
    
    loop {
        println!("\nDnD API Test Menu:");
        println!("1. Search Spells");
        println!("2. Get Spell Details");
        println!("3. List All Classes");
        println!("4. List All Races");
        println!("5. Exit");
        print!("\nEnter your choice (1-5): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter spell search term: ");
                io::stdout().flush()?;
                let mut search_term = String::new();
                io::stdin().read_line(&mut search_term)?;
                let search_term = search_term.trim();

                println!("\nSearching for spells containing '{search_term}'...");
                match client.search_spells(search_term).await {
                    Ok(spells) => {
                        println!("\nFound {} spells:", spells.len());
                        for (i, spell) in spells.iter().enumerate() {
                            let name = spell.get_string("name").unwrap_or_else(|| "Unknown".to_string());
                            let level = spell.get_string("level").unwrap_or_else(|| "Unknown".to_string());
                            println!("{}. {} (Level {})", i + 1, name, level);
                        }
                        
                        if !spells.is_empty() {
                            print!("\nEnter the number of the spell to see details (or 0 to skip): ");
                            io::stdout().flush()?;
                            let mut selection = String::new();
                            io::stdin().read_line(&mut selection)?;
                            if let Ok(index) = selection.trim().parse::<usize>() {
                                if index > 0 && index <= spells.len() {
                                    let spell = &spells[index - 1];
                                    println!("\nSpell Details:");
                                    println!("Name: {}", spell.get_string("name").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Level: {}", spell.get_string("level").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("School: {}", spell.get_string("school").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Casting Time: {}", spell.get_string("casting_time").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Range: {}", spell.get_string("range").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Components: {}", spell.get_string("components").unwrap_or_else(|| "Unknown".to_string()));
                                    if let Some(material) = spell.get_string("material") {
                                        if !material.is_empty() {
                                            println!("Material: {}", material);
                                        }
                                    }
                                    println!("Duration: {}", spell.get_string("duration").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Concentration: {}", spell.get_string("concentration").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("Ritual: {}", spell.get_string("ritual").unwrap_or_else(|| "Unknown".to_string()));
                                    println!("\nDescription:");
                                    println!("{}", spell.get_string("desc").unwrap_or_else(|| "No description available".to_string()));
                                    if let Some(higher_level) = spell.get_string("higher_level") {
                                        if !higher_level.is_empty() {
                                            println!("\nAt Higher Levels:");
                                            println!("{}", higher_level);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => println!("Error searching spells: {e}"),
                }
            }
            "2" => {
                print!("Enter spell name: ");
                io::stdout().flush()?;
                let mut spell_name = String::new();
                io::stdin().read_line(&mut spell_name)?;
                let spell_name = spell_name.trim();

                println!("\nFetching details for '{spell_name}'...");
                match client.find_spell_by_name(spell_name).await {
                    Ok(Some(spell)) => {
                        println!("\nSpell Details:");
                        println!("Name: {}", spell.get_string("name").unwrap_or_else(|| "Unknown".to_string()));
                        println!("Level: {}", spell.get_int("level").map(|l| l.to_string()).unwrap_or_else(|| "Unknown".to_string()));
                        
                        // Handle school as nested object
                        if let Some(school_dict) = spell.get_dict("school") {
                            println!("School: {}", school_dict.get_string("name").unwrap_or_else(|| "Unknown".to_string()));
                        } else {
                            println!("School: Unknown");
                        }
                        
                        println!("Casting Time: {}", spell.get_string("casting_time").unwrap_or_else(|| "Unknown".to_string()));
                        println!("Range: {}", spell.get_string("range_text").unwrap_or_else(|| "Unknown".to_string()));
                        
                        // Build components string from individual fields
                        let mut components = Vec::new();
                        if spell.get_bool("verbal").unwrap_or(false) { components.push("V"); }
                        if spell.get_bool("somatic").unwrap_or(false) { components.push("S"); }
                        if spell.get_bool("material").unwrap_or(false) { components.push("M"); }
                        println!("Components: {}", if components.is_empty() { "None".to_string() } else { components.join(", ") });
                        
                        if let Some(material) = spell.get_string("material_specified") {
                            if !material.is_empty() {
                                println!("Material: {}", material);
                            }
                        }
                        println!("Duration: {}", spell.get_string("duration").unwrap_or_else(|| "Unknown".to_string()));
                        println!("Concentration: {}", if spell.get_bool("concentration").unwrap_or(false) { "Yes" } else { "No" });
                        println!("Ritual: {}", if spell.get_bool("ritual").unwrap_or(false) { "Yes" } else { "No" });
                        println!("\nDescription:");
                        println!("{}", spell.get_string("desc").unwrap_or_else(|| "No description available".to_string()));
                        if let Some(higher_level) = spell.get_string("higher_level") {
                            if !higher_level.is_empty() {
                                println!("\nAt Higher Levels:");
                                println!("{}", higher_level);
                            }
                        }
                    }
                    Ok(None) => {
                        println!("Spell not found. Try searching for spells first to see available options.");
                    }
                    Err(e) => println!("Error fetching spell: {e}"),
                }
            }
            "3" => {
                println!("\nFetching classes...");
                match client.get_classes().await {
                    Ok(classes) => {
                        println!("\nAvailable Classes:");
                        for class in classes {
                            print!("{class}, ");
                        }
                        println!();
                    }
                    Err(e) => println!("Error fetching classes: {e}"),
                }
            }
            "4" => {
                println!("\nFetching races...");
                match client.get_races().await {
                    Ok(races) => {
                        println!("\nAvailable Races:");
                        for race in races {
                            print!("{race}, ");
                        }
                        println!();
                    }
                    Err(e) => println!("Error fetching races: {e}"),
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }

    Ok(())
} 