use std::{
    io::{BufReader, Cursor},
    thread,
};

use nvim_oxi::{
    api::{self, opts::CreateAutocmdOpts},
    libuv::AsyncHandle,
};
#[cfg(debug_assertions)]
use nvim_oxi::{print, schedule};
use rodio::{Decoder, OutputStream, Sink};
use thiserror::Error as ThisError;
use tokio::sync::mpsc::{self, UnboundedSender};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Nvim(#[from] nvim_oxi::Error),
    #[error(transparent)]
    NvimApi(#[from] api::Error),
    #[error(transparent)]
    Libuv(#[from] nvim_oxi::libuv::Error),
    #[error(transparent)]
    RodioDecoder(#[from] rodio::decoder::DecoderError),
    #[error(transparent)]
    RodioPlay(#[from] rodio::PlayError),
    #[error(transparent)]
    RodioStream(#[from] rodio::StreamError),
}

const ZOOMER_BOOM: &[u8; 28118] = include_bytes!("../assets/boom.ogg");

#[nvim_oxi::plugin]
pub fn boom() -> Result<(), Error> {
    let bufleave_opts = CreateAutocmdOpts::builder()
        .patterns(["*"])
        .callback(|_| {
            let ft: String = api::get_current_buf()
                .get_option("filetype")
                .unwrap_or_default();

            if ft == "TelescopePrompt" {
                let (sender, mut receiver) = mpsc::unbounded_channel::<()>();

                let handle = AsyncHandle::new(move || {
                    receiver.blocking_recv().unwrap();
                    #[cfg(debug_assertions)]
                    schedule(move |_| print!("Boom."));
                })
                .unwrap();

                let _ = thread::spawn(move || send_numbers(handle, sender));
            }

            // don't instruct Neovim to delete this autocommand
            false
        })
        .build();

    api::create_autocmd(["BufLeave"], &bufleave_opts)?;

    Ok(())
}

#[tokio::main]
async fn send_numbers(handle: AsyncHandle, sender: UnboundedSender<()>) {
    sender.send(()).unwrap();
    handle.send().unwrap();

    let (_sink, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new(BufReader::new(Cursor::new(ZOOMER_BOOM))).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
