use std::{
    io::{BufReader, Cursor},
    thread,
};

use nvim_oxi::{
    api::{
        self,
        opts::{CreateAutocmdOpts, CreateCommandOpts},
        types::AutocmdCallbackArgs,
    },
    libuv::AsyncHandle,
    Dictionary, Function, Object,
};
use rodio::{Decoder, OutputStream, Sink};
use thiserror::Error as ThisError;
use tokio::sync::mpsc::{self, UnboundedSender};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Libuv(#[from] nvim_oxi::libuv::Error),
    #[error(transparent)]
    MPSCSend(#[from] mpsc::error::SendError<()>),
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

#[nvim_oxi::plugin]
pub fn boom() -> Result<Dictionary, Error> {
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
                let (sender, mut receiver) = mpsc::unbounded_channel::<()>();

                let handle = AsyncHandle::new(move || receiver.blocking_recv().unwrap()).unwrap();

                thread::spawn(move || send_numbers(handle, sender));
            }

            // don't instruct Neovim to delete this autocommand
            false
        })
        .build();

    let autocmd_id = api::create_autocmd(["BufLeave"], &bufleave_opts)?;

    let _ = api::create_user_command(
        "TwitchBan",
        move |_| api::del_autocmd(autocmd_id),
        &CreateCommandOpts::default(),
    );

    Ok(())
}

#[tokio::main]
async fn send_numbers(handle: AsyncHandle, sender: UnboundedSender<()>) -> Result<(), Error> {
    sender.send(())?;
    handle.send()?;

    let (_sink, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let source = Decoder::new(BufReader::new(Cursor::new(ZOOMER_BOOM)))?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
