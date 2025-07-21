use leptos::*;
use rand::seq::IteratorRandom;
use wasm_bindgen::prelude::*;
use std::time::Duration;

fn check_winner(b: &[Option<&str>]) -> Option<String> {
    let lines = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];
    for [a, b_i, c] in lines {
        if let (Some(x), Some(y), Some(z)) = (b[a], b[b_i], b[c]) {
            if x == y && y == z {
                return Some(x.to_string());
            }
        }
    }
    None
}

#[component]
pub fn App() -> impl IntoView {
    let (board, set_board) = create_signal(vec![None; 9]);
    let (winner, set_winner) = create_signal::<Option<String>>(None);
    let (score, set_score) = create_signal((0, 0, 0));
    let (timer, set_timer) = create_signal(0);

    let is_game_over = move || winner.get().is_some() || !board.get().contains(&None);

    set_interval(
        move || {
            if !is_game_over() {
                set_timer.update(|t| *t += 1);
            }
        },
        Duration::from_secs(1),
    );

    let computer_turn = move || {
        if is_game_over() {
            return;
        }

        let mut b = board.get();
        let mut rng = rand::thread_rng();
        let empty: Vec<usize> = b
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c.is_none() { Some(i) } else { None })
            .collect();

        let bot = "🫐";
        let player = "🍓";

        let strategic = |symbol: &str| -> Option<usize> {
            for &i in &empty {
                let mut test = b.clone();
                test[i] = Some(symbol);
                if check_winner(&test) == Some(symbol.to_string()) {
                    return Some(i);
                }
            }
            None
        };

        let choice = strategic(bot)
            .or_else(|| strategic(player))
            .or_else(|| empty.iter().choose(&mut rng).copied());

        if let Some(i) = choice {
            b[i] = Some(bot);
            set_board.set(b.clone());

            if let Some(w) = check_winner(&b) {
                set_winner.set(Some(w.clone()));
                if w == bot {
                    set_score.update(|(_, l, _)| *l += 1);
                }
            }
        }
    };

    let handle_click = move |i: usize| {
        if is_game_over() || board.get()[i].is_some() {
            return;
        }

        let mut b = board.get();
        b[i] = Some("🍓");
        set_board.set(b.clone());

        if let Some(w) = check_winner(&b) {
            set_winner.set(Some(w.clone()));
            if w == "🍓" {
                set_score.update(|(w, _, _)| *w += 1);
            }
        } else {
            set_timeout(computer_turn, Duration::from_millis(500));
        }
    };

    let restart_game = move |_| {
        set_board.set(vec![None; 9]);
        set_winner.set(None);
        set_timer.set(0);
    };

    view! {
        <div style="font-family: sans-serif; display: flex; flex-direction: column; align-items: center; padding: 1em;">
            <h1 style="font-size: 2em;">"Boundless Berry Game by @sunkanmiAD"</h1>

            <div style="margin: 0.5em 0;">"⏱️ Time: " {move || timer.get()} "s"</div>

            <div style="display: flex; gap: 1em; font-size: 1.1em;">
                <div>"🍓 Wins: " {move || score.get().0}</div>
                <div>"🫐 Losses: " {move || score.get().1}</div>
                <div>"😐 Draws: " {move || score.get().2}</div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(3, 80px); gap: 10px; margin: 1em 0;">
                {
                    move || {
                        board.get()
                            .iter()
                            .enumerate()
                            .map(|(i, cell)| {
                              view! {
                                    <div
                                        on:click=move |_| handle_click(i)
                                        style="
                                            width: 80px;
                                            height: 80px;
                                            display: flex;
                                            align-items: center;
                                            justify-content: center;
                                            font-size: 2em;
                                            background-color: #f9f9f9;
                                            border-radius: 8px;
                                            cursor: pointer;
                                            user-select: none;
                                        "
                                    >
                                        { cell.unwrap_or("") }
                                    </div>
                                }
                            })
                            .collect_view()
                    }
                }
            </div>

            {
                move || match winner.get().as_deref() {
                    Some("🍓") => view! {
                        <div>
                            "🎉 You win!"
                            <br/>
                            <button on:click=restart_game>"🔁 Play Again"</button>
                        </div>
                    },
                    Some("🫐") => view! {
                        <div>
                            "💀 Computer wins!"
                            <br/>
                            <button on:click=restart_game>"🔁 Play Again"</button>
                        </div>
                    },
                    None if !board.get().contains(&None) => {
                        set_score.update(|(_, _, d)| *d += 1);
                        view! {
                            <div>
                                "😶 It's a draw!"
                                <br/>
                                <button on:click=restart_game>"🔁 Play Again"</button>
                            </div>
                        }
                    },
                    _ => view! { <div></div> }
                }
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}