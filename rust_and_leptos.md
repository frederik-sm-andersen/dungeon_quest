--rust og leptos

--start
    for at start i rust skal du først download rust brug det link "https://rust-lang.org/tools/install/"
    efter du har download rust gå ind i en taminal og skrive command (cargo new project_name)
    (cargo er hoved order der bliver brugt lige somme npm)
    derefter open det project i vscode der ser du en src mappe gå ind i den brug command (cd src) for at komme ind i mappe

    ind i mappe stå der allede 
    fn main(){
        println!("Hello, World")
    }
    fn er = med function 
    main er name på function (det er vigtigt at der altid er en main function (fn main()) siden det er end point at den code gå i gennem )
    println!("") er måden at få ting skrivet i taminal
    for at få terminal til at køre skriver du cargo run, når cargo run køre bliver der lavet en exe file hvor all de code bliver compiler hvilket gør at din code bliver maget hurtiger

--typer
    rust er et statically typed language, hvilke betyder at compiler skal bruge viden om hvad for nogle value den arbejder med
    der er 4 typer integers, floating-point numbers, Booleans, and characters

    --Integer Types
        integer er tall ude fractional component og bliver brugt nå en value med en tall skal settes 

        der har vi i i8(signed) og u8(unsigned) so integer types og grund til vi skriver dem er for at fortall rust hvad deres bit er 

        signed (i8) betyder en usikkerhed hvor mange bit det der skal bruges

        unsigend (u8) betyder en sikkerhed hvor mange bit der skal bruges

        isize (signed) og usize (unsigend) seter bit til hvad køre på hvilket kan være alt fra 32-bit til 64-bit 

        tabellen for signed og unsigned ser sådan ud
                     Length                     Signed  Unsigned
                     8-bit                      i8      u8
                     16-bit                     i16     u8
                     32-bit                     i32     u32
                     64-bit                     i64     u64
                     128-bit                    i128    u128
                     Architecture-dependent     isize   usize
    --floating-point types
        floating-point er brugt typisk nå en tall value skal ændres som regne styker, de bruger f32 og f64 som bit tal, hvis der ikke skives en bit fortolker rust det som f64 automatisk,

        det kan se sådan ud
        fn main() {
            let x = 2.0; //f64

            let y: f32 = 3.0; //f32
        }
    --Boolean types
        boolean type er en true eller false og er en byte i størrelse
        det kan se sådan ud
        fn main(){
            let t = true;

            let f: bool = false;
        }

    --Character Type
        character eller char value er meget simpel, er 4 bytes og kan enhold mere en bogstaver men også tegn, emojis og bare et enkle space " " er all godkendt i char

        det kan se sådan ud
        fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
        }
    
    --Compound Types
        compound er en gruppe a values, de kan være tuple, array, struct og enum

        --tuple 
            kan en hold en forskellig values 

            det kan se sådan ud
            let human = ("Alice", 30, false);

        --Array Type
            en array type kan have man values men de skal være af den sammen type og have en sat length

            det kan se sådan ud
            fn main() {
            let a = [1, 2, 3, 4, 5];
         }

        --structs
            structs kan inhold forskellige value types lihe som tuple men giver mere structure i values

            det kan se sådan ud
            struct CharacterStats {
            pub id: usize,
            pub stat_name: String,
            pub stat_number: u32,
            }

        --enums 
            enums vasital og giver den mulighed for at tag en type som kan representer mange forskelling varianter 
            enum kan ænders mellem values

            eksampel som ip addres hvor der er enten en version four eller en version six

            det kan se sådan ud
            enum IpAddr {
            V4(String),
            V6(String),
            }

            let home = IpAddr::V4(String::from("127.0.0.1"));

            let loopback = IpAddr::V6(String::from("::1"));

--borriwing og references
    i rust er all values imutable some standard hvilket betyder at value ikke kan ænders hvilket også betyder at den mere sikker i forhold til ander programming languages og hurtiger
    for at kunne ænder på en value skal du skrive (mut) før value name
    en and ting som at huske er borriwing/references, 
    når du borrow en value betyder det at vi laver en references til vadien i steden for a shadow det(jeg forklar mere om shadowing om let) får at borrow skal der et & teng på

    det kan se sådan ud
    let mut _x: i32 = 5; // først vadi
    let _r: &mut i32 = &mut _x;

--shadowing
    shadowing er nå en variable bliver brugt uden at blive references

    det kan se sådan ud 
    let x = 5; // først value

    let x = x + 1; // den value tag ejerskab over den først x inhold, hvilket betyder at vis du references til x vil du få 6 i stedet for 5

    fordelen er at man kan overføre value ind i ander vadi vis den skal brugs her og genbrug navn i sted for a find på nå navn med små ænder

--prinln! 
    for at printe noget ud i terminal, in i println!("") kan du skrive noget, for at få en value med skal der være {} in i den og value

    eksample
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    vis det er en samlen a value ser den sådan ud
    let alian: (String, i32, bool) = ("Alan".to_string(), 30, true); // "".to_string() er for at convater det til en string
    println!("Alian Tuple: {:?}", alian);

------------------------------------------------------------------------------------------

--leptos
    leptos er et rust web framework lit lige som react 
    --start
    for at start leptos use (rustup target add 'wasm32-unknow-unknow') for at installer det
    efter det skal du download trunk use (cargo install trunk) trunk som compiler din code og køre hjemside
    der efter gå du ind på src/main hvor du skrive code, oppe i toppen sriver du (use leptos::prelude::*;) den er en del mange forskelle traits som bliver brugt, man kan også import ander være for sige 

--component 
    for at lave en component skal du skrive (#[component])

    et component kan se sådan ud

    #[component]
    fn Action() -> impl IntoView {
    view! {
        <button>
            "action like use rope"
        </button>
    };
}

--main og app
    i main er der en function der header main det er dit end point hvor code gå igemmen derfor vil det være best at lave en component der har all de ander components også køre den igemmen main lidt af det her 

    #[component]
    fn App() -> impl IntoView {
    // let (random_number, set_random_number) = signal(0);

    view! {
        <div">
            <Action/>
        </div>
    }
    }

    fn main() {
    leptos::mount::mount_to_body(App); // leptos::mount::mount_to_body() er for at app ind i index.html body
    }

--data
    måden du laver et reactiv component er på den her måde

    --struct 
        føres lave en struct

        #[derive(Clone)] // er for at kun genbrug den 
        struct CharacterStats {
            id: usize,
            stat_name: String,
            stat_number: u32,
        }

    --component
        // -> betyder at den return
        // impl implementering og IntoView ind i hvors view
        #[component]
        fn Character_Stats() -> impl IntoView {
            // lave let som er et signal (virker lidt lige som usestat) hvor der er en vec(er lige som et array men kan ænders i størrelsen)
        let (character_stats, _set_character_stats) = signal(vec![
            // her lave CharacterStats holder på vores data
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
                //her lave vi for som fungerer som en unik let 
                    <For
                        each=move || character_stats.get() // each=move tag vi hvad der stå i character_stats letten 
                        key=|character_stats| character_stats.id // her lave vi en key id på være element
                        children=move |character_stats: CharacterStats| // her tag vi data typerne fra struct og fortaler vad data er  {
                            view! {
                                <li class="dun__char-stat-box">
                                    <h3 class="dun__char-stat-name">{character_stats.stat_name}</h3> // her tag vi lettens data og fortaler hvad den skal tag fat i 
                                    <p class="dun__char-stat-number">{character_stats.stat_number}</p>
                                </li>
                        }
                    }
                    />
                </ul>

    }
}