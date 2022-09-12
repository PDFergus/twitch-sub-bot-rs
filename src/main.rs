use winapi::um::winuser::BlockInput;
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::{IRCMessage, ServerMessage};
use std::process::Command;
use winput::{Vk, Button};
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use tokio::time::{sleep, Duration};



//mod recorder;
#[tokio::main]
pub async fn main() {
    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
//joins with anon credentials
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message{
                ServerMessage::Privmsg(msg) =>{
                    
                    for badge in msg.badges{
                        for word in msg.message_text.split(" "){
                           // tokio::spawn(async move{
                             //   let new_word = String::from( word);
                               // play_sound(new_word).await;
                            //});

                            if badge.name == String::from("subscriber")||badge.name == String::from("founder"){
                                
                                if word == "!minimize"{
                                    tokio::spawn(async move{
                                        desktop_view().await;

                                    });
                                    sleep(Duration::from_millis(0)).await;
                                    
                                    
                                }
                                else if word == "!stay"{
                                    tokio::spawn(async move{
                                        key_send_looper().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                    
                                }
                                else if word == "!yell"{
                                    tokio::spawn(async move{
                                        play_yell().await
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                    
                                }
                                else if word == "!nope"{
                                    tokio::spawn(async move{
                                        unsafe{
                                            block_input().await
                                        }
                                    });
                                    sleep(Duration::from_millis(10)).await;
                                }
                                
                           
                                
                            } 
                            
                        }
                        
                    }
                   
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
    });

    client.join("channelnamehere".to_owned()).unwrap();//change the string to channel of choice

    // keep the tokio executor alive.
    join_handle.await.unwrap();
}

async fn play_yell(){
    sleep(time::Duration::from_millis(10)).await;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open("sounds/angryscav.mp3").unwrap());
    let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    
    sink.append(source);
    
    sink.sleep_until_end();
}

//async fn play_sound(word: String){

  //  sleep(time::Duration::from_millis(10)).await;
    //let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //let sink = Sink::try_new(&stream_handle).unwrap();
    //let file = BufReader::new(File::open("sounds/angryscav.mp3").unwrap());
    //let source= Decoder::new(file).unwrap();
    //stream_handle.play_raw(source.convert_samples());
    
    //sink.append(source);
    //
    //sink.sleep_until_end();
//}


async fn desktop_view(){
    
    winput::press(Vk::LeftWin);
    winput::send(Vk::D);
    winput::release(Vk::LeftWin);
}

async fn key_send_looper(){

    sleep(time::Duration::from_millis(10)).await;
    let mut start:u32 = 0;
    let end:u32 = 333;
    while start < end{
        winput::press(Vk::A);
        winput::press(Vk::W);
        winput::press(Vk::S);
        winput::press(Vk::D);
        sleep(time::Duration::from_millis(30)).await;
        start +=1;
    }
    winput::release(Vk::A);
    winput::release(Vk::W);
    winput::release(Vk::S);
    winput::release(Vk::D);
}

async unsafe fn block_input(){
    
    BlockInput(1);
    sleep(time::Duration::from_millis(5)).await;
    BlockInput(0);
    
    
}

//async fn sound_commands(word: &str){
  //  let word = word;
   // println!("(> . . )>{}", word);
//}

