use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, row, text_input};
use iced::{event, window, Command, Element, Length, Subscription};

pub fn main() -> iced::Result {
    iced::program("Player", Player::update, Player::view)
        .subscription(Player::subscription)
        .exit_on_close_request(false)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Video(String),
    Play,
    Stop,
    Tick,
    Nothing,
}

#[derive(Debug, Default)]
struct Player {
    video: PathBuf,
    frame: Option<Handle>,
    playing: bool,
    sender: Option<Sender<Option<usize>>>,
    receiver: Option<Receiver<Vec<u8>>>,
}

impl Player {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Video(path) => self.video = PathBuf::from(path),
            Message::Play => {
                self.playing = !self.playing;
                if let Some(sender) = self.sender.take() {
                    sender.send(None).unwrap();
                }
                if self.video.exists() && self.playing {
                    let (tick_send, tick_recv) = channel();
                    let (frame_send, frame_recv) = channel();
                    let path = self.video.clone();

                    self.sender = Some(tick_send);
                    self.receiver = Some(frame_recv);
                    return Command::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                decode_file(&path, frame_send, tick_recv).unwrap()
                            })
                            .await
                        },
                        |_| Message::Nothing,
                    );
                }
            }
            Message::Stop => {
                if self.playing {
                    self.playing = false;
                }
                if let Some(sender) = self.sender.take() {
                    sender.send(None).unwrap();
                }
                return window::close(iced::window::Id::MAIN);
            }
            Message::Tick => {
                if self.playing {
                    if let Some(sender) = self.sender.as_ref() {
                        sender.send(Some(1)).unwrap();
                    }
                    if let Some(receiver) = self.receiver.as_ref() {
                        self.frame = Some(Handle::from_bytes(receiver.recv().unwrap()));
                    }
                }
            }
            Message::Nothing => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text_input("Path of video file", &format!("{}", self.video.display()))
                    .on_input(Message::Video)
                    .on_paste(Message::Video)
                    .on_submit(Message::Play),
                button("Play").on_press(Message::Play)
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fill),
            if let Some(handle) = self.frame.as_ref() {
                container(image(handle.clone()))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            } else {
                container("Ready")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            }
        ]
        .padding(10)
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.playing {
            let tick = iced::time::every(Duration::from_secs_f64(1. / 24.)).map(|_| Message::Tick);
            let key = event::listen_with(|e, _| match e {
                iced::Event::Window(_, iced::window::Event::CloseRequested) => Some(Message::Stop),
                _ => Some(Message::Nothing),
            });

            Subscription::batch(vec![key, tick])
        } else {
            Subscription::none()
        }
    }
}

pub fn decode_file(
    path: &Path,
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Option<usize>>,
) -> Result<(), ffmpeg_next::Error> {
    use ffmpeg_next::format::input;
    use ffmpeg_next::format::Pixel;
    use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
    use ffmpeg_next::util::frame::video::Video;
    use ffmpeg_next::Rational;

    let mut ictx = input(path)?;
    let input = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let video_stream_index = input.index();
    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;
    let codec = ffmpeg_next::codec::encoder::find(ffmpeg_next::codec::Id::PNG).unwrap();
    let context_decoder = ffmpeg_next::codec::context::Context::new_with_codec(codec);
    let mut png_encoder = context_decoder.encoder().video()?;

    png_encoder.set_width(decoder.width());
    png_encoder.set_height(decoder.height());
    png_encoder.set_frame_rate(decoder.frame_rate());
    png_encoder.set_time_base(Rational::new(24, 1));
    png_encoder.set_format(Pixel::RGB24);
    png_encoder.set_max_b_frames(1);

    let mut png_encoder = png_encoder.open()?;
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;
    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg_next::decoder::Video| -> Result<bool, ffmpeg_next::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                let mut png_frame = ffmpeg_next::Packet::empty();
                let mut png_data = vec![];

                scaler.run(&decoded, &mut rgb_frame)?;
                png_encoder.send_frame(&rgb_frame)?;
                while png_encoder.receive_packet(&mut png_frame).is_ok() {
                    png_frame.set_stream(0);
                    if let Some(data) = png_frame.data() {
                        png_data.extend_from_slice(data);
                    }
                }

                if let Ok(Some(_index)) = receiver.recv() {
                    sender.send(png_data).unwrap();
                } else {
                    println!("jump out decode loop");
                    return Ok(false);
                }
                frame_index += 1;
            }
            Ok(true)
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            if !receive_and_process_decoded_frames(&mut decoder)? {
                return Ok(());
            }
        }
    }
    decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut decoder)?;
    Ok(())
}
