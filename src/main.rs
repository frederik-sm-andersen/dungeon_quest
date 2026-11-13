use leptos::prelude::*;

use rand::RngExt;

#[derive(Clone)]
struct Items {
    id: usize,
    item_name: String,
    item_value: u32,
    item_power: String,
}

#[derive(Clone)]
struct CharacterStats {
    id: usize,
    stat_name: String,
    stat_number: u32,
}

// #[derive(Clone)]
// struct DiceValue {
//     random_number: i32,
// }

#[component]
fn Inventory() -> impl IntoView {
    let (items, _set_items) = signal(vec![
        Items {
            id: 1,
            item_name: "sword".to_string(),
            item_value: 3,
            item_power: "atk".to_string(),
        },
        Items {
            id: 2,
            item_name: "armor".to_string(),
            item_value: 5,
            item_power: "def".to_string(),
        },
        Items {
            id: 3,
            item_name: "crossbow".to_string(),
            item_value: 2,
            item_power: "atk".to_string(),
        },
        Items {
            id: 4,
            item_name: "rope".to_string(),
            item_value: 4,
            item_power: "nav".to_string(),
        },
    ]);

    view! {
        <div class="dun__inv-box">
            <h2 class="dun__inv-head">"Inventory"</h2>
            <ul class="dun__inv-list">
                <For
                    each=move || items.get()
                    key=|items| items.id
                    children=move |items: Items| {
                        view! {
                            <li class="dun__inv-item">
                                <p>{items.item_name}</p>
                                <p>{items.item_value}{items.item_power}</p>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Character_Title() -> impl IntoView {
    view! {
        <div class="dun__char-name-an-hp">
            <h2>"Henrik the warrior"</h2>
            <div class="dun__char-hp-box">
                <p>"20/20"</p>
            </div>
        </div>
    }
}

#[component]
fn Character_Stats() -> impl IntoView {
    let (character_stats, _set_character_stats) = signal(vec![
        CharacterStats {
            id: 1,
            stat_name: "Strength".to_string(),
            stat_number: 7,
        },
        CharacterStats {
            id: 2,
            stat_name: "Dexterity".to_string(),
            stat_number: 5,
        },
        CharacterStats {
            id: 3,
            stat_name: "Contitution".to_string(),
            stat_number: 6,
        },
        CharacterStats {
            id: 4,
            stat_name: "Intellingence".to_string(),
            stat_number: 3,
        },
        CharacterStats {
            id: 5,
            stat_name: "Wisdom".to_string(),
            stat_number: 3,
        },
        CharacterStats {
            id: 6,
            stat_name: "Charisma".to_string(),
            stat_number: 4,
        },
    ]);

    view! {

                <ul class="dun__char-stats">
                    <For
                        each=move || character_stats.get()
                        key=|character_stats| character_stats.id
                        children=move |character_stats: CharacterStats| {
                            view! {
                                <li class="dun__char-stat-box">
                                    <h3 class="dun__char-stat-name">{character_stats.stat_name}</h3>
                                    <p class="dun__char-stat-number">{character_stats.stat_number}</p>
                                </li>
                        }
                    }
                    />
                </ul>

    }
}

// impl DiceValue {
//     fn dice_roll() {
//         let (random_number, set_random_number) = signal(0);

//         let genrate_random = move |_| {
//             let mut rng = rand::rng();
//             let value = rng.random_range(1..=20);
//             set_random_number.set(value);
//         };
//     }

//     #[component]
//     fn Dice() -> impl IntoView {
//         view! {
//             <button on:click=genrate_random>
//                 "Dice"
//             </button>
//             <p>{random_number}</p>
//         }
//     }
//

#[component]
fn Dice() -> impl IntoView {
    let (random_number, set_random_number) = signal(0);

    let genrate_random = move |_| {
        let mut rng = rand::rng();
        let value = rng.random_range(1..=20);
        set_random_number.set(value);
    };

    view! {
        <div class="dun__dice-and-reso">
        <button class="dun__dice-but" on:click=genrate_random>
            <svg class="dun__dice-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640"><path d="M288.4 55.8C308 44.7 332 44.7 351.5 55.8L543.5 164.6C563.5 176 575.9 197.2 575.9 220.3L575.9 435.9C575.9 458.9 563.5 480.2 543.5 491.6L351.5 600.4C331.9 611.5 307.9 611.5 288.4 600.4L96.4 491.5C76.4 480.1 64 458.8 64 435.8L64 220.2C64 197.2 76.4 175.9 96.4 164.5L288.4 55.8zM340.4 129C331 113.8 309 113.8 299.6 129L223 252.7L137.6 206.4L133.8 204.8C124.9 202.1 115 205.9 110.4 214.4C105.8 222.9 108 233.3 115.1 239.2L118.4 241.5L201.8 286.7L127.2 407.3C119.3 420.2 125 437 139 442.4L300 504.3L300 544C300 555 309 564 320 564C331 564 340 555 340 544L340 504.3L501 442.4C515 437 520.7 420.2 512.8 407.4L438.1 286.8L521.5 241.6C531.2 236.3 534.8 224.2 529.6 214.5C524.4 204.8 512.2 201.2 502.5 206.4L417 252.7L340.4 129zM293.4 458.9L171.4 412L225.9 323.9L293.4 458.9zM468.6 412L346.6 458.9L414.1 323.9L468.6 412zM383.3 296L320 422.6L256.7 296L383.3 296zM372 256L267.9 256L319.9 172L371.9 256z"/></svg>
        </button>
        <p class="dun__dice-reso">{random_number}</p>
        </div>
    }
}

#[component]
fn Action() -> impl IntoView {
    view! {
        <button>
            "action like use rope"
        </button>
    };
}

#[component]
fn App() -> impl IntoView {
    // let (random_number, set_random_number) = signal(0);

    view! {
        <div class="dun__game-box">
            <div class="dun__inv-and-text">
                <div class="dun__inv-and-coo">
                    <Inventory/>
                </div>
                <article class="dun__text-box">
                    <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Debitis, natus! Ad ipsum non assumenda inventore similique, labore sapiente aliquam, atque aspernatur dolorum consectetur veniam in reiciendis, velit a autem iste?</p>
                </article>
            </div>
            <div class="dun__char-box">
                <Character_Title/>
                <Character_Stats/>
                <Dice/>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
