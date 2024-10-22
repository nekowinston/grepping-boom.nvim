use std::{
    cell::RefCell,
    io::{BufReader, Cursor},
    rc::Rc,
    thread,
};

use nvim_oxi::{
    api::{
        self,
        opts::{CreateAutocmdOpts, CreateCommandOpts},
        types::{AutocmdCallbackArgs, CommandArgs, CommandNArgs},
    },
    Dictionary, Function, Object,
};
use rodio::{Decoder, OutputStream, Sink};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
enum Error {
    #[error(transparent)]
    Nvim(#[from] nvim_oxi::Error),
    #[error(transparent)]
    NvimApi(#[from] api::Error),
    #[error(transparent)]
    RodioDecoder(#[from] rodio::decoder::DecoderError),
    #[error(transparent)]
    RodioPlay(#[from] rodio::PlayError),
    #[error(transparent)]
    RodioStream(#[from] rodio::StreamError),
}

const ZOOMER_BOOM: &[u8; 28118] = include_bytes!("../assets/boom.ogg");

thread_local! {
    static MAX_VOLUME: Rc<RefCell<f32>> = Rc::new((1.0).into());
}

#[nvim_oxi::plugin]
fn boom() -> Result<Dictionary, Error> {
    let setup_fn: Function<Object, Result<(), Error>> = Function::from_fn(setup);

    Ok(Dictionary::from_iter([("setup", setup_fn)]))
}

fn setup(_: Object) -> Result<(), Error> {
    let bufleave_opts = CreateAutocmdOpts::builder()
        .patterns(["*"])
        .callback(|cmd_args: AutocmdCallbackArgs| {
            let in_telescope_prompt = cmd_args
                .buffer
                .get_option::<String>("filetype")
                .unwrap_or_default()
                .eq("TelescopePrompt");

            if in_telescope_prompt {
                let vol = MAX_VOLUME.with(|f| *f.borrow());
                thread::spawn(move || play_boom(vol));
            }

            // don't instruct Neovim to delete this autocommand
            false
        })
        .build();

    let mut autocmd_id = api::create_autocmd(["BufLeave"], &bufleave_opts)?;

    let volume_opts = CreateCommandOpts::builder()
        .nargs(CommandNArgs::One)
        .build();

    api::create_user_command(
        "BoomVolume",
        move |args: CommandArgs| -> Result<(), Error> {
            if let Some(args) = args.args {
                if let Ok(volume) = args.parse::<f32>() {
                    MAX_VOLUME.with(|f| *f.borrow_mut() = volume);
                }
            }
            Ok(())
        },
        &volume_opts,
    )?;

    api::create_user_command(
        "TwitchBan",
        move |_| -> Result<(), Error> {
            if (autocmd_id) != 0 {
                api::del_autocmd(autocmd_id)?;
                autocmd_id = 0;
            }
            Ok(())
        },
        &CreateCommandOpts::default(),
    )?;

    Ok(())
}

#[tokio::main]
async fn play_boom(volume: f32) -> Result<(), Error> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let source = Decoder::new(BufReader::new(Cursor::new(ZOOMER_BOOM)))?;
    sink.set_volume(volume);
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
