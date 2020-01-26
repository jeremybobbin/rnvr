use neovim_lib::{
    Neovim,
    NeovimApi,
    Session
};

use std::{
    env::var,
    env::args,
};

fn main() {

    let socket = var("NVIM_LISTEN_ADDRESS")
        .expect("set NVIM_LISTEN_ADDRESS environment variable");

    let mut session = Session::new_unix_socket(socket).unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);

    let cmd = args()
        .skip(1)
        .collect::<Vec<String>>();

    if let Some(action) = cmd.get(0) {
        match action.as_ref() {
            "--remote-send" => {
                nvim.input(&cmd[1..].join(" ")).unwrap();
            },
            "--remote-expr" => {
                nvim.eval(&cmd[1..].join(" ")).unwrap();
            },
            _ => {
                nvim.input(&cmd.join(" ")).unwrap();
            },
        };
    }
}
